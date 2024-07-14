use std::collections::HashMap;
use std::fmt;
use serde::{Deserialize, Serialize};
use confy;
use md5;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CacheElement {
    pub key: String,
    pub source_langcode: Option<String>,
    pub target_langcode: String,
    pub value: String,
}

// Cache struct
#[derive(Serialize, Deserialize, Debug)]
struct Cache {
    pub saved_version: String,
    pub elements: HashMap<String, CacheElement>,
}
impl Default for Cache {
    fn default() -> Self {
        Self {
            saved_version: env!("CARGO_PKG_VERSION").to_string(),
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

fn cache_hash(text: &String, source_lang: &Option<String>, target_lang: &String) -> String {
    let mut s = format!("text:{}:", text);
    if source_lang.is_some() {
        s.push_str(format!(":source:{}", target_lang).as_str());
    }
    s.push_str(format!("target:{}", target_lang).as_str());
    let hash = md5::compute(s.as_bytes());
    format!("{:x}", hash)
}

pub fn into_cache_element(source_text: &String, value: &String, source_lang: &Option<String>, target_lang: &String, max_entries: usize) -> Result<(), CacheError> {
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
    let key = cache_hash(&s, source_lang, target_lang);
    // create cache element
    let element = CacheElement {
        key: key.clone(),
        source_langcode: source_lang.clone(),
        target_langcode: target_lang.clone(),
        value: v,
    };
    // insert element into cache_data
    cache_data.elements.insert(key, element);
    // save cache data
    save_cache_data(cache_data)?;
    Ok(())
}

pub fn search_cache(value: &String, source_lang: &Option<String>, target_lang: &String) -> Result<Option<String>, CacheError> {
    let cache_data = get_cache_data()?;
    let v = value.clone();
    let key = cache_hash(&v, source_lang, target_lang);

    if let Some(element) = cache_data.elements.get(&key) {
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

pub fn clear_cache() -> Result<(), CacheError> {
    let cache_data = Cache::default();
    save_cache_data(cache_data)
}
