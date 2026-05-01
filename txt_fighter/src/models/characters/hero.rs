use crate::models::Item;
use crate::models::fighter::Fighter;
use crate::models::types::StatusEffect;

pub struct Hero {
    health: u32,
    damage: u32,
    shield_health: u32,
    armor: u32,
    coins: u32,
    name: String,
    weapon: Item,
    state: Vec<StatusEffect>,
}

impl Hero {
    const BASE_HEALTH: u32 = 100;
    const BASE_DAMAGE: u32 = 35;
    pub fn new() -> Self {
        Hero {
            health: Self::BASE_HEALTH,
            damage: Self::BASE_DAMAGE,
            shield_health: 75,
            armor: 10,
            coins: 0,
            name: String::from("Hero 🤴"),
            weapon: Item::Weapon(String::from("Basic Sword"), Self::BASE_DAMAGE),
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
            if self.armor > 0 {
                let remaining_armor = self.armor as i32 - amount;

                if remaining_armor < 0 {
                    self.armor = 0;
                    self.health = self.health.saturating_sub(remaining_armor.unsigned_abs());
                } else {
                    self.armor = remaining_armor as u32;
                }
            } else {
                self.health = self.health.saturating_sub(amount as u32);
            }
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
                if e.damage_per_turn > 0 {
                    println!(
                        "> {} suffers {} {} damage <",
                        fighter_name, e.damage_per_turn, e.name
                    );
                } else {
                    println!(
                        "> {} gained {} HP using {} <",
                        fighter_name,
                        (e.damage_per_turn).unsigned_abs(),
                        e.name
                    );
                }
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
        let max_shielded = (self.shield_health as f32 * 0.5).round() as u32;
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

    fn armor_health(&self) -> Option<u32> {
        Some(self.armor)
    }

    fn consume_items(&mut self, items: Vec<Item>) {
        if items.is_empty() {
            return;
        }

        for item in items {
            println!("~ Picked up: {} ~", item.name());

            match item {
                Item::Coins(amount) => self.coins += amount,
                Item::Potion(effect) => self.add_effect(effect),
                Item::Armor(amount) => self.armor += amount,
                Item::Shield(amount) => self.shield_health += amount,
                Item::Weapon(name, damage) => {
                    if damage > self.damage {
                        self.damage = damage;
                        self.weapon = Item::Weapon(name, damage);
                    }
                },
            }
        }
    }
}
