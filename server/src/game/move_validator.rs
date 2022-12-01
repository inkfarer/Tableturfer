use std::sync::Arc;
use serde::Serialize;
use crate::game::card::{CardProvider, CardSquareType};
use crate::game::squares::{MapSquareType, MST};
use crate::game::state::PlayerMove;
use crate::game::team::PlayerTeam;
use crate::matrix::{Matrix, Slice};

#[derive(Serialize, Debug, Eq, PartialEq)]
pub enum InvalidMoveError {
    CardNotFound,
    CannotAffordSpecial,
    CardOutOfBounds,
    CardOnDisallowedSquares,
    NoExpectedSquaresNearCard,
}

pub trait MoveValidator {
    fn validate(
        &self,
        board: &Matrix<MapSquareType>,
        available_special_points: usize,
        team: &PlayerTeam,
        player_move: &PlayerMove,
    ) -> Result<(), InvalidMoveError>;
}

pub struct MoveValidatorImpl {
    card_square_provider: Arc<dyn CardProvider + Send + Sync>,
}

impl MoveValidator for MoveValidatorImpl {
    fn validate(
        &self,
        board: &Matrix<MapSquareType>,
        available_special_points: usize,
        team: &PlayerTeam,
        player_move: &PlayerMove
    ) -> Result<(), InvalidMoveError> {
        match self.card_square_provider.get(&player_move.card_name) {
            Some(card) => {
                if player_move.special && card.special_cost > available_special_points {
                    return Err(InvalidMoveError::CannotAffordSpecial);
                }

                let squares = card.squares.rotate_clockwise(player_move.rotation.into());

                if !Self::card_within_bounds(player_move, board, &squares) {
                    Err(InvalidMoveError::CardOutOfBounds)
                } else if !Self::card_on_correct_squares(player_move, board, &squares) {
                    Err(InvalidMoveError::CardOnDisallowedSquares)
                } else if !Self::correct_squares_near_card(player_move, board, &squares, team) {
                    Err(InvalidMoveError::NoExpectedSquaresNearCard)
                } else {
                    Ok(())
                }
            }
            None => {
                Err(InvalidMoveError::CardNotFound)
            }
        }
    }
}

