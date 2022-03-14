use crate::declaration::Declaration;
use crate::player::{Player, Players};
use crate::trick::TrickResult;
// use std::collections::HashMap;

#[allow(dead_code)]
enum Team {
    NapoleonTeam,
    Union,
}

#[allow(dead_code)]
struct PlayerScore {
    player: Player,
    score: usize,
}

#[allow(dead_code)]
type PlayerScores = [PlayerScore; 5];

#[allow(dead_code)]
pub struct RoundResult {
    trick_results: Vec<TrickResult>,
    declaration: Declaration,
}

impl RoundResult {
    #[allow(dead_code)]
    fn new(declaration: Declaration) -> Self {
        let trick_results: Vec<TrickResult> = Vec::new();
        RoundResult {
            trick_results,
            declaration,
        }
    }

    #[allow(dead_code)]
    fn add(&mut self, result: TrickResult) {
        self.trick_results.push(result);
    }

    // #[allow(dead_code)]
    // fn judge(&self) {
    //     let mut counter = HashMap::new();
    //     self.trick_results.iter().map(|t| {
    //         let s = t.face_cards.len();
    //         let c = counter.entry(t.winner.clone()).or_insert(s);
    //         *c += s;
    //     });
    // }

    // #[allow(dead_code)]
    // fn get_score(&self) -> PlayerScores {}
}

impl PlayerScore {
    #[allow(dead_code)]
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
pub struct Game {
    players: Players,
    round_results: Vec<RoundResult>,
}

impl Game {
    #[allow(dead_code)]
    fn new(players: Players) -> Self {
        let round_results: Vec<RoundResult> = Vec::new();
        Game {
            players,
            round_results,
        }
    }

    #[allow(dead_code)]
    fn add(&mut self, result: RoundResult) {
        self.round_results.push(result);
    }

    // #[allow(dead_code)]
    // fn get_scores(&self) {
    //     let scores = self.players.iter().map(|u| PlayerScore::new(u.clone()));
    //     self.round_results
    //         .iter()
    //         .map(|r| scores.map(|s| r.get_score()));
    // }
}
