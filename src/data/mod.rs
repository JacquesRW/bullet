pub mod chess;
pub mod marlinformat;

pub use chess::ChessBoard;

use crate::{network::InputType, Data, Input};

const MAX_FEATURES: usize = Data::MAX_FEATURES * (1 + Input::FACTORISER as usize);

pub trait DataType {
    type FeatureType;
    const INPUTS: usize;
}

pub struct Features {
    features: [(usize, usize); MAX_FEATURES],
    len: usize,
    consumed: usize,
}

impl Default for Features {
    fn default() -> Self {
        Self {
            features: [(0, 0); MAX_FEATURES],
            len: 0,
            consumed: 0,
        }
    }
}

impl Features {
    pub fn push(&mut self, wfeat: usize, bfeat: usize) {
        self.features[self.len] = (wfeat, bfeat);
        self.len += 1;
    }
}

impl Iterator for Features {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.consumed == self.len {
            return None;
        }

        let ret = self.features[self.consumed];

        self.consumed += 1;

        Some(ret)
    }
}

#[cfg(test)]
mod test {
    use super::{marlinformat::MarlinFormat, *};

    #[test]
    fn working_conversion() {
        let board = ChessBoard::from_epd(
            "r2k3r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b - - 0 1 5 [1.0]",
        )
        .unwrap();
        let mf = MarlinFormat::from_epd(
            "r2k3r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b - - 0 1 5 [1.0]",
        )
        .unwrap();
        let mf_board = ChessBoard::from_marlinformat(&mf);

        println!("{mf:?}");
        println!("{board:?}");
        println!("{mf_board:?}");

        assert_eq!(board, mf_board);
    }
}
