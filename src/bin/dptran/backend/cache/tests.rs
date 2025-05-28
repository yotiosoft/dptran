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
