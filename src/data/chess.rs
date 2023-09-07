use crate::util::sigmoid;

use super::marlinformat::MarlinFormat;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ChessBoard {
    occ: u64,
    pcs: [u8; 16],
    score: i16,
    result: u8,
    ksq: u8,
}

// just in case
const _RIGHT_SIZE: () = assert!(std::mem::size_of::<ChessBoard>() == 32);

impl ChessBoard {
    pub fn score(&self) -> i16 {
        self.score
    }

    pub fn result(&self) -> f32 {
        f32::from(self.result) / 2.
    }

    pub fn result_idx(&self) -> usize {
        usize::from(self.result)
    }

    pub fn blended_result(&self, blend: f32, scale: f32) -> f32 {
        blend * self.result() + (1. - blend) * sigmoid(f32::from(self.score), scale)
    }

    pub fn from_epd(fen: &str) -> Result<Self, String> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        let board_str = parts[0];
        let stm_str = parts[1];

        let stm = u8::from(stm_str == "b");

        let mut occ = 0;
        let mut pcs = [0; 16];

        let mut idx = 0;

        let mut ksq = 0;

        let mut parse_row = |i: usize, row: &str| {
            let mut col = 0;
            for ch in row.chars() {
                if ('1'..='8').contains(&ch) {
                    col += ch.to_digit(10).expect("hard coded") as usize;
                } else if let Some(mut piece) = "PNBRQKpnbrqk".chars().position(|el| el == ch) {
                    let mut square = 8 * i + col;

                    piece = (piece / 6) << 3 | (piece % 6);

                    // black to move
                    if stm == 1 {
                        piece ^= 8;
                        square ^= 56;
                    }

                    if piece == 5 {
                        ksq = square as u8;
                    }

                    occ |= 1 << square;
                    pcs[idx / 2] |= (piece as u8) << (4 * (idx & 1));
                    idx += 1;
                    col += 1;
                }
            }
        };

        if stm == 1 {
            for (i, row) in board_str.split('/').enumerate() {
                parse_row(7 - i, row);
            }
        } else {
            for (i, row) in board_str.split('/').rev().enumerate() {
                parse_row(i, row);
            }
        }

        let mut score = parts[6].parse::<i16>().unwrap_or(0);

        let mut result = match parts[7] {
            "[1.0]" => 2,
            "[0.5]" => 1,
            "[0.0]" => 0,
            _ => {
                println!("{fen}");
                return Err(String::from("Bad game result!"));
            }
        };

        if stm == 1 {
            score = -score;
            result = 2 - result;
        }

        Ok(Self {
            occ,
            pcs,
            score,
            result,
            ksq,
        })
    }

    pub fn from_marlinformat(mf: &MarlinFormat) -> Self {
        let mut board = Self::default();

        let stm = mf.stm();

        if stm == 1 {
            board.score = -mf.score();
            board.result = 2 - mf.result_idx() as u8;
        } else {
            board.score = mf.score();
            board.result = mf.result_idx() as u8;
        }

        let mut features = [(0, 0); 32];
        let mut fidx = 0;

        for (colour, mut piece, mut square) in mf.into_iter() {
            piece |= colour << 3;

            if stm == 1 {
                piece ^= 8;
                square ^= 56;
            }

            if piece == 5 {
                board.ksq = square;
            }

            features[fidx] = (piece, square);
            fidx += 1;
        }

        features[..fidx].sort_by_key(|feat| feat.1);

        for (idx, (piece, square)) in features.iter().enumerate().take(fidx) {
            board.occ |= 1 << square;
            board.pcs[idx / 2] |= piece << (4 * (idx & 1));
        }

        board
    }
}

impl IntoIterator for ChessBoard {
    type Item = (u8, u8, u8, u8);
    type IntoIter = BoardIter;
    fn into_iter(self) -> Self::IntoIter {
        BoardIter {
            board: self,
            idx: 0,
        }
    }
}

pub struct BoardIter {
    board: ChessBoard,
    idx: usize,
}

impl Iterator for BoardIter {
    type Item = (u8, u8, u8, u8);
    fn next(&mut self) -> Option<Self::Item> {
        if self.board.occ == 0 {
            return None;
        }

        let square = self.board.occ.trailing_zeros() as u8;
        let mut piece = (self.board.pcs[self.idx / 2] >> (4 * (self.idx & 1))) & 0b1111;

        let colour = u8::from(piece & 8 > 0);
        piece &= 7;

        self.board.occ &= self.board.occ - 1;
        self.idx += 1;

        Some((colour, piece, square, self.board.ksq))
    }
}
