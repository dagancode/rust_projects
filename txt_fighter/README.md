# txt_fighter

A text-based RPG combat simulator built in Rust.

## Overview

Two fighters battle in rounds until one is defeated. Each fighter has unique
abilities, and the outcome depends on stats, special attacks, and status effects.

## Fighters

| Fighter | Health | Damage | Special Ability |
|---------|--------|--------|-----------------|
| Hero    | 100    | 35     | Shield — absorbs 30% of incoming damage |
| Goblin  | 80     | 20     | Poison Dart — applies 10 poison damage over 3 turns |
| Dragon  | 120    | 40     | Fireball — deals 60 bonus damage once |

## Usage

```bash
# Run a battle
cargo run

# Slower output, readable at human speed
cargo run --features slow
```

To change the matchup, edit `main.rs`:

```rust
game_loop(&mut enemy_goblin, &mut my_hero);
```

## Project Structure

```
src/
├── main.rs
├── lib.rs
├── game/
│   ├── game_loop.rs
│   └── execute_turn.rs
└── models/
    ├── fighter.rs
    ├── ai.rs
    ├── types.rs
    └── characters/
        ├── hero.rs
        ├── goblin.rs
        └── dragon.rs
```

## Concepts

Built as a learning project to explore:

- Trait design and default implementations
- Dynamic dispatch with `dyn Trait`
- Status effect systems
- Modular project structure
- Feature flags

---

*Part of [rust_projects](../README.md)*