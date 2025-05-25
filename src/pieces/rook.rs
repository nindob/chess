use crate::utils::{is_valid, get_piece_color, is_cell_color_ally, cleaned_positions};
use super::{PieceColor, PieceType};

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

  pub fn authorized_positions(coordinates: [i32; 2], color: PieceColor, board: [[Option<(PieceType, PieceColor)>; 8]; 8]) -> Vec<Vec<i32>> {
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
    use crate::{board::Board, pieces::{PieceType, PieceColor, rook::Rook}};

    #[test]
    fn rook_moves_no_enemies() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, Some((PieceType::Rook, PieceColor::White)), None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![
          vec![7, 4],
          vec![6, 4],
          vec![5, 4],
          vec![3, 4],
          vec![2, 4],
          vec![1, 4],
          vec![0, 4],

          vec![4, 0],
          vec![4, 1],
          vec![4, 2],
          vec![4, 3],
          vec![4, 5],
          vec![4, 6],
          vec![4, 7],

        ];
        right_positions.sort();
        
        let mut positions = Rook::authorized_positions([4, 4], PieceColor::White, board.board);
        positions.sort();
        assert_eq!(right_positions, positions);
    }

    #[test]
    fn rook_moves_one_enemies_front() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, Some((PieceType::Pawn, PieceColor::Black)), None, None, None],
            [None, None, None, None, Some((PieceType::Rook, PieceColor::White)), None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![
          vec![7, 4],
          vec![6, 4],
          vec![5, 4],
          vec![3, 4],

          vec![4, 0],
          vec![4, 1],
          vec![4, 2],
          vec![4, 3],
          vec![4, 5],
          vec![4, 6],
          vec![4, 7],

        ];
        right_positions.sort();

        let mut positions = Rook::authorized_positions([4, 4], PieceColor::White, board.board);
        positions.sort();
        assert_eq!(right_positions, positions);
    }

    #[test]
    fn rook_moves_multiple_enemies_and_ally_front() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, Some((PieceType::Pawn, PieceColor::Black)), None, None, None],
            [None, None, None, None, Some((PieceType::Rook, PieceColor::White)), None, Some((PieceType::Pawn, PieceColor::Black)), None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, Some((PieceType::Rook, PieceColor::White)), None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![
          vec![4, 0],
          vec![4, 1],
          vec![4, 2],
          vec![4, 3],
          vec![4, 5],
          vec![4, 6],
          vec![3, 4],
          vec![5, 4],
          ];
        right_positions.sort();

        let mut positions = Rook::authorized_positions([4, 4], PieceColor::White, board.board);
        positions.sort();

        assert_eq!(right_positions, positions);
    }
}