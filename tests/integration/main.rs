extern crate napo;
use anyhow::{bail, Result};
use napo::card::Suit;
use napo::trick::Play;

#[test]
fn game() -> Result<()> {
    // プレイヤーを揃えます
    let players = napo::player::get_dummy_players();

    // ゲーム開始
    let game = napo::game::Game::new(players);

    // カードを配ります
    #[allow(unused_variables)]
    let (mut field_players, opens) = game.distribute();

    // プレイヤー0がナポレオンになります
    let napoleon = field_players[0].clone();

    // 立ちを決めます
    let suit = Suit::Spade;
    let number = 15;

    // 副官を指名します
    let aide_card = field_players[1].hands[0].clone();

    // 立ちを宣言します
    #[allow(unused_variables)]
    let declaration = napo::declaration::Declaration::new(
        napoleon,
        &mut field_players,
        Some(suit),
        number,
        aide_card,
    )?;

    // let p = Play::new(
    //     declaration.players[0],
    //     game_cards.h
    // )

    // let round = declaration.create_round();
    // println!("{:?}", round);
    bail!("hoge")
}
