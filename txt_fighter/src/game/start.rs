use std::io::{self, Write};

use rand::{self, RngExt, rng};

use crate::game::game_loop;
use crate::models::characters::*;
use crate::models::fighter::Fighter;

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
            let mut hero = Hero::new();
            let enemies:Vec<Box<dyn Fighter>> = campaign_enemies();

            for mut enemy in enemies {
                game_loop(enemy.as_mut(), &mut hero);

                if !hero.is_alive() {
                    break;
                }
            }
        }
        GameMode::RandomBattle => {
            let mut hero = Hero::new();
            let mut enemies:Vec<Box<dyn Fighter>> = campaign_enemies();

            let random = rng().random_range(0..enemies.len());

            let random_enemy: &mut Box<dyn Fighter> = &mut enemies[random];

            game_loop(random_enemy.as_mut(), &mut hero);
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

fn campaign_enemies() -> Vec<Box<dyn Fighter>> {
    let goblin = Box::new(Goblin::new());
    let skeleton = Box::new(Skeleton::new());
    let dragon = Box::new(Dragon::new());

    vec![goblin, skeleton, dragon]
}
