use bevy::prelude::*;
use crate::game::{
    AppState, Gold,
    party::PlayerParty,
    shop::{ShopState, try_buy, try_refresh, UNIT_COST},
};

#[derive(Component)]
struct PreparationRoot;

#[derive(Component)]
struct ContinueButton;

#[derive(Component)]
struct ShopBuyButton(usize);

#[derive(Component)]
struct RefreshButton;

pub struct PreparationPlugin;

impl Plugin for PreparationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
                OnEnter(AppState::Preparation),
                (populate_shop, spawn_ui).chain(),
            )
            .add_systems(
                Update,
                (handle_buy, handle_refresh, handle_continue, maybe_rebuild)
                    .run_if(in_state(AppState::Preparation)),
            )
            .add_systems(OnExit(AppState::Preparation), despawn_ui);
    }
}

fn populate_shop(mut shop: ResMut<ShopState>) {
    if shop.slots.is_empty() {
        shop.populate();
    }
}

fn spawn_ui(
    mut commands: Commands,
    party: Res<PlayerParty>,
    shop: Res<ShopState>,
    gold: Res<Gold>,
) {
    build_ui(&mut commands, &party, &shop, &gold);
}

fn build_ui(commands: &mut Commands, party: &PlayerParty, shop: &ShopState, gold: &Gold) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(32.0)),
                row_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.05, 0.1, 0.97)),
            PreparationRoot,
        ))
        .with_children(|root| {
            // ── Top: title + gold + party cards ──────────────────────────────
            root.spawn(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(14.0),
                ..default()
            })
            .with_children(|top| {
                top.spawn((
                    Text::new("Preparation"),
                    TextFont { font_size: 42.0, ..default() },
                    TextColor(Color::WHITE),
                ));
                top.spawn((
                    Text::new(format!("Gold: {}", gold.0)),
                    TextFont { font_size: 20.0, ..default() },
                    TextColor(Color::srgb(1.0, 0.85, 0.2)),
                ));

                if party.units.is_empty() {
                    top.spawn((
                        Text::new("No units yet — buy one from the shop below."),
                        TextFont { font_size: 16.0, ..default() },
                        TextColor(Color::srgb(0.5, 0.5, 0.5)),
                    ));
                } else {
                    top.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        flex_wrap: FlexWrap::Wrap,
                        column_gap: Val::Px(12.0),
                        row_gap: Val::Px(12.0),
                        justify_content: JustifyContent::Center,
                        ..default()
                    })
                    .with_children(|row| {
                        for kind in &party.units {
                            row.spawn((
                                Node {
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    padding: UiRect::all(Val::Px(10.0)),
                                    row_gap: Val::Px(4.0),
                                    border: UiRect::all(Val::Px(2.0)),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.12, 0.12, 0.22)),
                                BorderColor(Color::srgb(0.35, 0.35, 0.55)),
                            ))
                            .with_children(|card| {
                                card.spawn((
                                    Node { width: Val::Px(38.0), height: Val::Px(38.0), ..default() },
                                    BackgroundColor(Color::WHITE),
                                ));
                                card.spawn((
                                    Text::new(kind.display_name()),
                                    TextFont { font_size: 13.0, ..default() },
                                    TextColor(Color::WHITE),
                                ));
                            });
                        }
                    });
                }
            });

            // ── Middle: shop row ──────────────────────────────────────────────
            root.spawn(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(8.0),
                ..default()
            })
            .with_children(|section| {
                section.spawn((
                    Text::new("Shop"),
                    TextFont { font_size: 18.0, ..default() },
                    TextColor(Color::srgb(0.7, 0.7, 0.7)),
                ));

                section.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(14.0),
                    align_items: AlignItems::Stretch,
                    ..default()
                })
                .with_children(|row| {
                    // Refresh button (left)
                    let can_refresh = gold.0 >= shop.refresh_cost;
                    row.spawn((
                        Button,
                        Node {
                            width: Val::Px(110.0),
                            min_height: Val::Px(110.0),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            row_gap: Val::Px(6.0),
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        BackgroundColor(if can_refresh {
                            Color::srgb(0.08, 0.22, 0.08)
                        } else {
                            Color::srgb(0.12, 0.12, 0.12)
                        }),
                        BorderColor(if can_refresh {
                            Color::srgb(0.25, 0.65, 0.25)
                        } else {
                            Color::srgb(0.28, 0.28, 0.28)
                        }),
                        RefreshButton,
                    ))
                    .with_children(|b| {
                        b.spawn((
                            Text::new("↻ Refresh"),
                            TextFont { font_size: 14.0, ..default() },
                            TextColor(if can_refresh {
                                Color::WHITE
                            } else {
                                Color::srgb(0.38, 0.38, 0.38)
                            }),
                        ));
                        b.spawn((
                            Text::new(format!("{} Gold", shop.refresh_cost)),
                            TextFont { font_size: 13.0, ..default() },
                            TextColor(if can_refresh {
                                Color::srgb(1.0, 0.85, 0.2)
                            } else {
                                Color::srgb(0.38, 0.38, 0.38)
                            }),
                        ));
                    });

                    // 3 unit slots
                    for (i, slot) in shop.slots.iter().enumerate() {
                        if slot.bought {
                            // Bought — non-interactive greyed card
                            row.spawn((
                                Node {
                                    width: Val::Px(150.0),
                                    min_height: Val::Px(110.0),
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    row_gap: Val::Px(6.0),
                                    border: UiRect::all(Val::Px(2.0)),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.07, 0.07, 0.07)),
                                BorderColor(Color::srgb(0.18, 0.18, 0.18)),
                            ))
                            .with_children(|card| {
                                card.spawn((
                                    Text::new(slot.kind.display_name()),
                                    TextFont { font_size: 15.0, ..default() },
                                    TextColor(Color::srgb(0.32, 0.32, 0.32)),
                                ));
                                card.spawn((
                                    Text::new("✓ Bought"),
                                    TextFont { font_size: 13.0, ..default() },
                                    TextColor(Color::srgb(0.28, 0.55, 0.28)),
                                ));
                            });
                        } else {
                            // Available — clickable buy card
                            let can_buy = gold.0 >= UNIT_COST;
                            row.spawn((
                                Button,
                                Node {
                                    width: Val::Px(150.0),
                                    min_height: Val::Px(110.0),
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    row_gap: Val::Px(6.0),
                                    border: UiRect::all(Val::Px(2.0)),
                                    ..default()
                                },
                                BackgroundColor(if can_buy {
                                    Color::srgb(0.1, 0.1, 0.22)
                                } else {
                                    Color::srgb(0.1, 0.1, 0.12)
                                }),
                                BorderColor(if can_buy {
                                    Color::srgb(0.35, 0.35, 0.65)
                                } else {
                                    Color::srgb(0.25, 0.25, 0.25)
                                }),
                                ShopBuyButton(i),
                            ))
                            .with_children(|card| {
                                card.spawn((
                                    Node { width: Val::Px(36.0), height: Val::Px(36.0), ..default() },
                                    BackgroundColor(if can_buy {
                                        Color::WHITE
                                    } else {
                                        Color::srgb(0.38, 0.38, 0.38)
                                    }),
                                ));
                                card.spawn((
                                    Text::new(slot.kind.display_name()),
                                    TextFont { font_size: 15.0, ..default() },
                                    TextColor(if can_buy {
                                        Color::WHITE
                                    } else {
                                        Color::srgb(0.38, 0.38, 0.38)
                                    }),
                                ));
                                card.spawn((
                                    Text::new(format!("{} Gold", UNIT_COST)),
                                    TextFont { font_size: 13.0, ..default() },
                                    TextColor(if can_buy {
                                        Color::srgb(1.0, 0.85, 0.2)
                                    } else {
                                        Color::srgb(0.38, 0.38, 0.38)
                                    }),
                                ));
                            });
                        }
                    }
                });
            });

            // ── Bottom: Continue button ───────────────────────────────────────
            root.spawn(Node {
                width: Val::Percent(100.0),
                justify_content: JustifyContent::FlexEnd,
                ..default()
            })
            .with_children(|bottom| {
                bottom
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(200.0),
                            height: Val::Px(55.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.1, 0.4, 0.15)),
                        ContinueButton,
                    ))
                    .with_children(|b| {
                        b.spawn((
                            Text::new("Continue  →"),
                            TextFont { font_size: 22.0, ..default() },
                            TextColor(Color::WHITE),
                        ));
                    });
            });
        });
}

