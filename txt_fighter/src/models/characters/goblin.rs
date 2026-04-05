use crate::models::ai::{Action, Ai};
use crate::models::fighter::Fighter;
use crate::models::item::{Drop, Item};
use crate::models::types::{AttackResult, SpecialAttackInfo, StatusEffect};

pub struct Goblin {
    health: u32,
    damage: u32,
    poison_damage: u32,
    poison_turns: u32,
    poison_uses: u32,
    name: String,
    drops: Vec<Drop>,
}

impl Goblin {
    const BASE_HEALTH: u32 = 80;
    pub fn new() -> Self {
        Goblin {
            health: Self::BASE_HEALTH,
            damage: 20,
            poison_damage: 10,
            poison_turns: 3,
            poison_uses: 1,
            name: String::from("Goblin 👺"),
            drops: vec![
                Drop {
                    item: Item::Coins(10),
                    chance: 0.5,
                },
                Drop {
                    item: Item::healing_potion_1(),
                    chance: 0.75,
                },
            ],
        }
    }
}

impl Fighter for Goblin {
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
        if self.poison_uses > 0 {
            self.poison_uses -= 1;
            return Some(AttackResult {
                name: "Poison Dart".to_string(),
                damage: 0,
                effect: Some(StatusEffect {
                    name: String::from("Poison"),
                    damage_per_turn: self.poison_damage as i32,
                    turns_remaining: self.poison_turns,
                }),
            });
        }
        None
    }

    fn special_attack_info(&self) -> Option<SpecialAttackInfo> {
        Some(SpecialAttackInfo {
            name: "Poison Dart".to_string(),
            damage: self.poison_damage,
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

impl Ai for Goblin {
    /// Decides whether to use a special attack or a normal attack based on the following criteria:
    /// 1. If the special attack resource is empty, use a normal attack.
    /// 2. If the opponent is poisoned, use a normal attack.
    fn choose_action(&self, opponent: &dyn Fighter) -> Action {
        // 1: Can do special attack? else Normal Attack
        if self.poison_uses == 0 {
            return Action::NormalAttack;
        }

        // 2: Is the opponent poisoned?
        if opponent
            .list_effects()
            .iter()
            .any(|e| e.name.eq_ignore_ascii_case("poison"))
        {
            return Action::NormalAttack;
        }

        println!("~~ {} uses Poison Dart Special Attack ~~", self.name());
        Action::SpecialAttack
    }
}
