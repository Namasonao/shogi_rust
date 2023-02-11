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

impl Piece {
    pub fn promote(&self) -> Piece {
        match *self {
            Piece::Hi => Piece::Ryuu,
            Piece::Kaku => Piece::Uma,
            Piece::Gin => Piece::Narigin,
            Piece::Keima => Piece::Narikei,
            Piece::Kyoo => Piece::Narikyoo,
            Piece::Fu => Piece::To,
            p => p,
        }
    }

    pub fn demote(&self) -> Piece {
        match *self {
            Piece::Ryuu => Piece::Hi,
            Piece::Uma => Piece::Kaku,
            Piece::Narigin => Piece::Gin,
            Piece::Narikei => Piece::Keima,
            Piece::Narikyoo => Piece::Kyoo,
            Piece::To => Piece::Fu,
            p => p,
        }
    }

    fn move_pattern(&self) -> [Pattern; 8] {
        use Pattern::*;
        use Piece::*;
        let kin_pattern = [Yes,Yes,Yes,   Yes,Yes,    No,Yes,No];
        match *self { // FL, FC, FR,   L, R,   BL, BC, BR
            Oo       => [Yes,Yes,Yes,  Yes,Yes,  Yes,Yes,Yes],
            Hi       => [No,Far,No,   Far,Far,  No,Far,No],
            Ryuu     => [Yes,Far,Yes,   Far,Far,   Yes,Far,Yes],
            Kaku     => [Far,No,Far,   No,No,   Far,No,Far],
            Uma      => [Far,Yes,Far,  Yes,Yes,  Far,Yes,Far],
            Kin      => kin_pattern,
            Gin      => [Yes,Yes,Yes,   No,No,    Yes,No,Yes],
            Narigin  => kin_pattern,
            Keima    => [Horse,No,Horse,   No,No,   No,No,No],
            Narikei  => kin_pattern,
            Kyoo     => [No,Far,No,    No,No,   No,No,No],
            Narikyoo => kin_pattern,
            Fu       => [No,Yes,No,    No,No,   No,No,No],
            To       => kin_pattern,
        }
    }
}

#[derive(PartialEq)]
enum Pattern {
    Yes,
    No,
    Far,
    Horse,
}


pub enum Move {
    Place(PlaceIdentifier),
    Move(MoveIdentifier),
}
impl Move {
    /// pre: sensible place move (position in bound)
    /// post: true if no rules are violated
    /// todo: uchifuzume (drop pawn mate is illegal)
    /// and a lot of moves
    pub fn is_legal(&self, gs: &GameState) -> bool {
        match self {
            Move::Place(p) => p.is_legal(gs),
            Move::Move(m) => m.is_legal(gs),
        }
    }
}

pub struct PlaceIdentifier {
    pub index: usize,
    pub end: (usize, usize),
}
pub struct MoveIdentifier {
    pub start: (usize, usize),
    pub end:   (usize, usize),
    pub promote: bool,
}

