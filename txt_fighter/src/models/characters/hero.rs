use crate::models::fighter::Fighter;
use crate::models::types::StatusEffect;

pub struct Hero {
    health: u32,
    damage: u32,
    shield_health: u32,
    name: String,
    state: Vec<StatusEffect>,
}

impl Hero {
    const BASE_HEALTH: u32 = 100;
    pub fn new() -> Self {
        Hero {
            health: Self::BASE_HEALTH,
            damage: 35,
            shield_health: 65,
            name: String::from("Hero"),
            state: Vec::new(),
        }
    }
}

impl Fighter for Hero {
    fn attack(&self) -> u32 {
        self.damage
    }

    fn health(&self) -> u32 {
        self.health
    }

    fn take_damage(&mut self, amount: i32) {
        if amount >= 0 {
            self.health = self.health.saturating_sub(amount as u32);
        } else {
            self.health = self.health.saturating_add(amount.unsigned_abs());
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn add_effect(&mut self, effect: StatusEffect) {
        self.state.push(effect)
    }

    fn clear_stale_effects(&mut self) {
        self.state.retain(|e| e.turns_remaining > 0);
    }

    fn apply_effects(&mut self) {
        let fighter_name = self.name.clone();

        let total_damage = self
            .state
            .iter_mut()
            .filter(|sfx| sfx.turns_remaining > 0)
            .map(|e| {
                e.turns_remaining -= 1;
                println!(
                    "~ {} suffers {} {} damage ~",
                    fighter_name, e.damage_per_turn, e.name
                );
                e.damage_per_turn
            })
            .sum();

        if total_damage != 0 {
            self.take_damage(total_damage);
        }
    }

    fn list_effects(&self) -> &[StatusEffect] {
        &self.state
    }

    fn shield_attack(&mut self, amount: u32) -> Option<u32> {
        // shield can only reduce damage by 30%
        let max_shielded = (self.shield_health as f32 * 0.3).round() as u32;
        let shielded = if amount > max_shielded {
            max_shielded
        } else {
            amount
        };
        self.shield_health = self.shield_health.saturating_sub(shielded);
        Some(shielded)
    }

    fn shield_health(&self) -> Option<u32> {
        Some(self.shield_health)
    }
}
