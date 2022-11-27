use crate::matrix::Matrix;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum CardSquareType {
    Empty,
    Fill,
    Special,
}

pub type CST = CardSquareType;

pub enum CardRarity {
    Common,
    Rare,
    Fresh,
}

pub struct Card {
    pub row_id: String,
    pub category: String,
    pub name: String,
    pub number: usize,
    pub rarity: CardRarity,
    pub season: usize,
    pub special_cost: usize,
    pub squares: Matrix<CardSquareType>,
}

// todo: rename to CardProvider
pub trait CardSquareProvider {
    fn get(&self, card_name: &str) -> Option<Card>;
}

pub struct CardSquareProviderImpl {}

impl CardSquareProviderImpl {
    pub fn new() -> Self {
        CardSquareProviderImpl {}
    }
}

impl CardSquareProvider for CardSquareProviderImpl {
    // todo: doesn't belong here
    fn get(&self, card_name: &str) -> Option<Card> {
        match card_name {
            "BombCurling" => Some(Card {
                row_id: "MiniGame_BombCurling".to_owned(),
                category: "WeaponSub".to_owned(),
                name: "BombCurling".to_owned(),
                number: 62,
                rarity: CardRarity::Common,
                season: 1,
                special_cost: 2,
                squares: Matrix::new(vec!(
                    vec!(CST::Empty, CST::Special, CST::Empty),
                    vec!(CST::Fill, CST::Fill, CST::Fill),
                )),
            }),
            "BombQuick" => Some(Card {
                row_id: "MiniGame_BombQuick".to_owned(),
                category: "WeaponSub".to_owned(),
                name: "BombQuick".to_owned(),
                number: 58,
                rarity: CardRarity::Common,
                season: 1,
                special_cost: 1,
                squares: Matrix::new(vec!(
                    vec!(CST::Special),
                )),
            }),
            "ChargerLight00" => Some(Card {
                row_id: "MiniGame_ChargerLight00".to_owned(),
                category: "WeaponMain".to_owned(),
                name: "ChargerLight00".to_owned(),
                number: 32,
                rarity: CardRarity::Common,
                season: 1,
                special_cost: 3,
                squares: Matrix::new(vec!(
                    vec!(CST::Fill, CST::Empty, CST::Empty, CST::Empty, CST::Fill),
                    vec!(CST::Fill, CST::Special, CST::Fill, CST::Fill, CST::Fill),
                )),
            }),
            "ChargerNormal00" => Some(Card {
                row_id: "MiniGame_ChargerNormal00".to_owned(),
                category: "WeaponMain".to_owned(),
                name: "ChargerNormal00".to_owned(),
                number: 28,
                rarity: CardRarity::Common,
                season: 1,
                special_cost: 3,
                squares: Matrix::new(vec!(
                    vec!(CST::Fill, CST::Fill, CST::Fill, CST::Fill, CST::Fill, CST::Fill, CST::Fill),
                    vec!(CST::Empty, CST::Empty, CST::Special, CST::Empty, CST::Empty, CST::Empty, CST::Empty),
                )),
            }),
            _ => None
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::sync::Arc;
    use super::*;

    pub struct TestCardSquareProvider {}

    impl TestCardSquareProvider {
        pub fn new() -> Arc<Self> {
            Arc::new(Self {})
        }
    }

    impl CardSquareProvider for TestCardSquareProvider {
        fn get(&self, card_name: &str) -> Option<Card> {
            match card_name {
                "card_1" => Some(Card {
                    row_id: "row_id_1".to_string(),
                    category: "test_cards".to_string(),
                    name: "card_1".to_string(),
                    number: 1,
                    rarity: CardRarity::Common,
                    season: 1,
                    special_cost: 2,
                    squares: Matrix::new(vec!(
                        vec!(CST::Empty, CST::Fill),
                        vec!(CST::Empty, CST::Special),
                        vec!(CST::Fill, CST::Fill),
                    )),
                }),
                "card_2" => Some(Card {
                    row_id: "row_id_2".to_string(),
                    category: "test_cards".to_string(),
                    name: "card_2".to_string(),
                    number: 2,
                    rarity: CardRarity::Rare,
                    season: 1,
                    special_cost: 1,
                    squares: Matrix::new(vec!(
                        vec!(CST::Fill, CST::Fill),
                        vec!(CST::Fill, CST::Special),
                    )),
                }),
                "card_3" => Some(Card {
                    row_id: "row_id_3".to_string(),
                    category: "test_cards".to_string(),
                    name: "card_3".to_string(),
                    number: 3,
                    rarity: CardRarity::Common,
                    season: 1,
                    special_cost: 1,
                    squares: Matrix::new(vec!(
                        vec!(CST::Fill),
                    )),
                }),
                "card_4" => Some(Card {
                    row_id: "row_id_3".to_string(),
                    category: "test_cards".to_string(),
                    name: "card_4".to_string(),
                    number: 4,
                    rarity: CardRarity::Fresh,
                    season: 1,
                    special_cost: 3,
                    squares: Matrix::new(vec!(
                        vec!(CST::Fill, CST::Fill),
                        vec!(CST::Special, CST::Empty),
                    )),
                }),
                _ => None,
            }
        }
    }
}
