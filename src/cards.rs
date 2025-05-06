use crate::card::Card;
use crate::player::{FieldPlayer, FieldPlayers, Players};
use rand::seq::SliceRandom;

pub fn distribute_cards(players: &Players) -> (FieldPlayers, [Card; 2]) {
    let mut v: Vec<u8> = (1..53).collect();
    v.shuffle(&mut rand::thread_rng());
    let players: FieldPlayers = players
        .0
        .iter()
        .enumerate()
        .map(|(pid, p)| {
            FieldPlayer::new(
                p.clone(),
                (0..10)
                    .map(|i| Card::try_from(v[(pid * 10) + i]).unwrap())
                    .collect::<Vec<Card>>()
                    .try_into()
                    .unwrap(),
            )
        })
        .collect::<Vec<FieldPlayer>>()
        .try_into()
        .unwrap();
    let opens = [
        Card::try_from(v[50]).unwrap(),
        Card::try_from(v[51]).unwrap(),
    ];

    (players, opens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_distribute() {
        let players = Players::default();
        let (field_players, opens) = distribute_cards(&players);

        let mut s = HashSet::new();
        field_players.0.map(|p| {
            p.hands.map(|c| {
                assert!(!s.contains(&c));
                s.insert(c);
            })
        });
        opens.map(|c| {
            assert!(!s.contains(&c));
            s.insert(c);
        });
    }
}
