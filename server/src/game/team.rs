use serde::Serialize;
use strum::EnumCount;

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, EnumCount)]
pub enum PlayerTeam {
    Alpha,
    Bravo,
}
