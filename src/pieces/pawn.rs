use super::{PieceColor, PieceType};
use crate::utils::{cleaned_positions, get_piece_color, is_cell_color_ally, is_valid};

pub struct Pawn {}

impl Pawn {
    pub fn to_string() -> &'static str {
        "\
        \n\
        \n\
      ▟█▙\n\
      ▜█▛\n\
     ▟███▙\n\
    "
    }

    pub fn pawn_moves(
        coordinates: [i32; 2],
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        allow_move_on_ally_positions: bool,
    ) -> Vec<Vec<i32>> {
        // pawns can only move in one direction depending of their color
        // -1 if they are white (go up) +1 if they are black (go down)
        let direction = if color == PieceColor::White { -1 } else { 1 };

        let mut positions: Vec<Vec<i32>> = vec![];

        let (y, x) = (coordinates[0], coordinates[1]);

        // move one in front
        let new_x_front_one = x;
        let new_y_front_one = y + direction;
        let new_coordinates_front_one = [new_y_front_one, new_x_front_one];

        if is_valid(new_coordinates_front_one)
            && !allow_move_on_ally_positions
            && get_piece_color(board, new_coordinates_front_one).is_none()
        {
            // empty cell
            positions.push(new_coordinates_front_one.to_vec());

            // move front a second cell
            let new_x_front_two = x;
            let new_y_front_two = y + direction * 2;
            let new_coordinates_front_two = [new_y_front_two, new_x_front_two];

            if is_valid(new_coordinates_front_two)
                && get_piece_color(board, new_coordinates_front_two).is_none()
                && ((color == PieceColor::White && y == 6)
                    || (color == PieceColor::Black && y == 1))
            {
                positions.push(new_coordinates_front_two.to_vec());
            }
        }

        // check for enemy piece on the right
        let new_x_right = x + 1;
        let new_y_right = y + direction;
        let new_coordinates_right = [new_y_right, new_x_right];

        // check for enemy piece on the left
        let new_x_left = x - 1;
        let new_y_left = y + direction;
        let new_coordinates_left = [new_y_left, new_x_left];

        // if we allow on ally position we push it anyway

        if allow_move_on_ally_positions {
            if is_valid(new_coordinates_right) {
                positions.push(new_coordinates_right.to_vec())
            };
            if is_valid(new_coordinates_left) {
                positions.push(new_coordinates_left.to_vec())
            };
        } else {
            // else we check if it's an ally piece
            if is_valid(new_coordinates_right)
                && get_piece_color(board, new_coordinates_right).is_some()
                && !is_cell_color_ally(board, new_coordinates_right, color)
            {
                positions.push(new_coordinates_right.to_vec());
            }
            if is_valid(new_coordinates_left)
                && get_piece_color(board, new_coordinates_left).is_some()
                && !is_cell_color_ally(board, new_coordinates_left, color)
            {
                positions.push(new_coordinates_left.to_vec());
            }
        }

        cleaned_positions(positions)
    }
    pub fn authorized_positions(
        coordinates: [i32; 2],
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
    ) -> Vec<Vec<i32>> {
        Self::pawn_moves(coordinates, color, board, false)
    }

    pub fn protecting_positions(
        coordinates: [i32; 2],
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
    ) -> Vec<Vec<i32>> {
        Self::pawn_moves(coordinates, color, board, true)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::Board,
        pieces::{pawn::Pawn, PieceColor, PieceType},
    };

    #[test]
    fn pawn_moves_one_cell_forward() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![vec![3, 4]];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions([4, 4], PieceColor::White, board.board);
        positions.sort();
        assert_eq!(right_positions, positions);
    }

    #[test]
    fn pawn_moves_one_cell_forward_two() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![vec![5, 4], vec![4, 4]];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions([6, 4], PieceColor::White, board.board);
        positions.sort();
        assert_eq!(right_positions, positions);
    }

    #[test]
    fn pawn_moves_one_cell_enemy_left_right() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                Some((PieceType::Pawn, PieceColor::Black)),
                None,
                None,
                None,
                None,
            ],
            [
                None,
                None,
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![vec![2, 3], vec![3, 3], vec![2, 4], vec![2, 2]];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions([1, 3], PieceColor::Black, board.board);
        positions.sort();
        assert_eq!(right_positions, positions);
    }

    #[test]
    fn pawn_moves_one_cell_3_enemies() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                Some((PieceType::Pawn, PieceColor::Black)),
                None,
                None,
                None,
                None,
            ],
            [
                None,
                None,
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![vec![2, 4], vec![2, 2]];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions([1, 3], PieceColor::Black, board.board);
        positions.sort();
        assert_eq!(right_positions, positions);
    }
}