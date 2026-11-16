// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-jvm: jvm component model.
//!
//! Real implementation: jvm_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JvmItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JvmError {
    EmptyJvmItemName,
    DuplicateJvmItem(String),
    EmptyEventSubject,
}

impl fmt::Display for JvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyJvmItemName => write!(f, "jvm_item name must not be empty"),
            Self::DuplicateJvmItem(id) => write!(f, "duplicate jvm_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for JvmError {}

pub struct Jvm {
    pub name: String,
    jvm_items: BTreeMap<String, JvmItem>,
    events: BTreeMap<String, Event>,
}

impl Jvm {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            jvm_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_jvm_item(&mut self, item: JvmItem) -> Result<(), JvmError> {
        if item.name.trim().is_empty() {
            return Err(JvmError::EmptyJvmItemName);
        }
        let id = item.id.clone();
        if self.jvm_items.contains_key(&id) {
            return Err(JvmError::DuplicateJvmItem(id.clone()));
        }
        self.jvm_items.insert(id, item);
        Ok(())
    }

    pub fn jvm_item_count(&self) -> usize {
        self.jvm_items.len()
    }

    pub fn get_jvm_item(&self, id: &str) -> Option<&JvmItem> {
        self.jvm_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), JvmError> {
        if event.subject.trim().is_empty() {
            return Err(JvmError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_jvm_item(id: &str, name: &str) -> JvmItem {
        JvmItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Jvm::new("Jvm");
        assert_eq!(m.name, "Jvm");
        assert_eq!(m.jvm_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_jvm_item() {
        let mut m = Jvm::new("Jvm");
        m.add_jvm_item(sample_jvm_item("a1", "Alpha")).unwrap();
        assert_eq!(m.jvm_item_count(), 1);
        assert!(m.get_jvm_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Jvm::new("Jvm");
        assert!(m.add_jvm_item(sample_jvm_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Jvm::new("Jvm");
        m.add_jvm_item(sample_jvm_item("x", "One")).unwrap();
        assert!(m.add_jvm_item(sample_jvm_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Jvm::new("Jvm");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
