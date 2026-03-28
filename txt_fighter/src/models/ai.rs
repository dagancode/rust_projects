use crate::models::fighter::Fighter;

pub trait Ai {
    /// Decides whether to use a special attack or a normal attack based on the following criteria:
    /// 1. If the special attack resource is empty, use a normal attack.
    /// 2. If the health of the Fighter is below 50% of its maximum health, use a special attack.
    fn choose_action(&self, opponent: &dyn Fighter) -> Action;
}

pub enum Action {
    NormalAttack,
    SpecialAttack,
}