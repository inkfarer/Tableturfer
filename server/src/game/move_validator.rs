use crate::game::card::CardSquareType;
use crate::game::squares::MapSquareType;
use crate::game::state::{GameError, PlayerMove};
use crate::game::team::PlayerTeam;
use crate::matrix::{Matrix, Slice};

pub struct MoveValidator {}

impl MoveValidator {
    pub fn validate(board: &Matrix<MapSquareType>, team: &PlayerTeam, player_move: &PlayerMove) -> Result<(), GameError> {
        match CardSquareType::from_card_name(&player_move.card_name) {
            Some(squares) => {
                let squares = squares.rotate_clockwise(player_move.rotation.into());

                if !Self::card_within_bounds(player_move, board, &squares)
                || !Self::card_on_blank_spaces(player_move, board, &squares)
                || !Self::map_squares_near_card(player_move, board, &squares, team) {
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

    fn card_within_bounds(player_move: &PlayerMove, board: &Matrix<MapSquareType>, card_squares: &Matrix<CardSquareType>) -> bool {
        if player_move.position.x < 0 || player_move.position.y < 0 {
            return false;
        }

        let board_size = board.size();
        let card_size = card_squares.size();

        (player_move.position.x + card_size.w as isize) <= board_size.w as isize
        && (player_move.position.y + card_size.h as isize) <= board_size.h as isize
    }

    fn card_on_blank_spaces(player_move: &PlayerMove, board: &Matrix<MapSquareType>, card_squares: &Matrix<CardSquareType>) -> bool {
        let card_size = card_squares.size();
        let pos_from = (player_move.position.x as usize, player_move.position.y as usize);
        let squares_under_card = board.slice(pos_from..(pos_from.0 + card_size.w, pos_from.1 + card_size.h));

        squares_under_card.into_iter()
            .zip(card_squares.clone().into_iter())
            .all(|((map_square, _), (card_square, _))| map_square == MapSquareType::Empty || card_square == CardSquareType::Empty)
    }

    fn map_squares_near_card(player_move: &PlayerMove, board: &Matrix<MapSquareType>, card_squares: &Matrix<CardSquareType>, team: &PlayerTeam) -> bool {
        let pos_from = (player_move.position.x as usize, player_move.position.y as usize);

        let accepted_nearby_squares = match team {
            PlayerTeam::Alpha => vec!(MapSquareType::FillAlpha, MapSquareType::SpecialAlpha),
            PlayerTeam::Bravo => vec!(MapSquareType::FillBravo, MapSquareType::SpecialBravo),
        };

        card_squares.clone().into_iter()
            .any(|(square, position)| {
                if square == CardSquareType::Empty {
                    return false;
                }

                let square_pos = (pos_from.0 + position.0, pos_from.1 + position.1);

                let nearby_squares = board.slice((square_pos.0.checked_sub(1).unwrap_or(0), square_pos.1.checked_sub(1).unwrap_or(0))..=(square_pos.0 + 1, square_pos.1 + 1));
                println!("at {:?}: {:?}", square_pos, nearby_squares);

                nearby_squares
                    .clone().into_iter()
                    .any(|(map_square, _)| accepted_nearby_squares.contains(&map_square))
            })
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
            &card()))
    }

    #[test]
    fn card_within_bounds_negative_y() {
        assert!(!MoveValidator::card_within_bounds(
            &player_move(INamedPosition::new(0, -1), CardRotation::Deg0),
            &board(),
            &card()))
    }

    #[test]
    fn card_within_bounds_y_too_high() {
        assert!(!MoveValidator::card_within_bounds(
            &player_move(INamedPosition::new(0, 8), CardRotation::Deg0),
            &board(),
            &card()))
    }

    #[test]
    fn card_within_bounds_max_y() {
        assert!(MoveValidator::card_within_bounds(
            &player_move(INamedPosition::new(0, 7), CardRotation::Deg0),
            &board(),
            &card()))
    }

    #[test]
    fn card_within_bounds_x_too_high() {
        assert!(!MoveValidator::card_within_bounds(
            &player_move(INamedPosition::new(9, 0), CardRotation::Deg0),
            &board(),
            &card()))
    }

    #[test]
    fn card_within_bounds_max_x() {
        assert!(MoveValidator::card_within_bounds(
            &player_move(INamedPosition::new(8, 0), CardRotation::Deg0),
            &board(),
            &card()))
    }

    #[test]
    fn card_on_blank_spaces_true() {
        let mut board = board();
        board[(1, 2)] = MapSquareType::FillAlpha;
        let mut card = card();
        card[(1, 2)] = CardSquareType::Empty;

        assert!(MoveValidator::card_on_blank_spaces(
            &player_move(INamedPosition::new(0, 0), CardRotation::Deg0),
            &board,
            &card))
    }

    #[test]
    fn card_on_blank_spaces_false() {
        let mut board = board();
        board[(1, 2)] = MapSquareType::FillAlpha;

        assert!(!MoveValidator::card_on_blank_spaces(
            &player_move(INamedPosition::new(0, 0), CardRotation::Deg0),
            &board,
            &card()))
    }

    #[test]
    fn map_squares_near_card() {
        let mut board = board();
        board[(2, 2)] = MapSquareType::FillAlpha;
        board[(0, 0)] = MapSquareType::FillBravo;

        assert!(MoveValidator::map_squares_near_card(
            &player_move(INamedPosition::new(0, 0), CardRotation::Deg0),
            &board,
            &card(),
            &PlayerTeam::Alpha))
    }
}
