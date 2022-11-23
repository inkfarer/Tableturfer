use crate::game::card::CardSquareType;
use crate::game::team::PlayerTeam;

#[derive(Clone, Copy, Debug)]
pub enum MapSquareType {
    Disabled,
    Empty,
    SpecialAlpha,
    SpecialBravo,
    FillAlpha,
    FillBravo,
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
}
