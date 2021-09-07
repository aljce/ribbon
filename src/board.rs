use std::fmt;
use std::ops::Not;
use std::option::Option;
use std::vec::IntoIter;

pub const HEIGHT: usize = 6;
pub const WIDTH: usize = 7;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Color {
    Red,
    Yellow,
}

impl Color {
    pub fn magnitude(self) -> i8 {
        match self {
            Color::Red => 1,
            Color::Yellow => -1,
        }
    }
}

impl Not for Color {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Color::Red => Color::Yellow,
            Color::Yellow => Color::Red,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color: &'static str = match *self {
            Color::Red => "R",
            Color::Yellow => "Y",
        };
        write!(f, "{}", color)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Move {
    pub col: usize, // 0 ... WIDTH - 1
}

impl Move {
    pub fn parse(mv: char) -> Result<Move, String> {
        match mv {
            '1' => Ok(Move { col: 0 }),
            '2' => Ok(Move { col: 1 }),
            '3' => Ok(Move { col: 2 }),
            '4' => Ok(Move { col: 3 }),
            '5' => Ok(Move { col: 4 }),
            '6' => Ok(Move { col: 5 }),
            '7' => Ok(Move { col: 6 }),
            _   => Err("unrecognized move".to_string())
        }
    }
}

const ZOBRIST_KEYS : [[u64; HEIGHT]; WIDTH] = [
    [ 7266447313870364031, 4946485549665804864, 16945909448695747420, 16394063075524226720, 4873882236456199058, 14877448043947020171 ],
    [ 5249110015610582907, 1235879089597390050, 17320312680810499042, 8942268601720066061, 14226945236717732373, 9383926873555417063 ],
    [ 6489677788245343319, 236502320419669032, 13670483975188204088, 8904234204977263924, 17251681303478610375, 13075804672185204371 ],
    [ 1074659097419704618, 17119870085051257224, 10949279256701751503, 17618792803942051220, 957923366004347591, 1012818702180800310 ],
    [ 1861591264068118966, 3809841506498447207, 13408683141069553686, 13900005529547645957, 16475327524349230602, 13554353441017344755 ],
    [ 6788940719869959076, 11670856244972073775, 2488756775360218862, 11016608897122070904, 13978444093099579683, 11628184459157386459 ],
    [ 7306216312942796257, 2889379661594013754, 17310575136995821873, 3435082195390932486, 7444710627467609883, 11216615872596820107 ],
];

pub trait Representation {
    type Moves: IntoIterator<Item = Move>;
    fn empty() -> Self;
    fn make_move<'a>(&'a mut self, mv: Move);
    fn unmake_move<'a>(&'a mut self, mv: Move);
    // O(1)
    fn zobrist<'a>(&'a self) -> u64;
    fn turn<'a>(&'a self) -> Color;
    fn generate_moves<'a>(&'a self) -> Self::Moves;
    fn is_terminal<'a>(&'a self) -> bool;
}

#[derive(Debug, Clone, PartialEq)]
pub struct PieceList {
    turn: Color,
    squares: [[Option<Color>; HEIGHT]; WIDTH],
    empties: [usize; WIDTH],
    zobrist: u64,
}

impl Representation for PieceList {
    type Moves = IntoIter<Move>;
    fn empty() -> Self {
        PieceList {
            turn: Color::Red,
            squares: [[None; HEIGHT]; WIDTH],
            empties: [0; WIDTH],
            zobrist: 0,
        }
    }

    fn make_move<'a>(&'a mut self, mv: Move) {
        let col = mv.col;
        let row = self.empties[col];
        self.squares[col][row] = Some(self.turn);
        self.empties[col] = row + 1;
        self.zobrist ^= ZOBRIST_KEYS[col][row];
        self.turn = !self.turn;
    }

    fn unmake_move<'a>(&'a mut self, mv: Move) {
        let col = mv.col;
        let row = self.empties[col] - 1;
        self.squares[col][row] = None;
        self.empties[col] = row;
        self.zobrist ^= ZOBRIST_KEYS[col][row];
        self.turn = !self.turn;
    }

    fn zobrist<'a>(&'a self) -> u64 {
        self.zobrist
    }

    fn turn<'a>(&'a self) -> Color {
        self.turn
    }

    fn generate_moves<'a>(&'a self) -> IntoIter<Move> {
        let mut moves = vec![];
        for col in 0..WIDTH {
            let mv = self.empties[col];
            if mv >= HEIGHT {
                continue;
            };
            moves.push(Move { col });
        }
        moves.into_iter()
    }

    fn is_terminal<'a>(&'a self) -> bool {
        for col in 0..WIDTH {
            for row in 0..HEIGHT {
                let square = match self.squares[col][row] {
                    Some(square) => square,
                    None => continue,
                };
                if square != self.turn {
                    continue;
                };
                // horizontal
                if col < WIDTH - 3
                    && [
                        self.squares[col + 1][row],
                        self.squares[col + 2][row],
                        self.squares[col + 3][row],
                    ]
                    .iter()
                    .all(|x| *x == Some(square))
                {
                    return true;
                };
                // vertical
                if row < HEIGHT - 3
                    && [
                        self.squares[col][row + 1],
                        self.squares[col][row + 2],
                        self.squares[col][row + 3],
                    ]
                    .iter()
                    .all(|x| *x == Some(square))
                {
                    return true;
                };
                // diagonal
                if col < WIDTH - 3 && row < HEIGHT - 3
                    && [
                        self.squares[col + 1][row + 1],
                        self.squares[col + 2][row + 2],
                        self.squares[col + 3][row + 3],
                    ]
                    .iter()
                    .all(|x| *x == Some(square))
                {
                    return true;
                };
                // antidiagonal
                if col > 2 && row < HEIGHT - 3
                    && [
                        self.squares[col - 1][row + 1],
                        self.squares[col - 2][row + 2],
                        self.squares[col - 3][row + 3],
                    ]
                    .iter()
                    .all(|x| *x == Some(square))
                {
                    return true;
                };

            }
        }
        false
    }

}

