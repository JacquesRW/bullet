use std::{
    sync::mpsc,
    time::{SystemTime, UNIX_EPOCH},
};

use sfbinpack::{
    chess::{color::Color, piecetype::PieceType},
    data_entry::TrainingDataEntry,
    reader::data_reader::CompressedTrainingDataEntryReader,
};

use crate::{format::ChessBoard, loader::DataLoader};

fn convert_to_bulletformat(entry: &TrainingDataEntry) -> ChessBoard {
    let mut bbs = [0; 8];

    let stm = usize::from(entry.pos.side_to_move().ordinal());
    let pc_bb =
        |pt| entry.pos.pieces_bb_color(Color::Black, pt).bits() | entry.pos.pieces_bb_color(Color::White, pt).bits();

    bbs[0] = entry.pos.pieces_bb(Color::White).bits();
    bbs[1] = entry.pos.pieces_bb(Color::Black).bits();
    bbs[2] = pc_bb(PieceType::Pawn);
    bbs[3] = pc_bb(PieceType::Knight);
    bbs[4] = pc_bb(PieceType::Bishop);
    bbs[5] = pc_bb(PieceType::Rook);
    bbs[6] = pc_bb(PieceType::Queen);
    bbs[7] = pc_bb(PieceType::King);

    let score = entry.score;
    let result = f32::from(entry.result) / 2.0;

    ChessBoard::from_raw(bbs, stm, score, result).expect("Binpack must be malformed!")
}

#[derive(Clone)]
pub struct SfBinpackLoader {
    file_path: [String; 1],
    buffer_size: usize,
}

impl SfBinpackLoader {
    pub fn new(path: &str, buffer_size_mb: usize) -> Self {
        Self {
            file_path: [path.to_string(); 1],
            buffer_size: buffer_size_mb * 1024 * 1024 / std::mem::size_of::<ChessBoard>() / 2,
        }
    }
}

impl DataLoader<ChessBoard> for SfBinpackLoader {
    fn data_file_paths(&self) -> &[String] {
        &self.file_path
    }

    fn count_positions(&self) -> Option<u64> {
        None
    }

    fn map_batches<F: FnMut(&[ChessBoard]) -> bool>(&self, batch_size: usize, mut f: F) {
        let (buffer_sender, buffer_receiver) = mpsc::sync_channel::<Vec<ChessBoard>>(0);
        let (buffer_msg_sender, buffer_msg_receiver) = mpsc::sync_channel::<bool>(1);

        let file_path = self.file_path[0].clone();
        let buffer_size = self.buffer_size;

        std::thread::spawn(move || {
            let mut shuffle_buffer = Vec::with_capacity(buffer_size);

            'dataloading: loop {
                let mut reader = CompressedTrainingDataEntryReader::new(&file_path).unwrap();

                while reader.has_next() {
                    let entry = reader.next();
                    shuffle_buffer.push(convert_to_bulletformat(&entry));

                    if shuffle_buffer.len() == buffer_size {
                        shuffle(&mut shuffle_buffer);

                        if buffer_msg_receiver.try_recv().unwrap_or(false)
                            || buffer_sender.send(shuffle_buffer).is_err()
                        {
                            break 'dataloading;
                        }

                        shuffle_buffer = Vec::with_capacity(buffer_size);
                    }
                }
            }
        });

        let (batch_sender, batch_reciever) = mpsc::sync_channel::<Vec<ChessBoard>>(16);
        let (batch_msg_sender, batch_msg_receiver) = mpsc::sync_channel::<bool>(1);

        std::thread::spawn(move || {
            'dataloading: while let Ok(shuffle_buffer) = buffer_receiver.recv() {
                for batch in shuffle_buffer.chunks(batch_size) {
                    if batch_msg_receiver.try_recv().unwrap_or(false) || batch_sender.send(batch.to_vec()).is_err() {
                        buffer_msg_sender.send(true).unwrap();
                        break 'dataloading;
                    }
                }
            }
        });

        'dataloading: while let Ok(inputs) = batch_reciever.recv() {
            for batch in inputs.chunks(batch_size) {
                let should_break = f(batch);

                if should_break {
                    batch_msg_sender.send(true).unwrap();
                    break 'dataloading;
                }
            }
        }

        drop(batch_reciever);
    }
}

fn shuffle(data: &mut [ChessBoard]) {
    let mut rng = Rand::with_seed();

    for i in (0..data.len()).rev() {
        let idx = rng.rng() as usize % (i + 1);
        data.swap(idx, i);
    }
}

pub struct Rand(u64);

impl Rand {
    pub fn with_seed() -> Self {
        let seed = SystemTime::now().duration_since(UNIX_EPOCH).expect("Guaranteed increasing.").as_micros() as u64
            & 0xFFFF_FFFF;

        Self(seed)
    }

    pub fn rng(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}
