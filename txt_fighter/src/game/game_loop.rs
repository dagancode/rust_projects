use crate::game::execute_turn::execute_turn;
use crate::models::fighter::Fighter;

/// A loop that continues until one of the fighters is no longer alive.
///
/// It takes two fighters as arguments and simulates a battle between them.
/// The fighters will take turns attacking each other until one of them is no longer alive.
///
/// In each turn, the fighters will display their current health and the damage they have dealt.
/// The loop will also display the round number and a VS separator between each turn.
///
/// The loop will continue until one of the fighters is no longer alive, at which point it will display the winner and the final health of both fighters.
pub fn game_loop(attacker: &mut dyn Fighter, defender: &mut dyn Fighter) {
    if !attacker.is_alive() || !defender.is_alive() {
        return;
    }

    let mut round = 1;
    loop {
        println!(
            "+====================================================+ [Round: {}]",
            round
        );
        #[cfg(feature = "slow")]
        std::thread::sleep(std::time::Duration::from_secs(1));
        // Turn 1 - Hero defends
        if execute_turn(attacker, defender) {
            #[cfg(feature = "slow")]
            std::thread::sleep(std::time::Duration::from_secs(1));
            handle_victory(attacker, defender);
            break;
        }

        #[cfg(feature = "slow")]
        std::thread::sleep(std::time::Duration::from_millis(500));
        println!("|-------------------------VS-------------------------|");
        #[cfg(feature = "slow")]
        std::thread::sleep(std::time::Duration::from_millis(500));


        // Turn 2 - Hero Attacks
        if execute_turn(defender, attacker) {
            #[cfg(feature = "slow")]
            std::thread::sleep(std::time::Duration::from_secs(1));
            handle_victory(defender, attacker);
            break;
        }
        round += 1;

        #[cfg(feature = "slow")]
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}

/// Handles the victory of a fighter by displaying the winner's name, the items dropped by the loser,
/// consuming the items and displaying the final health of both fighters.
fn handle_victory(winner: &mut dyn Fighter, loser: &mut dyn Fighter) {
    println!(
        "+====================================================+\n\n{} won!\n",
        winner.name()
    );

    let drops = loser.drop_items();

    winner.consume_items(drops);

    winner.display();
    loser.display();
}
