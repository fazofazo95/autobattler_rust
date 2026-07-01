use bevy::prelude::*;
use crate::game::AppState;

#[derive(Component)]
struct MainMenuRoot;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_menu)
            .add_systems(Update, handle_buttons.run_if(in_state(AppState::MainMenu)))
            .add_systems(OnExit(AppState::MainMenu), despawn_menu);
    }
}

#[derive(Component)]
enum MenuButton {
    Start,
    Quit,
}

fn spawn_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
            MainMenuRoot,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("AUTOBATTLER"),
                TextFont {
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            parent.spawn((
                Text::new("A top-down autobattler"),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));

            spawn_button(parent, "Start Game", MenuButton::Start);
            spawn_button(parent, "Quit",       MenuButton::Quit);
        });
}

fn spawn_button(parent: &mut ChildBuilder, label: &str, tag: MenuButton) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(220.0),
                height: Val::Px(55.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.25)),
            tag,
        ))
        .with_children(|b| {
            b.spawn((
                Text::new(label),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });
}

fn handle_buttons(
    mut next_state: ResMut<NextState<AppState>>,
    mut exit: EventWriter<AppExit>,
    interaction: Query<(&Interaction, &MenuButton), Changed<Interaction>>,
) {
    for (interaction, button) in &interaction {
        if *interaction != Interaction::Pressed {
            continue;
        }
        match button {
            MenuButton::Start => next_state.set(AppState::Preparation),
            MenuButton::Quit  => { exit.send(AppExit::Success); }
        }
    }
}

fn despawn_menu(mut commands: Commands, query: Query<Entity, With<MainMenuRoot>>) {
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
}
