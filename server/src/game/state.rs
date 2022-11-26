use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::EnumCount;
use itertools::Itertools;
use crate::game::card::{CardSquareProvider, CardSquareType};
use crate::game::move_validator::MoveValidator;
use crate::game::squares::MapSquareType;
use crate::game::team::PlayerTeam;
use crate::matrix::{Matrix, MatrixRotation};
use crate::position::{INamedPosition, UNamedPosition};

#[derive(Serialize, Debug, Eq, PartialEq)]
#[serde(tag = "code", content = "detail")]
pub enum GameError {
    InvalidPosition,
    CardNotFound,
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
}

struct AugmentedPlayerMove {
    player_move: PlayerMove,
    card_squares: Matrix<CardSquareType>,
    card_square_count: usize,
}

#[derive(Clone)]
pub struct GameState {
    pub board: Matrix<MapSquareType>,
    next_moves: HashMap<PlayerTeam, PlayerMove>,
    square_provider: Arc<dyn CardSquareProvider + Send + Sync>,
    move_validator: Arc<dyn MoveValidator + Send + Sync>,
}

impl GameState {
    pub fn new(
        board: Matrix<MapSquareType>,
        square_provider: Arc<dyn CardSquareProvider + Send + Sync>,
        move_validator: Arc<dyn MoveValidator + Send + Sync>,
    ) -> Self {
        Self {
            board,
            next_moves: HashMap::new(),
            square_provider,
            move_validator,
        }
    }

    pub fn propose_move(&mut self, team: PlayerTeam, player_move: PlayerMove) -> Result<(), GameError> {
        match self.move_validator.validate(&self.board, &team, &player_move) {
            Ok(()) => {
                self.next_moves.insert(team, player_move);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn all_players_have_moved(&self) -> bool {
        self.next_moves.len() == PlayerTeam::COUNT
    }

    pub fn apply_moves(&mut self) -> HashMap<PlayerTeam, PlayerMove> {
        let moves = std::mem::take(&mut self.next_moves);
        let mut board_updates = Matrix::filled_with(self.board.size(), MapSquareType::Empty);

        let augmented_moves: HashMap<PlayerTeam, AugmentedPlayerMove> = moves.into_iter()
            .map(|(team, player_move)| {
                let card_squares = self.square_provider.get(&player_move.card_name).unwrap();

                (team,
                 AugmentedPlayerMove {
                     player_move,
                     card_square_count: card_squares.clone().into_iter()
                         .filter(|(square, _)| square != &CardSquareType::Empty)
                         .count(),
                     card_squares,
                 })
            })
            .collect();

        let square_counts_match = augmented_moves.values().into_iter()
            .map(|player_move| player_move.card_square_count)
            .all_equal();

        for (team, aug_move) in augmented_moves.iter()
            .sorted_by(|(_, a), (_, b)| Ord::cmp(&b.card_square_count, &a.card_square_count))
        {
            let move_pos: UNamedPosition = aug_move.player_move.position.clone().try_into().unwrap();
            aug_move.card_squares.clone()
                .rotate_clockwise(aug_move.player_move.rotation.into())
                .into_iter()
                .filter(|(item, _)| item != &CardSquareType::Empty)
                .for_each(|(item, position)| {
                    let board_position = (move_pos.x + position.0, move_pos.y + position.1);
                    let existing_square = board_updates[board_position];
                    let mut new_square = MapSquareType::from_card_square(item, team);

                    if square_counts_match {
                        if (existing_square.is_fill() && new_square.is_fill()) || (existing_square.is_special() && new_square.is_special()) {
                            new_square = MapSquareType::Neutral;
                        }
                    }

                    if existing_square.is_special() && new_square.is_fill() {
                        return;
                    }

                    board_updates[board_position] = new_square;
                });
        }

        for (square, position) in board_updates.into_iter()
            .filter(|(square, _)| square != &MapSquareType::Empty)
        {
            self.board[position] = square;
        }

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
            Arc::new(TestMoveValidator {})
        )
    }

    #[test]
    fn apply_moves() {
        let mut state = create();
        state.next_moves.insert(PlayerTeam::Alpha, PlayerMove {
            card_name: "card_1".to_string(),
            position: INamedPosition::new(1, 1),
            rotation: CardRotation::Deg0
        });
        state.next_moves.insert(PlayerTeam::Bravo, PlayerMove {
            card_name: "card_2".to_string(),
            position: INamedPosition::new(0, 1),
            rotation: CardRotation::Deg90
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
            rotation: CardRotation::Deg0
        });
        state.next_moves.insert(PlayerTeam::Bravo, PlayerMove {
            card_name: "card_2".to_string(),
            position: INamedPosition::new(1, 1),
            rotation: CardRotation::Deg90
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
            rotation: CardRotation::Deg180
        });
        state.next_moves.insert(PlayerTeam::Bravo, PlayerMove {
            card_name: "card_1".to_string(),
            position: INamedPosition::new(1, 1),
            rotation: CardRotation::Deg180
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
            rotation: CardRotation::Deg0
        });
        state.next_moves.insert(PlayerTeam::Bravo, PlayerMove {
            card_name: "card_2".to_string(),
            position: INamedPosition::new(1, 1),
            rotation: CardRotation::Deg90
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
            rotation: CardRotation::Deg0
        });
        state.next_moves.insert(PlayerTeam::Bravo, PlayerMove {
            card_name: "card_4".to_string(),
            position: INamedPosition::new(1, 2),
            rotation: CardRotation::Deg0
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
            rotation: CardRotation::Deg0
        });
        state.next_moves.insert(PlayerTeam::Alpha, PlayerMove {
            card_name: "card_1".to_string(),
            position: INamedPosition::new(1, 1),
            rotation: CardRotation::Deg0
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
            rotation: CardRotation::Deg0
        });
        state.next_moves.insert(PlayerTeam::Bravo, PlayerMove {
            card_name: "card_4".to_string(),
            position: INamedPosition::new(1, 2),
            rotation: CardRotation::Deg180
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
            rotation: CardRotation::Deg180
        });
        state.next_moves.insert(PlayerTeam::Alpha, PlayerMove {
            card_name: "card_1".to_string(),
            position: INamedPosition::new(1, 1),
            rotation: CardRotation::Deg0
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
}
