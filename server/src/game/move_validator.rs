use crate::game::card::CardSquareType;
use crate::game::squares::MapSquareType;
use crate::game::state::{GameError, PlayerMove};
use crate::game::team::PlayerTeam;
use crate::matrix::Matrix;

pub struct MoveValidator {}

impl MoveValidator {
    pub fn validate(board: &Matrix<MapSquareType>, team: &PlayerTeam, player_move: &PlayerMove) -> Result<(), GameError> {
        match CardSquareType::from_card_name(&player_move.card_name) {
            Some(squares) => {
                let squares = squares.rotate_clockwise(player_move.rotation.into());

                if !Self::card_within_bounds(player_move, board, squares) {
                    Err(GameError::InvalidPosition)
                } else {
                    Ok(())
                }
            },
            None => {
                Err(GameError::CardNotFound)
            }
        }
    }

    fn card_within_bounds(player_move: &PlayerMove, board: &Matrix<MapSquareType>, card_squares: Matrix<CardSquareType>) -> bool {
        if player_move.position.x < 0 || player_move.position.y < 0 {
            return false;
        }

        let board_size = board.size();
        let card_size = card_squares.size();

        (player_move.position.x + card_size.w as isize) <= board_size.w as isize
        && (player_move.position.y + card_size.h as isize) <= board_size.h as isize
    }
}

#[cfg(test)]
mod tests {
    use crate::game::state::CardRotation;
    use crate::matrix::MatrixSize;
    use crate::position::INamedPosition;
    use super::*;

    fn player_move(position: INamedPosition, rotation: CardRotation) -> PlayerMove {
        PlayerMove {
            card_name: "cool_card".to_string(),
            position,
            rotation
        }
    }

    fn board() -> Matrix<MapSquareType> {
        Matrix::filled_with(MatrixSize::new(10, 10), MapSquareType::Empty)
    }

    fn card() -> Matrix<CardSquareType> {
        Matrix::filled_with(MatrixSize::new(3, 2), CardSquareType::Fill)
    }

    #[test]
    fn card_within_bounds_negative_x() {
        assert!(!MoveValidator::card_within_bounds(
            &player_move(INamedPosition::new(-1, 0), CardRotation::Deg0),
            &board(),
            card()))
    }

    #[test]
    fn card_within_bounds_negative_y() {
        assert!(!MoveValidator::card_within_bounds(
            &player_move(INamedPosition::new(0, -1), CardRotation::Deg0),
            &board(),
            card()))
    }

    #[test]
    fn card_within_bounds_y_too_high() {
        assert!(!MoveValidator::card_within_bounds(
            &player_move(INamedPosition::new(0, 8), CardRotation::Deg0),
            &board(),
            card()))
    }

    #[test]
    fn card_within_bounds_max_y() {
        assert!(MoveValidator::card_within_bounds(
            &player_move(INamedPosition::new(0, 7), CardRotation::Deg0),
            &board(),
            card()))
    }

    #[test]
    fn card_within_bounds_x_too_high() {
        assert!(!MoveValidator::card_within_bounds(
            &player_move(INamedPosition::new(9, 0), CardRotation::Deg0),
            &board(),
            card()))
    }

    #[test]
    fn card_within_bounds_max_x() {
        assert!(MoveValidator::card_within_bounds(
            &player_move(INamedPosition::new(8, 0), CardRotation::Deg0),
            &board(),
            card()))
    }
}
