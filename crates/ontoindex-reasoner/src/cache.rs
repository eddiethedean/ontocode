use crate::adapter::ReasonerId;
use crate::input::ReasonerInput;
use crate::result::{ClassificationResult, ReasonerSnapshot};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ReasonerCache {
    pub content_hash: String,
    pub profile: ReasonerId,
    pub snapshot: ReasonerSnapshot,
    pub classification: ClassificationResult,
    pub input: ReasonerInput,
}

#[derive(Debug, Default)]
pub struct ReasonerCacheStore {
    entries: HashMap<(String, String), ReasonerCache>,
}

impl ReasonerCacheStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, content_hash: &str, profile: ReasonerId) -> Option<&ReasonerCache> {
        self.entries.get(&(content_hash.to_string(), profile.as_str().to_string()))
    }

    pub fn get_any_for_profile(&self, profile: ReasonerId) -> Option<&ReasonerCache> {
        self.entries.values().find(|e| e.profile == profile)
    }

    pub fn insert(&mut self, cache: ReasonerCache) {
        let key = (cache.content_hash.clone(), cache.profile.as_str().to_string());
        self.entries.insert(key, cache);
    }

    pub fn invalidate(&mut self) {
        self.entries.clear();
    }

    pub fn latest_snapshot(&self) -> Option<&ReasonerSnapshot> {
        self.entries.values().max_by_key(|e| e.snapshot.classified_at).map(|e| &e.snapshot)
    }

    pub fn store_classification(
        &mut self,
        input: ReasonerInput,
        profile: ReasonerId,
        classification: ClassificationResult,
    ) -> ReasonerSnapshot {
        let snapshot = ReasonerSnapshot::from(classification.clone());
        let cache = ReasonerCache {
            content_hash: input.content_hash.clone(),
            profile,
            snapshot: snapshot.clone(),
            classification,
            input,
        };
        self.insert(cache);
        snapshot
    }
}
