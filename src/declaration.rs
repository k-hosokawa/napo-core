use crate::card::{Card, Suit};
use crate::cards::GameCards;
use crate::player::{FieldPlayer, FieldPlayers, Player, Role};
use anyhow::{bail, Result};
use std::convert::TryInto;

#[derive(Debug)]
pub struct Declaration {
    pub players: FieldPlayers,
    pub suit: Option<Suit>,
    pub number: usize,
    pub aide: Card,
}

impl Declaration {
    #[allow(dead_code)]
    fn new(
        napoleon: Player,
        others: [Player; 4],
        cards: GameCards,
        suit: Option<Suit>,
        number: usize,
        aide: Card,
    ) -> Result<Self> {
        let aide_player = if let Some(i) = cards.hands.iter().position(|h| h.has(&aide)) {
            cards.hands[i].player.clone()
        } else if cards.opens.iter().any(|c| *c == aide) {
            napoleon.clone()
        } else {
            bail!("not found aide")
        };

        let mut players: Vec<FieldPlayer> = others
            .iter()
            .map(|p| FieldPlayer {
                player: p.clone(),
                role: if *p == aide_player {
                    Role::Aide
                } else {
                    Role::Union
                },
            })
            .collect::<Vec<FieldPlayer>>();
        players.push(FieldPlayer {
            player: napoleon,
            role: Role::Napoleon,
        });
        Ok(Declaration {
            players: players.try_into().unwrap(),
            suit,
            number,
            aide,
        })
    }

    #[allow(dead_code)]
    fn others_score(&self) -> usize {
        self.number - 12
    }

    #[allow(dead_code)]
    fn napoleon_score(&self) -> usize {
        self.others_score() * 2
    }
}
