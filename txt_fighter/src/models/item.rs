use crate::models::types::StatusEffect;

#[derive(Clone, Debug)]
pub enum Item {
    Potion(StatusEffect),
    Coins(u32),
    Armor(u32),
    Shield(u32),
    // Weapon(name: String, damage: u32)
    Weapon(String, u32),
}

pub struct Drop {
    pub item: Item,
    pub chance: f32,
}

impl Item {
    pub fn name(&self) -> String {
        match self {
            Item::Potion(effect) => effect.name.clone(),
            Item::Coins(amount) => format!("{} coins 🪙 ", amount),
            Item::Armor(amount) => format!("{} armor 🪖 ", amount),
            Item::Shield(amount) => format!("{}HP shield 🛡️ ", amount),
            Item::Weapon(name, _amount) => format!("{} 🗡️ ", name),
        }
    }

    pub fn healing_potion_1() -> Item {
        Item::Potion(StatusEffect {
            name: String::from("Healing Potion I 🧉"),
            damage_per_turn: -25,
            turns_remaining: 1,
        })
    }

    pub fn healing_potion_2() -> Item {
        Item::Potion(StatusEffect {
            name: String::from("Healing Potion II 🧉"),
            damage_per_turn: -45,
            turns_remaining: 1,
        })
    }
}
