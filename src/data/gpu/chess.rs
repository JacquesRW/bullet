use crate::{
    data::{cpu::chess::ChessBoard, InputType, MAX_FEATURES},
    Input,
};


#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChessBoardCUDA {
    features: [u16; MAX_FEATURES],
}

impl Default for ChessBoardCUDA {
    fn default() -> Self {
        Self { features: [0; MAX_FEATURES] }
    }
}

impl ChessBoardCUDA {
    pub fn push(
        board: &ChessBoard,
        our_inputs: &mut Vec<ChessBoardCUDA>,
        opp_inputs: &mut Vec<ChessBoardCUDA>,
        results: &mut Vec<f32>,
        blend: f32,
        scale: f32,
    ) {
        assert!(std::any::type_name::<Input>() == "Chess768", "Only Chess768 is currently supported!");

        let mut our_board = ChessBoardCUDA::default();
        let mut opp_board = ChessBoardCUDA::default();

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