use bevy::prelude::*;

pub const ARENA_SIZE: f32 = 600.0;
pub const ARENA_HALF: f32 = ARENA_SIZE / 2.0;

/// Marker for the arena background entity.
#[derive(Component)]
pub struct ArenaBackground;

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(OnEnter(crate::game::AppState::Battle), spawn_arena)
            .add_systems(OnExit(crate::game::AppState::Battle), despawn_arena);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_arena(mut commands: Commands) {
    // Dark green grass floor
    commands.spawn((
        Sprite {
            color: Color::srgb(0.18, 0.32, 0.18),
            custom_size: Some(Vec2::splat(ARENA_SIZE)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        ArenaBackground,
    ));

    // Border lines (4 thin white rects)
    let border = 4.0;
    for (x, y, w, h) in [
        (0.0,  ARENA_HALF,  ARENA_SIZE + border * 2.0, border),
        (0.0, -ARENA_HALF,  ARENA_SIZE + border * 2.0, border),
        ( ARENA_HALF, 0.0, border, ARENA_SIZE),
        (-ARENA_HALF, 0.0, border, ARENA_SIZE),
    ] {
        commands.spawn((
            Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(w, h)),
                ..default()
            },
            Transform::from_xyz(x, y, 1.0),
            ArenaBackground,
        ));
    }
}

fn despawn_arena(mut commands: Commands, query: Query<Entity, With<ArenaBackground>>) {
    for e in &query {
        commands.entity(e).despawn();
    }
}
