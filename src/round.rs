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

#[allow(dead_code)]
pub struct RoundResult {
    winner_id: usize,
    face_cards: Vec<Trump>,
}

#[allow(dead_code)]
pub fn judge_winner(
    ids: &FieldCardIds,
    suit: Option<Suit>,
    n_round: usize,
) -> Result<RoundResult> {
    let cards = ids_to_cards(&ids);

    let face_cards: Vec<Trump> = cards
        .iter()
        .filter(|c| c.is_face())
        .cloned()
        .collect();

    let almighty = cards.iter().position(|c| c.is_almighty());
    let yoromeki = cards.iter().position(|c| c.is_yoromeki());

    if let Some(id) = almighty {
        return Ok(RoundResult{
            winner_id: yoromeki.unwrap_or(id),
            face_cards,
        });
    }

    // jack
    if let Some(s) = suit {
        let id = cards
            .iter()
            .position(|c| (c.number == 11) && (c.suit == s));
        if let Some(i) = id {
            return Ok(RoundResult{winner_id: i, face_cards});
        }
    }

    // reverse jack
    if let Some(s) = suit {
        let id = cards
            .iter()
            .position(|c| (c.number == 11) && (c.suit == s.reverse()));
        if let Some(i) = id {
            return Ok(RoundResult{winner_id: i, face_cards});
        }
    }

    // same2
    if n_round > 1 && (
        cards.iter().all(|c| c.suit == cards[0].suit) 
    ) {
        if let Some(id) = cards.iter().position(|c| c.number == 2) {
            return Ok(RoundResult{winner_id: id, face_cards});
        }
    }

    let first_suit = cards[0].suit;
    let mut winner_id = 0;
    for i in 1..5 {
        if cards[i].suit == first_suit {
            if cards[i].number == 1 {
                return Ok(RoundResult{winner_id: i, face_cards});
            }
            if cards[i].number > cards[winner_id].number {
                winner_id = i;
            }
        }
    }
    Ok(RoundResult{winner_id, face_cards})
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
        let r = judge_winner(&v, None, 1)?;
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
        let r = judge_winner(&v, None, 1)?;
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
        let r = judge_winner(&v, Some(Suit::Spade), 1)?;
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
        let r = judge_winner(&v, Some(Suit::Spade), 1)?;
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

        let r = judge_winner(&v, None, 2)?;
        assert_eq!(r.winner_id, 0);
        assert_eq!(
            r.face_cards,
            Vec::<Trump>::new(),
        );

        let r = judge_winner(&v, None, 1)?;
        assert_eq!(r.winner_id, 4);
        assert_eq!(
            r.face_cards,
            Vec::<Trump>::new(),
        );
        Ok(())
    }
}
