//! SigmaOS-archlinux-bashrc: Bash RC configuration model.

use std::collections::BTreeMap;
use std::fmt;

/// Primary domain entity for Bash RC configuration.
pub struct BashConfig {
    pub id: String,
    pub name: String,
    pub shell_opts: String,
    pub enabled: bool,
}

/// Secondary domain entity for Bash RC configuration.
pub struct Alias {
    pub id: String,
    pub command: String,
    pub description: String,
}

/// Errors for the Bash RC configuration domain.
#[derive(Debug, PartialEq)]
pub enum BashrcError {
    EmptyName,
    Duplicate(String),
    NotFound(String),
}

impl fmt::Display for BashrcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "name cannot be empty"),
            Self::Duplicate(id) => write!(f, "duplicate id: {id}"),
            Self::NotFound(id) => write!(f, "not found: {id}"),
        }
    }
}

impl std::error::Error for BashrcError {}

/// Registry and manager for Bash RC configuration.
pub struct BashrcManager {
    pub name: String,
    items: BTreeMap<String, BashConfig>,
    configs: BTreeMap<String, Alias>,
}

impl BashrcManager {
    /// Create a new manager instance.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            items: BTreeMap::new(),
            configs: BTreeMap::new(),
        }
    }

    /// Add a primary item with fail-fast validation.
    pub fn add_item(&mut self, item: BashConfig) -> Result<(), BashrcError> {
        if item.name.is_empty() {
            return Err(BashrcError::EmptyName);
        }
        if self.items.contains_key(&item.id) {
            return Err(BashrcError::Duplicate(item.id.clone()));
        }
        self.items.insert(item.id.clone(), item);
        Ok(())
    }

    /// Retrieve a primary item by id.
    pub fn get_item(&self, id: &str) -> Option<&BashConfig> {
        self.items.get(id)
    }

    /// Count of primary items.
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Add a secondary configuration entry.
    pub fn add_config(&mut self, cfg: Alias) {
        self.configs.insert(cfg.id.clone(), cfg);
    }

    /// Count of configuration entries.
    pub fn config_count(&self) -> usize {
        self.configs.len()
    }

    /// Enable or disable an item.
    pub fn set_enabled(&mut self, id: &str, enabled: bool) -> Result<(), BashrcError> {
        self.items
            .get_mut(id)
            .map(|item| item.enabled = enabled)
            .ok_or_else(|| BashrcError::NotFound(id.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_BashConfig(id: &str, name: &str) -> BashConfig {
        BashConfig {
            id: id.to_string(),
            name: name.to_string(),
            shell_opts: "archlinux-bashrc-default".to_string(),
            enabled: true,
        }
    }

    fn make_Alias(id: &str, val: &str) -> Alias {
        Alias {
            id: id.to_string(),
            command: val.to_string(),
            description: format!("{} config", id),
        }
    }

    #[test]
    fn test_create_aaa() {
        let mgr = BashrcManager::new("test");
        assert_eq!(mgr.name, "test");
        assert_eq!(mgr.count(), 0);
        assert_eq!(mgr.config_count(), 0);
    }

    #[test]
    fn test_add_item_aaa() {
        let mut mgr = BashrcManager::new("test");
        let item = make_BashConfig("item1", "First Item");
        assert!(mgr.add_item(item).is_ok());
        assert_eq!(mgr.count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut mgr = BashrcManager::new("test");
        let item = make_BashConfig("item1", "");
        assert_eq!(mgr.add_item(item), Err(BashrcError::EmptyName));
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut mgr = BashrcManager::new("test");
        let item1 = make_BashConfig("item1", "First");
        let item2 = make_BashConfig("item1", "Duplicate");
        mgr.add_item(item1).unwrap();
        assert_eq!(mgr.add_item(item2), Err(BashrcError::Duplicate("item1".to_string())));
    }

    #[test]
    fn test_config_addition_aaa() {
        let mut mgr = BashrcManager::new("test");
        let cfg = make_Alias("cfg1", "value1");
        mgr.add_config(cfg);
        assert_eq!(mgr.config_count(), 1);
    }

    #[test]
    fn test_get_item_aaa() {
        let mut mgr = BashrcManager::new("test");
        let item = make_BashConfig("item1", "First Item");
        mgr.add_item(item).unwrap();
        assert!(mgr.get_item("item1").is_some());
        assert!(mgr.get_item("none").is_none());
    }

    #[test]
    fn test_set_enabled_aaa() {
        let mut mgr = BashrcManager::new("test");
        let item = make_BashConfig("item1", "First Item");
        mgr.add_item(item).unwrap();
        assert!(mgr.set_enabled("item1", false).is_ok());
        assert!(!mgr.get_item("item1").unwrap().enabled);
        assert_eq!(mgr.set_enabled("none", true), Err(BashrcError::NotFound("none".to_string())));
    }
}
