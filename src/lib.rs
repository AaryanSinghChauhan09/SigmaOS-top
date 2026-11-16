// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-top: top component model.
//!
//! Real implementation: top_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopItem {
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
pub enum TopError {
    EmptyTopItemName,
    DuplicateTopItem(String),
    EmptyEventSubject,
}

impl fmt::Display for TopError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTopItemName => write!(f, "top_item name must not be empty"),
            Self::DuplicateTopItem(id) => write!(f, "duplicate top_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for TopError {}

pub struct Top {
    pub name: String,
    top_items: BTreeMap<String, TopItem>,
    events: BTreeMap<String, Event>,
}

impl Top {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            top_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_top_item(&mut self, item: TopItem) -> Result<(), TopError> {
        if item.name.trim().is_empty() {
            return Err(TopError::EmptyTopItemName);
        }
        let id = item.id.clone();
        if self.top_items.contains_key(&id) {
            return Err(TopError::DuplicateTopItem(id.clone()));
        }
        self.top_items.insert(id, item);
        Ok(())
    }

    pub fn top_item_count(&self) -> usize {
        self.top_items.len()
    }

    pub fn get_top_item(&self, id: &str) -> Option<&TopItem> {
        self.top_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), TopError> {
        if event.subject.trim().is_empty() {
            return Err(TopError::EmptyEventSubject);
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

    fn sample_top_item(id: &str, name: &str) -> TopItem {
        TopItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Top::new("Top");
        assert_eq!(m.name, "Top");
        assert_eq!(m.top_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_top_item() {
        let mut m = Top::new("Top");
        m.add_top_item(sample_top_item("a1", "Alpha")).unwrap();
        assert_eq!(m.top_item_count(), 1);
        assert!(m.get_top_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Top::new("Top");
        assert!(m.add_top_item(sample_top_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Top::new("Top");
        m.add_top_item(sample_top_item("x", "One")).unwrap();
        assert!(m.add_top_item(sample_top_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Top::new("Top");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
