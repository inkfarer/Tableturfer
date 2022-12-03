use serde_repr::Deserialize_repr;
use crate::game::card::CardSquareType;
use crate::game::team::PlayerTeam;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize_repr)]
#[repr(u8)]
pub enum MapSquareType {
    Disabled = 0,
    Empty = 1,
    SpecialAlpha = 2,
    SpecialBravo = 3,
    FillAlpha,
    FillBravo,
    Neutral,
}

pub type MST = MapSquareType;

impl MapSquareType {
    pub fn from_card_square(square: CardSquareType, team: &PlayerTeam) -> Self {
        match square {
            CardSquareType::Empty => Self::Empty,
            CardSquareType::Fill => match team {
                PlayerTeam::Alpha => Self::FillAlpha,
                PlayerTeam::Bravo => Self::FillBravo,
            },
            CardSquareType::Special => match team {
                PlayerTeam::Alpha => Self::SpecialAlpha,
                PlayerTeam::Bravo => Self::SpecialBravo,
            },
        }
    }

    pub fn is_fill(&self) -> bool {
        self == &Self::FillAlpha || self == &Self::FillBravo
    }

    pub fn is_special(&self) -> bool {
        self == &Self::SpecialAlpha || self == &Self::SpecialBravo
    }
}
