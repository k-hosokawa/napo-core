use crate::card::{Card, Suit};
use crate::player::Player;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Play {
    player: Player,
    card: Card,
}

type Trick = [Play; 5];

struct TrickResultBuilder {
    trick: Trick,
    face_cards: Vec<Card>,
}

impl TrickResultBuilder {
    fn new(trick: &Trick) -> Self {
        let face_cards: Vec<Card> = trick
            .iter()
            .filter(|p| p.card.is_face())
            .map(|p| p.card.clone())
            .collect();
        TrickResultBuilder {
            trick: (*trick).clone(),
            face_cards,
        }
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
#[derive(Serialize, Deserialize)]
pub struct TrickResult {
    pub trick: Trick,
    pub winner: Player,
    pub face_cards: Vec<Card>,
}

impl TrickResult {
    #[allow(dead_code)]
    pub fn new(trick: &Trick, suit: Option<Suit>, n_round: u8) -> Result<Self> {
        let builder = TrickResultBuilder::new(trick);

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

        let first_suit = builder.trick[0].card.suit;

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
    use serde_json::json;
    use std::convert::TryInto;

    #[allow(dead_code)]
    type FieldCardIds = [u8; 5];

    #[allow(dead_code)]
    fn get_trick(ids: &FieldCardIds) -> Trick {
        ids.iter()
            .enumerate()
            .map(|(i, c)| Play {
                player: Player { id: i.to_string() },
                card: Card::from_id(*c).unwrap(),
            })
            .collect::<Vec<Play>>()
            .try_into()
            .unwrap()
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
        assert_eq!(r.winner.id, "0");
        assert_eq!(
            r.face_cards,
            vec![
                Card::from_id(1)?,
                Card::from_id(24)?,
                Card::from_id(40)?,
                Card::from_id(52)?,
            ],
        );
        Ok(())
    }

    #[test]
    fn test_judge_winner_yoromeki() -> Result<()> {
        let v: FieldCardIds = [1, 4, 25, 40, 52];
        let t = get_trick(&v);
        let r = TrickResult::new(&t, None, 1)?;
        assert_eq!(r.winner.id, "2");
        assert_eq!(
            r.face_cards,
            vec![
                Card::from_id(1)?,
                Card::from_id(25)?,
                Card::from_id(40)?,
                Card::from_id(52)?,
            ],
        );
        Ok(())
    }

    #[test]
    fn test_judge_winner_jack() -> Result<()> {
        let v: FieldCardIds = [2, 11, 24, 40, 52];
        let t = get_trick(&v);
        let r = TrickResult::new(&t, Some(Suit::Spade), 1)?;
        assert_eq!(r.winner.id, "1");
        assert_eq!(
            r.face_cards,
            vec![
                Card::from_id(11)?,
                Card::from_id(24)?,
                Card::from_id(40)?,
                Card::from_id(52)?,
            ],
        );
        Ok(())
    }

    #[test]
    fn test_judge_winner_rev_jack() -> Result<()> {
        let v: FieldCardIds = [2, 4, 24, 40, 50];
        let t = get_trick(&v);
        let r = TrickResult::new(&t, Some(Suit::Spade), 1)?;
        assert_eq!(r.winner.id, "4");
        assert_eq!(
            r.face_cards,
            vec![Card::from_id(24)?, Card::from_id(40)?, Card::from_id(50)?,],
        );
        Ok(())
    }

    #[test]
    fn test_judge_winner_same2() -> Result<()> {
        let v: FieldCardIds = [2, 3, 4, 5, 6];

        let t = get_trick(&v);
        let r = TrickResult::new(&t, None, 2)?;
        assert_eq!(r.winner.id, "0");
        assert_eq!(r.face_cards, Vec::<Card>::new(),);

        let r = TrickResult::new(&t, None, 1)?;
        assert_eq!(r.winner.id, "4");
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
    fn test_from_json() -> Result<()> {
        let v: FieldCardIds = [2, 4, 24, 40, 50];
        let trick = get_trick(&v);
        let winner = Player {
            id: "4".to_string(),
        };
        let face_cards: [u8; 3] = [4, 24, 50];
        let j = json!({
            "trick": trick,
            "winner": winner,
            "face_cards": face_cards,
        });
        let r: TrickResult = serde_json::from_str(j.to_string().as_str())?;
        for i in 0..5 {
            assert_eq!(r.trick[i].card, trick[i].card);
        }
        for i in 0..3 {
            assert_eq!(r.face_cards[i].to_id(), face_cards[i]);
        }
        assert_eq!(r.winner, winner);
        Ok(())
    }
}
