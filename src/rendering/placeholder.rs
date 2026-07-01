use bevy::prelude::*;
use bevy::sprite::Anchor;
use crate::game::{
    AppState,
    data_loader::{UnitsDatabase, MonstersDatabase, UnitData},
    unit::{PlayerUnit, UnitKind, Stats, ManaStats, AttackTimer},
    monster::{Monster, MonsterKind},
    party::{PlayerParty, SelectedWave},
};

pub struct PlaceholderPlugin;

impl Plugin for PlaceholderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Battle), spawn_battle_entities)
            .add_systems(
                Update,
                update_health_bars.run_if(in_state(AppState::Battle)),
            )
            .add_systems(OnExit(AppState::Battle), despawn_battle_entities);
    }
}

#[derive(Component)]
struct BattleEntity;

#[derive(Component)]
pub struct HealthBarFill {
    pub full_width: f32,
}

const UNIT_SIZE:    Vec2 = Vec2::new(40.0, 40.0);
const MONSTER_SIZE: Vec2 = Vec2::new(36.0, 36.0);
const BAR_H:        f32  = 6.0;
const BAR_Y:        f32  = 28.0;

fn build_stats(data: &UnitData, defense: i32) -> Stats {
    Stats {
        hp:              data.base_life,
        max_hp:          data.base_life,
        attack:          data.base_atk,
        defense,
        attack_speed:    data.base_atk_spd,
        cast_speed:      data.base_cast_spd,
        move_speed:      data.move_speed as f32,
        range:           data.atk_range,
        life_regen:      data.life_regen,
        mana_regen:      data.mana_regen,
        lifesteal:       data.lifesteal,
        cleave:          data.cleave,
        cleave_aoe:      data.cleave_aoe,
        multishot_atk:   data.multishot_atk,
        multishot_atk_pct: data.multishot_atk_pct,
        repeat_atk:        data.repeat_atk,
        repeat_atk_pct:    data.repeat_atk_pct,
        resistance:        data.resistance,
        damage_reduction:  data.damage_reduction,
    }
}

/// Lay out `total` entities in a compact grid anchored around `base_x`.
/// Columns grow automatically so the grid stays within the arena height.
fn grid_pos(index: usize, total: usize, base_x: f32) -> Vec2 {
    let cols: usize = if total <= 6 { 1 } else if total <= 16 { 2 } else if total <= 30 { 3 } else { 4 };
    let col  = index % cols;
    let row  = index / cols;
    let rows = (total + cols - 1) / cols;
    let x    = base_x + (col as f32 - (cols as f32 - 1.0) / 2.0) * 52.0;
    let y    = (row  as f32 - (rows  as f32 - 1.0) / 2.0) * 50.0;
    Vec2::new(x, y)
}

fn spawn_battle_entities(
    mut commands: Commands,
    party:       Res<PlayerParty>,
    wave:        Res<SelectedWave>,
    units_db:    Res<UnitsDatabase>,
    monsters_db: Res<MonstersDatabase>,
) {
    // --- Player units (left side) ---
    let unit_count = party.units.len();
    for (i, kind) in party.units.iter().enumerate() {
        let Some(data) = units_db.0.get(kind.display_name()) else {
            error!("Missing JSON data for unit '{}'", kind.display_name());
            continue;
        };
        let Vec2 { x: ux, y } = grid_pos(i, unit_count, -200.0);
        let stats = build_stats(data, kind.base_defense());
        let mana  = ManaStats { has_mana: data.has_mana, mana: data.base_mana, max_mana: data.base_mana };
        let timer = AttackTimer(Timer::from_seconds(1.0 / data.base_atk_spd, TimerMode::Repeating));
        let name  = kind.display_name();

        commands.spawn((
            Sprite { color: Color::WHITE, custom_size: Some(UNIT_SIZE), ..default() },
            Transform::from_xyz(ux, y, 2.0),
            PlayerUnit,
            kind.clone(),
            stats,
            mana,
            timer,
            BattleEntity,
        ))
        .with_children(|p| {
            spawn_label(p, name);
            spawn_health_bar(p, UNIT_SIZE.x);
        });
    }

    // --- Monsters (right side) ---
    // Flatten (kind, count) pairs into an ordered list
    let all_monsters: Vec<MonsterKind> = wave.monsters.iter()
        .flat_map(|(kind, count)| std::iter::repeat(kind.clone()).take(*count as usize))
        .collect();

    let monster_count = all_monsters.len();
    for (i, kind) in all_monsters.iter().enumerate() {
        let Some(data) = monsters_db.0.get(kind.display_name()) else {
            error!("Missing JSON data for monster '{}'", kind.display_name());
            continue;
        };
        let Vec2 { x: mx, y } = grid_pos(i, monster_count, 200.0);
        let stats = build_stats(data, kind.base_defense());
        let timer = AttackTimer(Timer::from_seconds(1.0 / data.base_atk_spd, TimerMode::Repeating));
        let name  = kind.display_name();

        commands.spawn((
            Sprite { color: Color::srgb(0.9, 0.3, 0.3), custom_size: Some(MONSTER_SIZE), ..default() },
            Transform::from_xyz(mx, y, 2.0),
            Monster,
            kind.clone(),
            stats,
            timer,
            BattleEntity,
        ))
        .with_children(|p| {
            spawn_label(p, name);
            spawn_health_bar(p, MONSTER_SIZE.x);
        });
    }
}

fn spawn_label(parent: &mut ChildBuilder, name: &str) {
    parent.spawn((
        Text2d::new(name),
        TextFont { font_size: 10.0, ..default() },
        TextColor(Color::BLACK),
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}

fn spawn_health_bar(parent: &mut ChildBuilder, bar_width: f32) {
    // Dark red background
    parent.spawn((
        Sprite {
            color: Color::srgb(0.25, 0.0, 0.0),
            custom_size: Some(Vec2::new(bar_width, BAR_H)),
            ..default()
        },
        Transform::from_xyz(0.0, BAR_Y, 1.0),
    ));

    // Green fill — anchored left so it shrinks rightward
    parent.spawn((
        Sprite {
            color: Color::srgb(0.1, 0.78, 0.1),
            custom_size: Some(Vec2::new(bar_width, BAR_H)),
            anchor: Anchor::CenterLeft,
            ..default()
        },
        Transform::from_xyz(-bar_width / 2.0, BAR_Y, 1.5),
        HealthBarFill { full_width: bar_width },
    ));
}

fn update_health_bars(
    units: Query<(&Stats, &Children), Or<(With<PlayerUnit>, With<Monster>)>>,
    mut fills: Query<(&HealthBarFill, &mut Sprite)>,
) {
    for (stats, children) in &units {
        for &child in children.iter() {
            if let Ok((fill, mut sprite)) = fills.get_mut(child) {
                let pct = (stats.hp as f32 / stats.max_hp as f32).clamp(0.0, 1.0);
                sprite.custom_size = Some(Vec2::new(fill.full_width * pct, BAR_H));
                sprite.color = if pct > 0.5 {
                    Color::srgb(0.1, 0.78, 0.1)
                } else if pct > 0.25 {
                    Color::srgb(0.85, 0.65, 0.0)
                } else {
                    Color::srgb(0.85, 0.1, 0.1)
                };
            }
        }
    }
}

fn despawn_battle_entities(
    mut commands: Commands,
    query: Query<Entity, With<BattleEntity>>,
) {
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
}
