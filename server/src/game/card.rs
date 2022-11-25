use crate::matrix::Matrix;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum CardSquareType {
    Empty,
    Fill,
    Special,
}

pub type CST = CardSquareType;

impl CardSquareType {
    // todo: doesn't belong here
    pub fn from_card_name(card_name: &str) -> Option<Matrix<Self>> {
        match card_name {
            "BombCurling" => Some(Matrix::new(vec!(
                vec!(CST::Empty, CST::Special, CardSquareType::Empty),
                vec!(CST::Fill, CST::Fill, CardSquareType::Fill),
            ))),
            "BombQuick" => Some(Matrix::new(vec!(
                vec!(CST::Special),
            ))),
            "ChargerLight00" => Some(Matrix::new(vec!(
                vec!(CST::Fill, CST::Empty, CST::Empty, CST::Empty, CST::Fill),
                vec!(CST::Fill, CST::Special, CST::Fill, CST::Fill, CST::Fill),
            ))),
            "ChargerNormal00" => Some(Matrix::new(vec!(
                vec!(CST::Fill, CST::Fill, CST::Fill, CST::Fill, CST::Fill, CST::Fill, CST::Fill),
                vec!(CST::Empty, CST::Empty, CST::Special, CST::Empty, CST::Empty, CST::Empty, CST::Empty),
            ))),
            _ => None
        }
    }
}
