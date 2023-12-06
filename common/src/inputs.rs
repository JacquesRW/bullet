use crate::data::{AtaxxBoard, ChessBoard, DataType};

pub trait InputType {
    type RequiredDataType: DataType;
    const BUCKETS: usize;

    const SIZE: usize = Self::RequiredDataType::INPUTS * Self::BUCKETS;

    fn get_feature_indices(
        feat: <Self::RequiredDataType as DataType>::FeatureType,
    ) -> (usize, usize);
}

#[rustfmt::skip]
const FLIP: [usize; 49] = [
    42, 43, 44, 45, 46, 47, 48,
    35, 36, 37, 38, 39, 40, 41,
    28, 29, 30, 31, 32, 33, 34,
    21, 22, 23, 24, 25, 26, 27,
    14, 15, 16, 17, 18, 19, 20,
     7,  8,  9, 10, 11, 12, 13,
     0,  1,  2,  3,  4,  5,  6,
];

pub struct Ataxx147;
impl InputType for Ataxx147 {
    type RequiredDataType = AtaxxBoard;
    const BUCKETS: usize = 1;

    fn get_feature_indices(
        (piece, square): <Self::RequiredDataType as DataType>::FeatureType,
    ) -> (usize, usize) {
        let pc = 49 * usize::from(piece);
        let sq = usize::from(square);
        (pc + sq, pc + FLIP[sq])
    }
}

pub struct Chess768;
impl InputType for Chess768 {
    type RequiredDataType = ChessBoard;
    const BUCKETS: usize = 1;

    fn get_feature_indices(
        (piece, square, _, _): <Self::RequiredDataType as DataType>::FeatureType,
    ) -> (usize, usize) {
        let c = usize::from(piece & 8 > 0);
        let pc = 64 * usize::from(piece & 7);
        let sq = usize::from(square);
        let wfeat = [0, 384][c] + pc + sq;
        let bfeat = [384, 0][c] + pc + (sq ^ 56);
        (wfeat, bfeat)
    }
}

pub struct ChessBuckets;
impl ChessBuckets {
    const BUCKETING: [usize; 64] = crate::BUCKETS;

    const SCALED: [usize; 64] = {
        let mut idx = 0;
        let mut ret = [0; 64];
        while idx < 64 {
            ret[idx] = 768 * Self::BUCKETING[idx];
            idx += 1;
        }
        ret
    };
}

impl InputType for ChessBuckets {
    type RequiredDataType = ChessBoard;
    const BUCKETS: usize = {
        let mut idx = 0;
        let mut max = 1;
        while idx < 64 {
            let val = Self::BUCKETING[idx];
            if val > max {
                max = val;
            }
            idx += 1;
        }
        max + 1
    };

    fn get_feature_indices(
        (piece, square, our_ksq, opp_ksq): <Self::RequiredDataType as DataType>::FeatureType,
    ) -> (usize, usize) {
        let c = usize::from(piece & 8 > 0);
        let pc = 64 * usize::from(piece & 7);
        let sq = usize::from(square);
        let wfeat = Self::SCALED[usize::from(our_ksq)] + [0, 384][c] + pc + sq;
        let bfeat = Self::SCALED[usize::from(opp_ksq)] + [384, 0][c] + pc + (sq ^ 56);
        (wfeat, bfeat)
    }
}
