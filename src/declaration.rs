use crate::card::{Card, Suit};
use crate::game::Round;
use crate::player::{FieldPlayer, FieldPlayers, Role};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Declaration {
    pub players: FieldPlayers,
    pub suit: Option<Suit>,
    pub number: usize,
    pub aide: Card,
}

impl Declaration {
    #[allow(dead_code)]
    pub fn new(
        napoleon: FieldPlayer,
        players: &mut FieldPlayers,
        suit: Option<Suit>,
        number: usize,
        aide: Card,
    ) -> Result<Self> {
        for p in players.into_iter() {
            if *p == napoleon {
                p.assign_role(Role::Napoleon);
            } else if p.has(&aide) {
                p.assign_role(Role::Aide);
            } else {
                p.assign_role(Role::Union);
            }
        }
        ensure!(
            !players.into_iter().any(|p| p.role == Some(Role::Napoleon)),
            "Napoleon is not found"
        );
        Ok(Declaration {
            players: players.clone(),
            suit,
            number,
            aide,
        })
    }

    #[allow(dead_code)]
    pub fn create_round(self) -> Round {
        Round::new(self)
    }

    fn is_alone(&self) -> bool {
        self.players.iter().any(|p| p.role == Some(Role::Aide))
    }

    #[allow(dead_code)]
    fn others_score(&self) -> usize {
        self.number - 12
    }

    #[allow(dead_code)]
    fn napoleon_score(&self) -> usize {
        if self.is_alone() {
            self.others_score() * 4
        } else {
            self.others_score() * 2
        }
    }
}
