use crate::card::{Card, Suit};
use crate::player::Player;
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Declaration {
    pub napoleon: Player,
    pub suit: Option<Suit>,
    pub number: usize,
    pub aide: Card,
}

impl Declaration {
    #[allow(dead_code)]
    pub fn new(napoleon: Player, suit: Option<Suit>, number: usize, aide: Card) -> Result<Self> {
        ensure!(number > 12 && number < 21, "invalid declaration number");
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
    fn declaration_new() -> Result<()> {
        Declaration::new(
            Player {
                id: "a".to_string(),
            },
            None,
            13,
            Card::from_id(1)?,
        )?;
        Ok(())
    }

    #[test]
    fn declaration_new_invalid_number() -> Result<()> {
        assert_eq!(
            Declaration::new(
                Player {
                    id: "a".to_string(),
                },
                None,
                12,
                Card::from_id(1)?,
            )
            .is_err(),
            true
        );
        Ok(())
    }
}
