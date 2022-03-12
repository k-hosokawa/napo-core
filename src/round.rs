use anyhow::Result;
use crate::trump::{Trump, Suit};

#[allow(dead_code)]
type FieldCardIds = [usize; 5];

#[allow(dead_code)]
type FieldCards = [Trump; 5];

#[allow(dead_code)]
fn ids_to_cards(ids: &FieldCardIds) -> Vec<Trump> {
    ids
        .into_iter()
        .map(|c| Trump::from_id(*c).unwrap())
        .collect::<Vec<Trump>>()
}

struct RoundResultBuilder {
    ids: FieldCardIds,
    cards: Vec<Trump>,
    face_cards: Vec<Trump>,
}

impl RoundResultBuilder {
    fn new(ids: &FieldCardIds) -> Self {
        let cards = ids_to_cards(&ids);
        let face_cards: Vec<Trump> = cards
            .iter()
            .filter(|c| c.is_face())
            .cloned()
            .collect();
        RoundResultBuilder{
            ids: *ids,
            cards,
            face_cards,
        }
    }

    fn build(&self, winner_id: usize) -> Result<RoundResult> {
        Ok(RoundResult{
            cards: self.ids,
            face_cards: self.face_cards.clone(),
            winner_id,
        })
    }
}

#[allow(dead_code)]
pub struct RoundResult {
    cards: FieldCardIds,
    winner_id: usize,
    face_cards: Vec<Trump>,
}

impl RoundResult {
    #[allow(dead_code)]
    pub fn new(
        ids: &FieldCardIds,
        suit: Option<Suit>,
        n_round: usize,
    ) -> Result<Self> {
        let builder = RoundResultBuilder::new(ids);

        let almighty = builder.cards.iter().position(|c| c.is_almighty());
        let yoromeki = builder.cards.iter().position(|c| c.is_yoromeki());

        if let Some(id) = almighty {
            return builder.build(yoromeki.unwrap_or(id));
        }

        // jack
        if let Some(s) = suit {
            let id = builder.cards
                .iter()
                .position(|c| (c.number == 11) && (c.suit == s));
            if let Some(i) = id {
                return builder.build(i)
            }

            // reverse jack
            let id = builder.cards
                .iter()
                .position(|c| (c.number == 11) && (c.suit == s.reverse()));
            if let Some(i) = id {
                return builder.build(i);
            }
        }

        let first_suit = builder.cards[0].suit;

        // same2
        if n_round > 1 && (
            builder.cards.iter().all(|c| c.suit == first_suit) 
        ) {
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

    #[test]
    fn test_ids_to_cards() -> Result<()> {
        let v: FieldCardIds = [1, 4, 25, 40, 52];
        ids_to_cards(&v);
        Ok(())
    }

    #[test]
    fn test_judge_winner_almighty() -> Result<()> {
        let v: FieldCardIds = [1, 4, 24, 40, 52];
        let r = RoundResult::new(&v, None, 1)?;
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
        let r = RoundResult::new(&v, None, 1)?;
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
        let r = RoundResult::new(&v, Some(Suit::Spade), 1)?;
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
        let r = RoundResult::new(&v, Some(Suit::Spade), 1)?;
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

        let r = RoundResult::new(&v, None, 2)?;
        assert_eq!(r.winner_id, 0);
        assert_eq!(
            r.face_cards,
            Vec::<Trump>::new(),
        );

        let r = RoundResult::new(&v, None, 1)?;
        assert_eq!(r.winner_id, 4);
        assert_eq!(
            r.face_cards,
            Vec::<Trump>::new(),
        );
        Ok(())
    }
}
