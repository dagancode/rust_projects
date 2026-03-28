use crate::models::fighter::Fighter;

pub fn execute_turn(attacker: &mut dyn Fighter, defender: &mut dyn Fighter) -> bool {
    attacker.refresh_state();
    defender.refresh_state();

    let base_damage = attacker.attack();
    let special_attack = attacker.special_attack().unwrap_or_default();

    let shielded_damage = defender
        .shield_attack(base_damage + special_attack.damage as u32)
        .unwrap_or(0);

    let total_damage = base_damage + special_attack.damage as u32 - shielded_damage;
    defender.take_damage(total_damage as i32);

    println!(
        "{} took {}/{} damage from {}\n",
        defender.name(),
        total_damage,
        base_damage + special_attack.damage as u32,
        //bonus_damage_text,
        attacker.name()
    );

    if let Some(status_effect) = special_attack.effect {
        defender.add_effect(status_effect);
    }

    #[cfg(feature = "slow")]
    std::thread::sleep(std::time::Duration::from_secs(1));
    if special_attack.damage > 0 {
        println!(
            "~ {} caused {} damage ~",
            special_attack.name, special_attack.damage
        )
    }
    #[cfg(feature = "slow")]
    std::thread::sleep(std::time::Duration::from_secs(1));
    if shielded_damage > 0 {
        println!("~ Shield blocked {} damage ~", shielded_damage)
    }

    attacker.display();
    defender.display();

    !defender.is_alive()
}