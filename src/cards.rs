use crate::card::{Card, Hands};
use crate::player::Players;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::convert::TryInto;

#[derive(Debug)]
#[allow(dead_code)]
pub struct GameCards {
    pub hands: [Hands; 5],
    pub opens: [Card; 2],
}

impl GameCards {
    #[allow(dead_code)]
    pub fn new(players: Players) -> Self {
        let mut v: Vec<u8> = (1..53).collect();
        v.shuffle(&mut thread_rng());
        GameCards {
            hands: players
                .iter()
                .enumerate()
                .map(|(pid, _)| {
                    (0..10)
                        .map(|i| Card::from_id(v[(pid * 10) + i]).unwrap())
                        .collect::<Vec<Card>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<Hands>>()
                .try_into()
                .unwrap(),
            opens: [Card::from_id(v[50]).unwrap(), Card::from_id(v[51]).unwrap()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::get_dummy_players;
    use std::collections::HashSet;

    #[test]
    fn test_distribute() {
        let cards = GameCards::new(get_dummy_players());

        let mut s = HashSet::new();
        cards.hands.map(|h| {
            h.map(|c| {
                assert_eq!(s.contains(&c), false);
                s.insert(c.clone());
            })
        });
    }
}
