use colored::Colorize;

#[derive(Copy, Clone)]
enum Piece {
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
enum Cell {
    Empty,
    Fill(Piece, bool),
}
impl Cell {
    fn print(&self) {
    match *self {
        Cell::Empty     => print!(". "),
        Cell::Fill(piece, white) => {
            let s = match piece{
                Piece::Oo       => if white {"王"} else { "玉" },
                Piece::Hi       =>"飛",
                Piece::Ryuu     =>"龍",
                Piece::Kaku     =>"角",
                Piece::Uma      =>"馬",
                Piece::Kin      =>"金",
                Piece::Gin      =>"銀",
                Piece::Narigin  =>"全",
                Piece::Keima    =>"桂",
                Piece::Narikei  =>"圭",
                Piece::Kyoo     =>"香",
                Piece::Narikyoo =>"杏",
                Piece::Fu       =>"歩",
                Piece::To       =>"と",
            };
            if white {
                print!("{}", s.white());
            } else {
                print!("{}", s.bright_blue());
            }
        }
    }

        
    }
}

struct GameState {
   board: [[Cell; 9]; 9],
   white_hand: Vec<Piece>,
   blue_hand: Vec<Piece>,
}

impl GameState {
    fn empty() -> Self {
        Self {
            board: [[Cell::Empty; 9]; 9],
            white_hand: Vec::new(),
            blue_hand: Vec::new(),
        }
    }

    fn start_state() -> Self {
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

    fn print(&self) {

        println!("9 8 7 6 5 4 3 2 1\n");
        let mut h = 1;
        for row in self.board {
            for cell in row {
                cell.print();
            }
            println!("  {}", h);
            h += 1;
        }
        println!("\n------------------");
        print!("White Hand:");
        let mut letter = b'a';
        for &p in &self.white_hand {
            print!(" {}", letter as char);
            Cell::Fill(p, true).print();
            letter += 1;
        }
        print!("\nBlue Hand: ");
        let mut letter = b'a';
        for &p in &self.blue_hand {
            print!(" {}", letter as char);
            Cell::Fill(p, true).print();
            letter += 1;
        }
        println!("");
    }
}

fn main() {
    let mut gs: GameState = GameState::start_state();
    gs.white_hand.push(Piece::Fu);
    gs.white_hand.push(Piece::Oo);

    gs.print();
}
