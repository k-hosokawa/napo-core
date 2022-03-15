use anyhow::{bail, ensure, Result};
use serde;
use serde::de;
use serde::de::{Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use std::fmt;
use std::result::Result as stdResult;

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

impl Suit {
    pub fn reverse(&self) -> Self {
        match self {
            Suit::Spade => Suit::Club,
            Suit::Heart => Suit::Diamond,
            Suit::Diamond => Suit::Heart,
            Suit::Club => Suit::Spade,
        }
    }
}

impl Default for Suit {
    fn default() -> Self {
        Suit::Spade
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Card {
    pub number: u8,
    pub suit: Suit,
}

impl Serialize for Card {
    fn serialize<S>(&self, serializer: S) -> stdResult<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.to_id())
    }
}

struct CardVisitor;

impl<'de> Visitor<'de> for CardVisitor {
    type Value = Card;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between 1 and 52")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Card::from_id(value as u8).map_err(|e| E::custom(e))
    }
}

impl<'de> Deserialize<'de> for Card {
    fn deserialize<D>(deserializer: D) -> Result<Card, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u8(CardVisitor)
    }
}

impl Card {
    pub fn from_id(id: u8) -> Result<Self> {
        ensure!((1..=52).contains(&id), "invalid id \"{}\"", id);
        let number = ((id - 1) % 13) + 1;
        let suit = match (id - 1) / 13 {
            0 => Suit::Spade,
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Club,
            _ => bail!("invalid id \"{}\"", id),
        };
        Ok(Card { number, suit })
    }

    pub fn to_id(&self) -> u8 {
        let suit_num: u8 = match self.suit {
            Suit::Spade => 0,
            Suit::Heart => 1,
            Suit::Diamond => 2,
            Suit::Club => 3,
        };
        (suit_num * 13) + self.number
    }

    pub fn is_almighty(&self) -> bool {
        (self.number == 1) && (self.suit == Suit::Spade)
    }

    pub fn is_yoromeki(&self) -> bool {
        (self.number == 12) && (self.suit == Suit::Heart)
    }

    pub fn is_face(&self) -> bool {
        (self.number == 1) || (self.number >= 10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_trump_from_id() -> Result<()> {
        let t = Card::from_id(1)?;
        assert_eq!(t.number, 1);
        assert_eq!(t.suit, Suit::Spade);

        let t = Card::from_id(53);
        assert_eq!(t.is_err(), true);

        let t = Card::from_id(36)?;
        assert_eq!(t.number, 10);
        assert_eq!(t.suit, Suit::Diamond);

        let t = Card::from_id(52)?;
        assert_eq!(t.number, 13);
        assert_eq!(t.suit, Suit::Club);

        Ok(())
    }

    #[test]
    fn test_trump_to_id() {
        let t = Card {
            number: 1,
            suit: Suit::Spade,
        };
        assert_eq!(t.to_id(), 1);

        let t = Card {
            number: 10,
            suit: Suit::Club,
        };
        assert_eq!(t.to_id(), 49);

        let t = Card {
            number: 13,
            suit: Suit::Diamond,
        };
        assert_eq!(t.to_id(), 39);
    }

    #[test]
    fn test_is_almighty() {
        let almighty = Card {
            number: 1,
            suit: Suit::Spade,
        };
        assert_eq!(almighty.is_almighty(), true);

        let normal = Card {
            number: 3,
            suit: Suit::Spade,
        };
        assert_eq!(normal.is_almighty(), false);
    }

    #[test]
    fn test_is_yoromeki() {
        let almighty = Card {
            number: 12,
            suit: Suit::Heart,
        };
        assert_eq!(almighty.is_yoromeki(), true);

        let normal = Card {
            number: 3,
            suit: Suit::Spade,
        };
        assert_eq!(normal.is_yoromeki(), false);
    }

    #[test]
    fn test_trump_to_json() {
        let t = Card {
            number: 2,
            suit: Suit::Heart,
        };
        assert_eq!(serde_json::to_string(&t).unwrap(), "15");
    }

    #[test]
    fn test_json_to_trumps() -> Result<()> {
        let j = "[1, 2, 30, 4, 52]";
        let trumps: Vec<Card> = serde_json::from_str(j)?;
        let answers = vec![
            Card {
                number: 1,
                suit: Suit::Spade,
            },
            Card {
                number: 2,
                suit: Suit::Spade,
            },
            Card {
                number: 4,
                suit: Suit::Diamond,
            },
            Card {
                number: 4,
                suit: Suit::Spade,
            },
            Card {
                number: 13,
                suit: Suit::Club,
            },
        ];
        for i in 0..5 {
            assert_eq!(trumps[i], answers[i]);
        }
        Ok(())
    }
}
