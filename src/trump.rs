use anyhow::{bail, ensure, Result};
use serde::de;
use serde::de::{Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use std::fmt;
use std::result::Result as stdResult;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

impl Suit {
    #[allow(dead_code)]
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

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Trump {
    pub number: u8,
    pub suit: Suit,
}

impl Serialize for Trump {
    fn serialize<S>(&self, serializer: S) -> stdResult<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.to_id())
    }
}

struct TrumpVisitor;

impl<'de> Visitor<'de> for TrumpVisitor {
    type Value = Trump;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between 1 and 52")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let t = Trump::from_id(value as u8).or_else(|e| Err(E::custom(e)));
        println!("trump: {:?}", t);
        t
    }
}

impl<'de> Deserialize<'de> for Trump {
    fn deserialize<D>(deserializer: D) -> Result<Trump, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u8(TrumpVisitor)
    }
}

impl Trump {
    #[allow(dead_code)]
    pub fn from_id(id: u8) -> Result<Self> {
        ensure!((id >= 1) && (id <= 52), "invalid id \"{}\"", id);
        let number = ((id - 1) % 13) + 1;
        let suit = match (id - 1) / 13 {
            0 => Suit::Spade,
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Club,
            _ => bail!("invalid id \"{}\"", id),
        };
        Ok(Trump { number, suit })
    }

    #[allow(dead_code)]
    pub fn to_id(&self) -> u8 {
        let suit_num: u8 = match self.suit {
            Suit::Spade => 0,
            Suit::Heart => 1,
            Suit::Diamond => 2,
            Suit::Club => 3,
        };
        (suit_num * 13) + self.number
    }

    #[allow(dead_code)]
    pub fn is_almighty(&self) -> bool {
        (self.number == 1) && (self.suit == Suit::Spade)
    }

    #[allow(dead_code)]
    pub fn is_yoromeki(&self) -> bool {
        (self.number == 12) && (self.suit == Suit::Heart)
    }

    #[allow(dead_code)]
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
        let t = Trump::from_id(1)?;
        assert_eq!(t.number, 1);
        assert_eq!(t.suit, Suit::Spade);

        let t = Trump::from_id(53);
        assert_eq!(t.is_err(), true);

        let t = Trump::from_id(36)?;
        assert_eq!(t.number, 10);
        assert_eq!(t.suit, Suit::Diamond);

        let t = Trump::from_id(52)?;
        assert_eq!(t.number, 13);
        assert_eq!(t.suit, Suit::Club);

        Ok(())
    }

    #[test]
    fn test_trump_to_id() {
        let t = Trump {
            number: 1,
            suit: Suit::Spade,
        };
        assert_eq!(t.to_id(), 1);

        let t = Trump {
            number: 10,
            suit: Suit::Club,
        };
        assert_eq!(t.to_id(), 49);

        let t = Trump {
            number: 13,
            suit: Suit::Diamond,
        };
        assert_eq!(t.to_id(), 39);
    }

    #[test]
    fn test_is_almighty() {
        let almighty = Trump {
            number: 1,
            suit: Suit::Spade,
        };
        assert_eq!(almighty.is_almighty(), true);

        let normal = Trump {
            number: 3,
            suit: Suit::Spade,
        };
        assert_eq!(normal.is_almighty(), false);
    }

    #[test]
    fn test_is_yoromeki() {
        let almighty = Trump {
            number: 12,
            suit: Suit::Heart,
        };
        assert_eq!(almighty.is_yoromeki(), true);

        let normal = Trump {
            number: 3,
            suit: Suit::Spade,
        };
        assert_eq!(normal.is_yoromeki(), false);
    }

    #[test]
    fn test_trump_to_json() {
        let t = Trump {
            number: 2,
            suit: Suit::Heart,
        };
        assert_eq!(serde_json::to_string(&t).unwrap(), "15");
    }

    #[test]
    fn test_json_to_trumps() -> Result<()> {
        let j = "[1, 2, 30, 4, 52]";
        let trumps: Vec<Trump> = serde_json::from_str(j)?;
        let answers = vec![
            Trump {
                number: 1,
                suit: Suit::Spade,
            },
            Trump {
                number: 2,
                suit: Suit::Spade,
            },
            Trump {
                number: 4,
                suit: Suit::Diamond,
            },
            Trump {
                number: 4,
                suit: Suit::Spade,
            },
            Trump {
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
