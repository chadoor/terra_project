use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Ability {
    ANTI(String, u8),
    ASSAULT,
    HEAVY,
    PISTOL,
    NONE,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Weapon {
    name: String,
    n_attacks: i32,
    b_skill: i32,
    strength: i32,
    ap: i32,
    damage: i32,
    damage_die: i32,
    abilities: Vec<Ability>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Model {
    name: String,
    toughness: i32,
    a_save: i32,
    i_save: i32,
    wounds: i32,
    weapons: Vec<Weapon>,
    keywords: Vec<String>,
}
