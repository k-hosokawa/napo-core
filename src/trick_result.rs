use crate::card::{Card, Suit};
use crate::player::Player;
use crate::trick::{Trick, TrickArray};
use anyhow::Result;
use serde::{Deserialize, Serialize};

struct TrickResultBuilder {
    trick: TrickArray,
    face_cards: Vec<Card>,
}

impl TrickResultBuilder {
    fn new(trick: &Trick) -> Result<Self> {
        let face_cards: Vec<Card> = trick
            .plays
            .iter()
            .filter(|p| p.card.is_face())
            .map(|p| p.card.clone())
            .collect();
        Ok(TrickResultBuilder {
            trick: (*trick).array()?,
            face_cards,
        })
    }

    fn build(&self, winner_id: usize) -> Result<TrickResult> {
        Ok(TrickResult {
            trick: self.trick.clone(),
            face_cards: self.face_cards.clone(),
            winner: self.trick[winner_id].player.clone(),
        })
    }
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct TrickResult {
    pub trick: TrickArray,
    pub winner: Player,
    pub face_cards: Vec<Card>,
}

impl TrickResult {
    #[allow(dead_code)]
    pub fn new(trick: &Trick, suit: Option<Suit>, n_round: u8) -> Result<Self> {
        let builder = TrickResultBuilder::new(trick)?;

        // almighty
        if let Some(id) = builder.trick.iter().position(|c| c.card.is_almighty()) {
            return builder.build(
                builder
                    .trick
                    .iter()
                    .position(|c| c.card.is_yoromeki())
                    .unwrap_or(id),
            );
        }

        // jack
        if let Some(s) = suit {
            let id = builder
                .trick
                .iter()
                .position(|c| (c.card.number == 11) && (c.card.suit == s));
            if let Some(i) = id {
                return builder.build(i);
            }

            // reverse jack
            let rev_suit = s.reverse();
            let id = builder
                .trick
                .iter()
                .position(|c| (c.card.number == 11) && (c.card.suit == rev_suit));
            if let Some(i) = id {
                return builder.build(i);
            }
        }

        let first_suit = builder.trick[0].card.suit.clone();

        // same2
        if n_round > 1 && (builder.trick.iter().all(|c| c.card.suit == first_suit)) {
            if let Some(id) = builder.trick.iter().position(|c| c.card.number == 2) {
                return builder.build(id);
            }
        }

        let mut winner_id = 0;
        for i in 1..5 {
            let c = &builder.trick[i].card;
            if c.suit == first_suit {
                if c.number == 1 {
                    return builder.build(i);
                }
                if c.number > builder.trick[winner_id].card.number {
                    winner_id = i;
                }
            }
        }
        builder.build(winner_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trick::Play;
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use serde_json::json;
    use std::convert::TryInto;

    #[allow(dead_code)]
    type FieldCardIds = [u8; 5];

    #[allow(dead_code)]
    fn get_trick(ids: &FieldCardIds) -> Trick {
        let players = crate::player::Players::default();
        let roles = [
            crate::player::Role::Napoleon,
            crate::player::Role::Aide,
            crate::player::Role::Union,
            crate::player::Role::Union,
            crate::player::Role::Union,
        ];

        let all_cards: Vec<u8> = (1..53).collect();
        let mut all_cards: Vec<u8> = all_cards
            .into_iter()
            .filter(|c| !ids.iter().any(|g| g == c))
            .collect();
        all_cards.shuffle(&mut thread_rng());

        let field_players: crate::player::FieldPlayers = players
            .0
            .into_iter()
            .enumerate()
            .map(|(pid, p)| {
                let mut hands: Vec<Card> = (0..9)
                    .map(|i| Card::try_from(all_cards[(pid * 9) + i]).unwrap())
                    .collect();
                hands.push(Card::try_from(ids[pid]).unwrap());
                crate::player::FieldPlayer {
                    player: p,
                    role: Some(roles[pid].clone()),
                    hands: hands.try_into().unwrap(),
                }
            })
            .collect::<Vec<crate::player::FieldPlayer>>()
            .try_into()
            .unwrap();
        let mut trick = Trick::new();
        for p in field_players {
            trick.add(Play::new(p.player.clone(), p.hands[9].clone()));
        }
        trick
    }

    #[test]
    fn test_get_field_cards() -> Result<()> {
        let v: FieldCardIds = [1, 4, 25, 40, 52];
        get_trick(&v);
        Ok(())
    }

    #[test]
    fn test_judge_winner_almighty() -> Result<()> {
        let v: FieldCardIds = [1, 4, 24, 40, 52];
        let t = get_trick(&v);
        let r = TrickResult::new(&t, None, 1)?;
        assert_eq!(r.winner.id, "a");
        assert_eq!(
            r.face_cards,
            vec![
                Card::try_from(1)?,
                Card::try_from(24)?,
                Card::try_from(40)?,
                Card::try_from(52)?,
            ],
        );
        Ok(())
    }

    #[test]
    fn test_judge_winner_yoromeki() -> Result<()> {
        let v: FieldCardIds = [1, 4, 25, 40, 52];
        let t = get_trick(&v);
        let r = TrickResult::new(&t, None, 1)?;
        assert_eq!(r.winner.id, "c");
        assert_eq!(
            r.face_cards,
            vec![
                Card::try_from(1)?,
                Card::try_from(25)?,
                Card::try_from(40)?,
                Card::try_from(52)?,
            ],
        );
        Ok(())
    }

    #[test]
    fn test_judge_winner_jack() -> Result<()> {
        let v: FieldCardIds = [2, 11, 24, 40, 52];
        let t = get_trick(&v);
        let r = TrickResult::new(&t, Some(Suit::Spade), 1)?;
        assert_eq!(r.winner.id, "b");
        assert_eq!(
            r.face_cards,
            vec![
                Card::try_from(11)?,
                Card::try_from(24)?,
                Card::try_from(40)?,
                Card::try_from(52)?,
            ],
        );
        Ok(())
    }

    #[test]
    fn test_judge_winner_rev_jack() -> Result<()> {
        let v: FieldCardIds = [2, 4, 24, 40, 50];
        let t = get_trick(&v);
        let r = TrickResult::new(&t, Some(Suit::Spade), 1)?;
        assert_eq!(r.winner.id, "e");
        assert_eq!(
            r.face_cards,
            vec![
                Card::try_from(24)?,
                Card::try_from(40)?,
                Card::try_from(50)?,
            ],
        );
        Ok(())
    }

    #[test]
    fn test_judge_winner_same2() -> Result<()> {
        let v: FieldCardIds = [2, 3, 4, 5, 6];

        let t = get_trick(&v);
        let r = TrickResult::new(&t, None, 2)?;
        assert_eq!(r.winner.id, "a");
        assert_eq!(r.face_cards, Vec::<Card>::new(),);

        let r = TrickResult::new(&t, None, 1)?;
        assert_eq!(r.winner.id, "e");
        assert_eq!(r.face_cards, Vec::<Card>::new(),);
        Ok(())
    }

    #[test]
    fn test_to_json() -> Result<()> {
        let v: FieldCardIds = [2, 4, 24, 40, 50];
        let t = get_trick(&v);
        let r = TrickResult::new(&t, Some(Suit::Spade), 1)?;
        serde_json::to_string(&r)?;
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_from_json() -> Result<()> {
        let v: FieldCardIds = [2, 4, 24, 40, 50];
        let trick = get_trick(&v).array()?;
        let winner = Player {
            id: "e".to_string(),
        };
        let face_cards: [u8; 3] = [4, 24, 50];
        let j = json!({
            "trick": trick,
            "winner": winner,
            "face_cards": face_cards,
        });
        let r: TrickResult = serde_json::from_str(j.to_string().as_str())?;
        for (r_card, t_card) in r.trick.iter().zip(trick.iter()) {
            assert_eq!(r_card.card, t_card.card);
        }
        for (r_card, t_id) in r.face_cards.iter().zip(face_cards.iter()) {
            assert_eq!(u8::try_from(*r_card).unwrap(), *t_id);
        }
        assert_eq!(r.winner, winner);
        Ok(())
    }
}
