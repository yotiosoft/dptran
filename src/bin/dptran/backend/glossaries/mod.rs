use std::collections::HashMap;
use std::fmt;
use serde::{Deserialize, Serialize};
use confy;
use md5;

// An alias of dptran::glossaries::GlossaryDictionary
pub type StoredGlossaryDictionary = dptran::glossaries::GlossaryDictionary;

// An alias of dptran::Glossary
pub type StoredGlossary = dptran::glossaries::Glossary;

// Stored glossaries (Vec)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StoredGlossaries {
    pub saved_version: String,
    glossaries: Vec<StoredGlossary>
}
impl Default for StoredGlossaries {
    fn default() -> Self {
        Self {
            saved_version: env!("CARGO_PKG_VERSION").to_string(),
            glossaries: Vec::new()
        }
    }
}

// Stored glossaries wrapper
pub struct StoredGlossariesWrapper {
    glossaries_name: String,
    stored_glossaries: StoredGlossaries,
}

/// Cache error
#[derive(Debug, PartialEq)]
pub enum GlossariesError {
    FailToReadCache(String),
}
impl fmt::Display for GlossariesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GlossariesError::FailToReadCache(ref e) => write!(f, "Failed to read glossaries data: {}", e),
        }
    }
}

pub fn get_glossaries_data(glossaries_name: &str) -> Result<StoredGlossariesWrapper, GlossariesError> {
    let stored_glossaries = confy::load::<StoredGlossaries>("dptran", glossaries_name).map_err(|e| GlossariesError::FailToReadCache(e.to_string()))?;
    Ok(StoredGlossariesWrapper { 
        glossaries_name: glossaries_name.to_string().clone(),
        stored_glossaries 
    })
}

impl StoredGlossariesWrapper {
    fn save_glossaries_data(&self) -> Result<(), GlossariesError> {
        confy::store("dptran", self.glossaries_name.as_str(), self.stored_glossaries.clone()).map_err(|e| GlossariesError::FailToReadCache(e.to_string()))
    }

    fn cache_hash(&self, text: &String, source_lang: &Option<String>, target_lang: &String) -> String {
        let mut s = format!("text:{}:", text);
        if source_lang.is_some() {
            s.push_str(format!(":source:{}", target_lang).as_str());
        }
        s.push_str(format!("target:{}", target_lang).as_str());
        let hash = md5::compute(s.as_bytes());
        format!("{:x}", hash)
    }

    pub fn into_cache_element(&mut self, source_text: &String, value: &String, source_lang: &Option<String>, target_lang: &String, max_entries: usize) -> Result<(), CacheError> {
        // if caches are more than max_entries, remove the oldest one
        if self.cache.elements.len() >= max_entries {
            // Find the oldest key
            if let Some(oldest_key) = self.cache.elements.keys().next().cloned() {
                self.cache.elements.remove(&oldest_key);
            }
        }
        // clone source_text and value
        let s = source_text.clone();
        let v = value.clone();
        // create key by md5
        let key = self.cache_hash(&s, source_lang, target_lang);
        // create cache element
        let element = CacheElement {
            key: key.clone(),
            source_langcode: source_lang.clone(),
            target_langcode: target_lang.clone(),
            value: v,
        };
        // insert element into cache_data
        self.cache.elements.insert(key, element);
        // save cache data
        self.save_cache_data()?;
        Ok(())
    }

    pub fn search_cache(&self, value: &String, source_lang: &Option<String>, target_lang: &String) -> Result<Option<String>, CacheError> {
        let v = value.clone();
        let key = self.cache_hash(&v, source_lang, target_lang);

        if let Some(element) = self.cache.elements.get(&key) {
            if source_lang.is_none() {
                if element.target_langcode == *target_lang && element.source_langcode.is_none() {
                    return Ok(Some(element.value.clone()));
                }
            }
            else if element.source_langcode.is_some() {
                if element.target_langcode == *target_lang && element.source_langcode.as_ref().unwrap() == source_lang.as_ref().unwrap() {
                    return Ok(Some(element.value.clone()));
                }
            }
        }

        Ok(None)
    }

    pub fn clear_cache(&mut self) -> Result<(), CacheError> {
        self.cache = Cache::default();
        self.save_cache_data()
    }
}

//#[cfg(test)]
//include!("./tests.rs");
