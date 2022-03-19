use crate::card::Card;
use crate::cards::distribute_cards;
use crate::declaration::Declaration;
use crate::player::{FieldPlayers, Player, Players, Role};
use crate::trick_result::TrickResult;
use anyhow::{anyhow, bail, ensure, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Team {
    Napoleon,
    Union,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Round {
    pub field_players: FieldPlayers,
    pub opens: [Card; 2],
    trick_results: Vec<TrickResult>,
    declaration: Option<Declaration>,
    face_card_counter: HashMap<Player, Vec<Card>>,
}

impl Round {
    #[allow(dead_code)]
    pub fn new(players: Players) -> Self {
        let trick_results: Vec<TrickResult> = Vec::new();
        let (field_players, opens) = distribute_cards(&players);
        Round {
            field_players,
            opens,
            trick_results,
            declaration: None,
            face_card_counter: HashMap::new(),
        }
    }

    pub fn set_declaration(&mut self, declaration: Declaration) -> Result<()> {
        ensure!(self.declaration.is_none(), "Napoleon is already set");

        for p in self.field_players.iter_mut() {
            if p.player == declaration.napoleon {
                p.assign_role(Role::Napoleon);
            } else if p.has(&declaration.aide) {
                p.assign_role(Role::Aide);
            } else {
                p.assign_role(Role::Union);
            }
        }
        ensure!(
            self.field_players
                .iter()
                .any(|p| p.role == Some(Role::Napoleon)),
            "Napoleon is not found"
        );
        self.declaration = Some(declaration);
        Ok(())
    }

    #[allow(dead_code)]
    fn is_alone(&self) -> bool {
        !self
            .field_players
            .iter()
            .any(|p| p.role == Some(Role::Aide))
    }

    #[allow(dead_code)]
    fn last_winner(&self) -> Result<Player> {
        Ok(match self.trick_results.last() {
            Some(r) => r.winner.clone(),
            None => self
                .field_players
                .iter()
                .filter(|p| p.role == Some(Role::Napoleon))
                .next()
                .context("napoleon is not found")?
                .player
                .clone(),
        })
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
        ensure!(self.face_card_counter.len() > 0, "round is not set yet");
        for (player, face_cards) in &self.face_card_counter {
            let s = face_cards.len();
            let role = self
                .field_players
                .iter()
                .filter(|p| p.player == *player)
                .next()
                .unwrap()
                .role
                .as_ref();
            match role {
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
        let declaration = self
            .declaration
            .as_ref()
            .ok_or(anyhow!("declaration is not set"))?;
        if union_score > 20 - declaration.number {
            return Ok(Some(Team::Union));
        }
        if declaration.number == 20 {
            return if napo_score == 20 {
                Ok(Some(Team::Napoleon))
            } else {
                Ok(None)
            };
        }
        if napo_score == 20 {
            return Ok(Some(Team::Union));
        }
        if napo_score >= declaration.number {
            return Ok(Some(Team::Napoleon));
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::get_dummy_players;
    use crate::trick::{Play, Trick, TrickArray};

    #[test]
    fn test_round_new() {
        Round::new(get_dummy_players());
    }

    #[test]
    fn test_set_declaration() -> Result<()> {
        let players = get_dummy_players();
        let mut r = Round::new(players.clone());
        let d = Declaration::new(players[0].clone(), None, 13, Card::from_id(1)?)?;
        r.set_declaration(d)?;
        Ok(())
    }

    #[test]
    fn test_set_declaration_invalid_napoleon() -> Result<()> {
        let players = get_dummy_players();
        let mut r = Round::new(players.clone());
        let d = Declaration::new(players[0].clone(), None, 13, Card::from_id(1)?)?;
        r.set_declaration(d)?;
        Ok(())
    }

    #[test]
    fn test_set_declaration_already_set() -> Result<()> {
        let players = get_dummy_players();
        let mut r = Round::new(players.clone());
        let d = Declaration::new(players[0].clone(), None, 13, Card::from_id(1)?)?;
        r.set_declaration(d)?;

        let d = Declaration::new(players[1].clone(), None, 13, Card::from_id(1)?)?;
        assert_eq!(r.set_declaration(d).is_err(), true);
        Ok(())
    }

    #[test]
    fn test_is_alone() -> Result<()> {
        let players = get_dummy_players();
        let mut r = Round::new(players.clone());
        let d = Declaration::new(
            r.field_players[0].player.clone(),
            None,
            13,
            r.field_players[0].hands[0].clone(),
        )?;
        r.set_declaration(d)?;
        assert_eq!(r.is_alone(), true);

        let mut r = Round::new(players.clone());
        let d = Declaration::new(
            r.field_players[0].player.clone(),
            None,
            13,
            r.field_players[1].hands[0].clone(),
        )?;
        r.set_declaration(d)?;
        assert_eq!(r.is_alone(), false);
        Ok(())
    }

    fn dummy_trick(field_players: FieldPlayers, i: usize) -> TrickArray {
        let mut trick = Trick::new();
        for p in field_players {
            trick.add(Play::new(p.player.clone(), p.hands[i].clone()));
        }
        trick.array().unwrap()
    }

    #[test]
    fn test_add() -> Result<()> {
        let players = get_dummy_players();
        let mut r = Round::new(players.clone());
        let d = Declaration::new(
            r.field_players[0].player.clone(),
            None,
            13,
            r.field_players[1].hands[0].clone(),
        )?;
        r.set_declaration(d)?;

        let trick_result = TrickResult {
            trick: dummy_trick(r.field_players.clone(), 0),
            winner: players[0].clone(),
            face_cards: vec![Card::from_id(1)?, Card::from_id(10)?],
        };

        r.add(trick_result);
        assert_eq!(r.trick_results.len(), 1);
        Ok(())
    }

    #[test]
    fn test_team_score() -> Result<()> {
        let players = get_dummy_players();
        let mut r = Round::new(players.clone());
        assert_eq!(r.team_score().is_err(), true);

        let d = Declaration::new(
            r.field_players[0].player.clone(),
            None,
            13,
            r.field_players[1].hands[0].clone(),
        )?;
        r.set_declaration(d)?;

        let trick_result = TrickResult {
            trick: dummy_trick(r.field_players.clone(), 0),
            winner: players[0].clone(),
            face_cards: vec![Card::from_id(1)?, Card::from_id(10)?],
        };
        r.add(trick_result);

        let trick_result = TrickResult {
            trick: dummy_trick(r.field_players.clone(), 1),
            winner: players[2].clone(),
            face_cards: vec![Card::from_id(11)?, Card::from_id(12)?, Card::from_id(13)?],
        };
        r.add(trick_result);

        let (napo, union) = r.team_score()?;
        println!("napo: {:?}, union: {:?}", napo, union);
        assert_eq!(napo, 2);
        assert_eq!(union, 3);
        Ok(())
    }

    #[test]
    fn test_winner_napoleon() -> Result<()> {
        let players = get_dummy_players();
        let mut r = Round::new(players.clone());
        assert_eq!(r.winner().is_err(), true);

        let d = Declaration::new(
            r.field_players[0].player.clone(),
            None,
            13,
            r.field_players[1].hands[0].clone(),
        )?;
        r.set_declaration(d)?;

        r.add(TrickResult {
            trick: dummy_trick(r.field_players.clone(), 0),
            winner: players[0].clone(),
            face_cards: vec![Card::from_id(1).unwrap()],
        });
        // ナポレオンが1枚
        assert_eq!(r.team_score()?.0, 1);
        assert_eq!(r.winner()?.is_none(), true);

        r.add(TrickResult {
            trick: dummy_trick(r.field_players.clone(), 0),
            winner: players[1].clone(),
            face_cards: (2..14).map(|i| Card::from_id(i).unwrap()).collect(),
        });
        // 副官が11枚とって13
        assert_eq!(r.team_score()?.0, 13);
        assert_eq!(r.winner()?, Some(Team::Napoleon));

        r.add(TrickResult {
            trick: dummy_trick(r.field_players.clone(), 0),
            winner: players[0].clone(),
            face_cards: (14..21).map(|i| Card::from_id(i).unwrap()).collect(),
        });
        // ナポレオンが7枚とって全取り
        assert_eq!(r.team_score()?.0, 20);
        assert_eq!(r.winner()?, Some(Team::Union));

        Ok(())
    }

    #[test]
    fn test_winner_union() -> Result<()> {
        let players = get_dummy_players();
        let mut r = Round::new(players.clone());
        assert_eq!(r.winner().is_err(), true);

        let d = Declaration::new(
            r.field_players[0].player.clone(),
            None,
            13,
            r.field_players[1].hands[0].clone(),
        )?;
        r.set_declaration(d)?;

        r.add(TrickResult {
            trick: dummy_trick(r.field_players.clone(), 0),
            winner: players[2].clone(),
            face_cards: vec![Card::from_id(1).unwrap()],
        });
        // 連合が1枚
        assert_eq!(r.team_score()?.1, 1);
        assert_eq!(r.winner()?.is_none(), true);

        r.add(TrickResult {
            trick: dummy_trick(r.field_players.clone(), 0),
            winner: players[3].clone(),
            face_cards: (2..9).map(|i| Card::from_id(i).unwrap()).collect(),
        });
        // 副官が7枚とって8枚。終了。
        assert_eq!(r.team_score()?.1, 8);
        assert_eq!(r.winner()?, Some(Team::Union));
        Ok(())
    }

    #[test]
    fn test_last_winner() -> Result<()> {
        let players = get_dummy_players();
        let mut r = Round::new(players.clone());
        assert_eq!(r.last_winner().is_err(), true);

        let d = Declaration::new(
            r.field_players[0].player.clone(),
            None,
            13,
            r.field_players[1].hands[0].clone(),
        )?;
        r.set_declaration(d)?;
        assert_eq!(r.last_winner().unwrap(), r.field_players[0].player.clone());

        r.add(TrickResult {
            trick: dummy_trick(r.field_players.clone(), 0),
            winner: players[2].clone(),
            face_cards: vec![Card::from_id(1).unwrap()],
        });
        assert_eq!(r.last_winner().unwrap(), r.field_players[2].player.clone());
        Ok(())
    }
}
