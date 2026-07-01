use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Clone, Debug)]
pub struct UnitData {
    pub name: String,
    pub type1: String,
    pub description: String,
    pub base_life: i32,
    pub base_atk: i32,
    pub has_mana: bool,
    pub base_mana: i32,
    pub atk_range: f32,
    pub atk_range_type: String,
    pub lifesteal: i32,
    pub multishot_atk: i32,
    pub multishot_atk_pct: i32,
    pub repeat_atk: i32,
    pub repeat_atk_pct: i32,
    pub base_atk_spd: f32,
    pub base_cast_spd: f32,
    pub skill_list: Vec<String>,
    pub cleave: i32,
    pub cleave_aoe: i32,
    pub life_regen: f32,
    pub mana_regen: f32,
    pub move_speed: i32,
    pub resistance: i32,
    pub damage_reduction: i32,
}

/// A single skill definition loaded from skills.json.
#[derive(Deserialize, Clone, Debug)]
pub struct SkillData {
    pub name: String,
    pub description: String,
    /// "Spell" | "Aura" | "Passive" | "Active"
    pub skill_type: String,
    pub mana_cost: i32,
    pub cooldown: f32,
    pub damage: i32,
    /// "Fire" | "Ice" | "Lightning" | "Holy" | "Shadow" | "Poison" | "Physical" | "Arcane" | "None"
    pub damage_type: String,
    /// How far the caster can target (0.0 for self-cast, auras, passives).
    pub range: f32,
    /// Radius of the area of effect (0.0 for single-target).
    pub aoe_radius: f32,
    /// Where the AOE originates: "Caster" | "Target" | "None"
    pub aoe_origin: String,
    /// Free-text description of non-damage effects (empty string if none).
    pub effect: String,
}

/// One enemy entry inside a round option (name must match monsters.json).
#[derive(Deserialize, Clone, Debug)]
pub struct MonsterEntry {
    pub name: String,
    pub count: u32,
}

/// One of the three choices shown during wave selection.
#[derive(Deserialize, Clone, Debug)]
pub struct RoundOption {
    pub label: String,
    pub monsters: Vec<MonsterEntry>,
}

/// All data for one round (three options).
#[derive(Deserialize, Clone, Debug)]
pub struct RoundData {
    pub round: u32,
    pub gold_reward: u32,
    pub options: Vec<RoundOption>,
}

#[derive(Deserialize)]
struct UnitsFile    { units: Vec<UnitData> }
#[derive(Deserialize)]
struct MonstersFile { monsters: Vec<UnitData> }
#[derive(Deserialize)]
struct RoundsFile   { rounds: Vec<RoundData> }
#[derive(Deserialize)]
struct SkillsFile   { skills: Vec<SkillData> }

#[derive(Resource, Default)]
pub struct UnitsDatabase(pub HashMap<String, UnitData>);

#[derive(Resource, Default)]
pub struct MonstersDatabase(pub HashMap<String, UnitData>);

/// Keyed by round number (1-100).
#[derive(Resource, Default)]
pub struct RoundsDatabase(pub HashMap<u32, RoundData>);

/// Keyed by skill name (e.g. "Fireball").
#[derive(Resource, Default)]
pub struct SkillsDatabase(pub HashMap<String, SkillData>);

pub struct DataLoaderPlugin;

impl Plugin for DataLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UnitsDatabase>()
            .init_resource::<MonstersDatabase>()
            .init_resource::<RoundsDatabase>()
            .init_resource::<SkillsDatabase>()
            .add_systems(Startup, load_data);
    }
}

fn load_data(
    mut units_db:    ResMut<UnitsDatabase>,
    mut monsters_db: ResMut<MonstersDatabase>,
    mut rounds_db:   ResMut<RoundsDatabase>,
    mut skills_db:   ResMut<SkillsDatabase>,
) {
    load_file("assets/data/units.json", |text| {
        let file: UnitsFile = serde_json::from_str(text)?;
        for u in file.units { units_db.0.insert(u.name.clone(), u); }
        info!("Loaded {} unit definitions", units_db.0.len());
        Ok(())
    });

    load_file("assets/data/monsters.json", |text| {
        let file: MonstersFile = serde_json::from_str(text)?;
        for m in file.monsters { monsters_db.0.insert(m.name.clone(), m); }
        info!("Loaded {} monster definitions", monsters_db.0.len());
        Ok(())
    });

    load_file("assets/data/rounds.json", |text| {
        let file: RoundsFile = serde_json::from_str(text)?;
        for r in file.rounds { rounds_db.0.insert(r.round, r); }
        info!("Loaded {} round definitions", rounds_db.0.len());
        Ok(())
    });

    load_file("assets/data/skills.json", |text| {
        let file: SkillsFile = serde_json::from_str(text)?;
        for s in file.skills { skills_db.0.insert(s.name.clone(), s); }
        info!("Loaded {} skill definitions", skills_db.0.len());
        Ok(())
    });
}

fn load_file<F>(path: &str, mut handler: F)
where
    F: FnMut(&str) -> Result<(), serde_json::Error>,
{
    match std::fs::read_to_string(path) {
        Ok(text) => {
            if let Err(e) = handler(&text) {
                error!("Failed to parse {path}: {e}");
            }
        }
        Err(e) => error!("Failed to read {path}: {e}"),
    }
}
