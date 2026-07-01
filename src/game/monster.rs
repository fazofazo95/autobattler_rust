use bevy::prelude::*;

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub enum MonsterKind {
    // Tier 1 — rounds 1-5
    Goblin,
    Skeleton,
    // Tier 2 — rounds 6-10
    Orc,
    Troll,
    // Tier 3 — rounds 11-20
    Bandit,
    Werewolf,
    // Tier 4 — rounds 21-35
    DarkKnight,
    Vampire,
    Necromancer,
    // Tier 5 — rounds 36-50
    Golem,
    Witch,
    Ogre,
    // Tier 6 — rounds 51-65
    DragonWhelp,
    Demon,
    // Tier 7 — rounds 66+
    Lich,
    Dragon,
}

impl MonsterKind {
    pub fn display_name(&self) -> &'static str {
        match self {
            MonsterKind::Goblin      => "Goblin",
            MonsterKind::Skeleton    => "Skeleton",
            MonsterKind::Orc         => "Orc",
            MonsterKind::Troll       => "Troll",
            MonsterKind::Bandit      => "Bandit",
            MonsterKind::Werewolf    => "Werewolf",
            MonsterKind::DarkKnight  => "Dark Knight",
            MonsterKind::Vampire     => "Vampire",
            MonsterKind::Necromancer => "Necromancer",
            MonsterKind::Golem       => "Golem",
            MonsterKind::Witch       => "Witch",
            MonsterKind::Ogre        => "Ogre",
            MonsterKind::DragonWhelp => "Dragon Whelp",
            MonsterKind::Demon       => "Demon",
            MonsterKind::Lich        => "Lich",
            MonsterKind::Dragon      => "Dragon",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "Goblin"       => Some(MonsterKind::Goblin),
            "Skeleton"     => Some(MonsterKind::Skeleton),
            "Orc"          => Some(MonsterKind::Orc),
            "Troll"        => Some(MonsterKind::Troll),
            "Bandit"       => Some(MonsterKind::Bandit),
            "Werewolf"     => Some(MonsterKind::Werewolf),
            "Dark Knight"  => Some(MonsterKind::DarkKnight),
            "Vampire"      => Some(MonsterKind::Vampire),
            "Necromancer"  => Some(MonsterKind::Necromancer),
            "Golem"        => Some(MonsterKind::Golem),
            "Witch"        => Some(MonsterKind::Witch),
            "Ogre"         => Some(MonsterKind::Ogre),
            "Dragon Whelp" => Some(MonsterKind::DragonWhelp),
            "Demon"        => Some(MonsterKind::Demon),
            "Lich"         => Some(MonsterKind::Lich),
            "Dragon"       => Some(MonsterKind::Dragon),
            _              => None,
        }
    }

    pub fn base_defense(&self) -> i32 {
        match self {
            MonsterKind::Goblin      => 2,
            MonsterKind::Skeleton    => 3,
            MonsterKind::Orc         => 8,
            MonsterKind::Troll       => 12,
            MonsterKind::Bandit      => 3,
            MonsterKind::Werewolf    => 5,
            MonsterKind::DarkKnight  => 15,
            MonsterKind::Vampire     => 6,
            MonsterKind::Necromancer => 2,
            MonsterKind::Golem       => 25,
            MonsterKind::Witch       => 1,
            MonsterKind::Ogre        => 12,
            MonsterKind::DragonWhelp => 8,
            MonsterKind::Demon       => 10,
            MonsterKind::Lich        => 8,
            MonsterKind::Dragon      => 30,
        }
    }
}

#[derive(Component)]
pub struct Monster;

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, _app: &mut App) {}
}
