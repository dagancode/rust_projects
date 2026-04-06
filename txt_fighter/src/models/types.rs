//! Status effects and attack results

#[derive(Clone, Debug)]
pub struct StatusEffect {
    pub name: String,
    pub damage_per_turn: i32,
    pub(crate) turns_remaining: u32,
}

pub struct AttackResult {
    pub name: String,
    pub damage: i32,
    pub effect: Option<StatusEffect>,
}

#[derive(Debug)]
pub struct SpecialAttackInfo {
    pub name: String,
    pub damage: u32,
}

impl Default for AttackResult {
    fn default() -> Self {
        AttackResult {
            name: String::from("Basic Attack"),
            damage: 0,
            effect: None,
        }
    }
}
