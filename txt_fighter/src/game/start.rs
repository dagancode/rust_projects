use std::io::{self, Write};

use rand::{Rng, rng};

use crate::game::game_loop;
use crate::models::characters::*;

/// Starts a game of TXT Fighter.
///
/// This function will prompt the user to select a game mode and then
/// start a battle between the user's hero and a randomly selected enemy.
///
/// The game mode can either be a campaign or a random battle.
///
/// If the user selects a campaign, they will be pitted against a Goblin
/// and then a Dragon in a best-of-two battle.
///
/// If the user selects a random battle, they will be pitted against a
/// randomly selected enemy. The enemy can either be a Goblin or a Dragon.
///
/// The function will continue to loop until the user's hero is no longer
/// alive or all enemies have been defeated.
pub fn start() {
    match select_mode() {
        GameMode::Campaign => {
            let mut hero = crate::models::characters::hero::Hero::new();
            let mut level_1_enemy = Goblin::new();

            game_loop(&mut level_1_enemy, &mut hero);

            let mut level_2_enemy = Dragon::new();

            game_loop(&mut level_2_enemy, &mut hero);
        }
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
                }
                Enemy::Dragon(mut dragon) => {
                    game_loop(&mut dragon, &mut hero);
                }
            };
        }
    }
}

/// Selects a game mode
fn select_mode() -> GameMode {
    let mut user_input = String::new();

    println!("Select a game mode:\n1. Campaign\n2. Random Battle");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    match user_input.trim().to_lowercase().as_str() {
        "1" => GameMode::Campaign,
        "2" => GameMode::RandomBattle,
        "campaign" => GameMode::Campaign,
        "randombattle" => GameMode::RandomBattle,
        _ => GameMode::RandomBattle,
    }
}

enum GameMode {
    Campaign,
    RandomBattle,
}

enum Enemy {
    Goblin(Goblin),
    Dragon(Dragon),
}
