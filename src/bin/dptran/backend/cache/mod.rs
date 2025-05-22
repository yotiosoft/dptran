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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CacheWrapper {
    cache_name: String,
    cache: Cache,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
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

pub fn get_cache_data(cache_name: &str) -> Result<CacheWrapper, CacheError> {
    let cache = confy::load::<Cache>("dptran", cache_name).map_err(|e| CacheError::FailToReadCache(e.to_string()))?;
    Ok(CacheWrapper {
        cache_name: cache_name.to_string(),
        cache,
    })
}

impl CacheWrapper {
    fn save_cache_data(&self) -> Result<(), CacheError> {
        confy::store("dptran", self.cache_name.as_str(), self.cache.clone()).map_err(|e| CacheError::FailToReadCache(e.to_string()))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_hash_test() {
        let text = String::from("Hello");
        let source_lang = Some(String::from("en"));
        let target_lang = String::from("fr");
        let expected_hash = "e19f0a05bb2edd7b53bbc66dd0c8ec5e";
        let hash = get_cache_data("cache_test").map_err(|e| CacheError::FailToReadCache(e.to_string())).unwrap()
            .cache_hash(&text, &source_lang, &target_lang);
        assert_eq!(hash.len(), 32);
        assert_eq!(hash, expected_hash);
    }

    #[test]
    fn cache_into_and_search_test() {
        let source_text = String::from("Hello");
        let value = String::from("Bonjour");
        let source_lang = Some(String::from("en"));
        let target_lang = String::from("fr");
        let max_entries = 10;

        // Clear cache before test
        get_cache_data("cache_test").map_err(|e| CacheError::FailToReadCache(e.to_string())).unwrap()
            .clear_cache().unwrap();

        // Insert into cache
        let result = get_cache_data("cache_test").map_err(|e| CacheError::FailToReadCache(e.to_string())).unwrap()
            .into_cache_element(&source_text, &value, &source_lang, &target_lang, max_entries);
        assert!(result.is_ok());

        // Search in cache
        let search_result = get_cache_data("cache_test").map_err(|e| CacheError::FailToReadCache(e.to_string())).unwrap()
            .search_cache(&source_text, &source_lang, &target_lang);
        assert!(search_result.is_ok());
        assert_eq!(search_result.unwrap(), Some(value));
    }

    #[test]
    fn cache_clear_test() {
        let mut cache = get_cache_data("test_cache").map_err(|e| CacheError::FailToReadCache(e.to_string())).unwrap();

        _ = cache.clear_cache();

        // Insert some data into cache
        let source_text = String::from("Hello");
        let value = String::from("Bonjour");
        let source_lang = Some(String::from("en"));
        let target_lang = String::from("fr");
        let max_entries = 10;
        let result = cache.into_cache_element(&source_text, &value, &source_lang, &target_lang, max_entries);
        assert!(result.is_ok());

        // Check if cache has data
        let mut cache_data = get_cache_data("test_cache").map_err(|e| CacheError::FailToReadCache(e.to_string())).unwrap();
        assert_eq!(cache_data.cache.elements.len(), 1);

        // Clear cache
        let result = cache_data.clear_cache();
        assert!(result.is_ok());

        // Check if cache is empty
        let cache_data = get_cache_data("test_cache").map_err(|e| CacheError::FailToReadCache(e.to_string())).unwrap();
        assert_eq!(cache_data.cache.elements.len(), 0);
    }
}
