use anyhow::{bail, ensure, Result};

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
    pub number: usize,
    pub suit: Suit,
}

impl Trump {
    #[allow(dead_code)]
    pub fn from_id(id: usize) -> Result<Self> {
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
    pub fn to_id(&self) -> usize {
        let suit_num: usize = match self.suit {
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
}
