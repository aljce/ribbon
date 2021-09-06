mod board;
use board::{parse, Representation, Bitboard, Move};

fn main() {
    // let notation = "44455554221";
    let notation = "1471116462531526523152622637576544";
    // let notation = "";
    let mut board = parse::<Bitboard>(notation).unwrap();
    loop {
        println!("{}", board);
        if board.is_terminal() {
            println!("Game Over");
            std::process::exit(0)
        }
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok().expect("Error: Failed to Read Line");
        let chr = input.chars().nth(0).expect("No Input");
        let mv = match Move::parse(chr) {
            Ok(mv) => mv,
            Err(err) => {
                println!("Error: {}", err);
                continue
            },
        };
        board.make_move(mv);
    }
}
