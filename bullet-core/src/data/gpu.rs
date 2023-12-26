use crate::{
    data::{BulletFormat, MAX_FEATURES},
    inputs::InputType,
    Data, Input,
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoardCUDA {
    features: [u16; MAX_FEATURES],
}

impl Default for BoardCUDA {
    fn default() -> Self {
        Self {
            features: [0; MAX_FEATURES],
        }
    }
}

impl BoardCUDA {
    pub fn len() -> usize {
        MAX_FEATURES
    }

    pub fn push(
        board: &Data,
        our_inputs: &mut Vec<BoardCUDA>,
        opp_inputs: &mut Vec<BoardCUDA>,
        results: &mut Vec<f32>,
        blend: f32,
        scale: f32,
    ) {
        let mut our_board = BoardCUDA::default();
        let mut opp_board = BoardCUDA::default();

        let mut i = 0;

        for feat in board.into_iter() {
            let (wfeat, bfeat) = Input::get_feature_indices(feat);

            our_board.features[i] = wfeat as u16;
            opp_board.features[i] = bfeat as u16;
            i += 1;
        }

        if i < MAX_FEATURES {
            our_board.features[i] = u16::MAX;
            opp_board.features[i] = u16::MAX;
        }

        our_inputs.push(our_board);
        opp_inputs.push(opp_board);
        results.push(board.blended_result(blend, scale));
    }
}