use crate::logic::{Piece, GameState};

pub mod logic;
pub mod view;
pub mod input;


fn main() {
    let mut gs: GameState = GameState::start_state();
    gs.white_hand.push(Piece::Fu);
    gs.white_hand.push(Piece::Oo);

    loop {
        gs.print();
        match input::read_move() {
            Ok(m) => {
                if !m.is_legal(&gs) {
                    println!("Illegal move.");
                    continue;
                }

                gs.perform(m);

            },
            Err(e) => println!("Unable to parse: {}", e),
        };
    }
}
