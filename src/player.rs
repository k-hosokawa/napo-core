use crate::card::{Card, Hands};

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Player {
    pub id: String,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            id: "a".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Role {
    Napoleon,
    Aide,
    Union,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct FieldPlayer {
    pub player: Player,
    pub hands: Hands,
    pub role: Role,
}

impl FieldPlayer {
    pub fn new(player: Player, hands: Hands) -> Self {
        FieldPlayer {
            player,
            hands,
            role: Role::Union,
        }
    }

    pub fn assign_role(&mut self, role: Role) {
        self.role = role;
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct FieldPlayers(pub [FieldPlayer; 5]);

impl From<FieldPlayers> for Vec<FieldPlayer> {
    fn from(players: FieldPlayers) -> Self {
        players.0.to_vec()
    }
}

impl From<Vec<FieldPlayer>> for FieldPlayers {
    fn from(players: Vec<FieldPlayer>) -> Self {
        FieldPlayers(players.try_into().unwrap())
    }
}

impl FieldPlayers {
    pub fn isolated(&self) -> bool {
        // 一人立ち。aideがいない。
        self.0.iter().filter(|p| p.role == Role::Aide).count() == 0
    }
}
