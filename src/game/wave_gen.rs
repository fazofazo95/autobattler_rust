use super::monster::MonsterKind;

/// Runtime wave option built from rounds.json data.
#[derive(Clone)]
pub struct WaveOption {
    pub label: String,
    pub monsters: Vec<(MonsterKind, u32)>,
}

impl WaveOption {
    pub fn describe(&self) -> String {
        self.monsters.iter()
            .filter(|(_, n)| *n > 0)
            .map(|(k, n)| format!("{}x {}", n, k.display_name()))
            .collect::<Vec<_>>()
            .join(", ")
    }
}