impl MoveValidatorImpl {
    pub fn new(card_square_provider: Arc<dyn CardProvider + Send + Sync>) -> Self {
        Self {
            card_square_provider,
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

    fn card_on_correct_squares(player_move: &PlayerMove, board: &Matrix<MapSquareType>, card_squares: &Matrix<CardSquareType>) -> bool {
        let card_size = card_squares.size();
        let pos_from = (player_move.position.x as usize, player_move.position.y as usize);
        let squares_under_card = board.slice(pos_from..(pos_from.0 + card_size.w, pos_from.1 + card_size.h));
        let accepted_covering_map_squares = if player_move.special {
            vec!(MST::Empty, MST::FillAlpha, MST::FillBravo)
        } else {
            vec!(MST::Empty)
        };

        squares_under_card.into_iter()
            .zip(card_squares.clone().into_iter())
            .all(|((map_square, _), (card_square, _))| {
                if card_square == CardSquareType::Empty {
                    true
                } else {
                    accepted_covering_map_squares.contains(&map_square)
                }
            })
    }

    fn correct_squares_near_card(player_move: &PlayerMove, board: &Matrix<MapSquareType>, card_squares: &Matrix<CardSquareType>, team: &PlayerTeam) -> bool {
        let pos_from = (player_move.position.x as usize, player_move.position.y as usize);

        let accepted_nearby_squares = match team {
            PlayerTeam::Alpha => if player_move.special {
                vec!(MapSquareType::SpecialAlpha)
            } else {
                vec!(MapSquareType::FillAlpha, MapSquareType::SpecialAlpha)
            },
            PlayerTeam::Bravo => if player_move.special {
                vec!(MapSquareType::SpecialBravo)
            } else {
                vec!(MapSquareType::FillBravo, MapSquareType::SpecialBravo)
            },
        };

        card_squares.clone().into_iter()
            .any(|(square, position)| {
                if square == CardSquareType::Empty {
                    return false;
                }

                let square_pos = (pos_from.0 + position.0, pos_from.1 + position.1);
                board.slice((square_pos.0.checked_sub(1).unwrap_or(0), square_pos.1.checked_sub(1).unwrap_or(0))..=(square_pos.0 + 1, square_pos.1 + 1))
                    .into_iter()
                    .any(|(map_square, _)| accepted_nearby_squares.contains(&map_square))
            })
    }
}

#[cfg(test)]
pub mod tests {
    use std::borrow::Borrow;
    use parameterized::parameterized as pm;
    use crate::game::squares::MST;
    use crate::game::state::CardRotation;
    use crate::game::card::tests::TestCardSquareProvider;
    use crate::position::INamedPosition;
    use super::*;

    pub struct TestMoveValidator {}

    impl MoveValidator for TestMoveValidator {
        fn validate(
            &self,
            _board: &Matrix<MapSquareType>,
            _available_special_points: usize,
            _team: &PlayerTeam,
            player_move: &PlayerMove
        ) -> Result<(), InvalidMoveError> {
            match player_move.card_name.borrow() {
                "invalid_pos_card" => Err(InvalidMoveError::CardOutOfBounds),
                "not_found_card" => Err(InvalidMoveError::CardNotFound),
                _ => Ok(()),
            }
        }
    }

    fn player_move(card_name: &str, position: INamedPosition, rotation: CardRotation) -> PlayerMove {
        PlayerMove {
            card_name: card_name.to_owned(),
            position,
            rotation,
            special: false,
        }
    }

    fn board() -> Matrix<MapSquareType> {
        Matrix::new(vec!(
            vec!(MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled),
            vec!(MST::Disabled, MST::SpecialAlpha, MST::Empty, MST::Empty, MST::Empty, MST::FillAlpha, MST::Disabled),
            vec!(MST::Disabled, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Disabled),
            vec!(MST::Disabled, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Disabled),
            vec!(MST::Disabled, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Disabled),
            vec!(MST::Disabled, MST::SpecialBravo, MST::Empty, MST::Empty, MST::Empty, MST::FillBravo, MST::Disabled),
            vec!(MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled),
        ))
    }

    #[test]
    fn validate_card_not_found() {
        let result = MoveValidatorImpl::new(TestCardSquareProvider::new()).validate(
            &board(),
            0,
            &PlayerTeam::Alpha,
            &player_move("card_999", INamedPosition::new(0, 0), CardRotation::Deg0));

        assert_eq!(result, Err(InvalidMoveError::CardNotFound));
    }

    macro_rules! common_team_tests {
        {$team:expr} => {
            #[pm(
                x = { 2, 2, -2, 12 },
                y = { -3, 15, 2, 2 }
            )]
            fn validate_out_of_bounds(x: isize, y: isize) {
                assert_eq!(MoveValidatorImpl::new(TestCardSquareProvider::new()).validate(
                    &board(),
                    0,
                    $team,
                    &player_move("card_1", INamedPosition::new(x, y), CardRotation::Deg0)), Err(InvalidMoveError::CardOutOfBounds));
            }

            #[pm(
                x = { 1, 0, 5, 2 },
                y = { 0, 2, 2, 4 }
            )]
            fn validate_card_on_disabled_tiles(x: isize, y: isize) {
                assert_eq!(MoveValidatorImpl::new(TestCardSquareProvider::new()).validate(
                    &board(),
                    0,
                    $team,
                    &player_move("card_1", INamedPosition::new(x, y), CardRotation::Deg0)), Err(InvalidMoveError::CardOnDisallowedSquares));
            }

