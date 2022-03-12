use crate::trump::Trump;
use anyhow::Result;
use rand::seq::SliceRandom;
use rand::thread_rng;

type Hands = [Trump; 10];

#[derive(Debug)]
#[allow(dead_code)]
struct GameCards {
    hands: [Hands; 5],
    opens: [Trump; 2],
}

impl GameCards {
    #[allow(dead_code)]
    fn distribute() -> Result<Self> {
        let mut v: Vec<u8> = (1..53).collect();
        v.shuffle(&mut thread_rng());
        let mut hands: [Hands; 5] = Default::default();
        for pid in 0..5 {
            let mut h: Hands = Default::default();
            for i in 0..10 {
                let trump_id = v[(pid * 10) + i];
                h[i] = Trump::from_id(trump_id)?;
            }
            hands[pid] = h;
        }
        let opens: [Trump; 2] = [Trump::from_id(v[50])?, Trump::from_id(v[51])?];
        Ok(GameCards { hands, opens })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distribute() -> Result<()> {
        GameCards::distribute()?;
        Ok(())
    }
}
