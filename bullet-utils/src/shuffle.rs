use std::{
    env,
    fs::{self, File},
    io::{BufWriter, Read, Result, Write},
    path::PathBuf,
    time::Instant,
};

use bullet::{util, Rand};
use bulletformat::ChessBoard;
use structopt::StructOpt;

use crate::interleave::InterleaveOptions;

#[derive(StructOpt)]
pub struct ShuffleOptions {
    #[structopt(required = true, short, long)]
    pub input: PathBuf,
    #[structopt(required = true, short, long)]
    pub output: PathBuf,
    #[structopt(required = true, short, long)]
    pub mem_used_mb: usize,
}

const CHESS_BOARD_SIZE: usize = std::mem::size_of::<ChessBoard>();
const MIN_TMP_FILES: usize = 4;

impl ShuffleOptions {
    pub fn run(&self) {
        let input_size = fs::metadata(self.input.clone())
            .expect("Input file is valid")
            .len() as usize;
        let num_tmp_files = (input_size / (self.mem_used_mb * 1000) + 1).max(MIN_TMP_FILES);

        let temp_dir = env::temp_dir();

        let mut temp_files = (0..num_tmp_files)
            .map(|idx| {
                let output_file = format!("{}/part_{}.bin", temp_dir.to_str().unwrap(), idx + 1);
                // File::create(output_file).unwrap()
                PathBuf::from(output_file)
            })
            .collect::<Vec<_>>();

        println!("# [Shuffling Data]");
        let time = Instant::now();
        assert!(self.split_file(&mut temp_files, input_size).is_ok());

        println!("# [Finished splitting data. Shuffling...]");
        let interleave = InterleaveOptions::new(temp_files.to_vec(), self.output.clone());
        interleave.run();

        println!("> Took {:.2} seconds.", time.elapsed().as_secs_f32());
    }

    fn split_file(&self, temp_files: &mut [PathBuf], input_size: usize) -> Result<()> {
        let mut input = File::open(self.input.clone()).unwrap();
        let mut temp_files = temp_files
            .iter()
            .map(|f| File::create(f).expect("Tmp file could not be created."))
            .collect::<Vec<_>>();

        let buff_size = self.actual_buffer_size(temp_files.len(), input_size);

        for file in temp_files.iter_mut() {
            let mut buffer = vec![0u8; buff_size];
            let bytes_read = input.read(&mut buffer)?;

            let data = util::to_slice_with_lifetime_mut(&mut buffer[0..bytes_read]);
            shuffle_positions(data);
            let data_slice = util::to_slice_with_lifetime(data);
            assert_eq!(0, bytes_read % CHESS_BOARD_SIZE);

            let mut writer = BufWriter::new(file);
            assert!(writer.write(&data_slice[0..bytes_read]).is_ok());
        }

        Ok(())
    }

    /// Input size should be in bytes
    fn actual_buffer_size(&self, num_tmp_files: usize, input_size: usize) -> usize {
        input_size / num_tmp_files / CHESS_BOARD_SIZE * CHESS_BOARD_SIZE + CHESS_BOARD_SIZE
    }
}

fn shuffle_positions(data: &mut [ChessBoard]) {
    let mut rng = Rand::default();

    for i in (0..data.len()).rev() {
        let idx = rng.rand_int() as usize % (i + 1);
        data.swap(idx, i);
    }
}
