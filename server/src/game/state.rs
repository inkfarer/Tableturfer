use std::borrow::Borrow;
use std::collections::HashMap;
use std::ops::{AddAssign, SubAssign};
use std::sync::Arc;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::EnumCount;
use itertools::Itertools;
use rand::prelude::IteratorRandom;
use crate::game::card::{Card, CardProvider, CardSquareType};
use crate::game::move_validator::{InvalidMoveError, MoveValidator};
use crate::game::squares::MapSquareType;
use crate::game::team::PlayerTeam;
use crate::matrix::{Matrix, MatrixRotation, Slice};
use crate::position::{INamedPosition, UNamedPosition};

pub const HAND_SIZE: usize = 4;
pub const DECK_SIZE: usize = 15;
pub const TURN_COUNT: usize = DECK_SIZE - (HAND_SIZE - 1);

#[derive(Serialize, Debug, Eq, PartialEq)]
#[serde(tag = "code", content = "detail")]
pub enum GameError {
    InvalidMove(InvalidMoveError),
    CardNotFound,
    IncorrectDeckSize,
    GameEnded,
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
#[serde(tag = "type")]
pub enum PlayerMove {
    #[serde(rename_all = "camelCase")]
    PlaceCard {
        card_name: String,
        position: INamedPosition,
        rotation: CardRotation,
        special: bool,
    },
    #[serde(rename_all = "camelCase")]
    Pass { card_name: String },
}

impl PlayerMove {
    pub fn card_name(&self) -> &str {
        match self {
            PlayerMove::PlaceCard { card_name, .. } => card_name,
            PlayerMove::Pass { card_name } => card_name,
        }
    }
}

#[derive(Clone)]
pub struct PlayerDeck {
    pub cards: IndexSet<String>,
    pub used_cards: IndexSet<String>,
    pub current_hand: IndexSet<String>,
}

impl PlayerDeck {
    pub fn new(cards: IndexSet<String>) -> Self {
        Self {
            cards,
            used_cards: IndexSet::new(),
            current_hand: IndexSet::new(),
        }
    }

    fn available_cards(&self) -> IndexSet<&String> {
        self.cards.difference(&self.used_cards).collect()
    }

    fn upcoming_cards(&self) -> IndexSet<String> {
        let used_or_available_cards: IndexSet<String> = self.used_cards.union(&self.current_hand).map(|str| str.to_string()).collect();
        self.cards.difference(&used_or_available_cards).map(|str| str.to_owned()).collect()
    }

    pub fn assign_cards(&mut self) -> &IndexSet<String> {
        let mut rng = rand::thread_rng();
        self.current_hand = self.available_cards().into_iter()
            .choose_multiple(&mut rng, HAND_SIZE).iter()
            .map(|card| card.to_string())
            .collect();

        &self.current_hand
    }

    pub fn draw_new_card(&mut self, card_to_replace: &str) -> Option<String> {
        self.used_cards.insert(card_to_replace.to_string());

        self.current_hand.remove(card_to_replace);
        let upcoming_cards = self.upcoming_cards();
        if upcoming_cards.is_empty() {
            None
        } else {
            let mut rng = rand::thread_rng();
            let new_card = upcoming_cards.iter().choose(&mut rng).unwrap().to_string();
            self.current_hand.insert(new_card.clone());
            Some(new_card)
        }
    }
}

struct AugmentedPlayerMove {
    player_move: PlayerMove,
    card: Card,
    card_square_count: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ApplyMovesResult {
    pub applied_moves: HashMap<PlayerTeam, PlayerMove>,
    pub next_cards: HashMap<PlayerTeam, String>,
}

#[derive(Clone)]
pub struct GameState {
    pub board: Matrix<MapSquareType>,
    next_moves: HashMap<PlayerTeam, PlayerMove>,
    // todo: for cleanup, maybe have a separate ScoreCounter struct that keeps track of these?
    special_points: HashMap<PlayerTeam, usize>,
    used_special_points: HashMap<PlayerTeam, usize>,
    decks: HashMap<PlayerTeam, PlayerDeck>,
    remaining_turns: usize,

