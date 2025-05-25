use crate::utils::{is_valid, get_piece_color, is_cell_color_ally, cleaned_positions};
use super::{PieceColor, PieceType};
pub struct Bishop{}
impl Bishop{
  pub fn to_string() -> &'static str{
    "\
    \n\
     ███\n\
    ██ ██\n\
     ███\n\
    █████\n\
    "
  }
  pub fn authorized_positions(coordinates: [i32; 2], color: PieceColor, board: [[Option<(PieceType, PieceColor)>; 8]; 8]) -> Vec<Vec<i32>> {
    let mut positions: Vec<Vec<i32>> = vec![];

    let y = coordinates[0];
    let x = coordinates[1];

    // for diagonal from piece to top left
    for i in 1..8i32 {
        let new_x = x - i;
        let new_y = y - i;
        let new_coordinates = [new_y, new_x];

      // invalid coords
      if !is_valid(new_coordinates) {
        break;
      }
  
      // empty cell 
      if get_piece_color(board, new_coordinates).is_none() {
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

    // for diagonal from piece to bottom left
    for i in 1..8i32 {
      let new_x = x + i;
      let new_y = y + i;
      let new_coordinates = [new_y, new_x];

      // invalid coords
      if !is_valid(new_coordinates) {
        break;
      }
  
      // empty cell 
      if get_piece_color(board, new_coordinates).is_none() {
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

     // for diagonal from piece to bottom right
    for i in 1..8i32 {
      let new_x = x - i;
      let new_y = y + i;
      let new_coordinates = [new_y, new_x];

    // invalid coords
    if !is_valid(new_coordinates) {
      break;
    }

    // empty cell 
    if get_piece_color(board, new_coordinates).is_none() {
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

  // for diagonal from piece to top right
  for i in 1..8i32 {
    let new_x = x + i;
    let new_y = y - i;
    let new_coordinates = [new_y, new_x];

    // invalid coords
    if !is_valid(new_coordinates) {
      break;
    }

    // empty cell 
    if get_piece_color(board, new_coordinates).is_none() {
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

#[cfg(test)]
mod tests {
  use crate::{board::Board, pieces::{PieceType, PieceColor, bishop::Bishop}};

  #[test]
  fn bishop_moves_no_enemies() {
      let custom_board = [
          [None, None, None, None, None, None, None, None],
          [None, None, None, None, None, None, None, None],
          [None, None, None, None, None, None, None, None],
          [None, None, None, None, None, None, None, None],
          [None, None, None, None, Some((PieceType::Bishop, PieceColor::White)), None, None, None],
          [None, None, None, None, None, None, None, None],
          [None, None, None, None, None, None, None, None],
          [None, None, None, None, None, None, None, None],
      ];
      let mut board = Board::default();
      board.set_board(custom_board);

      let mut right_positions = vec![
        vec![0, 0],
        vec![1, 1],
        vec![2, 2],
        vec![3, 3],

        vec![5, 5],
        vec![6, 6],
        vec![7, 7],

        vec![1, 7],
        vec![2, 6],
        vec![3, 5],
        
        vec![5, 3],
        vec![6, 2],
        vec![7, 1],

      ];
      right_positions.sort();

      let mut positions = Bishop::authorized_positions([4, 4], PieceColor::White, board.board);
      positions.sort();

      assert_eq!(right_positions, positions);
  }

  #[test]
  fn bishop_moves_one_enemies_top_right() {
      let custom_board = [
          [None, None, None, None, None, None, None, None],
          [None, None, None, None, None, None, None, None],
          [None, None, None, None, None, None, None, None],
          [None, None, None, None, None, Some((PieceType::Pawn, PieceColor::Black)), None, None],
          [None, None, None, None, Some((PieceType::Rook, PieceColor::White)), None, None, None],
          [None, None, None, None, None, None, None, None],
          [None, None, None, None, None, None, None, None],
          [None, None, None, None, None, None, None, None],
      ];
      let mut board = Board::default();
      board.set_board(custom_board);

      let mut right_positions = vec![
        vec![0, 0],
        vec![1, 1],
        vec![2, 2],
        vec![3, 3],

        vec![5, 5],
        vec![6, 6],
        vec![7, 7],

        vec![3, 5],
        
        vec![5, 3],
        vec![6, 2],
        vec![7, 1],
      ];
      right_positions.sort();

      let mut positions = Bishop::authorized_positions([4, 4], PieceColor::White, board.board);
      positions.sort();

      assert_eq!(right_positions, positions);
  }

  #[test]
  fn bishop_moves_multiple_enemies_and_ally_front() {
      let custom_board = [
          [None, None, None, None, None, None, None, None],
          [None, None, None, None, None, None, None, None],
          [None, None, None, None, None, None, None, None],
          [None, None, None, Some((PieceType::Pawn, PieceColor::Black)), None, None, None, None],
          [None, None, None, None, Some((PieceType::Rook, PieceColor::White)), None, None, None],
          [None, None, None, None, None, Some((PieceType::Pawn, PieceColor::Black)), None, None],
          [None, None, None, None, None, None, None, None],
          [None, Some((PieceType::Rook, PieceColor::White)), None, None, None, None, None, None],
      ];
      let mut board = Board::default();
      board.set_board(custom_board);

      let mut right_positions = vec![
        vec![3, 3],

        vec![5, 5],

        vec![3, 5],
        vec![2, 6],
        vec![1, 7],
        
        vec![5, 3],
        vec![6, 2],
      ];
      right_positions.sort();


      let mut positions = Bishop::authorized_positions([4, 4], PieceColor::White, board.board);
      positions.sort();

      assert_eq!(right_positions, positions);
  }
}