impl fmt::Display for PieceList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in (0..HEIGHT).rev() {
            for col in 0..WIDTH {
                match self.squares[col][row] {
                    None => write!(f, " ")?,
                    Some(color) => write!(f, "{}", color)?,
                }
            }
            writeln!(f, "")?;
        }
        writeln!(f, "{}", "-".repeat(WIDTH))?;
        writeln!(f, "Turn: {}", self.turn)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bitboard {
    turn: Color,
    red: u64,
    yellow: u64,
    empties: [u8; WIDTH],
    zobrist: u64,
}

impl Representation for Bitboard {
    type Moves = IntoIter<Move>;

    fn empty() -> Self {
        Bitboard {
            turn: Color::Red,
            red: 0,
            yellow: 0,
            empties: [0, 7, 14, 21, 28, 35, 42],
            zobrist: 0,
        }
    }

    fn make_move<'a>(&'a mut self, mv: Move) {
        match self.turn {
            Color::Red => {
                self.red ^= 1 << self.empties[mv.col];
            },
            Color::Yellow => {
                self.yellow ^= 1 << self.empties[mv.col];
            }
        }
        self.empties[mv.col] += 1;
        self.turn = !self.turn;
    }

    fn unmake_move<'a>(&'a mut self, mv: Move) {
        self.turn = !self.turn;
        self.empties[mv.col] -= 1;
        match self.turn {
            Color::Red => {
                self.red ^= 1 << self.empties[mv.col];
            },
            Color::Yellow => {
                self.yellow ^= 1 << self.empties[mv.col];
            }
        }
    }

    fn turn<'a>(&'a self) -> Color {
        self.turn
    }

    fn zobrist<'a>(&'a self) -> u64 {
        self.red ^ self.yellow
    }

    fn generate_moves<'a>(&'a self) -> Self::Moves {
        const TOPS : [u8; WIDTH] = [6, 13, 20, 27, 34, 41, 48];
        let mut moves = vec![];
        for col in 0 .. WIDTH {
            if self.empties[col] < TOPS[col] {
                moves.push(Move { col })
            }
        }
        moves.into_iter()
    }

    fn is_terminal<'a>(&'a self) -> bool {
        let bitboard = match self.turn {
            Color::Red => self.red,
            Color::Yellow => self.yellow,
        };
        if bitboard & (bitboard >> 6) & (bitboard >> 12) & (bitboard >> 18) != 0 { return true };
        if bitboard & (bitboard >> 8) & (bitboard >> 16) & (bitboard >> 24) != 0 { return true };
        if bitboard & (bitboard >> 7) & (bitboard >> 14) & (bitboard >> 21) != 0 { return true };
        if bitboard & (bitboard >> 1) & (bitboard >>  2) & (bitboard >>  3) != 0 { return true };
        false
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in (0..HEIGHT).rev() {
            for col in 0..WIDTH {
                let index = row + col * 7;
                let square = if self.red & 1 << index != 0 {
                    "R"
                } else if self.yellow & 1 << index != 0 {
                    "Y"
                } else {
                    " "
                };
                write!(f, "{}", square)?;
            }
            writeln!(f, "")?;
        }
        writeln!(f, "{}", "-".repeat(WIDTH))?;
        writeln!(f, "Turn: {}", self.turn)
    }
}

pub fn parse<'a, R: Representation>(input: &'a str) -> Result<R, String> {
    let mut board = R::empty();
    for c in input.chars() {
        let mv = Move::parse(c)?;
        board.make_move(mv);
    }
    Ok(board)
}



#[cfg(test)]
mod tests {
    use super::*;

    fn perft<'a, R: Representation>(board: &'a mut R) -> u64 {
        let mut sum = 1;
        if board.is_terminal() {
            return sum;
        }
        for mv in board.generate_moves() {
            board.make_move(mv);
            sum += perft(board);
            board.unmake_move(mv);
        }
        sum
    }

    fn perft_unit<'a>(notation: &'a str) {
        let mut piece_list = parse::<PieceList>(notation).unwrap();
        let piece_list_clone = piece_list.clone();
        let mut bitboard = parse::<Bitboard>(notation).unwrap();
        let bitboard_clone = bitboard.clone();
        assert_eq!(perft(&mut piece_list), perft(&mut bitboard));
        assert_eq!(piece_list, piece_list_clone);
        assert_eq!(bitboard, bitboard_clone);
    }

    #[test]
    fn perft_unit_1() {
        perft_unit("1471116462531526523152622637576544");
    }

    #[test]
    fn perft_unit_2() {
        perft_unit("455155137133342477531351477744");
    }

    #[test]
    fn perft_unit_3() {
        perft_unit("61134124344226244271352657663");
    }
}
