use crate::models::ai::Action;
use crate::models::item::Item;
use crate::models::types::{AttackResult, SpecialAttackInfo, StatusEffect};

pub trait Fighter {
    /// Returns the attack power of the fighter
    fn attack(&self) -> u32;

    /// Returns the health of the fighter
    fn health(&self) -> u32;

    /// Applies damage to the fighter
    fn take_damage(&mut self, amount: i32);

    /// Returns the name of the fighter
    fn name(&self) -> &str;

    /// Applies effects to the fighter
    fn apply_effects(&mut self) {}

    /// Adds an effect to the fighter
    fn add_effect(&mut self, _effect: StatusEffect) {}

    /// Clears stale effects
    fn clear_stale_effects(&mut self) {}

    /// Consumes all items held by the fighter
    fn consume_items(&mut self, _items: Vec<Item>) {}

    /// Checks if the fighter is alive
    fn is_alive(&self) -> bool {
        self.health() > 0
    }

    /// Returns the result of a special attack
    fn special_attack(&mut self) -> Option<AttackResult> {
        None
    }

    /// Returns the result of a shield attack
    fn shield_attack(&mut self, _amount: u32) -> Option<u32> {
        None
    }

    /// Returns the health of the shield
    fn shield_health(&self) -> Option<u32> {
        None
    }

    /// Returns the armor health of the fighter
    fn armor_health(&self) -> Option<u32> {
        None
    }

    /// Returns the info of a special attack
    fn special_attack_info(&self) -> Option<SpecialAttackInfo> {
        None
    }

    /// Returns the applied status effects of the fighter
    fn list_effects(&self) -> &[StatusEffect] {
        &[]
    }

    /// Decides whether to use a special attack or a normal attack
    fn decide(&mut self, _opponent: &dyn Fighter) -> Action {
        Action::NormalAttack
    }
    
    /// Drops all items held by the fighter and returns them in a vector.
    fn drop_items(&mut self) -> Vec<Item> {
        vec![]
    }

    /// Refreshes the state of the fighter
    fn refresh_state(&mut self) {
        self.apply_effects();
        self.clear_stale_effects();
    }

    /// Displays the state of the fighter
    fn display(&self) {
        let shield_text = if let Some(shield_health) = self.shield_health() {
            format!(", Shield: {}", shield_health)
        } else {
            "".to_string()
        };

        let armor_text = if let Some(armor_health) = self.armor_health() {
            format!(", Armor: {}", armor_health)
        } else {
            "".to_string()
        };

        #[cfg(feature = "slow")]
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("{} - Health: {}{}{}", self.name(), self.health(), armor_text, shield_text,);
    }
}
