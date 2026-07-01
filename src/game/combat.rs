use bevy::prelude::*;
use super::{unit::{PlayerUnit, Stats, AttackTimer}, AppState, WaveNumber, Gold};
use super::monster::Monster;
use super::data_loader::RoundsDatabase;
use super::shop::ShopState;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                move_toward_target,
                attack_nearest,
                move_monsters_toward_target,
                monsters_attack_nearest,
            )
                .run_if(in_state(AppState::Battle)),
        )
        .add_systems(
            PostUpdate,
            despawn_dead.run_if(in_state(AppState::Battle)),
        )
        .add_systems(
            Update,
            check_battle_end.run_if(in_state(AppState::Battle)),
        );
    }
}

fn move_toward_target(
    time: Res<Time>,
    mut units: Query<(&mut Transform, &Stats), (With<PlayerUnit>, Without<Monster>)>,
    monsters: Query<&Transform, (With<Monster>, Without<PlayerUnit>)>,
) {
    for (mut unit_tf, stats) in &mut units {
        let Some(target_pos) = nearest_pos(unit_tf.translation.truncate(), &monsters) else {
            continue;
        };
        let dist = unit_tf.translation.truncate().distance(target_pos);
        if dist > stats.range {
            let dir = (target_pos - unit_tf.translation.truncate()).normalize_or_zero();
            unit_tf.translation += (dir * stats.move_speed * time.delta_secs()).extend(0.0);
        }
    }
}

fn attack_nearest(
    time: Res<Time>,
    mut units: Query<(&Transform, &Stats, &mut AttackTimer), (With<PlayerUnit>, Without<Monster>)>,
    monster_pos: Query<(Entity, &Transform), With<Monster>>,
    mut monster_stats: Query<&mut Stats, (With<Monster>, Without<PlayerUnit>)>,
) {
    for (unit_tf, unit_stats, mut timer) in &mut units {
        timer.0.tick(time.delta());
        if !timer.0.just_finished() {
            continue;
        }
        let unit_pos = unit_tf.translation.truncate();
        let target = monster_pos.iter()
            .filter(|(_, tf)| unit_pos.distance(tf.translation.truncate()) <= unit_stats.range)
            .min_by(|(_, a), (_, b)| {
                let da = unit_pos.distance(a.translation.truncate());
                let db = unit_pos.distance(b.translation.truncate());
                da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(e, _)| e);

        if let Some(entity) = target {
            if let Ok(mut target_stats) = monster_stats.get_mut(entity) {
                let dmg = (unit_stats.attack - target_stats.defense).max(1);
                target_stats.hp -= dmg;
            }
        }
    }
}

fn despawn_dead(
    mut commands: Commands,
    dead: Query<(Entity, &Stats), Or<(With<PlayerUnit>, With<Monster>)>>,
) {
    for (entity, stats) in &dead {
        if stats.hp <= 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn nearest_pos(from: Vec2, query: &Query<&Transform, (With<Monster>, Without<PlayerUnit>)>) -> Option<Vec2> {
    query.iter()
        .map(|tf| tf.translation.truncate())
        .min_by(|a, b| {
            from.distance(*a).partial_cmp(&from.distance(*b)).unwrap_or(std::cmp::Ordering::Equal)
        })
}

fn move_monsters_toward_target(
    time: Res<Time>,
    mut monsters: Query<(&mut Transform, &Stats), (With<Monster>, Without<PlayerUnit>)>,
    units: Query<&Transform, (With<PlayerUnit>, Without<Monster>)>,
) {
    for (mut monster_tf, stats) in &mut monsters {
        let monster_pos = monster_tf.translation.truncate();
        let Some(target_pos) = nearest_unit_pos(monster_pos, &units) else { continue };
        let dist = monster_pos.distance(target_pos);
        if dist > stats.range {
            let dir = (target_pos - monster_pos).normalize_or_zero();
            monster_tf.translation += (dir * stats.move_speed * time.delta_secs()).extend(0.0);
        }
    }
}

fn monsters_attack_nearest(
    time: Res<Time>,
    mut monsters: Query<(&Transform, &Stats, &mut AttackTimer), (With<Monster>, Without<PlayerUnit>)>,
    unit_pos: Query<(Entity, &Transform), (With<PlayerUnit>, Without<Monster>)>,
    mut unit_stats: Query<&mut Stats, (With<PlayerUnit>, Without<Monster>)>,
) {
    for (monster_tf, monster_stats, mut timer) in &mut monsters {
        timer.0.tick(time.delta());
        if !timer.0.just_finished() {
            continue;
        }
        let monster_pos = monster_tf.translation.truncate();
        let target = unit_pos.iter()
            .filter(|(_, tf)| monster_pos.distance(tf.translation.truncate()) <= monster_stats.range)
            .min_by(|(_, a), (_, b)| {
                let da = monster_pos.distance(a.translation.truncate());
                let db = monster_pos.distance(b.translation.truncate());
                da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(e, _)| e);

        if let Some(entity) = target {
            if let Ok(mut target_stats) = unit_stats.get_mut(entity) {
                let dmg = (monster_stats.attack - target_stats.defense).max(1);
                target_stats.hp -= dmg;
            }
        }
    }
}

fn nearest_unit_pos(from: Vec2, query: &Query<&Transform, (With<PlayerUnit>, Without<Monster>)>) -> Option<Vec2> {
    query.iter()
        .map(|tf| tf.translation.truncate())
        .min_by(|a, b| {
            from.distance(*a).partial_cmp(&from.distance(*b)).unwrap_or(std::cmp::Ordering::Equal)
        })
}

/// Waits until both sides have at least one entity, then watches for either
/// side being wiped out. Uses Local<bool> so it resets cleanly each time
/// the Battle state is re-entered.
fn check_battle_end(
    mut next_state: ResMut<NextState<AppState>>,
    mut wave: ResMut<WaveNumber>,
    mut gold: ResMut<Gold>,
    mut shop: ResMut<ShopState>,
    rounds_db: Res<RoundsDatabase>,
    units: Query<Entity, With<PlayerUnit>>,
    monsters: Query<Entity, With<Monster>>,
    mut engaged: Local<bool>,
) {
    if !*engaged {
        if !units.is_empty() && !monsters.is_empty() {
            *engaged = true;
        }
        return;
    }

    if monsters.is_empty() {
        if let Some(round) = rounds_db.0.get(&wave.0.clamp(1, 100)) {
            gold.0 += round.gold_reward;
            info!("Wave {} won — awarded {} gold (total: {})", wave.0, round.gold_reward, gold.0);
        }
        shop.reset_after_battle();
        wave.0 += 1;
        *engaged = false;
        next_state.set(AppState::Preparation);
    } else if units.is_empty() {
        *engaged = false;
        next_state.set(AppState::MainMenu);
    }
}
