use std::collections::HashMap;
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use crate::matrix::Matrix;

static CARDS_JSON: &str = include_str!("cards.json");

#[derive(Clone, Copy, Eq, PartialEq, Debug, Deserialize_repr)]
#[repr(u8)]
pub enum CardSquareType {
    Empty = 0,
    Fill = 1,
    Special = 2,
}

pub type CST = CardSquareType;

#[derive(Clone, Deserialize_repr)]
#[repr(u8)]
pub enum CardRarity {
    Common = 0,
    Rare = 1,
    Fresh = 2,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub category: String,
    pub name: String,
    pub number: usize,
    pub rarity: CardRarity,
    pub season: usize,
    pub special_cost: usize,
    pub squares: Matrix<CardSquareType>,
}

pub trait CardProvider {
    fn get(&self, card_name: &str) -> Option<Card>;

    fn exists(&self, card_name: &str) -> bool;
}

pub struct CardSquareProviderImpl {
    cards: HashMap<String, Card>,
}

impl CardSquareProviderImpl {
    pub fn new() -> Self {
        let card_list: Vec<Card> = serde_json::from_str(CARDS_JSON).unwrap();

        CardSquareProviderImpl {
            cards: card_list.into_iter()
                .map(|card| (card.name.clone(), card))
                .collect()
        }
    }
}

impl CardProvider for CardSquareProviderImpl {
    fn get(&self, card_name: &str) -> Option<Card> {
        self.cards.get(card_name).map(|card| card.clone())
    }

    fn exists(&self, card_name: &str) -> bool {
        self.cards.contains_key(card_name)
    }
}

#[cfg(test)]
pub mod tests {
    use std::sync::Arc;
    use super::*;

    pub struct TestCardSquareProvider {
        cards: HashMap<String, Card>,
    }

    impl TestCardSquareProvider {
        pub fn new() -> Arc<Self> {
            Arc::new(Self {
                cards: HashMap::from([
                    ("card_1".to_owned(), Card {
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
                    ("card_2".to_owned(), Card {
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
                    ("card_3".to_owned(), Card {
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
                    ("card_4".to_owned(), Card {
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
                ])
            })
        }
    }

    impl CardProvider for TestCardSquareProvider {
        fn get(&self, card_name: &str) -> Option<Card> {
            self.cards.get(card_name).map(|card| card.clone())
        }

        fn exists(&self, card_name: &str) -> bool {
            self.cards.contains_key(card_name)
        }
    }
}
