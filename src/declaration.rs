use crate::card::{Card, Suit};
use crate::player::Player;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Declaration {
    pub napoleon: Player,
    pub suit: Option<Suit>,
    pub number: usize,
    pub aide: Card,
}

impl Declaration {
    #[allow(dead_code)]
    pub fn new(
        napoleon: Player,
        suit: Option<Suit>,
        number: usize,
        aide: Card,
    ) -> anyhow::Result<Self> {
        anyhow::ensure!(number > 12 && number < 21, "invalid declaration number");
        Ok(Declaration {
            napoleon,
            suit,
            number,
            aide,
        })
    }

    #[allow(dead_code)]
    fn base_score(&self) -> usize {
        self.number - 12
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn declaration_new() -> anyhow::Result<()> {
        Declaration::new(
            Player {
                id: "a".to_string(),
            },
            None,
            13,
            Card::try_from(1)?,
        )?;
        Ok(())
    }

    #[test]
    fn declaration_new_invalid_number() -> anyhow::Result<()> {
        assert!(Declaration::new(
            Player {
                id: "a".to_string(),
            },
            None,
            12,
            Card::try_from(1)?,
        )
        .is_err());
        Ok(())
    }

    #[test]
    fn declaration_base_score() -> anyhow::Result<()> {
        let d = Declaration::new(Player::default(), None, 13, Card::try_from(1)?)?;
        assert_eq!(d.base_score(), 1);
        Ok(())
    }
}
