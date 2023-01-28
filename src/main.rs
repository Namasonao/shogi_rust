use crate::logic::{Piece, GameState};

pub mod logic;
pub mod view;


fn main() {
    let mut gs: GameState = GameState::start_state();
    gs.white_hand.push(Piece::Fu);
    gs.white_hand.push(Piece::Oo);

    gs.print();
}
