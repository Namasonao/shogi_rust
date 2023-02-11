use crate::logic::{GameState};
use crate::logic::*;

pub mod logic;
pub mod view;
pub mod input;


fn main() {
    let mut gs: GameState = GameState::start_state();
    gs.white_hand.push(Piece::Hi);
    show_moves((2,0), &gs);
    play(gs);

}


fn show_moves(start: (usize, usize), gs: &GameState) {
    for y in 0..9 {
        for x in 0..9 {
            let m = MoveIdentifier {
                start,
                end: (x, y),
                promote: true,
            };

            if m.is_legal(&gs) {
                println!("({},{}) is a legal move", x, y);
            }
        }
    }
    println!("Those were the moves");
}

fn play(mut gs: GameState) {
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
