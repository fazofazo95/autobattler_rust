use bevy::prelude::*;
use super::unit::UnitKind;
use super::monster::MonsterKind;

/// Persistent list of player-owned units, carries across waves.
#[derive(Resource, Default)]
pub struct PlayerParty {
    pub units: Vec<UnitKind>,
}

/// The enemy composition the player chose for the upcoming battle.
#[derive(Resource, Default)]
pub struct SelectedWave {
    pub monsters: Vec<(MonsterKind, u32)>,
}
