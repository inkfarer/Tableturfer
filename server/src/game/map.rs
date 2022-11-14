use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum GameMap {
    Square,
    WDiamond,
}

pub const DEFAULT_GAME_MAP: GameMap = GameMap::Square;