            #[pm(
                x = { 1, 5, 3, 3, 3 },
                y = { 3, 3, 1, 5, 3 }
            )]
            fn validate_no_adjacent_tiles(x: isize, y: isize) {
                assert_eq!(MoveValidatorImpl::new(TestCardSquareProvider::new()).validate(
                    &board(),
                    0,
                    $team,
                    &player_move("card_3", INamedPosition::new(x, y), CardRotation::Deg0)), Err(InvalidMoveError::NoExpectedSquaresNearCard));
            }

            #[pm(
                x = { 1, 4, 1, 4 },
                y = { 1, 1, 4, 4 }
            )]
            fn validate_covers_existing_tiles(x: isize, y: isize) {
                assert_eq!(MoveValidatorImpl::new(TestCardSquareProvider::new()).validate(
                    &board(),
                    0,
                    $team,
                    &player_move("card_2", INamedPosition::new(x, y), CardRotation::Deg0)), Err(InvalidMoveError::CardOnDisallowedSquares));
            }

            #[test]
            fn validate_special_too_expensive() {
                let mut player_move = player_move("card_2", INamedPosition::new(0, 0), CardRotation::Deg0);
                player_move.special = true;

                assert_eq!(MoveValidatorImpl::new(TestCardSquareProvider::new()).validate(
                    &board(),
                    0,
                    $team,
                    &player_move), Err(InvalidMoveError::CannotAffordSpecial));
            }
        }
    }

    mod team_alpha {
        use super::*;

        common_team_tests!(&PlayerTeam::Alpha);

        #[pm(
            x = { 2, 3 },
            y = { 3, 3 }
        )]
        fn validate_next_to_opposing_team_squares(x: isize, y: isize) {
            assert_eq!(MoveValidatorImpl::new(TestCardSquareProvider::new()).validate(
                &board(),
                0,
                &PlayerTeam::Alpha,
                &player_move("card_1", INamedPosition::new(x, y), CardRotation::Deg0)), Err(InvalidMoveError::NoExpectedSquaresNearCard));
        }

        #[pm(
            x = { 1, 3 },
            y = { 1, 1 }
        )]
        fn validate_next_to_own_team_squares(x: isize, y: isize) {
            assert_eq!(MoveValidatorImpl::new(TestCardSquareProvider::new()).validate(
                &board(),
                0,
                &PlayerTeam::Alpha,
                &player_move("card_1", INamedPosition::new(x, y), CardRotation::Deg0)), Ok(()));
        }

        #[test]
        fn validate_can_afford_special() {
            let mut player_move = player_move("card_1", INamedPosition::new(1, 1), CardRotation::Deg0);
            player_move.special = true;
            let mut board = board();
            board[(2, 1)] = MapSquareType::FillAlpha;
            board[(2, 2)] = MapSquareType::FillBravo;

            assert_eq!(MoveValidatorImpl::new(TestCardSquareProvider::new()).validate(
                &board,
                2,
                &PlayerTeam::Alpha,
                &player_move), Ok(()));
        }

        #[pm(
            x = { 2, 3, 3 },
            y = { 3, 1, 3 }
        )]
        fn validate_special_next_to_invalid_squares(x: isize, y: isize) {
            let mut player_move = player_move("card_1", INamedPosition::new(x, y), CardRotation::Deg0);
            player_move.special = true;

            assert_eq!(MoveValidatorImpl::new(TestCardSquareProvider::new()).validate(
                &board(),
                2,
                &PlayerTeam::Alpha,
                &player_move), Err(InvalidMoveError::NoExpectedSquaresNearCard));
        }
    }

    mod team_bravo {
        use super::*;

        common_team_tests!(&PlayerTeam::Bravo);

        #[pm(
            x = { 1, 3 },
            y = { 1, 1 }
        )]
        fn validate_next_to_opposing_team_squares(x: isize, y: isize) {
            assert_eq!(MoveValidatorImpl::new(TestCardSquareProvider::new()).validate(
                &board(),
                0,
                &PlayerTeam::Bravo,
                &player_move("card_1", INamedPosition::new(x, y), CardRotation::Deg0)), Err(InvalidMoveError::NoExpectedSquaresNearCard));
        }

        #[pm(
            x = { 2, 3 },
            y = { 3, 3 }
        )]
        fn validate_next_to_own_team_squares(x: isize, y: isize) {
            assert_eq!(MoveValidatorImpl::new(TestCardSquareProvider::new()).validate(
                &board(),
                0,
                &PlayerTeam::Bravo,
                &player_move("card_1", INamedPosition::new(x, y), CardRotation::Deg0)), Ok(()));
        }

        #[test]
        fn validate_can_afford_special() {
            let mut player_move = player_move("card_1", INamedPosition::new(2, 3), CardRotation::Deg0);
            player_move.special = true;
            let mut board = board();
            board[(3, 3)] = MapSquareType::FillAlpha;
            board[(3, 4)] = MapSquareType::FillBravo;

            assert_eq!(MoveValidatorImpl::new(TestCardSquareProvider::new()).validate(
                &board,
                2,
                &PlayerTeam::Bravo,
                &player_move), Ok(()));
        }

        #[pm(
            x = { 1, 3, 3 },
            y = { 1, 1, 3 }
        )]
        fn validate_special_next_to_invalid_squares(x: isize, y: isize) {
            let mut player_move = player_move("card_1", INamedPosition::new(x, y), CardRotation::Deg0);
            player_move.special = true;

            assert_eq!(MoveValidatorImpl::new(TestCardSquareProvider::new()).validate(
                &board(),
                2,
                &PlayerTeam::Bravo,
                &player_move), Err(InvalidMoveError::NoExpectedSquaresNearCard));
        }
    }
}