    square_provider: Arc<dyn CardProvider + Send + Sync>,
    move_validator: Arc<dyn MoveValidator + Send + Sync>,
}

impl GameState {
    pub fn new(
        board: Matrix<MapSquareType>,
        square_provider: Arc<dyn CardProvider + Send + Sync>,
        move_validator: Arc<dyn MoveValidator + Send + Sync>,
        decks: HashMap<PlayerTeam, IndexSet<String>>,
    ) -> Self {
        Self {
            board,
            next_moves: HashMap::new(),
            special_points: Self::score_counter(),
            used_special_points: Self::score_counter(),
            decks: decks.into_iter().map(|(team, cards)| (team, PlayerDeck::new(cards))).collect(),
            remaining_turns: TURN_COUNT.to_owned(),
            square_provider,
            move_validator,
        }
    }

    fn score_counter() -> HashMap<PlayerTeam, usize> {
        HashMap::from([(PlayerTeam::Alpha, 0), (PlayerTeam::Bravo, 0)])
    }

    pub fn completed(&self) -> bool {
        self.remaining_turns <= 0
    }

    pub fn score(&self) -> HashMap<PlayerTeam, usize> {
        let mut result = HashMap::new();

        result.insert(PlayerTeam::Alpha, self.board.clone().into_iter()
            .filter(|(square, _)| square == &MapSquareType::SpecialAlpha || square == &MapSquareType::FillAlpha)
            .count());
        result.insert(PlayerTeam::Bravo, self.board.clone().into_iter()
            .filter(|(square, _)| square == &MapSquareType::SpecialBravo || square == &MapSquareType::FillBravo)
            .count());

        result
    }

    pub fn assign_initial_hands(&mut self) -> HashMap<PlayerTeam, IndexSet<String>> {
        self.decks.iter_mut().map(|(team, deck)| (team.clone(), deck.assign_cards().clone())).collect()
    }

