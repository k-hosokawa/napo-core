#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash, serde::Serialize, serde::Deserialize, Default,
)]
pub enum Suit {
    #[default]
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card {
    pub number: u8,
    pub suit: Suit,
}

impl serde::Serialize for Card {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_u8(u8::from(*self))
    }
}

struct CardVisitor;

impl serde::de::Visitor<'_> for CardVisitor {
    type Value = Card;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an integer between 1 and 52")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Card::try_from(value as u8).map_err(|e| E::custom(e))
    }
}

impl<'de> serde::Deserialize<'de> for Card {
    fn deserialize<D>(deserializer: D) -> Result<Card, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_u8(CardVisitor)
    }
}

impl TryFrom<u8> for Card {
    type Error = anyhow::Error;

    fn try_from(id: u8) -> anyhow::Result<Self> {
        anyhow::ensure!((1..=52).contains(&id), "invalid id \"{}\"", id);
        let number = ((id - 1) % 13) + 1;
        let suit = match (id - 1) / 13 {
            0 => Suit::Spade,
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Club,
            _ => anyhow::bail!("invalid id \"{}\"", id),
        };
        Ok(Card { number, suit })
    }
}

impl From<Card> for u8 {
    fn from(card: Card) -> Self {
        let suit_num: u8 = match card.suit {
            Suit::Spade => 0,
            Suit::Heart => 1,
            Suit::Diamond => 2,
            Suit::Club => 3,
        };
        (suit_num * 13) + card.number
    }
}

impl Card {
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

pub type Hands = [Card; 10];

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest::rstest]
    #[test]
    #[case(1, 1, Suit::Spade)]
    #[case(36, 10, Suit::Diamond)]
    #[case(52, 13, Suit::Club)]
    fn test_trump_from_id(
        #[case] id: u8,
        #[case] number: u8,
        #[case] suit: Suit,
    ) -> anyhow::Result<()> {
        let t = Card::try_from(id)?;
        assert_eq!(t.number, number);
        assert_eq!(t.suit, suit);
        Ok(())
    }

    #[test]
    fn test_trump_from_id_error() -> anyhow::Result<()> {
        let t = Card::try_from(53);
        assert!(t.is_err());
        Ok(())
    }

    #[rstest::rstest]
    #[test]
    #[case(Card { number: 1, suit: Suit::Spade }, 1)]
    #[case(Card { number: 10, suit: Suit::Club }, 49)]
    #[case(Card { number: 13, suit: Suit::Diamond }, 39)]
    fn test_trump_to_id(#[case] card: Card, #[case] id: u8) {
        assert_eq!(u8::from(card), id);
    }

    #[rstest::rstest]
    #[test]
    #[case(Card { number: 1, suit: Suit::Spade }, true)]
    #[case(Card { number: 3, suit: Suit::Spade }, false)]
    fn test_is_almighty(#[case] card: Card, #[case] is_almighty: bool) {
        assert_eq!(card.is_almighty(), is_almighty);
    }

    #[rstest::rstest]
    #[test]
    #[case(Card { number: 12, suit: Suit::Heart }, true)]
    #[case(Card { number: 3, suit: Suit::Spade }, false)]
    fn test_is_yoromeki(#[case] card: Card, #[case] is_yoromeki: bool) {
        assert_eq!(card.is_yoromeki(), is_yoromeki);
    }

    #[rstest::rstest]
    #[test]
    #[case(Card { number: 1, suit: Suit::Spade }, true)]
    #[case(Card { number: 10, suit: Suit::Club }, true)]
    #[case(Card { number: 13, suit: Suit::Diamond }, true)]
    #[case(Card { number: 2, suit: Suit::Spade }, false)]
    fn test_is_face(#[case] card: Card, #[case] is_face: bool) {
        assert_eq!(card.is_face(), is_face);
    }

    #[rstest::rstest]
    #[test]
    #[case(Card { number: 2, suit: Suit::Heart }, "15")]
    fn test_trump_to_json(#[case] card: Card, #[case] json: &str) {
        assert_eq!(serde_json::to_string(&card).unwrap(), json);
    }

    #[rstest::rstest]
    #[test]
    #[case("[1,2,30,4,52]", [Card { number: 1, suit: Suit::Spade }, Card { number: 2, suit: Suit::Spade }, Card { number: 4, suit: Suit::Diamond }, Card { number: 4, suit: Suit::Spade }, Card { number: 13, suit: Suit::Club }])]
    fn test_json_to_trumps(#[case] json: &str, #[case] trumps: [Card; 5]) -> anyhow::Result<()> {
        assert_eq!(json, serde_json::to_string(&trumps)?);
        Ok(())
    }
}
