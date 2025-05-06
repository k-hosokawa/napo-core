use crate::card::Card;
use crate::player::{FieldPlayer, FieldPlayers, Players};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::convert::TryInto;

pub fn distribute_cards(players: &Players) -> (FieldPlayers, [Card; 2]) {
    let mut v: Vec<u8> = (1..53).collect();
    v.shuffle(&mut thread_rng());
    let players: FieldPlayers = players
        .iter()
        .enumerate()
        .map(|(pid, p)| {
            FieldPlayer::new(
                p.clone(),
                (0..10)
                    .map(|i| Card::from_id(v[(pid * 10) + i]).unwrap())
                    .collect::<Vec<Card>>()
                    .try_into()
                    .unwrap(),
            )
        })
        .collect::<Vec<FieldPlayer>>()
        .try_into()
        .unwrap();
    let opens = [Card::from_id(v[50]).unwrap(), Card::from_id(v[51]).unwrap()];

    (players, opens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::get_dummy_players;
    use std::collections::HashSet;

    #[test]
    fn test_distribute() {
        let (field_players, opens) = distribute_cards(&get_dummy_players());

        let mut s = HashSet::new();
        field_players.map(|p| {
            p.hands.map(|c| {
                assert!(!s.contains(&c));
                s.insert(c.clone());
            })
        });
        opens.map(|c| {
            assert!(!s.contains(&c));
            s.insert(c.clone());
        });
    }
}
