use self::{bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook};

pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Bishop,
    Queen,
    King,
    Knight,
}

impl PieceType {
    pub fn authorized_positions(
        self,
        coordinates: [i8; 2],
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        move_history: Vec<(Option<PieceType>, String)>,
        is_king_checked: bool,
    ) -> Vec<Vec<i8>> {
        match self {
            PieceType::Pawn => {
                Pawn::authorized_positions(coordinates, color, board, move_history, is_king_checked)
            }
            PieceType::Rook => {
                Rook::authorized_positions(coordinates, color, board, move_history, is_king_checked)
            }
            PieceType::Bishop => Bishop::authorized_positions(
                coordinates,
                color,
                board,
                move_history,
                is_king_checked,
            ),
            PieceType::Queen => Queen::authorized_positions(
                coordinates,
                color,
                board,
                move_history,
                is_king_checked,
            ),
            PieceType::King => {
                King::authorized_positions(coordinates, color, board, move_history, is_king_checked)
            }
            PieceType::Knight => Knight::authorized_positions(
                coordinates,
                color,
                board,
                move_history,
                is_king_checked,
            ),
        }
    }

    pub fn protected_positions(
        selected_coordinates: [i8; 2],
        piece_type: PieceType,
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        move_history: Vec<(Option<PieceType>, String)>,
    ) -> Vec<Vec<i8>> {
        match piece_type {
            PieceType::Pawn => {
                Pawn::protected_positions(selected_coordinates, color, board, move_history)
            }
            PieceType::Rook => {
                Rook::protected_positions(selected_coordinates, color, board, move_history)
            }
            PieceType::Bishop => {
                Bishop::protected_positions(selected_coordinates, color, board, move_history)
            }
            PieceType::Queen => {
                Queen::protected_positions(selected_coordinates, color, board, move_history)
            }
            PieceType::King => {
                King::protected_positions(selected_coordinates, color, board, move_history)
            }
            PieceType::Knight => {
                Knight::protected_positions(selected_coordinates, color, board, move_history)
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceColor {
    Black,
    White,
}

pub trait Movable {
    fn piece_move(
        coordinates: [i8; 2],
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        allow_move_on_ally_positions: bool,
        move_history: Vec<(Option<PieceType>, String)>,
    ) -> Vec<Vec<i8>>;
}

pub trait Position {
    fn authorized_positions(
        coordinates: [i8; 2],
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        move_history: Vec<(Option<PieceType>, String)>,
        is_king_checked: bool,
    ) -> Vec<Vec<i8>>;

    fn protected_positions(
        coordinates: [i8; 2],
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        move_history: Vec<(Option<PieceType>, String)>,
    ) -> Vec<Vec<i8>>;
}