use serde::{Deserialize, Serialize};
use crate::game::squares::{MapSquareType, MST};
use crate::matrix::Matrix;

#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
pub enum GameMap {
    Square,
    WDiamond,
}

// todo: this is stupid; maybe keep map details in a common data directory the server and client can reference (at build time)?
impl GameMap {
    pub fn to_squares(&self) -> Matrix<MapSquareType> {
        match self {
            GameMap::Square => {
                Matrix::new(vec!(
                    vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                    vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                    vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                    vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::SpecialBravo, MST::Empty, MST::Empty, MST::Empty),
                    vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                    vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                    vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                    vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                    vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                    vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                    vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                    vec!(MST::Empty, MST::Empty, MST::Empty, MST::SpecialAlpha, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                    vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                    vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                    vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty)
                ))
            }
            GameMap::WDiamond => {
                Matrix::new(vec!(
                    vec!(MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Empty, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled),
                    vec!(MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Empty, MST::Empty, MST::Empty, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled),
                    vec!(MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled),
                    vec!(MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled),
                    vec!(MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled),
                    vec!(MST::Disabled, MST::Disabled, MST::Disabled, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::SpecialBravo, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Disabled, MST::Disabled, MST::Disabled),
                    vec!(MST::Disabled, MST::Disabled, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Disabled, MST::Disabled),
                    vec!(MST::Disabled, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Disabled),
                    vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                    vec!(MST::Disabled, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Disabled),
                    vec!(MST::Disabled, MST::Disabled, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Disabled, MST::Disabled),
                    vec!(MST::Disabled, MST::Disabled, MST::Disabled, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Disabled, MST::Disabled, MST::Disabled),
                    vec!(MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled),
                    vec!(MST::Disabled, MST::Disabled, MST::Disabled, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Disabled, MST::Disabled, MST::Disabled),
                    vec!(MST::Disabled, MST::Disabled, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Disabled, MST::Disabled),
                    vec!(MST::Disabled, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Disabled),
                    vec!(MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty),
                    vec!(MST::Disabled, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Disabled),
                    vec!(MST::Disabled, MST::Disabled, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Disabled, MST::Disabled),
                    vec!(MST::Disabled, MST::Disabled, MST::Disabled, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::SpecialAlpha, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Disabled, MST::Disabled, MST::Disabled),
                    vec!(MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled),
                    vec!(MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled),
                    vec!(MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Empty, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled),
                    vec!(MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Empty, MST::Empty, MST::Empty, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled),
                    vec!(MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Empty, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled, MST::Disabled),
                ))
            }
        }
    }
}

pub const DEFAULT_GAME_MAP: GameMap = GameMap::Square;
