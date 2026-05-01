use crate::models::ai::{Action, Ai};
use crate::models::fighter::Fighter;
use crate::models::types::{AttackResult, SpecialAttackInfo};
use crate::models::{Item, item::Drop};

pub struct Troll {
    health: u32,
    damage: u32,
    name: String,
    throw_damage: u32,
    uses: u32,
    drops: Vec<Drop>,
}

impl Troll {
    const BASE_HEALTH: u32 = 90;
    pub fn new() -> Self {
        Troll {
            health: Self::BASE_HEALTH,
            damage: 20,
            name: String::from("Troll 🧌 "),
            throw_damage: 40,
            uses: 1,
            drops: vec![
                Drop {
                    item: Item::Coins(20),
                    chance: 1.0,
                },
                Drop {
                    item: Item::Coins(50),
                    chance: 0.25,
                },
                Drop {
                    item: Item::Shield(40),
                    chance: 0.50,
                },
                Drop {
                    item: Item::healing_potion_1(),
                    chance: 0.75,
                },
                Drop {
                    item: Item::healing_potion_2(),
                    chance: 0.25,
                },
            ],
        }
    }
}

impl Fighter for Troll {
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
                name: "Flying Boulder 🪨 ".to_string(),
                damage: self.throw_damage as i32,
                effect: None,
            });
        }
        None
    }

    fn special_attack_info(&self) -> Option<SpecialAttackInfo> {
        Some(SpecialAttackInfo {
            name: "Flying Boulder 🪨 ".to_string(),
            damage: self.throw_damage,
        })
    }

    fn decide(&mut self, opponent: &dyn Fighter) -> Action {
        self.choose_action(opponent)
    }

    fn drop_items(&mut self) -> Vec<Item> {
        let items = self
            .drops
            .iter()
            .filter(|d| {
                let roll = rand::random_range(0.0..=1.0);

                roll < d.chance
            })
            .map(|drop| drop.item.clone())
            .collect();

        self.drops.clear();
        items
    }
}

impl Ai for Troll {
    /// Decides whether to use a special attack or a normal attack based on the following criteria:
    /// 1. If the special attack resource is empty, use a normal attack.
    fn choose_action(&self, _opponent: &dyn Fighter) -> Action {
        // 1: Can do special attack? else Normal Attack
        if self.uses == 0 {
            return Action::NormalAttack;
        }

        Action::SpecialAttack
    }
}