    pub fn propose_move(&mut self, team: PlayerTeam, player_move: PlayerMove) -> Result<(), GameError> {
        if self.remaining_turns <= 0 {
            return Err(GameError::GameEnded);
        }

        match self.move_validator.validate(
            &self.board,
            self.available_special_points(&team),
            &team,
            &player_move,
            &self.decks[&team]
        ) {
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

    pub fn apply_moves(&mut self) -> ApplyMovesResult {
        let moves = std::mem::take(&mut self.next_moves);
        let mut board_updates = Matrix::filled_with(self.board.size(), MapSquareType::Empty);

        let augmented_moves: HashMap<PlayerTeam, AugmentedPlayerMove> = moves.into_iter()
            .map(|(team, player_move)| {
                let card = self.square_provider.get(&player_move.card_name()).unwrap();

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
        let mut next_cards: HashMap<PlayerTeam, String> = HashMap::new();

        for (team, aug_move) in augmented_moves.iter()
            .sorted_by(|(_, a), (_, b)| Ord::cmp(&b.card_square_count, &a.card_square_count))
        {
            if let Some(next_card) = self.decks.get_mut(&team).unwrap().draw_new_card(&aug_move.card.name) {
                next_cards.insert(team.clone(), next_card);
            }

            if let PlayerMove::PlaceCard { position, special, rotation, .. } = aug_move.player_move.borrow() {
                if *special {
                    self.used_special_points.get_mut(&team).unwrap().add_assign(aug_move.card.special_cost);
                }

                let move_pos: UNamedPosition = position.clone().try_into().unwrap();
                aug_move.card.squares.clone()
                    .rotate_clockwise(rotation.clone().into())
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
        }

        let mut new_board = self.board.clone();
        for (square, position) in board_updates.into_iter()
            .filter(|(square, _)| square != &MapSquareType::Empty)
        {
            new_board[position] = square;
        }
        self.update_board(new_board);

        self.remaining_turns.sub_assign(1);

        ApplyMovesResult {
            applied_moves: augmented_moves.into_iter().map(|(team, aug_move)| (team, aug_move.player_move)).collect(),
            next_cards,
        }
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
            HashMap::from([
                (PlayerTeam::Alpha, IndexSet::from(["card_1".to_string(), "card_2".to_string()])),
                (PlayerTeam::Bravo, IndexSet::from(["card_3".to_string(), "card_4".to_string()])),
            ]),
        )
    }

    mod apply_moves {
        use super::*;

        #[test]
        fn apply_moves() {
            let mut state = create();
            state.next_moves.insert(PlayerTeam::Alpha, PlayerMove::PlaceCard {
                card_name: "card_1".to_string(),
                position: INamedPosition::new(1, 1),
                rotation: CardRotation::Deg0,
                special: false,
            });
            state.next_moves.insert(PlayerTeam::Bravo, PlayerMove::PlaceCard {
                card_name: "card_2".to_string(),
                position: INamedPosition::new(0, 1),
                rotation: CardRotation::Deg90,
                special: false,
            });
            let moves = state.next_moves.clone();

            let result = state.apply_moves();

            assert_eq!(HashMap::new(), state.next_moves);
            assert_eq!(result.applied_moves, moves);
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
            state.next_moves.insert(PlayerTeam::Alpha, PlayerMove::PlaceCard {
                card_name: "card_1".to_string(),
                position: INamedPosition::new(1, 1),
                rotation: CardRotation::Deg0,
                special: false,
            });
            state.next_moves.insert(PlayerTeam::Bravo, PlayerMove::PlaceCard {
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
            state.next_moves.insert(PlayerTeam::Alpha, PlayerMove::PlaceCard {
                card_name: "card_1".to_string(),
                position: INamedPosition::new(1, 1),
                rotation: CardRotation::Deg180,
                special: false,
            });
            state.next_moves.insert(PlayerTeam::Bravo, PlayerMove::PlaceCard {
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
            state.next_moves.insert(PlayerTeam::Alpha, PlayerMove::PlaceCard {
                card_name: "card_1".to_string(),
                position: INamedPosition::new(1, 1),
                rotation: CardRotation::Deg0,
                special: false,
            });
            state.next_moves.insert(PlayerTeam::Bravo, PlayerMove::PlaceCard {
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
            state.next_moves.insert(PlayerTeam::Alpha, PlayerMove::PlaceCard {
                card_name: "card_1".to_string(),
                position: INamedPosition::new(1, 1),
                rotation: CardRotation::Deg0,
                special: false,
            });
            state.next_moves.insert(PlayerTeam::Bravo, PlayerMove::PlaceCard {
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
            state.next_moves.insert(PlayerTeam::Bravo, PlayerMove::PlaceCard {
                card_name: "card_4".to_string(),
                position: INamedPosition::new(1, 2),
                rotation: CardRotation::Deg0,
                special: false,
            });
            state.next_moves.insert(PlayerTeam::Alpha, PlayerMove::PlaceCard {
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
            state.next_moves.insert(PlayerTeam::Alpha, PlayerMove::PlaceCard {
                card_name: "card_1".to_string(),
                position: INamedPosition::new(1, 1),
                rotation: CardRotation::Deg0,
                special: false,
            });
            state.next_moves.insert(PlayerTeam::Bravo, PlayerMove::PlaceCard {
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
            state.next_moves.insert(PlayerTeam::Bravo, PlayerMove::PlaceCard {
                card_name: "card_4".to_string(),
                position: INamedPosition::new(1, 2),
                rotation: CardRotation::Deg180,
                special: false,
            });
            state.next_moves.insert(PlayerTeam::Alpha, PlayerMove::PlaceCard {
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
            state.next_moves.insert(PlayerTeam::Bravo, PlayerMove::PlaceCard {
                card_name: "card_4".to_string(),
                position: INamedPosition::new(1, 2),
                rotation: CardRotation::Deg180,
                special: true,
            });
            state.next_moves.insert(PlayerTeam::Alpha, PlayerMove::PlaceCard {
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

    mod player_deck {
        use super::*;

        fn set(items: Vec<&str>) -> IndexSet<String> {
            items.into_iter().map(|str| str.to_string()).collect()
        }

        #[test]
        fn available_cards() {
            let mut deck = PlayerDeck::new(set(vec!(
                "card_1",
                "card_2",
                "card_3",
                "card_4",
            )));

            assert_eq!(deck.available_cards(), IndexSet::from([
                &"card_1".to_owned(),
                &"card_2".to_owned(),
                &"card_3".to_owned(),
                &"card_4".to_owned(),
            ]));

            deck.used_cards = set(vec!(
                "card_2",
                "card_4",
            ));

            assert_eq!(deck.available_cards(), IndexSet::from([
                &"card_1".to_owned(),
                &"card_3".to_owned(),
            ]));
        }

        #[test]
        fn upcoming_cards() {
            let mut deck = PlayerDeck::new(set(vec!(
                "card_1",
                "card_2",
                "card_3",
                "card_4",
                "card_5",
            )));
            deck.current_hand = set(vec!(
                "card_1",
                "card_2",
            ));
            deck.used_cards = set(vec!(
                "card_2",
                "card_3",
            ));

            assert_eq!(deck.upcoming_cards(), set(vec!(
                "card_4",
                "card_5",
            )))
        }
    }
}
