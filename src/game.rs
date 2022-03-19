use crate::player::{Player, Players};
use crate::round::Round;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

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

    pub fn new_round(&mut self) -> &mut Round {
        let players: Players = self
            .player_scores
            .iter()
            .map(|ps| ps.player.clone())
            .collect::<Vec<Player>>()
            .try_into()
            .unwrap();
        let round = Round::new(players);
        self.rounds.push(round);
        self.rounds.last_mut().unwrap()
    }

    // #[allow(dead_code)]
    // fn get_scores(&self) {
    //     let scores = self.players.iter().map(|u| PlayerScore::new(u.clone()));
    //     self.round_results
    //         .iter()
    //         .map(|r| scores.map(|s| r.get_score()));
    // }
}
