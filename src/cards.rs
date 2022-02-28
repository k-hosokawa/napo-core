use anyhow::Result;
use crate::trump::Trump;
use rand::thread_rng;
use rand::seq::SliceRandom;


#[derive(Debug)]
#[allow(dead_code)]
struct Distributed {
    hands: Vec<Vec<Trump>>,
    opens: Vec<Trump>,
}

impl Distributed {
    #[allow(dead_code)]
    fn new() -> Result<Self> {
        let mut v: Vec<usize> = (1..53).collect();
        v.shuffle(&mut thread_rng());
        let mut hands: Vec<Vec<Trump>> = Vec::new();
        for pid in 0..5 {
            let mut h: Vec<Trump> = Vec::new();
            for i in 0..10 {
                let trump_id = v[(pid * 10) + i];
                h.push(Trump::from_id(trump_id)?);
            }
            hands.push(h);
        }
        let opens = vec![
            Trump::from_id(v[50])?,
            Trump::from_id(v[51])?,
        ];
        Ok(Distributed{hands, opens})
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distributed_new() -> Result<()> {
        Distributed::new()?;
        Ok(())
    }
}
