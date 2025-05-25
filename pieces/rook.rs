use crate::utils::{is_valid, get_piece_color, is_cell_color_ally, cleaned_positions};

pub struct Rook{}

impl Rook{
  pub fn to_string() -> &'static str{
    "\
    \n\
    █ █ █\n\
    █████\n\
     ███\n\
    █████\n\
    "
  }

  pub fn authorized_positions(coordinates: [i32; 2], color: char, board: [[&'static str; 8]; 8]) -> Vec<Vec<i32>> {
    // pawns can only move in one direction depending on their color
    let mut positions: Vec<Vec<i32>> = vec![];

    let (y, x) = (coordinates[0], coordinates[1]);

    

    // right row
    for i in 1..8i32 {
      let new_x = x + i;
      let new_y = y;
      let new_coordinates = [new_y, new_x];

    // invalid coords
    if !is_valid(new_coordinates) {
      break;
    }

    // empty cell 
    if get_piece_color(board, new_coordinates) == ' ' {
        positions.push(new_coordinates.to_vec());
        continue;
    }
    // ally cell
    if is_cell_color_ally(board, new_coordinates, color) {
        break;
    }
    // enemy cell
    positions.push(new_coordinates.to_vec());
    break;
  }

  // left row
  for i in 1..8i32 {
    let new_x = x - i;
    let new_y = y;
    let new_coordinates = [new_y, new_x];

      // invalid coords
      if !is_valid(new_coordinates) {
        break;
      }

      // empty cell 
      if get_piece_color(board, new_coordinates) == ' ' {
          positions.push(new_coordinates.to_vec());
          continue;
      }
      // ally cell
      if is_cell_color_ally(board, new_coordinates, color) {
          break;
      }
      // enemy cell
      positions.push(new_coordinates.to_vec());
      break;
    }

    // bottom row
    for i in 1..8i32 {
      let new_x = x;
      let new_y = y + i;
      let new_coordinates = [new_y, new_x];

    // invalid coords
    if !is_valid(new_coordinates) {
      break;
    }

    // empty cell 
    if get_piece_color(board, new_coordinates) == ' ' {
        positions.push(new_coordinates.to_vec());
        continue;
    }
    // ally cell
    if is_cell_color_ally(board, new_coordinates, color) {
        break;
    }
    // enemy cell
    positions.push(new_coordinates.to_vec());
    break;
    }

    // up row
    for i in 1..8i32 {
      let new_x = x;
      let new_y = y - i;
      let new_coordinates = [new_y, new_x];

    // invalid coords
    if !is_valid(new_coordinates) {
      break;
    }

    // empty cell 
    if get_piece_color(board, new_coordinates) == ' ' {
        positions.push(new_coordinates.to_vec());
        continue;
    }
    // ally cell
    if is_cell_color_ally(board, new_coordinates, color) {
        break;
    }
    // enemy cell
    positions.push(new_coordinates.to_vec());
    break;
    }

    cleaned_positions(positions)
  }
}