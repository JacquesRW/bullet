use crate::{position::{Features, Position}, network::{Accumulator, NNUEParams}};

pub trait InputType {
    type DataType;
    const SIZE: usize;

    fn update_features_and_accumulator(
        pos: &Self::DataType,
        stm: usize,
        features: &mut Features,
        accs: &mut [Accumulator; 2],
        nnue: &NNUEParams,
    );
}

pub struct Chess768;

impl InputType for Chess768 {
    type DataType = Position;
    const SIZE: usize = 768;

    #[inline]
    fn update_features_and_accumulator(
        pos: &Self::DataType,
        stm: usize,
        features: &mut Features,
        accs: &mut [Accumulator; 2],
        nnue: &NNUEParams
    ) {
        let opp = stm ^ 1;
        for (colour, piece, square) in pos.into_iter() {
            let c = usize::from(colour);
            let pc = 64 * usize::from(piece);
            let sq = usize::from(square);
            let wfeat = [0, 384][c] + pc + sq;
            let bfeat = [384, 0][c] + pc + (sq ^ 56);

            features.push(wfeat, bfeat);
            accs[stm].add_feature(wfeat, nnue);
            accs[opp].add_feature(bfeat, nnue);
        }
    }
}