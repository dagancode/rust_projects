use std::io::{self, Write};

use rand::{Rng, rng};

use crate::game::game_loop;
use crate::models::characters::*;

pub fn start() {
    match select_mode() {
        GameMode::_Campaign => {}, // TODO
        GameMode::RandomBattle => {
            let mut hero = crate::models::characters::hero::Hero::new();

            let random = rng().next_u32();

            let random_enemy: Enemy = if random % 2 == 0 {
                Enemy::Goblin(crate::models::characters::goblin::Goblin::new())
            } else {
                Enemy::Dragon(crate::models::characters::dragon::Dragon::new())
            };
            
            match random_enemy {
                Enemy::Goblin(mut goblin) => {
                    game_loop(&mut goblin, &mut hero);
                },
                Enemy::Dragon(mut dragon) => {
                    game_loop(&mut dragon, &mut hero);
                },
            };
        },
    }
}


/// Selects a game mode
fn select_mode() -> GameMode {
    let mut user_input = String::new();

    println!("Select a game mode:\n1. Random Battle");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    match user_input.trim().to_lowercase().as_str() {
        //"1" => GameMode::Campaign,
        "2" => GameMode::RandomBattle,
        //"campaign" => GameMode::Campaign,
        "randombattle" => GameMode::RandomBattle,
        _ => GameMode::RandomBattle,
    }
}

enum GameMode {
    _Campaign,
    RandomBattle,
}

enum Enemy {
    Goblin(Goblin),
    Dragon(Dragon),
}
