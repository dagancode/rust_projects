use txt_fighter::models::{Dragon, Goblin, Hero};
use txt_fighter::game::game_loop;

fn main() {
    println!("Hello, players! Let's battle!\n");
    let start = std::time::Instant::now();

    let mut my_hero = Hero::new();
    let mut _enemy_goblin = Goblin::new();
    let mut _enemy_dragon = Dragon::new();

    game_loop(&mut _enemy_goblin, &mut my_hero);

    let end = std::time::Instant::now();

    let duration = end - start;
    println!("\nCompleted in {:?}", duration);
}
