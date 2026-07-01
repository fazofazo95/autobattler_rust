use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EquipmentKind {
    // Weapons
    IronSword,
    WoodBow,
    MagicStaff,
    // Armor
    LeatherArmor,
    ChainMail,
    // Accessories
    SpeedRing,
    HealthAmulet,
}

#[derive(Debug, Clone, Default)]
pub struct StatModifiers {
    pub hp_bonus: i32,
    pub attack_bonus: i32,
    pub defense_bonus: i32,
    pub attack_speed_bonus: f32,
    pub move_speed_bonus: f32,
    pub range_bonus: f32,
}

impl EquipmentKind {
    pub fn display_name(&self) -> &'static str {
        match self {
            EquipmentKind::IronSword    => "Iron Sword",
            EquipmentKind::WoodBow      => "Wood Bow",
            EquipmentKind::MagicStaff   => "Magic Staff",
            EquipmentKind::LeatherArmor => "Leather Armor",
            EquipmentKind::ChainMail    => "Chain Mail",
            EquipmentKind::SpeedRing    => "Speed Ring",
            EquipmentKind::HealthAmulet => "Health Amulet",
        }
    }

    pub fn modifiers(&self) -> StatModifiers {
        match self {
            EquipmentKind::IronSword    => StatModifiers { attack_bonus: 10, ..default() },
            EquipmentKind::WoodBow      => StatModifiers { attack_bonus: 6, range_bonus: 40.0, ..default() },
            EquipmentKind::MagicStaff   => StatModifiers { attack_bonus: 15, range_bonus: 30.0, ..default() },
            EquipmentKind::LeatherArmor => StatModifiers { defense_bonus: 6, hp_bonus: 30, ..default() },
            EquipmentKind::ChainMail    => StatModifiers { defense_bonus: 14, hp_bonus: 20, ..default() },
            EquipmentKind::SpeedRing    => StatModifiers { attack_speed_bonus: 0.3, move_speed_bonus: 20.0, ..default() },
            EquipmentKind::HealthAmulet => StatModifiers { hp_bonus: 80, ..default() },
        }
    }

    pub fn cost(&self) -> u32 {
        match self {
            EquipmentKind::IronSword    => 8,
            EquipmentKind::WoodBow      => 10,
            EquipmentKind::MagicStaff   => 16,
            EquipmentKind::LeatherArmor => 7,
            EquipmentKind::ChainMail    => 12,
            EquipmentKind::SpeedRing    => 9,
            EquipmentKind::HealthAmulet => 11,
        }
    }
}

/// Equipment slots on a unit. None means the slot is empty.
#[derive(Component, Default)]
pub struct EquipmentSlots {
    pub weapon: Option<EquipmentKind>,
    pub armor: Option<EquipmentKind>,
    pub accessory: Option<EquipmentKind>,
}