impl MoveIdentifier {
    pub fn is_legal(&self, gs: &GameState) -> bool {
        let f_s = gs.filledness(self.start);
        if f_s == 0 || (f_s == 1 && !gs.white_turn) || (f_s == -1 && gs.white_turn) {
            return false;
        }
        let f_e = gs.filledness(self.end);
        if f_e == 0 || (f_e == 1 && !gs.white_turn) || (f_e == -1 && gs.white_turn) {
            self.reachable(gs)
        } else {
            false
        }
    }
    fn reachable(&self, gs: &GameState) -> bool {
        let start_cell = gs.board[self.start.1][self.start.0];
        match start_cell {
            Cell::Empty => false,
            Cell::Fill(piece, white) => { //NOT FINISHED!!!
                let pattern = piece.move_pattern();
                let mut i = 0;
                for y in 0..3 {
                    for x in 0..3 {
                        if x == 1 && y == 1 {
                            continue;
                        }
                        let cur_x = self.start.0 + x;
                        let cur_y = self.start.1 + y;
                        if cur_x == 0 || cur_y == 0 {
                            i += 1;
                            continue;
                        }
                        let cur_x = cur_x - 1;
                        let cur_y = cur_y - 1;

                        if !within_bounds(cur_x, cur_y) {
                            i += 1;
                            continue;
                        }
                        if cur_x == self.end.0 && cur_y == self.end.1 {
                            let actual_i = if white {7 - i} else {i};
                            return pattern[actual_i] == Pattern::Yes;
                        }

                        let actual_i = if white {7 - i} else {i};
                        match pattern[actual_i] {
                            Pattern::Yes => if cur_x == self.end.0 && cur_y == self.end.1 {
                                return true;
                            },

                            Pattern::No => (),
                            Pattern::Far => {

                            },
                            Pattern::Horse => {
                                if white {
                                    let horse_y = cur_y + 1;
                                    if !within_bounds(cur_x, horse_y) {
                                        break;
                                    }
                                    if cur_x == self.end.0 && horse_y == self.end.1 {
                                        return true;
                                    }
                                } else {
                                    if cur_y == 0 {
                                        break;
                                    }
                                    let horse_y = cur_y - 1;
                                    if !within_bounds(cur_x, horse_y) {
                                        break;
                                    }
                                    if cur_x == self.end.0 && horse_y == self.end.1 {
                                        return true;
                                    }
                                }
                            }
                        }

                        i += 1;
                    }
                }
                false
            },
        }
    }
}

impl PlaceIdentifier {
    pub fn is_legal(&self, gs: &GameState) -> bool {
        let hand = match gs.white_turn {
            true => &gs.white_hand,
            false => &gs.blue_hand,
        };
        if self.index >= hand.len() {
            return false;
        }

        if let Piece::Fu = hand[self.index] {
            // Check File for existing pawn
            let x = self.end.0;
            for y in 0..9 {
                if let Cell::Fill(Piece::Fu, b) = gs.board[y][x] {
                    if b == gs.white_turn {
                        return false;
                    }
                }
            }
        }
        match gs.board[self.end.1][self.end.0] {
            Cell::Empty => true,
            _ => false,
        }
    }
}

fn within_bounds(x: usize, y: usize) -> bool {
    x < 9 && y < 9
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
   pub white_turn: bool,
}

impl GameState {
    pub fn empty() -> Self {
        Self {
            board: [[Cell::Empty; 9]; 9],
            white_hand: Vec::new(),
            blue_hand: Vec::new(),
            white_turn: true,
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

    /// pre: x,y within bounds
    /// post:
    /// 1 = white
    /// 0 = empty
    /// -1 = blue
    pub fn filledness(&self, pos: (usize, usize)) -> i8 {
        match self.board[pos.1][pos.0] {
            Cell::Empty => 0,
            Cell::Fill(_, true) => 1,
            Cell::Fill(_, false) => -1,
        }
    }
    
    /// pre: m is a legal move
    /// post: changes gamestate so the move is performed (including hand, turn)
    pub fn perform(&mut self, m: Move) {
        let hand = match self.white_turn {
            true => &mut self.white_hand,
            false => &mut self.blue_hand,
        };
        match m {
            Move::Move(m) => {
                match self.board[m.end.1][m.end.0] {
                    Cell::Empty => (),
                    Cell::Fill(piece, _) => {
                        hand.push(piece.demote());
                    },
                }
                self.board[m.end.1][m.end.0] = self.board[m.start.1][m.start.0];
                self.board[m.start.1][m.start.0] = Cell::Empty;
            },
            Move::Place(p) => {
                self.board[p.end.1][p.end.0] = Cell::Fill(hand[p.index], self.white_turn);
                hand.remove(p.index);
            }
        };
        self.white_turn = !self.white_turn;
    }
}
