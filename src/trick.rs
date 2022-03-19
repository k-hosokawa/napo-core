use crate::card::Card;
use crate::player::{FieldPlayer, Player};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Play {
    pub player: Player,
    pub card: Card,
}

impl Play {
    #[allow(dead_code)]
    pub fn new(player: FieldPlayer, card: Card) -> Self {
        Play {
            player: player.player,
            card,
        }
    }
}

pub type TrickArray = [Play; 5];

#[derive(Debug)]
pub struct Trick {
    pub plays: Vec<Play>,
}

impl Trick {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Trick { plays: Vec::new() }
    }

    #[allow(dead_code)]
    pub fn add(&mut self, play: Play) {
        self.plays.push(play);
    }

    #[allow(dead_code)]
    pub fn last_player(&self) -> Option<Player> {
        Some(self.plays.last()?.player.clone())
    }

    pub(crate) fn array(&self) -> Result<TrickArray> {
        ensure!(self.plays.len() == 5, "This Trick is not finished yet");
        Ok(self.plays.clone().try_into().unwrap())
    }
}
