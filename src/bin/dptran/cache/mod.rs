use std::fmt;
use serde::{Deserialize, Serialize};
use confy;
use std::path::PathBuf;
use md5;

#[derive(Serialize, Deserialize, Debug)]
struct CacheElement {
    pub key: String,
    pub value: String,
}

// Cache struct
#[derive(Serialize, Deserialize, Debug)]
struct Cache {
    pub elements: Vec<CacheElement>,
}
impl Default for Cache {
    fn default() -> Self {
        Self {
            elements: Vec::new(),
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

pub fn into_cache_element(value: String) -> Result<(), CacheError> {
    let mut cache_data = get_cache_data()?;
    let hash = md5::compute(value.as_bytes());
    let key = format!("{:x}", hash);
    let element = CacheElement {
        key: key,
        value: value,
    };
    cache_data.elements.push(element);
    save_cache_data(cache_data)?;
    Ok(())
}

pub fn search_cache(value: String) -> Result<Option<String>, CacheError> {
    let cache_data = get_cache_data()?;
    let hash = md5::compute(value.as_bytes());
    let key = format!("{:x}", hash);

    for element in cache_data.elements {
        if element.key == key {
            return Ok(Some(element.value.clone()));
        }
    }

    Ok(None)
}
