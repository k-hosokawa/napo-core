use crate::card::Card;
use crate::declaration::Declaration;
use crate::player::{FieldPlayer, FieldPlayers, Player, Players, Role};
use crate::trick::TrickResult;
use anyhow::{bail, Result};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryInto;

#[allow(dead_code)]
enum Team {
    Napoleon,
    Union,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Round {
    trick_results: Vec<TrickResult>,
    declaration: Declaration,
    face_card_counter: HashMap<FieldPlayer, Vec<Card>>,
}

impl Round {
    #[allow(dead_code)]
    pub fn new(declaration: Declaration) -> Self {
        let trick_results: Vec<TrickResult> = Vec::new();
        Round {
            trick_results,
            declaration,
            face_card_counter: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    fn add(&mut self, result: TrickResult) {
        (*self
            .face_card_counter
            .entry(result.winner.clone())
            .or_insert(Vec::new()))
        .extend(result.face_cards.iter().cloned());
        self.trick_results.push(result);
    }

    #[allow(dead_code)]
    fn team_score(&self) -> Result<(usize, usize)> {
        let mut napo_score = 0;
        let mut union_score = 0;
        for (player, face_cards) in &self.face_card_counter {
            let s = face_cards.len();
            match player.role {
                Some(Role::Napoleon) | Some(Role::Aide) => napo_score += s,
                Some(Role::Union) => union_score += s,
                None => bail!("role is not set"),
            }
        }
        Ok((napo_score, union_score))
    }

    #[allow(dead_code)]
    fn winner(&self) -> Result<Option<Team>> {
        let (napo_score, union_score) = self.team_score()?;
        if union_score > 20 - self.declaration.number {
            return Ok(Some(Team::Union));
        }
        if self.declaration.number == 20 {
            return if napo_score == 20 {
                Ok(Some(Team::Napoleon))
            } else {
                Ok(None)
            };
        }
        if napo_score == 20 {
            return Ok(Some(Team::Union));
        }
        if napo_score >= self.declaration.number {
            return Ok(Some(Team::Napoleon));
        }
        Ok(None)
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
struct PlayerScore {
    player: Player,
    score: usize,
}

type PlayerScores = [PlayerScore; 5];

impl PlayerScore {
    fn new(player: Player) -> Self {
        PlayerScore { player, score: 0 }
    }

    #[allow(dead_code)]
    fn add(&mut self, player: Player, score: usize) {
        if self.player == player {
            self.score += score;
        }
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct Game {
    player_scores: PlayerScores,
    rounds: Vec<Round>,
}

impl Game {
    #[allow(dead_code)]
    pub fn new(players: Players) -> Self {
        let rounds: Vec<Round> = Vec::new();
        Game {
            player_scores: players.map(PlayerScore::new),
            rounds,
        }
    }

    pub fn distribute(&self) -> (FieldPlayers, [Card; 2]) {
        let mut v: Vec<u8> = (1..53).collect();
        v.shuffle(&mut thread_rng());
        let players: FieldPlayers = self
            .player_scores
            .iter()
            .enumerate()
            .map(|(pid, ps)| {
                FieldPlayer::new(
                    ps.player.clone(),
                    (0..10)
                        .map(|i| Card::from_id(v[(pid * 10) + i]).unwrap())
                        .collect::<Vec<Card>>()
                        .try_into()
                        .unwrap(),
                )
            })
            .collect::<Vec<FieldPlayer>>()
            .try_into()
            .unwrap();
        let opens = [Card::from_id(v[50]).unwrap(), Card::from_id(v[51]).unwrap()];

        (players, opens)
    }

    #[allow(dead_code)]
    fn add(&mut self, round: Round) {
        self.rounds.push(round);
    }

    // #[allow(dead_code)]
    // fn get_scores(&self) {
    //     let scores = self.players.iter().map(|u| PlayerScore::new(u.clone()));
    //     self.round_results
    //         .iter()
    //         .map(|r| scores.map(|s| r.get_score()));
    // }
}
