use colored::Colorize;
use crate::logic::{Piece, Cell, GameState};

impl Cell {
    pub fn print(&self) {
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


impl GameState {

    pub fn print(&self) {

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

