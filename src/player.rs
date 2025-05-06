use crate::card::{Card, Hands};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct Players(pub [Player; 5]);

impl Default for Players {
    fn default() -> Self {
        Self(["a", "b", "c", "d", "e"].map(|s| Player { id: s.to_string() }))
    }
}

impl From<Players> for Vec<Player> {
    fn from(players: Players) -> Self {
        players.0.to_vec()
    }
}

impl From<Vec<Player>> for Players {
    fn from(players: Vec<Player>) -> Self {
        Players(players.try_into().unwrap())
    }
}

impl std::iter::FromIterator<Player> for Players {
    fn from_iter<T: IntoIterator<Item = Player>>(iter: T) -> Self {
        Self(iter.into_iter().collect::<Vec<_>>().try_into().unwrap())
    }
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
