use anyhow::{Result, bail};
use crate::trump::{Trump, Suit};

fn ids_to_cards(ids: &Vec<usize>) -> Vec<Trump> {
    ids
        .iter()
        .map(|c| Trump::from_id(*c).unwrap())
        .collect::<Vec<Trump>>()
}

pub fn judge_winner(
    ids: &Vec<usize>,
    suit: Option<Suit>,
    n_round: usize,
) -> Result<usize> {
    let cards = ids_to_cards(&ids);

    let almighty = cards.iter().position(|c| c.is_almighty());
    let yoromeki = cards.iter().position(|c| c.is_yoromeki());

    if let Some(id) = almighty {
        return Ok(yoromeki.unwrap_or(id));
    }

    // jack
    if let Some(s) = suit {
        let id = cards
            .iter()
            .position(|c| (c.number == 11) && (c.suit == s));
        if let Some(i) = id {
            return Ok(i);
        }
    }

    // reverse jack
    if let Some(s) = suit {
        let id = cards
            .iter()
            .position(|c| (c.number == 11) && (c.suit == s.reverse()));
        if let Some(i) = id {
            return Ok(i);
        }
    }

    // same2
    if n_round > 1 && (
        cards.iter().all(|c| c.suit == cards[0].suit) 
    ) {
        if let Some(id) = cards.iter().position(|c| c.number == 2) {
            return Ok(id);
        }
    }

    let first_suit = cards[0].suit;
    let mut winner_id = 0;
    for i in 1..5 {
        if cards[i].suit == first_suit {
            if cards[i].number == 1 {
                return Ok(i);
            }
            if cards[i].number > cards[winner_id].number {
                winner_id = i;
            }
        }
    }
    Ok(winner_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ids_to_cards() -> Result<()> {
        let v = vec![1, 4, 25, 40, 52];
        let cards = ids_to_cards(&v);
        Ok(())
    }

    #[test]
    fn test_judge_winner_almighty() -> Result<()> {
        let v = vec![1, 4, 24, 40, 52];
        assert_eq!(judge_winner(&v, None, 1)?, 0);
        Ok(())
    }

    #[test]
    fn test_judge_winner_yoromeki() -> Result<()> {
        let v = vec![1, 4, 25, 40, 52];
        assert_eq!(judge_winner(&v, None, 1)?, 2);
        Ok(())
    }

    #[test]
    fn test_judge_winner_jack() -> Result<()> {
        let v = vec![2, 11, 24, 40, 52];
        assert_eq!(judge_winner(&v, Some(Suit::Spade), 1)?, 1);
        Ok(())
    }

    #[test]
    fn test_judge_winner_rev_jack() -> Result<()> {
        let v = vec![2, 4, 24, 40, 50];
        assert_eq!(judge_winner(&v, Some(Suit::Spade), 1)?, 4);
        Ok(())
    }

    #[test]
    fn test_judge_winner_same2() -> Result<()> {
        let v = vec![2, 3, 4, 5, 6];
        assert_eq!(judge_winner(&v, None, 2)?, 0);
        assert_eq!(judge_winner(&v, None, 1)?, 4);
        Ok(())
    }
}
