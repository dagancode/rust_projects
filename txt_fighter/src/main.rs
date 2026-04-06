use txt_fighter::game;

fn main() {
    println!("Hello, players! Let's battle!\n");
    let start = std::time::Instant::now();

    game::start();

    let end = std::time::Instant::now();

    let duration = end - start;
    println!("\nCompleted in {:?}", duration);

    println!("╔════╗");
    println!("║    ║");
    println!("╚════╝");
}
