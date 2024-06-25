use std::collections::HashMap;
use std::fmt;
use serde::{Deserialize, Serialize};
use confy;
use md5;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CacheElement {
    pub key: String,
    pub target_langcode: String,
    pub value: String,
}

// Cache struct
#[derive(Serialize, Deserialize, Debug)]
struct Cache {
    pub elements: HashMap<String, CacheElement>,
}
impl Default for Cache {
    fn default() -> Self {
        Self {
            elements: HashMap::new(),
        }
    }
}

/// Cache error
#[derive(Debug, PartialEq)]
pub enum CacheError {
    FailToReadCache(String),
}
impl fmt::Display for CacheError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CacheError::FailToReadCache(ref e) => write!(f, "Failed to read cache: {}", e),
        }
    }
}

fn get_cache_data() -> Result<Cache, CacheError> {
    confy::load::<Cache>("dptran", "cache").map_err(|e| CacheError::FailToReadCache(e.to_string()))
}

fn save_cache_data(cache_data: Cache) -> Result<(), CacheError> {
    confy::store("dptran", "cache", cache_data).map_err(|e| CacheError::FailToReadCache(e.to_string()))
}

pub fn into_cache_element(source_text: &String, value: &String, target_lang: &String, max_entries: usize) -> Result<(), CacheError> {
    // read cache data file
    let mut cache_data = get_cache_data()?;
    // if caches are more than max_entries, remove the oldest one
    if cache_data.elements.len() >= max_entries {
        // Find the oldest key
        if let Some(oldest_key) = cache_data.elements.keys().next().cloned() {
            cache_data.elements.remove(&oldest_key);
        }
    }
    // clone source_text and value
    let s = source_text.clone();
    let v = value.clone();
    // create key by md5
    let hash = md5::compute(s.as_bytes());
    let key = format!("{:x}", hash);
    // create cache element
    let element = CacheElement {
        key: key.clone(),
        target_langcode: target_lang.clone(),
        value: v,
    };
    // insert element into cache_data
    cache_data.elements.insert(key, element);
    // save cache data
    save_cache_data(cache_data)?;
    Ok(())
}

pub fn search_cache(value: &String, target_lang: &String) -> Result<Option<String>, CacheError> {
    let cache_data = get_cache_data()?;
    let v = value.clone();
    let hash = md5::compute(v.as_bytes());
    let key = format!("{:x}", hash);

    if let Some(element) = cache_data.elements.get(&key) {
        if element.target_langcode == *target_lang {
            return Ok(Some(element.value.clone()));
        }
    }

    Ok(None)
}
