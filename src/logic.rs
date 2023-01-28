#[derive(Copy, Clone)]
pub enum Piece {
    Oo,
    Hi,
    Ryuu,
    Kaku,
    Uma,
    Kin,
    Gin,
    Narigin,
    Keima,
    Narikei,
    Kyoo,
    Narikyoo,
    Fu,
    To,
}

#[derive(Copy, Clone)]
pub enum Cell {
    Empty,
    Fill(Piece, bool),
}


pub struct GameState {
   pub board: [[Cell; 9]; 9],
   pub white_hand: Vec<Piece>,
   pub blue_hand: Vec<Piece>,
}

impl GameState {
    pub fn empty() -> Self {
        Self {
            board: [[Cell::Empty; 9]; 9],
            white_hand: Vec::new(),
            blue_hand: Vec::new(),
        }
    }

    pub fn start_state() -> Self {
        let mut gs = GameState::empty();
        gs.board[0][0] = Cell::Fill(Piece::Kyoo, true);
        gs.board[0][8] = Cell::Fill(Piece::Kyoo, true);
        gs.board[8][0] = Cell::Fill(Piece::Kyoo, false);
        gs.board[8][8] = Cell::Fill(Piece::Kyoo, false);

        gs.board[0][1] = Cell::Fill(Piece::Keima, true);
        gs.board[0][7] = Cell::Fill(Piece::Keima, true);
        gs.board[8][1] = Cell::Fill(Piece::Keima, false);
        gs.board[8][7] = Cell::Fill(Piece::Keima, false);

        gs.board[0][2] = Cell::Fill(Piece::Gin, true);
        gs.board[0][6] = Cell::Fill(Piece::Gin, true);
        gs.board[8][2] = Cell::Fill(Piece::Gin, false);
        gs.board[8][6] = Cell::Fill(Piece::Gin, false);

        gs.board[0][3] = Cell::Fill(Piece::Kin, true);
        gs.board[0][5] = Cell::Fill(Piece::Kin, true);
        gs.board[8][3] = Cell::Fill(Piece::Kin, false);
        gs.board[8][5] = Cell::Fill(Piece::Kin, false);

        gs.board[0][4] = Cell::Fill(Piece::Oo, true);
        gs.board[8][4] = Cell::Fill(Piece::Oo, false);

        gs.board[1][1] = Cell::Fill(Piece::Hi, true);
        gs.board[1][7] = Cell::Fill(Piece::Kaku, true);
        gs.board[7][7] = Cell::Fill(Piece::Hi, false);
        gs.board[7][1] = Cell::Fill(Piece::Kaku, false);

        for c in &mut gs.board[2] {
            *c = Cell::Fill(Piece::Fu, true);
        }
        for c in &mut gs.board[6] {
            *c = Cell::Fill(Piece::Fu, false);
        }

        gs
    }
}
