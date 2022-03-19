use crate::card::{Card, Hands};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
}

pub type Players = [Player; 5];

#[allow(dead_code)]
pub fn get_dummy_players() -> Players {
    ["a", "b", "c", "d", "e"].map(|s| Player { id: s.to_string() })
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Role {
    Napoleon,
    Aide,
    Union,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FieldPlayer {
    pub player: Player,
    pub hands: Hands,
    pub role: Option<Role>,
}

impl FieldPlayer {
    pub fn new(player: Player, hands: Hands) -> Self {
        FieldPlayer {
            player,
            hands,
            role: None,
        }
    }

    pub fn assign_role(&mut self, role: Role) {
        self.role = Some(role);
    }

    pub fn choice_opens(&mut self, opens: [Card; 2], discard: [Card; 2]) {
        let mut cards = self.hands.to_vec();
        cards.append(&mut opens.to_vec());
        self.hands = cards
            .into_iter()
            .filter(|c| !discard.iter().any(|g| g == c))
            .collect::<Vec<Card>>()
            .try_into()
            .unwrap();
    }

    pub fn has(&self, card: &Card) -> bool {
        self.hands.iter().any(|c| c == card)
    }
}

pub type FieldPlayers = [FieldPlayer; 5];
