use crate::card::Card;
use crate::declaration::Declaration;
use crate::player::{FieldPlayer, Player, Players, Role};
use crate::trick::TrickResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[allow(dead_code)]
enum Team {
    Napoleon,
    Union,
}

#[derive(Serialize, Deserialize)]
pub struct Round {
    trick_results: Vec<TrickResult>,
    declaration: Declaration,
    face_card_counter: HashMap<FieldPlayer, Vec<Card>>,
}

impl Round {
    #[allow(dead_code)]
    fn new(declaration: Declaration) -> Self {
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
    fn team_score(&self) -> (usize, usize) {
        let mut napo_score = 0;
        let mut union_score = 0;
        for (player, face_cards) in &self.face_card_counter {
            let s = face_cards.len();
            match player.role {
                Role::Napoleon | Role::Aide => napo_score += s,
                Role::Union => union_score += s,
            }
        }
        (napo_score, union_score)
    }

    #[allow(dead_code)]
    fn winner(&self) -> Option<Team> {
        let (napo_score, union_score) = self.team_score();
        if union_score > 20 - self.declaration.number {
            return Some(Team::Union);
        }
        if self.declaration.number == 20 {
            return if napo_score == 20 {
                Some(Team::Napoleon)
            } else {
                None
            };
        }
        if napo_score == 20 {
            return Some(Team::Union);
        }
        if napo_score >= self.declaration.number {
            return Some(Team::Napoleon);
        }
        None
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
    fn new(players: Players) -> Self {
        let rounds: Vec<Round> = Vec::new();
        Game {
            player_scores: players.map(PlayerScore::new),
            rounds,
        }
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
