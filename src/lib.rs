//! SigmaOS-archlinux-gstreamer0_10: GStreamer 0.10 legacy multimedia model.

use std::collections::BTreeMap;
use std::fmt;

/// Primary domain entity for GStreamer 0.10 legacy.
#[derive(Debug, Clone, PartialEq)]
pub struct LegacyPipeline {
    pub id: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
}

/// Secondary domain entity for GStreamer 0.10 legacy.
#[derive(Debug, Clone, PartialEq)]
pub struct LegacyElement {
    pub id: String,
    pub factory: String,
    pub description: String,
}

/// Errors for GStreamer 0.10 domain.
#[derive(Debug, PartialEq)]
pub enum GstreamerLegacyError {
    EmptyName,
    Duplicate(String),
    NotFound(String),
}

impl fmt::Display for GstreamerLegacyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "name cannot be empty"),
            Self::Duplicate(id) => write!(f, "duplicate id: {id}"),
            Self::NotFound(id) => write!(f, "not found: {id}"),
        }
    }
}

impl std::error::Error for GstreamerLegacyError {}

/// Registry and manager for GStreamer 0.10.
pub struct GstreamerLegacyManager {
    pub name: String,
    items: BTreeMap<String, LegacyPipeline>,
    configs: BTreeMap<String, LegacyElement>,
}

impl GstreamerLegacyManager {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            items: BTreeMap::new(),
            configs: BTreeMap::new(),
        }
    }

    pub fn add_item(&mut self, item: LegacyPipeline) -> Result<(), GstreamerLegacyError> {
        if item.name.is_empty() {
            return Err(GstreamerLegacyError::EmptyName);
        }
        if self.items.contains_key(&item.id) {
            return Err(GstreamerLegacyError::Duplicate(item.id.clone()));
        }
        self.items.insert(item.id.clone(), item);
        Ok(())
    }

    pub fn get_item(&self, id: &str) -> Option<&LegacyPipeline> {
        self.items.get(id)
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }

    pub fn add_config(&mut self, cfg: LegacyElement) {
        self.configs.insert(cfg.id.clone(), cfg);
    }

    pub fn config_count(&self) -> usize {
        self.configs.len()
    }

    pub fn set_enabled(&mut self, id: &str, enabled: bool) -> Result<(), GstreamerLegacyError> {
        self.items
            .get_mut(id)
            .map(|item| item.enabled = enabled)
            .ok_or_else(|| GstreamerLegacyError::NotFound(id.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_item(id: &str, name: &str) -> LegacyPipeline {
        LegacyPipeline {
            id: id.to_string(),
            name: name.to_string(),
            description: "gstreamer0_10-default".to_string(),
            enabled: true,
        }
    }

    fn make_config(id: &str, val: &str) -> LegacyElement {
        LegacyElement {
            id: id.to_string(),
            factory: val.to_string(),
            description: format!("{id} element"),
        }
    }

    #[test]
    fn test_create_aaa() {
        let mgr = GstreamerLegacyManager::new("test");
        assert_eq!(mgr.name, "test");
        assert_eq!(mgr.count(), 0);
        assert_eq!(mgr.config_count(), 0);
    }

    #[test]
    fn test_add_item_aaa() {
        let mut mgr = GstreamerLegacyManager::new("test");
        let item = make_item("item1", "First Item");
        assert!(mgr.add_item(item).is_ok());
        assert_eq!(mgr.count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut mgr = GstreamerLegacyManager::new("test");
        let item = make_item("item1", "");
        assert_eq!(mgr.add_item(item), Err(GstreamerLegacyError::EmptyName));
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut mgr = GstreamerLegacyManager::new("test");
        let item1 = make_item("item1", "First");
        let item2 = make_item("item1", "Duplicate");
        mgr.add_item(item1).unwrap();
        assert_eq!(mgr.add_item(item2), Err(GstreamerLegacyError::Duplicate("item1".to_string())));
    }

    #[test]
    fn test_config_addition_aaa() {
        let mut mgr = GstreamerLegacyManager::new("test");
        let cfg = make_config("cfg1", "value1");
        mgr.add_config(cfg);
        assert_eq!(mgr.config_count(), 1);
    }

    #[test]
    fn test_get_item_aaa() {
        let mut mgr = GstreamerLegacyManager::new("test");
        let item = make_item("item1", "First Item");
        mgr.add_item(item).unwrap();
        assert!(mgr.get_item("item1").is_some());
        assert!(mgr.get_item("none").is_none());
    }

    #[test]
    fn test_set_enabled_aaa() {
        let mut mgr = GstreamerLegacyManager::new("test");
        let item = make_item("item1", "First Item");
        mgr.add_item(item).unwrap();
        assert!(mgr.set_enabled("item1", false).is_ok());
        assert!(!mgr.get_item("item1").unwrap().enabled);
        assert_eq!(mgr.set_enabled("none", true), Err(GstreamerLegacyError::NotFound("none".to_string())));
    }
}
