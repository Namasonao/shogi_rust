use crate::logic::{Move, PlaceIdentifier, MoveIdentifier};
use std::io;

pub fn read_move() -> Result<Move, String> {
   let mut line = String::new();
   let stdin = io::stdin();
   stdin.read_line(&mut line).expect("Failed to read line");

   let splits: Vec<&str> = line.split_whitespace().collect();
   if splits.len() == 0 {
        return Err("No input".to_string());
   }

   match splits[0].as_bytes()[0] as char {
       'm' => match parse_move(splits) {
           Ok(m) => Ok(Move::Move(m)),
           Err(e) => Err(e),
       },
       'p' => match parse_place(splits) {
           Ok(p) => Ok(Move::Place(p)),
           Err(e) => Err(e),
       },
       _   => Err("Invalid first character".to_string()),
   }
}

fn parse_move(splits: Vec<&str>) -> Result<MoveIdentifier, String> {
    if splits.len() < 5 {
        return Err("Not enough inputs".to_string());
    }
   let start_x = match splits[1].parse::<usize>() {
       Ok(val) => from_coordinate_x(val)?,
       Err(err) => return Err(err.to_string()),
   };
   let start_y = match splits[2].parse::<usize>() {
       Ok(val) => from_coordinate_y(val)?,
       Err(err) => return Err(err.to_string()),
   };
   let end_x = match splits[3].parse::<usize>() {
       Ok(val) => from_coordinate_x(val)?,
       Err(err) => return Err(err.to_string()),
   };
   let end_y = match splits[4].parse::<usize>() {
       Ok(val) => from_coordinate_y(val)?,
       Err(err) => return Err(err.to_string()),
   };

   Ok(MoveIdentifier {
        start: (start_x, start_y),
        end: (end_x, end_y),
        promote: true,
   })
}

fn parse_place(splits: Vec<&str>) -> Result<PlaceIdentifier, String> {
    if splits.len() < 4 {
        return Err("Not enough inputs".to_string());
    }

    let c = splits[1].as_bytes()[0] as u8;
    if c > 'z' as u8 || c < 'a' as u8 {
        return Err("Invalid character".to_string());
    }
    let i: usize = (c - 'a' as u8).into();
   let end_x = match splits[2].parse::<usize>() {
       Ok(val) => from_coordinate_x(val)?,
       Err(err) => return Err(err.to_string()),
   };
   let end_y = match splits[3].parse::<usize>() {
       Ok(val) => from_coordinate_y(val)?,
       Err(err) => return Err(err.to_string()),
   };

   println!("{}", i);
   Ok(PlaceIdentifier {
    index: i,
    end: (end_x, end_y),
   })
}

fn from_coordinate_x(p: usize) -> Result<usize, String> {
    if p > 9 || p < 1 {
        return Err("Out of bounds".to_string());
    }
    return Ok(9 - p);
}
fn from_coordinate_y(p: usize) -> Result<usize, String> {
    if p > 9 || p < 1 {
        return Err("Out of bounds".to_string());
    }
    return Ok(p - 1);
}
