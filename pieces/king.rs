use crate::utils::{is_valid, get_piece_color, is_cell_color_ally, cleaned_positions};

pub struct King{}
impl King{
  pub fn to_string() -> &'static str{
    "\
    \n\
      █\n\
    ██░██\n\
      █\n\
    █████\n\
    "
  }

  pub fn authorized_positions(coordinates: [i32; 2], color: char, board: [[&'static str; 8]; 8]) -> Vec<Vec<i32>>{
    // pawns can only move in one direction depending of their color
    let mut positions: Vec<Vec<i32>> = vec![];
    
    let y = coordinates[0];
    let x = coordinates[1]; 

    // can move on a complete row
    // generate positions in all eight possible directions
    // TODO: Calculate the cells where it can't go because of check
    for &dy in &[-1, 0, 1] {
      for &dx in &[-1, 0, 1] {
          // skip the case where both dx and dy are zero (the current position)
          let new_x = x + dx;
          let new_y = y + dy;
          if new_x > 0 && new_y > 0 && new_x <= 7 && new_y <= 7 && get_piece_color(board, [new_y, new_x]) != color {
            positions.push(vec![y + dy, x + dx]);
          }
      }
  }

    cleaned_positions(positions)
  }
}