use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
}

pub type Players = [Player; 5];

#[allow(dead_code)]
pub(crate) fn get_dummy_players() -> Players {
    ["a", "b", "c", "d", "e"].map(|s| Player { id: s.to_string() })
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    Napoleon,
    Aide,
    Union,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldPlayer {
    pub player: Player,
    pub role: Role,
}

pub type FieldPlayers = [FieldPlayer; 5];
