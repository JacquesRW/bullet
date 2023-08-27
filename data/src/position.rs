use crate::util::sigmoid;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Position {
    occ: u64,
    pcs: [u8; 16],
    stm_enp: u8,
    hfm: u8,
    fmc: u16,
    score: i16,
    result: u8,
    extra: u8,
}

impl Position {
    pub fn score(&self) -> i16 {
        self.score
    }

    pub fn result(&self) -> f64 {
        f64::from(self.result) / 2.
    }

    pub fn result_idx(&self) -> usize {
        usize::from(self.result)
    }

    pub fn blended_result(&self, blend: f64) -> f64 {
        blend * self.result() + (1. - blend) * sigmoid(f64::from(self.score), 0.009)
    }

    pub fn stm(&self) -> usize {
        usize::from(self.stm_enp >> 7)
    }

    pub fn from_epd(fen: &str) -> Result<Self, String> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        let board_str = parts[0];
        let stm_str = parts[1];

        let mut pos = Self::default();

        let mut idx = 0;
        for (i, row) in board_str.split('/').rev().enumerate() {
            let mut col = 0;
            for ch in row.chars() {
                if ('1'..='8').contains(&ch) {
                    col += ch.to_digit(10).expect("hard coded") as usize;
                } else if let Some(piece) = "PNBRQKpnbrqk".chars().position(|el| el == ch) {
                    let square = 8 * i + col;
                    pos.occ |= 1 << square;
                    pos.pcs[idx / 2] |= (piece as u8) << (4 * (idx & 1));
                    idx += 1;
                    col += 1;
                }
            }
        }

        // don't currently worry about en passant square
        pos.stm_enp = u8::from(stm_str == "b") << 7;

        pos.hfm = parts[4].parse().unwrap_or(0);

        pos.fmc = parts[5].parse().unwrap_or(1);

        pos.score = parts[6].parse::<i16>().unwrap_or(0);

        pos.result = match parts[7] {
            "[1.0]" => 2,
            "[0.5]" => 1,
            "[0.0]" => 0,
            _ => {
                println!("{fen}");
                return Err(String::from("Bad game result!"));
            }
        };

        Ok(pos)
    }
}

impl IntoIterator for Position {
    type IntoIter = BoardIter;
    type Item = (u8, u8);
    fn into_iter(self) -> Self::IntoIter {
        BoardIter {
            board: self,
            idx: 0,
        }
    }
}

pub struct BoardIter {
    board: Position,
    idx: usize,
}

impl Iterator for BoardIter {
    type Item = (u8, u8);
    fn next(&mut self) -> Option<Self::Item> {
        if self.board.occ == 0 {
            return None;
        }

        let square = self.board.occ.trailing_zeros() as u8;
        let piece = (self.board.pcs[self.idx / 2] >> (4 * (self.idx & 1))) & 0b1111;

        self.board.occ &= self.board.occ - 1;
        self.idx += 1;

        Some((piece, square))
    }
}

#[test]
fn test_parse() {
    let pos = Position::from_epd(
        "r1bq1bnr/pppp1kp1/2n1p3/5N1p/1PP5/8/P2PPPPP/RNBQKB1R w - - 0 1 55 [1.0]",
    )
    .unwrap();

    let pieces = [
        "WHITE PAWN",
        "WHITE KNIGHT",
        "WHITE BISHOP",
        "WHITE ROOK",
        "WHITE QUEEN",
        "WHITE KING",
        "BLACK PAWN",
        "BLACK KNIGHT",
        "BLACK BISHOP",
        "BLACK ROOK",
        "BLACK QUEEN",
        "BLACK KING",
    ];

    let files = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

    for (piece, square) in pos {
        let pc = pieces[piece as usize];
        let sq = format!("{}{}", files[square as usize % 8], 1 + square / 8);
        println!("{pc}: {sq}")
    }

    println!("{pos:#?}");

    println!("res: {}", pos.result());
    println!("stm: {}", pos.stm());
    println!("score: {}", pos.score());
}
