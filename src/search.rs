use crate::board::*;

pub fn evaluate<'a, R: Representation>(_: &'a R) -> i8 {
    return 0
}

pub fn negamax<'a, R: Representation>(board: &'a mut R, depth: usize) -> i8 {
    let mag = board.turn().magnitude();
    if board.is_terminal() {
        return mag * 100;
    }
    if depth == 0 {
        return mag * evaluate(board);
    }
    let mut value = -100;
    for mv in board.generate_moves() {
        board.make_move(mv);
        value = i8::max(value, -negamax(board, depth - 1));
        board.unmake_move(mv);
    }
    return value
}

pub fn alphabeta<'a, R: Representation>(board: &'a mut R, depth: usize, mut alpha: i8, beta: i8, color: i8) -> i8 {
    if board.is_terminal() {
        return color * 100;
    }
    if depth == 0 {
        return color * evaluate(board);
    }
    let mut value = -100;
    for mv in board.generate_moves() {
        board.make_move(mv);
        value = i8::max(value, -alphabeta(board, depth - 1, -beta, -alpha, -color));
        alpha = i8::max(alpha, value);
        board.unmake_move(mv);
        if alpha >= beta { break }
    }
    return value
}

pub fn search<'a, R: Representation>(board: &'a mut R, depth: usize) -> (Move, i8) {
    assert!(!board.is_terminal());
    let mut best_move = Move { col: 0 };
    let mut best_value = -100;
    for mv in board.generate_moves() {
        board.make_move(mv);
        let value = -alphabeta(board, depth - 1, 100, -100, board.turn().magnitude());
        board.unmake_move(mv);
        if value >= best_value {
            best_value = value;
            best_move = mv;
        }
    }
    return (best_move, best_value)
}
