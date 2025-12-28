//! Empire data loading
//!
//! Loads empire definitions from .roe files

use bevy_ecs::prelude::*;
use std::collections::HashMap;

use shared::{EmpireDef, EmpireId, LeaderId, LeaderDef};

/// Resource containing all loaded empire data
#[derive(Resource, Debug, Clone, Default)]
pub struct EmpireData {
    pub empires: HashMap<EmpireId, EmpireDef>,
}

impl EmpireData {
    /// Get an empire by ID
    pub fn get_empire(&self, id: &EmpireId) -> Option<&EmpireDef> {
        self.empires.get(id)
    }

    /// Get a leader by ID (searches all empires)
    pub fn get_leader(&self, leader_id: &LeaderId) -> Option<(&EmpireDef, &LeaderDef)> {
        for empire in self.empires.values() {
            for leader in &empire.leaders {
                if &leader.id == leader_id {
                    return Some((empire, leader));
                }
            }
        }
        None
    }

    /// Get all empires as a sorted list
    pub fn all_empires(&self) -> Vec<&EmpireDef> {
        let mut empires: Vec<_> = self.empires.values().collect();
        empires.sort_by(|a, b| a.name.cmp(&b.name));
        empires
    }

    /// Get leaders for a specific empire
    pub fn get_leaders_for_empire(&self, empire_id: &EmpireId) -> Vec<&LeaderDef> {
        self.empires
            .get(empire_id)
            .map(|e| e.leaders.iter().collect())
            .unwrap_or_default()
    }
}

// Embed empire data at compile time for WASM compatibility
const EMPIRE_DATA: &[(&str, &str)] = &[
    ("aztecs", include_str!("../../../assets/data/empires/aztecs.roe")),
    ("chinese", include_str!("../../../assets/data/empires/chinese.roe")),
    ("egyptians", include_str!("../../../assets/data/empires/egyptians.roe")),
    ("greeks", include_str!("../../../assets/data/empires/greeks.roe")),
    ("mongols", include_str!("../../../assets/data/empires/mongols.roe")),
    ("persians", include_str!("../../../assets/data/empires/persians.roe")),
    ("romans", include_str!("../../../assets/data/empires/romans.roe")),
    ("russians", include_str!("../../../assets/data/empires/russians.roe")),
    ("vikings", include_str!("../../../assets/data/empires/vikings.roe")),
];

/// Load empire data from embedded strings (works in WASM)
pub fn load_empire_data() -> EmpireData {
    let mut empires = HashMap::new();

    for (name, content) in EMPIRE_DATA {
        match ron::from_str::<EmpireDef>(content) {
            Ok(empire) => {
                bevy_log::info!("Loaded empire: {} ({})", empire.name, empire.id);
                empires.insert(empire.id.clone(), empire);
            }
            Err(e) => {
                bevy_log::warn!("Failed to parse empire {}: {}", name, e);
            }
        }
    }

    bevy_log::info!("Loaded {} empires", empires.len());
    EmpireData { empires }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empire_data_default() {
        let data = EmpireData::default();
        assert!(data.empires.is_empty());
    }

    #[test]
    fn test_load_embedded_empires() {
        let data = load_empire_data();
        assert!(!data.empires.is_empty());
    }
}
