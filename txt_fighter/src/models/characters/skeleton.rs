use crate::models::fighter::Fighter;
use crate::models::{Item, item::Drop};

pub struct Skeleton {
    health: u32,
    damage: u32,
    name: String,
    drops: Vec<Drop>,
}

impl Skeleton {
    const BASE_HEALTH: u32 = 60;
    pub fn new() -> Self {
        Skeleton {
            health: Self::BASE_HEALTH,
            damage: 25,
            name: String::from("Skeleton 🩻 "),
            drops: vec![
                Drop {
                    item: Item::Coins(15),
                    chance: 0.66,
                },
                Drop {
                    item: Item::Shield(40),
                    chance: 0.50,
                },
                Drop {
                    item: Item::healing_potion_1(),
                    chance: 0.50,
                },
            ],
        }
    }
}

impl Fighter for Skeleton {
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
