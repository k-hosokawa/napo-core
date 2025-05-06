extern crate napo;
use anyhow::{bail, Result};
use napo::card::Suit;

#[test]
#[ignore]
fn game() -> Result<()> {
    // プレイヤーを揃えます
    let players = napo::player::Players::default();

    // ゲーム開始
    let mut game = napo::game::Game::new(players);

    // 一回戦開始
    let round = game.new_round();

    // プレイヤー0がナポレオンになります
    let napoleon = round.field_players[0].player.clone();

    // 立ちを決めます
    let suit = Suit::Spade;
    let number = 15;

    // 副官を指名します
    let aide_card = round.field_players[1].hands[0].clone();

    // 立ちを宣言します
    let declaration = napo::declaration::Declaration::new(napoleon, Some(suit), number, aide_card)?;

    // let p = Play::new(
    //     declaration.players[0],
    //     game_cards.h
    // )

    round.set_declaration(declaration)?;
    // println!("{:?}", round);
    bail!("hoge")
}
