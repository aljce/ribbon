mod board;
use board::*;

mod search;
use search::*;

fn main() {
    // let notation = "1471116462531526523152622637576544";
    let notation = "722335";
    let mut board = parse::<PieceList>(notation).unwrap();
    println!("{}", board);
    let depth = 11;
    let (Move { col }, eval) = search(&mut board, depth);
    println!("Move {} has evalutation {}", col + 1, eval);
    // let eval = alphabeta(&mut board, depth, -100, 100);
    // println!("Alphabeta {}", eval);
    // let eval = negamax(&mut board, 6);
    // println!("Negamax {}", eval);

    // loop {
    //     println!("{}", board);
    //     if board.is_terminal() {
    //         println!("Game Over");
    //         std::process::exit(0)
    //     }
    //     let mut input = String::new();
    //     std::io::stdin().read_line(&mut input).ok().expect("Error: Failed to Read Line");
    //     let chr = input.chars().nth(0).expect("No Input");
    //     let mv = match Move::parse(chr) {
    //         Ok(mv) => mv,
    //         Err(err) => {
    //             println!("Error: {}", err);
    //             continue
    //         },
    //     };
    //     board.make_move(mv);
    // }
}
