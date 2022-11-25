use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::EnumCount;
use crate::game::card::CardSquareType;
use crate::game::move_validator::MoveValidator;
use crate::game::squares::MapSquareType;
use crate::game::team::PlayerTeam;
use crate::matrix::{Matrix, MatrixRotation};
use crate::position::INamedPosition;

#[derive(Serialize, Debug)]
#[serde(tag = "code", content = "detail")]
pub enum GameError {
    InvalidPosition,
    CardNotFound,
}

#[derive(Clone, Copy, Debug, Serialize_repr, Deserialize_repr)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMove {
    pub card_name: String,
    pub position: INamedPosition,
    pub rotation: CardRotation,
}

#[derive(Clone)]
pub struct GameState {
    pub board: Matrix<MapSquareType>,
    next_moves: HashMap<PlayerTeam, PlayerMove>,
}

impl GameState {
    pub fn new(board: Matrix<MapSquareType>) -> Self {
        Self {
            board,
            next_moves: HashMap::new(),
        }
    }

    pub fn propose_move(&mut self, team: PlayerTeam, player_move: PlayerMove) -> Result<(), GameError> {
        match MoveValidator::validate(&self.board, &team, &player_move) {
            Ok(()) => {
                self.next_moves.insert(team, player_move);
                Ok(())
            },
            Err(err) => Err(err),
        }
    }

    pub fn all_players_have_moved(&self) -> bool {
        self.next_moves.len() == PlayerTeam::COUNT
    }

    pub fn apply_moves(&mut self) -> HashMap<PlayerTeam, PlayerMove> {
        let moves = std::mem::take(&mut self.next_moves);
        let mut new_board = self.board.clone();

        for (team, player_move) in moves.iter() {
            let card_squares: Matrix<MapSquareType> = CardSquareType::from_card_name(&player_move.card_name).unwrap()
                .rotate_clockwise(player_move.rotation.into())
                .into_iter()
                .map(|(item, position)| (MapSquareType::from_card_square(item, team), position))
                .collect();

            new_board.replace(player_move.position.clone().try_into().unwrap(), card_squares);
        }

        self.board = new_board;
        moves
    }
}
