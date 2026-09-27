//! Recorded kit macros, persisted as JSON in the app config directory.

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Click {
    pub x: i32,
    pub y: i32,
    /// Ctrl was held (Ctrl+left click). Missing in kits saved before this existed.
    #[serde(default)]
    pub ctrl: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Kit {
    pub id: String,
    pub name: String,
    pub clicks: Vec<Click>,
}

pub struct KitStore {
    path: PathBuf,
    pub kits: Vec<Kit>,
}

impl KitStore {
    pub fn load(path: PathBuf) -> Self {
        let kits = fs::read_to_string(&path)
            .ok()
            .and_then(|json| serde_json::from_str(&json).ok())
            .unwrap_or_default();
        Self { path, kits }
    }

    fn save(&self) -> Result<(), String> {
        if let Some(dir) = self.path.parent() {
            fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        }
        let json = serde_json::to_string_pretty(&self.kits).map_err(|e| e.to_string())?;
        fs::write(&self.path, json).map_err(|e| e.to_string())
    }

    pub fn add(&mut self, name: String, clicks: Vec<Click>) -> Result<(), String> {
        let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis();
        self.kits.push(Kit { id: id.to_string(), name, clicks });
        self.save()
    }

    pub fn remove(&mut self, id: &str) -> Result<(), String> {
        self.kits.retain(|kit| kit.id != id);
        self.save()
    }

    pub fn get(&self, id: &str) -> Option<&Kit> {
        self.kits.iter().find(|kit| kit.id == id)
    }
}
