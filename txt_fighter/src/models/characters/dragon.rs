use crate::models::fighter::Fighter;
use crate::models::types::{AttackResult, SpecialAttackInfo};

pub struct Dragon {
    health: u32,
    damage: u32,
    fireball_damage: u32,
    uses: u32,
    name: String,
}

impl Dragon {
    const BASE_HEALTH: u32 = 120;
    pub fn new() -> Self {
        Dragon {
            health: Self::BASE_HEALTH,
            damage: 40,
            fireball_damage: 60,
            uses: 1,
            name: String::from("Dragon"),
        }
    }
}

impl Fighter for Dragon {
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

    fn special_attack(&mut self) -> Option<AttackResult> {
        if self.uses > 0 {
            self.uses -= 1;
            return Some(AttackResult {
                name: "Fireball".to_string(),
                damage: self.fireball_damage as i32,
                effect: None,
            });
        }
        None
    }

    fn special_attack_info(&self) -> Option<SpecialAttackInfo> {
        Some(SpecialAttackInfo {
            name: "Fireball".to_string(),
            damage: self.fireball_damage,
        })
    }
}
