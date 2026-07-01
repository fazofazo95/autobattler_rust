use bevy::prelude::*;

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnitKind {
    Warrior,
    Archer,
    Mage,
    Paladin,
    Rogue,
}

impl UnitKind {
    pub fn display_name(&self) -> &'static str {
        match self {
            UnitKind::Warrior => "Warrior",
            UnitKind::Archer  => "Archer",
            UnitKind::Mage    => "Mage",
            UnitKind::Paladin => "Paladin",
            UnitKind::Rogue   => "Rogue",
        }
    }

    pub fn shop_cost(&self) -> u32 {
        match self {
            UnitKind::Warrior => 10,
            UnitKind::Archer  => 12,
            UnitKind::Mage    => 18,
            UnitKind::Paladin => 15,
            UnitKind::Rogue   => 14,
        }
    }

    // Not in JSON yet — kept here until design is finalised
    pub fn base_defense(&self) -> i32 {
        match self {
            UnitKind::Warrior => 10,
            UnitKind::Archer  => 4,
            UnitKind::Mage    => 2,
            UnitKind::Paladin => 18,
            UnitKind::Rogue   => 5,
        }
    }


}

/// Runtime stats component. All values here are the live, modified values
/// (base from JSON + equipment bonuses). The JSON is the source of truth for
/// the base values; this struct is what the combat system reads.
#[derive(Component, Debug, Clone)]
pub struct Stats {
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub attack_speed: f32,
    pub cast_speed: f32,
    pub move_speed: f32,
    pub range: f32,
    pub life_regen: f32,
    pub mana_regen: f32,
    pub lifesteal: i32,
    pub cleave: i32,
    pub cleave_aoe: i32,
    pub multishot_atk: i32,
    pub multishot_atk_pct: i32,
    pub repeat_atk: i32,
    pub repeat_atk_pct: i32,
    /// % of incoming magical/elemental damage negated (0–100).
    pub resistance: i32,
    /// % of all incoming damage negated before application (0–100).
    pub damage_reduction: i32,
}

/// Separate mana component so units without mana pay no memory overhead.
#[derive(Component, Debug, Clone)]
pub struct ManaStats {
    pub has_mana: bool,
    pub mana: i32,
    pub max_mana: i32,
}

/// Marks an entity as a player-controlled unit.
#[derive(Component)]
pub struct PlayerUnit;

/// Timer controlling how often this entity attacks.
#[derive(Component)]
pub struct AttackTimer(pub Timer);

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, _app: &mut App) {}
}
