use std::{thread, time};

pub mod game;
use game::{Game,Table};

fn main() {
    let table = &mut Table::start_empty();
    let game = &mut Game::start(table);
    println!("Jogo começando");
    while !game.is_over() {
        while !game.play(game.players[game.turn].side, game.players[game.turn].choose_position()) {
            println!("Jogada não permitida");
        }
        game.turn = (game.turn + 1) % 2;
        println!("Jogada feita");
        thread::sleep(time::Duration::from_millis(2000));
    }
    println!("Jogo finalizado");
}