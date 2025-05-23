use crate::card::Card;
use crate::player::Player;

#[allow(dead_code)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Play {
    pub player: Player,
    pub card: Card,
}

impl Play {
    #[allow(dead_code)]
    pub fn new(player: Player, card: Card) -> Self {
        Play { player, card }
    }
}

pub type TrickArray = [Play; 5];

#[derive(Debug)]
pub struct Trick {
    pub plays: Vec<Play>,
}

impl Default for Trick {
    fn default() -> Self {
        Self::new()
    }
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

    pub(crate) fn array(&self) -> anyhow::Result<TrickArray> {
        anyhow::ensure!(self.plays.len() == 5, "This Trick is not finished yet");
        Ok(self.plays.clone().try_into().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::round::Round;

    #[test]
    fn test_trick_add() -> anyhow::Result<()> {
        let mut trick = Trick::new();

        let players = crate::player::Players::default();
        let r = Round::new(players.clone());
        trick.add(Play::new(
            r.field_players.0[0].player.clone(),
            r.field_players.0[0].hands[0],
        ));

        Ok(())
    }

    #[test]
    fn test_trick_last_player() -> anyhow::Result<()> {
        let mut trick = Trick::new();
        assert_eq!(trick.last_player(), None);

        let players = crate::player::Players::default();
        let r = Round::new(players.clone());
        trick.add(Play::new(
            r.field_players.0[0].player.clone(),
            r.field_players.0[0].hands[0],
        ));
        assert_eq!(
            trick.last_player(),
            Some(r.field_players.0[0].player.clone())
        );

        trick.add(Play::new(
            r.field_players.0[1].player.clone(),
            r.field_players.0[1].hands[0],
        ));
        assert_eq!(
            trick.last_player(),
            Some(r.field_players.0[1].player.clone())
        );
        Ok(())
    }

    #[test]
    fn test_trick_array() -> anyhow::Result<()> {
        let mut trick = Trick::new();
        assert!(trick.array().is_err());

        let players = crate::player::Players::default();
        let r = Round::new(players.clone());
        for p in r.field_players.0.iter() {
            trick.add(Play::new(p.player.clone(), p.hands[0]));
        }

        for (t, p) in std::iter::zip(trick.array()?, r.field_players.0.iter()) {
            assert_eq!(t.player, p.player);
            assert_eq!(t.card, p.hands[0]);
        }
        Ok(())
    }
}
