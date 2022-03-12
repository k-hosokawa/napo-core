use crate::trump::{Suit, Trump};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

#[allow(dead_code)]
type FieldCards = [Trump; 5];

#[allow(dead_code)]
type FieldCardIds = [u8; 5];

#[allow(dead_code)]
fn get_field_cards(ids: &FieldCardIds) -> FieldCards {
    ids.iter()
        .map(|c| Trump::from_id(*c).unwrap())
        .collect::<Vec<Trump>>()
        .try_into()
        .unwrap()
}

struct RoundResultBuilder {
    cards: FieldCards,
    face_cards: Vec<Trump>,
}

impl RoundResultBuilder {
    fn new(cards: &FieldCards) -> Self {
        let face_cards: Vec<Trump> = cards.iter().filter(|c| c.is_face()).cloned().collect();
        RoundResultBuilder {
            cards: (*cards).clone(),
            face_cards,
        }
    }

    fn build(&self, winner_id: usize) -> Result<RoundResult> {
        Ok(RoundResult {
            cards: self.cards.clone(),
            face_cards: self.face_cards.clone(),
            winner_id,
        })
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct RoundResult {
    cards: FieldCards,
    winner_id: usize,
    face_cards: Vec<Trump>,
}

impl RoundResult {
    #[allow(dead_code)]
    pub fn new(cards: &FieldCards, suit: Option<Suit>, n_round: u8) -> Result<Self> {
        let builder = RoundResultBuilder::new(cards);

        // almighty
        if let Some(id) = builder.cards.iter().position(|c| c.is_almighty()) {
            return builder.build(
                builder
                    .cards
                    .iter()
                    .position(|c| c.is_yoromeki())
                    .unwrap_or(id),
            );
        }

        // jack
        if let Some(s) = suit {
            let id = builder
                .cards
                .iter()
                .position(|c| (c.number == 11) && (c.suit == s));
            if let Some(i) = id {
                return builder.build(i);
            }

            // reverse jack
            let rev_suit = s.reverse();
            let id = builder
                .cards
                .iter()
                .position(|c| (c.number == 11) && (c.suit == rev_suit));
            if let Some(i) = id {
                return builder.build(i);
            }
        }

        let first_suit = builder.cards[0].suit;

        // same2
        if n_round > 1 && (builder.cards.iter().all(|c| c.suit == first_suit)) {
            if let Some(id) = builder.cards.iter().position(|c| c.number == 2) {
                return builder.build(id);
            }
        }

        let mut winner_id = 0;
        for i in 1..5 {
            let c = &builder.cards[i];
            if c.suit == first_suit {
                if c.number == 1 {
                    return builder.build(i);
                }
                if c.number > builder.cards[winner_id].number {
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

    #[test]
    fn test_ids_to_cards() -> Result<()> {
        let v: FieldCardIds = [1, 4, 25, 40, 52];
        get_field_cards(&v);
        Ok(())
    }

    #[test]
    fn test_judge_winner_almighty() -> Result<()> {
        let v: FieldCardIds = [1, 4, 24, 40, 52];
        let cs = get_field_cards(&v);
        let r = RoundResult::new(&cs, None, 1)?;
        assert_eq!(r.winner_id, 0);
        assert_eq!(
            r.face_cards,
            vec![
                Trump::from_id(1)?,
                Trump::from_id(24)?,
                Trump::from_id(40)?,
                Trump::from_id(52)?,
            ],
        );
        Ok(())
    }

    #[test]
    fn test_judge_winner_yoromeki() -> Result<()> {
        let v: FieldCardIds = [1, 4, 25, 40, 52];
        let cs = get_field_cards(&v);
        let r = RoundResult::new(&cs, None, 1)?;
        assert_eq!(r.winner_id, 2);
        assert_eq!(
            r.face_cards,
            vec![
                Trump::from_id(1)?,
                Trump::from_id(25)?,
                Trump::from_id(40)?,
                Trump::from_id(52)?,
            ],
        );
        Ok(())
    }

    #[test]
    fn test_judge_winner_jack() -> Result<()> {
        let v: FieldCardIds = [2, 11, 24, 40, 52];
        let cs = get_field_cards(&v);
        let r = RoundResult::new(&cs, Some(Suit::Spade), 1)?;
        assert_eq!(r.winner_id, 1);
        assert_eq!(
            r.face_cards,
            vec![
                Trump::from_id(11)?,
                Trump::from_id(24)?,
                Trump::from_id(40)?,
                Trump::from_id(52)?,
            ],
        );
        Ok(())
    }

    #[test]
    fn test_judge_winner_rev_jack() -> Result<()> {
        let v: FieldCardIds = [2, 4, 24, 40, 50];
        let cs = get_field_cards(&v);
        let r = RoundResult::new(&cs, Some(Suit::Spade), 1)?;
        assert_eq!(r.winner_id, 4);
        assert_eq!(
            r.face_cards,
            vec![
                Trump::from_id(24)?,
                Trump::from_id(40)?,
                Trump::from_id(50)?,
            ],
        );
        Ok(())
    }

    #[test]
    fn test_judge_winner_same2() -> Result<()> {
        let v: FieldCardIds = [2, 3, 4, 5, 6];

        let cs = get_field_cards(&v);
        let r = RoundResult::new(&cs, None, 2)?;
        assert_eq!(r.winner_id, 0);
        assert_eq!(r.face_cards, Vec::<Trump>::new(),);

        let r = RoundResult::new(&cs, None, 1)?;
        assert_eq!(r.winner_id, 4);
        assert_eq!(r.face_cards, Vec::<Trump>::new(),);
        Ok(())
    }

    #[test]
    fn test_to_json() -> Result<()> {
        let v: FieldCardIds = [2, 4, 24, 40, 50];
        let cs = get_field_cards(&v);
        let r = RoundResult::new(&cs, Some(Suit::Spade), 1)?;
        serde_json::to_string(&r)?;
        Ok(())
    }

    #[test]
    fn test_from_json() -> Result<()> {
        let cards: [u8; 5] = [2, 4, 24, 40, 50];
        let winner_id: usize = 4;
        let face_cards: [u8; 3] = [4, 24, 50];
        let j = json!({
            "cards": cards,
            "winner_id": winner_id,
            "face_cards": face_cards,
        });
        let r: RoundResult = serde_json::from_str(j.to_string().as_str())?;
        for i in 0..5 {
            assert_eq!(r.cards[i].to_id(), cards[i]);
        }
        for i in 0..3 {
            assert_eq!(r.face_cards[i].to_id(), face_cards[i]);
        }
        assert_eq!(r.winner_id, r.winner_id);
        Ok(())
    }
}
