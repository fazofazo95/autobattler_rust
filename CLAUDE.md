# Autobattler — CLAUDE.md

## What This Is

A top-down autobattler game built in Rust using the Bevy game engine. The player buys units and equipment from a shop between waves; units then fight autonomously against monster waves on a square arena. Intended as a shippable product, not a prototype.

## Tech Stack

| Layer | Choice | Reason |
|-------|--------|--------|
| Language | Rust | Performance, safety |
| Engine | Bevy 0.15 | ECS, GPU via wgpu, scales to 1000+ entities |
| Rendering | Bevy 2D (sprites/meshes) | Top-down; GPU-accelerated |
| UI | Bevy UI (bevy_ui) | In-engine shop/menu panels |

## Build & Run

> **IMPORTANT:** All `cargo` commands must be run inside **"MSVC Dev Cmd (VS2022)"**. `cargo test` does not work in that terminal.

```
cargo run --release    # release build (always use for perf testing)
cargo run              # dev build
cargo build            # compile only
```

Dynamic linking is enabled in dev builds via `bevy/dynamic_linking` to keep recompile times low.

## Project Structure

```
src/
  main.rs           — app entry point, plugin registration
  game/
    mod.rs
    arena.rs        — battleground grid, spatial layout
    unit.rs         — Unit component, stats, UnitKind enum
    monster.rs      — Monster component, spawn waves
    combat.rs       — autobattle logic (target selection, damage)
    equipment.rs    — equipment definitions, stat modifiers
    shop.rs         — shop economy, buy/sell actions
  ui/
    mod.rs
    menu.rs         — main menu screen
    hud.rs          — in-battle HUD (wave counter, gold, etc.)
    shop_ui.rs      — shop panel UI
  rendering/
    mod.rs
    placeholder.rs  — white rect + name label for units (pre-spritesheet)
assets/
  fonts/            — font files for labels
  sprites/          — spritesheets (empty until art is ready)
```

## Game Loop States

```
AppState::MainMenu  →  AppState::Shopping  →  AppState::Battle  →  (loop)
```

- **MainMenu**: title screen, start/quit
- **Shopping**: place/buy/sell units and equipment before a wave
- **Battle**: combat runs; player watches; wave ends → back to Shopping

## Placeholder Rendering (Pre-Spritesheet)

Until real sprites exist, every unit/monster renders as:
- White filled rectangle sized to the unit's collision box
- Black text label centered on the rect showing the `UnitKind` name

Use `SpriteBundle` with a white 1×1 pixel texture scaled up, plus a `Text2dBundle` child.

## Unit Design

Each unit has:
- `UnitKind` enum variant (Warrior, Archer, Mage, …)
- `Stats { hp, max_hp, attack, defense, attack_speed, move_speed, range }`
- `Abilities` (list of ability identifiers, resolved at combat time)
- `Equipment` slots (weapon, armor, accessory)

## Combat Rules (Autobattle)

1. Each unit picks the nearest valid target each tick (respecting range).
2. Attacks fire on a timer based on `attack_speed`.
3. Damage = `attacker.attack - defender.defense` (min 1).
4. Unit dies when `hp <= 0`; entity despawned.
5. Wave ends when all monsters are dead (player wins) or all player units are dead (player loses).

## Performance Goals

- Target 1000+ simultaneous entities at 60 fps.
- Use Bevy's parallel query system; avoid single-threaded bottlenecks.
- Spatial queries use a simple grid/cell partition (no physics engine needed).
- Keep texture atlases batched; minimize draw calls.

## Coding Conventions

- One Bevy plugin per `mod` file; register in `main.rs`.
- Components are plain structs — no logic in them.
- Systems are free functions; no closures capturing state.
- No `unwrap()` in gameplay code; use `if let` or `?`.
- Comments only when the *why* is non-obvious.
