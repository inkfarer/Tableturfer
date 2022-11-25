use crate::matrix::Matrix;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum CardSquareType {
    Empty,
    Fill,
    Special,
}

pub type CST = CardSquareType;

pub trait CardSquareProvider {
    fn get(&self, card_name: &str) -> Option<Matrix<CardSquareType>>;
}

pub struct CardSquareProviderImpl {}

impl CardSquareProviderImpl {
    pub fn new() -> Self {
        CardSquareProviderImpl {}
    }
}

impl CardSquareProvider for CardSquareProviderImpl {
    // todo: doesn't belong here
    fn get(&self, card_name: &str) -> Option<Matrix<CardSquareType>> {
        match card_name {
            "BombCurling" => Some(Matrix::new(vec!(
                vec!(CST::Empty, CST::Special, CST::Empty),
                vec!(CST::Fill, CST::Fill, CST::Fill),
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