fn handle_buy(
    mut party: ResMut<PlayerParty>,
    mut gold: ResMut<Gold>,
    mut shop: ResMut<ShopState>,
    interactions: Query<(&Interaction, &ShopBuyButton), Changed<Interaction>>,
) {
    for (interaction, button) in &interactions {
        if *interaction != Interaction::Pressed { continue; }
        if try_buy(&mut shop, &mut gold, &mut party, button.0) {
            shop.dirty = true;
        }
    }
}

fn handle_refresh(
    mut gold: ResMut<Gold>,
    mut shop: ResMut<ShopState>,
    interactions: Query<&Interaction, (Changed<Interaction>, With<RefreshButton>)>,
) {
    for interaction in &interactions {
        if *interaction != Interaction::Pressed { continue; }
        if try_refresh(&mut shop, &mut gold) {
            shop.dirty = true;
        }
    }
}

fn handle_continue(
    mut next_state: ResMut<NextState<AppState>>,
    interactions: Query<&Interaction, (Changed<Interaction>, With<ContinueButton>)>,
) {
    for interaction in &interactions {
        if *interaction == Interaction::Pressed {
            next_state.set(AppState::WaveSelection);
        }
    }
}

/// Rebuild the entire preparation UI when a buy or refresh modifies the shop state.
fn maybe_rebuild(
    mut commands: Commands,
    mut shop: ResMut<ShopState>,
    party: Res<PlayerParty>,
    gold: Res<Gold>,
    root: Query<Entity, With<PreparationRoot>>,
) {
    if !shop.dirty { return; }
    shop.dirty = false;
    for e in &root {
        commands.entity(e).despawn_recursive();
    }
    build_ui(&mut commands, &party, &*shop, &gold);
}

fn despawn_ui(mut commands: Commands, query: Query<Entity, With<PreparationRoot>>) {
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
}
