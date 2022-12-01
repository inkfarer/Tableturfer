use std::collections::HashMap;
use std::ops::AddAssign;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::EnumCount;
use itertools::Itertools;
use crate::game::card::{Card, CardProvider, CardSquareType};
use crate::game::move_validator::{InvalidMoveError, MoveValidator};
use crate::game::squares::MapSquareType;
use crate::game::team::PlayerTeam;
use crate::matrix::{Matrix, MatrixRotation, Slice};
use crate::position::{INamedPosition, UNamedPosition};

#[derive(Serialize, Debug, Eq, PartialEq)]
#[serde(tag = "code", content = "detail")]
pub enum GameError {
    InvalidMove(InvalidMoveError),
}

#[derive(Clone, Copy, Debug, Serialize_repr, Deserialize_repr, Eq, PartialEq)]
#[repr(u16)]
pub enum CardRotation {
    Deg0 = 0,
    Deg90 = 90,
    Deg180 = 180,
    Deg270 = 270,
}

impl From<CardRotation> for MatrixRotation {
    fn from(r: CardRotation) -> Self {
        match r {
            CardRotation::Deg0 => MatrixRotation::None,
            CardRotation::Deg90 => MatrixRotation::Deg90,
            CardRotation::Deg180 => MatrixRotation::Deg180,
            CardRotation::Deg270 => MatrixRotation::Deg270,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMove {
    pub card_name: String,
    pub position: INamedPosition,
    pub rotation: CardRotation,
    pub special: bool,
}

struct AugmentedPlayerMove {
    player_move: PlayerMove,
    card: Card,
    card_square_count: usize,
}

#[derive(Clone)]
pub struct GameState {
    pub board: Matrix<MapSquareType>,
    next_moves: HashMap<PlayerTeam, PlayerMove>,
    // todo: for cleanup, maybe have a separate ScoreCounter struct that keeps track of these?
    special_points: HashMap<PlayerTeam, usize>,
    used_special_points: HashMap<PlayerTeam, usize>,
    square_provider: Arc<dyn CardProvider + Send + Sync>,
    move_validator: Arc<dyn MoveValidator + Send + Sync>,
}

impl GameState {
    pub fn new(
        board: Matrix<MapSquareType>,
        square_provider: Arc<dyn CardProvider + Send + Sync>,
        move_validator: Arc<dyn MoveValidator + Send + Sync>,
    ) -> Self {
        Self {
            board,
            next_moves: HashMap::new(),
            special_points: Self::score_counter(),
            used_special_points: Self::score_counter(),
            square_provider,
            move_validator,
        }
    }

    fn score_counter() -> HashMap<PlayerTeam, usize> {
        HashMap::from([(PlayerTeam::Alpha, 0), (PlayerTeam::Bravo, 0)])
    }

    pub fn propose_move(&mut self, team: PlayerTeam, player_move: PlayerMove) -> Result<(), GameError> {
        match self.move_validator.validate(&self.board, self.available_special_points(&team), &team, &player_move) {
            Ok(()) => {
                self.next_moves.insert(team, player_move);
                Ok(())
            }
            Err(err) => Err(GameError::InvalidMove(err)),
        }
    }

    fn available_special_points(&self, team: &PlayerTeam) -> usize {
        self.special_points[team].saturating_sub(self.used_special_points[team])
    }

    pub fn all_players_have_moved(&self) -> bool {
        self.next_moves.len() == PlayerTeam::COUNT
    }

    fn update_board(&mut self, board: Matrix<MapSquareType>) {
        let mut new_special_points = Self::score_counter();
        let board_size = board.size();

        board.clone().into_iter()
            .filter(|(square, _)| square.is_special())
            .for_each(|(square, position)| {
                let x_from = if position.0 == 0 { 0 } else { position.0 - 1 };
                let y_from = if position.1 == 0 { 0 } else { position.1 - 1 };
                let x_to = if position.0 >= board_size.w - 1 { position.0 } else { position.0 + 1 };
                let y_to = if position.1 >= board_size.h - 1 { position.1 } else { position.1 + 1 };

                let squares_around = board.slice((x_from, y_from)..=(x_to, y_to));
                if squares_around.into_iter().all(|(square, _)| square != MapSquareType::Empty) {
                    if square == MapSquareType::SpecialAlpha {
                        new_special_points.get_mut(&PlayerTeam::Alpha).unwrap().add_assign(1);
                    } else if square == MapSquareType::SpecialBravo {
                        new_special_points.get_mut(&PlayerTeam::Bravo).unwrap().add_assign(1);
                    }
                }
            });

        self.special_points = new_special_points;
        self.board = board;
    }

    pub fn apply_moves(&mut self) -> HashMap<PlayerTeam, PlayerMove> {
        let moves = std::mem::take(&mut self.next_moves);
        let mut board_updates = Matrix::filled_with(self.board.size(), MapSquareType::Empty);

        let augmented_moves: HashMap<PlayerTeam, AugmentedPlayerMove> = moves.into_iter()
            .map(|(team, player_move)| {
                let card = self.square_provider.get(&player_move.card_name).unwrap();

                (team,
                 AugmentedPlayerMove {
                     player_move,
                     card_square_count: card.squares.clone().into_iter()
                         .filter(|(square, _)| square != &CardSquareType::Empty)
                         .count(),
                     card,
                 })
            })
            .collect();

        let square_counts_match = augmented_moves.values().into_iter()
            .map(|player_move| player_move.card_square_count)
            .all_equal();

        for (team, aug_move) in augmented_moves.iter()
            .sorted_by(|(_, a), (_, b)| Ord::cmp(&b.card_square_count, &a.card_square_count))
        {
            if aug_move.player_move.special {
                self.used_special_points.get_mut(&team).unwrap().add_assign(aug_move.card.special_cost);
            }

            let move_pos: UNamedPosition = aug_move.player_move.position.clone().try_into().unwrap();
            aug_move.card.squares.clone()
                .rotate_clockwise(aug_move.player_move.rotation.into())
                .into_iter()
                .filter(|(item, _)| item != &CardSquareType::Empty)
                .for_each(|(item, position)| {
                    let board_position = (move_pos.x + position.0, move_pos.y + position.1);
                    let existing_square = board_updates[board_position];
                    let mut new_square = MapSquareType::from_card_square(item, team);

                    if existing_square.is_special() && new_square.is_fill() {
                        return;
                    }

                    if square_counts_match
                        && ((existing_square.is_fill() && new_square.is_fill())
                        || (existing_square.is_special() && new_square.is_special()))
                    {
                        new_square = MapSquareType::Neutral;
                    }

                    board_updates[board_position] = new_square;
                });
        }

        let mut new_board = self.board.clone();
        for (square, position) in board_updates.into_iter()
            .filter(|(square, _)| square != &MapSquareType::Empty)
        {
            new_board[position] = square;
        }
        self.update_board(new_board);

        augmented_moves.into_iter().map(|(team, aug_move)| (team, aug_move.player_move)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::card::tests::TestCardSquareProvider;
    use crate::game::move_validator::tests::TestMoveValidator;
    use crate::game::squares::MST;
    use crate::matrix::MatrixSize;

    fn create() -> GameState {
        GameState::new(
            Matrix::filled_with(MatrixSize::new(6, 6), MST::Empty),
            TestCardSquareProvider::new().clone(),
            Arc::new(TestMoveValidator {}),
        )
    }

    mod apply_moves {
        use super::*;

        #[test]
        fn apply_moves() {
            let mut state = create();
            state.next_moves.insert(PlayerTeam::Alpha, PlayerMove {
                card_name: "card_1".to_string(),
                position: INamedPosition::new(1, 1),
                rotation: CardRotation::Deg0,
                special: false,
            });
            state.next_moves.insert(PlayerTeam::Bravo, PlayerMove {
                card_name: "card_2".to_string(),
                position: INamedPosition::new(0, 1),
                rotation: CardRotation::Deg90,
                special: false,
            });
            let moves = state.next_moves.clone();

            let result = state.apply_moves();

            assert_eq!(HashMap::new(), state.next_moves);
            assert_eq!(result, moves);
            assert_eq!(state.board, Matrix::new(vec!(
                vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::FillBravo, MST::FillBravo, MST::FillAlpha, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::SpecialBravo, MST::FillBravo, MST::SpecialAlpha, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::FillAlpha, MST::FillAlpha, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
            )));
        }

        #[test]
        fn apply_overlapping_moves_same_card_cost() {
            let mut state = create();
            state.next_moves.insert(PlayerTeam::Alpha, PlayerMove {
                card_name: "card_1".to_string(),
                position: INamedPosition::new(1, 1),
                rotation: CardRotation::Deg0,
                special: false,
            });
            state.next_moves.insert(PlayerTeam::Bravo, PlayerMove {
                card_name: "card_2".to_string(),
                position: INamedPosition::new(1, 1),
                rotation: CardRotation::Deg90,
                special: false,
            });

            state.apply_moves();

            assert_eq!(state.board, Matrix::new(vec!(
                vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::FillBravo, MST::Neutral, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::SpecialBravo, MST::SpecialAlpha, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::FillAlpha, MST::FillAlpha, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
            )));
        }

        #[test]
        fn apply_identical_moves() {
            let mut state = create();
            state.next_moves.insert(PlayerTeam::Alpha, PlayerMove {
                card_name: "card_1".to_string(),
                position: INamedPosition::new(1, 1),
                rotation: CardRotation::Deg180,
                special: false,
            });
            state.next_moves.insert(PlayerTeam::Bravo, PlayerMove {
                card_name: "card_1".to_string(),
                position: INamedPosition::new(1, 1),
                rotation: CardRotation::Deg180,
                special: false,
            });

            state.apply_moves();

            assert_eq!(state.board, Matrix::new(vec!(
                vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::Neutral, MST::Neutral, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::Neutral, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::Neutral, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
            )));
        }

        #[test]
        fn apply_overlapping_moves_over_existing_squares() {
            let mut state = create();
            state.board[(0, 0)] = MST::FillBravo;
            state.board[(2, 1)] = MST::FillAlpha;
            state.next_moves.insert(PlayerTeam::Alpha, PlayerMove {
                card_name: "card_1".to_string(),
                position: INamedPosition::new(1, 1),
                rotation: CardRotation::Deg0,
                special: false,
            });
            state.next_moves.insert(PlayerTeam::Bravo, PlayerMove {
                card_name: "card_2".to_string(),
                position: INamedPosition::new(1, 1),
                rotation: CardRotation::Deg90,
                special: false,
            });

            state.apply_moves();

            assert_eq!(state.board, Matrix::new(vec!(
                vec!(MST::FillBravo, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::FillBravo, MST::Neutral, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::SpecialBravo, MST::SpecialAlpha, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::FillAlpha, MST::FillAlpha, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
            )));
        }

        #[test]
        fn apply_overlapping_moves() {
            let mut state = create();
            state.next_moves.insert(PlayerTeam::Alpha, PlayerMove {
                card_name: "card_1".to_string(),
                position: INamedPosition::new(1, 1),
                rotation: CardRotation::Deg0,
                special: false,
            });
            state.next_moves.insert(PlayerTeam::Bravo, PlayerMove {
                card_name: "card_4".to_string(),
                position: INamedPosition::new(1, 2),
                rotation: CardRotation::Deg0,
                special: false,
            });

            state.apply_moves();

            assert_eq!(state.board, Matrix::new(vec!(
                vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::Empty, MST::FillAlpha, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::FillBravo, MST::SpecialAlpha, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::SpecialBravo, MST::FillAlpha, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
            )));
        }

        #[test]
        fn apply_overlapping_moves_ignores_insertion_order() {
            let mut state = create();
            state.next_moves.insert(PlayerTeam::Bravo, PlayerMove {
                card_name: "card_4".to_string(),
                position: INamedPosition::new(1, 2),
                rotation: CardRotation::Deg0,
                special: false,
            });
            state.next_moves.insert(PlayerTeam::Alpha, PlayerMove {
                card_name: "card_1".to_string(),
                position: INamedPosition::new(1, 1),
                rotation: CardRotation::Deg0,
                special: false,
            });

            state.apply_moves();

            assert_eq!(state.board, Matrix::new(vec!(
                vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::Empty, MST::FillAlpha, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::FillBravo, MST::SpecialAlpha, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::SpecialBravo, MST::FillAlpha, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
            )));
        }

        #[test]
        fn apply_overlapping_moves_and_special_squares() {
            let mut state = create();
            state.next_moves.insert(PlayerTeam::Alpha, PlayerMove {
                card_name: "card_1".to_string(),
                position: INamedPosition::new(1, 1),
                rotation: CardRotation::Deg0,
                special: false,
            });
            state.next_moves.insert(PlayerTeam::Bravo, PlayerMove {
                card_name: "card_4".to_string(),
                position: INamedPosition::new(1, 2),
                rotation: CardRotation::Deg180,
                special: false,
            });

            state.apply_moves();

            assert_eq!(state.board, Matrix::new(vec!(
                vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::Empty, MST::FillAlpha, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::Empty, MST::SpecialBravo, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::FillBravo, MST::FillBravo, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
            )));
        }

        #[test]
        fn apply_overlapping_moves_and_special_squares_ignores_insertion_order() {
            let mut state = create();
            state.next_moves.insert(PlayerTeam::Bravo, PlayerMove {
                card_name: "card_4".to_string(),
                position: INamedPosition::new(1, 2),
                rotation: CardRotation::Deg180,
                special: false,
            });
            state.next_moves.insert(PlayerTeam::Alpha, PlayerMove {
                card_name: "card_1".to_string(),
                position: INamedPosition::new(1, 1),
                rotation: CardRotation::Deg0,
                special: false,
            });

            state.apply_moves();

            assert_eq!(state.board, Matrix::new(vec!(
                vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::Empty, MST::FillAlpha, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::Empty, MST::SpecialBravo, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::FillBravo, MST::FillBravo, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
            )));
        }

        #[test]
        fn apply_special_moves() {
            let mut state = create();
            state.used_special_points = HashMap::from([(PlayerTeam::Alpha, 1), (PlayerTeam::Bravo, 2)]);
            state.next_moves.insert(PlayerTeam::Bravo, PlayerMove {
                card_name: "card_4".to_string(),
                position: INamedPosition::new(1, 2),
                rotation: CardRotation::Deg180,
                special: true,
            });
            state.next_moves.insert(PlayerTeam::Alpha, PlayerMove {
                card_name: "card_1".to_string(),
                position: INamedPosition::new(1, 1),
                rotation: CardRotation::Deg0,
                special: true,
            });

            state.apply_moves();

            assert_eq!(state.used_special_points, HashMap::from([(PlayerTeam::Alpha, 3), (PlayerTeam::Bravo, 5)]));
        }
    }

    mod update_board {
        use super::*;

        #[test]
        fn updates_special_counts() {
            let mut state = create();
            let mut new_board = Matrix::filled_with(MatrixSize::new(6, 6), MST::Empty);
            new_board[(0, 0)] = MST::FillAlpha;
            new_board[(0, 1)] = MST::Disabled;
            new_board[(0, 2)] = MST::Neutral;
            new_board[(1, 0)] = MST::FillBravo;
            new_board[(1, 1)] = MST::SpecialAlpha;
            new_board[(1, 2)] = MST::SpecialBravo;
            new_board[(2, 0)] = MST::FillAlpha;
            new_board[(2, 1)] = MST::FillAlpha;
            new_board[(2, 2)] = MST::FillAlpha;

            state.update_board(new_board.clone());

            assert_eq!(state.board, new_board);
            assert_eq!(state.special_points, HashMap::from([(PlayerTeam::Alpha, 1), (PlayerTeam::Bravo, 0)]));
        }

        #[test]
        fn handles_special_squares_on_any_position() {
            let mut state = create();
            let mut new_board = Matrix::filled_with(MatrixSize::new(4, 4), MST::SpecialAlpha);
            new_board[(1, 2)] = MST::SpecialBravo;

            state.update_board(new_board.clone());

            assert_eq!(state.board, new_board);
            assert_eq!(state.special_points, HashMap::from([(PlayerTeam::Alpha, 15), (PlayerTeam::Bravo, 1)]));
        }
    }
}
