use bevy::prelude::*;
use rand::seq::SliceRandom;
use super::unit::UnitKind;
use super::party::PlayerParty;
use super::Gold;

pub const UNIT_COST: u32 = 2;

#[derive(Clone, Debug)]
pub struct ShopSlot {
    pub kind: UnitKind,
    pub bought: bool,
}

#[derive(Resource)]
pub struct ShopState {
    pub slots: Vec<ShopSlot>,
    /// Cost of the next manual refresh. Doubles each time; resets to 1 after a battle.
    pub refresh_cost: u32,
    /// Signals the preparation UI that it needs to rebuild.
    pub dirty: bool,
}

impl Default for ShopState {
    fn default() -> Self {
        Self { slots: Vec::new(), refresh_cost: 1, dirty: false }
    }
}

impl ShopState {
    /// Fill all 3 slots with random units (no replacement within a single roll).
    pub fn populate(&mut self) {
        let all: &[UnitKind] = &[
            UnitKind::Warrior, UnitKind::Archer, UnitKind::Mage,
            UnitKind::Paladin, UnitKind::Rogue,
        ];
        let chosen: Vec<UnitKind> = all
            .choose_multiple(&mut rand::thread_rng(), 3)
            .cloned()
            .collect();
        self.slots = chosen.into_iter()
            .map(|k| ShopSlot { kind: k, bought: false })
            .collect();
    }

    /// Re-roll all slots and double the refresh cost for next time.
    pub fn manual_refresh(&mut self) {
        self.populate();
        self.refresh_cost *= 2;
    }

    /// Called after a successful battle: clear slots so they regenerate fresh, reset refresh cost.
    pub fn reset_after_battle(&mut self) {
        self.slots.clear();
        self.refresh_cost = 1;
        self.dirty = false;
    }
}

/// Attempt to buy the unit at slot `idx`. Returns true if the purchase went through.
pub fn try_buy(shop: &mut ShopState, gold: &mut Gold, party: &mut PlayerParty, idx: usize) -> bool {
    if idx >= shop.slots.len() { return false; }
    if shop.slots[idx].bought || gold.0 < UNIT_COST { return false; }
    let kind = shop.slots[idx].kind.clone();
    gold.0 -= UNIT_COST;
    shop.slots[idx].bought = true;
    party.units.push(kind);
    true
}

/// Attempt to refresh the shop. Returns true if the refresh went through.
pub fn try_refresh(shop: &mut ShopState, gold: &mut Gold) -> bool {
    if gold.0 < shop.refresh_cost { return false; }
    gold.0 -= shop.refresh_cost;
    shop.manual_refresh();
    true
}

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShopState>();
    }
}
