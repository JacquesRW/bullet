use crate::util::sigmoid;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct MarlinFormat {
    occ: u64,
    pcs: [u8; 16],
    stm_enp: u8,
    hfm: u8,
    fmc: u16,
    score: i16,
    result: u8,
    extra: u8,
}

impl MarlinFormat {
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
        let (wdl, score) = if self.stm() == 1 {
            (1.0 - self.result(), -self.score)
        } else {
            (self.result(), self.score)
        };
        blend * wdl + (1. - blend) * sigmoid(f32::from(score), scale)
    }

    pub fn stm(&self) -> usize {
        usize::from(self.stm_enp >> 7)
    }
}

impl IntoIterator for MarlinFormat {
    type Item = (u8, u8, u8);
    type IntoIter = MarlinFormatIter;
    fn into_iter(self) -> Self::IntoIter {
        MarlinFormatIter {
            board: self,
            idx: 0,
        }
    }
}

pub struct MarlinFormatIter {
    board: MarlinFormat,
    idx: usize,
}

impl Iterator for MarlinFormatIter {
    type Item = (u8, u8, u8);
    fn next(&mut self) -> Option<Self::Item> {
        if self.board.occ == 0 {
            return None;
        }

        let square = self.board.occ.trailing_zeros() as u8;
        let coloured_piece = (self.board.pcs[self.idx / 2] >> (4 * (self.idx & 1))) & 0b1111;

        let mut piece = coloured_piece & 0b111;
        if piece == 6 {
            piece = 3;
        }

        let colour = coloured_piece >> 3;

        self.board.occ &= self.board.occ - 1;
        self.idx += 1;

        Some((colour, piece, square))
    }
}