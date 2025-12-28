//! Empire data loading
//!
//! Loads empire definitions from .roe files

use bevy_ecs::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use shared::{EmpireDef, EmpireId, LeaderId, LeaderDef};

/// Resource containing all loaded empire data
#[derive(Resource, Debug, Clone, Default)]
pub struct EmpireData {
    pub empires: HashMap<EmpireId, EmpireDef>,
}

impl EmpireData {
    /// Load all empire definitions from a directory
    pub fn load_from_directory(dir_path: &Path) -> Result<Self, String> {
        let mut empires = HashMap::new();

        if !dir_path.exists() {
            return Err(format!("Empire data directory not found: {:?}", dir_path));
        }

        let entries = fs::read_dir(dir_path)
            .map_err(|e| format!("Failed to read empire directory: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();

            if path.extension().map(|e| e == "roe").unwrap_or(false) {
                match Self::load_empire_file(&path) {
                    Ok(empire) => {
                        bevy_log::info!("Loaded empire: {} ({})", empire.name, empire.id);
                        empires.insert(empire.id.clone(), empire);
                    }
                    Err(e) => {
                        bevy_log::warn!("Failed to load empire from {:?}: {}", path, e);
                    }
                }
            }
        }

        bevy_log::info!("Loaded {} empires", empires.len());
        Ok(Self { empires })
    }

    /// Load a single empire definition file
    fn load_empire_file(path: &Path) -> Result<EmpireDef, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        ron::from_str(&content)
            .map_err(|e| format!("Failed to parse RON: {}", e))
    }

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

/// Load empire data from the default location
pub fn load_empire_data() -> EmpireData {
    // Try multiple possible paths
    let possible_paths = [
        "assets/data/empires",
        "../assets/data/empires",
        "../../assets/data/empires",
    ];

    for path_str in &possible_paths {
        let path = Path::new(path_str);
        if path.exists() {
            match EmpireData::load_from_directory(path) {
                Ok(data) => return data,
                Err(e) => {
                    bevy_log::warn!("Failed to load empires from {:?}: {}", path, e);
                }
            }
        }
    }

    bevy_log::warn!("No empire data found, using empty defaults");
    EmpireData::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empire_data_default() {
        let data = EmpireData::default();
        assert!(data.empires.is_empty());
    }
}
