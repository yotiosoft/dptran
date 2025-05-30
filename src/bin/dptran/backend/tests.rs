use dptran::DeeplAPIError;

fn retry_or_panic(e: &RuntimeError, times: u8) -> bool {
    if e == &RuntimeError::DeeplApiError(DpTranError::DeeplApiError(DeeplAPIError::ConnectionError(dptran::ConnectionError::TooManyRequests))) && times < 3 {
        // Because the DeepL API has a limit on the number of requests per second, retry after 2 seconds if the error is TooManyRequests.
        std::thread::sleep(std::time::Duration::from_secs(2));
        return true;
    }
    else {
        panic!("Error: {}", e.to_string());
    }
}

fn impl_backend_get_usage(times: u8) {
    let usage = get_usage();
    if let Err(e) = &usage {
        if retry_or_panic(&e, 1) {
            return impl_backend_get_usage(times + 1);
        }
    }
    assert!(usage.is_ok());
}

#[test]
fn backend_get_and_set_api_key_test() {
    // Set as a free API key
    let api_key = ApiKey {
        api_key: "test_api_key".to_string(),
        api_key_type: dptran::ApiKeyType::Free,
    };
    clear_settings().unwrap();
    set_api_key(api_key).unwrap();
    let retrieved_api_key = get_api_key().unwrap().unwrap();
    assert_eq!(retrieved_api_key.api_key, "test_api_key");
    assert_eq!(retrieved_api_key.api_key_type, dptran::ApiKeyType::Free);

    // Set as a pro API key
    let api_key = ApiKey {
        api_key: "test_pro_api_key".to_string(),
        api_key_type: dptran::ApiKeyType::Pro,
    };
    set_api_key(api_key).unwrap();
    let retrieved_api_key = get_api_key().unwrap().unwrap();
    assert_eq!(retrieved_api_key.api_key, "test_pro_api_key");
    assert_eq!(retrieved_api_key.api_key_type, dptran::ApiKeyType::Pro);
}

#[test]
fn backend_get_and_set_editor_command_test() {
    let editor_command = "test_editor".to_string();
    set_editor_command(editor_command.clone()).unwrap();
    let retrieved_editor_command = get_editor_command_str().unwrap();
    assert_eq!(retrieved_editor_command, Some(editor_command));
    clear_settings().unwrap();
    let retrieved_editor_command = get_editor_command_str().unwrap();
    assert_eq!(retrieved_editor_command, None);
}

#[test]
fn backend_get_and_set_cache_max_entries_test() {
    let cache_max_entries = 50;
    configure::ConfigureWrapper::get(CONFIG_NAME).map_err(|e| RuntimeError::ConfigError(e)).unwrap()
        .set_cache_max_entries(cache_max_entries).map_err(|e| RuntimeError::ConfigError(e)).unwrap();
    let retrieved_cache_max_entries = get_cache_max_entries().unwrap();
    assert_eq!(retrieved_cache_max_entries, cache_max_entries);
    clear_settings().unwrap();
    let retrieved_cache_max_entries = get_cache_max_entries().unwrap();
    assert_eq!(retrieved_cache_max_entries, 100);
}

#[test]
fn backend_get_and_set_cache_enabled_test() {
    let cache_enabled = false;
    configure::ConfigureWrapper::get(CONFIG_NAME).map_err(|e| RuntimeError::ConfigError(e)).unwrap()
        .set_cache_enabled(cache_enabled).map_err(|e| RuntimeError::ConfigError(e)).unwrap();
    let retrieved_cache_enabled = get_cache_enabled().unwrap();
    assert_eq!(retrieved_cache_enabled, cache_enabled);
    clear_settings().unwrap();
    let retrieved_cache_enabled = get_cache_enabled().unwrap();
    assert_eq!(retrieved_cache_enabled, true);
}

#[test]
fn backend_into_and_search_cache_test() {
    let source_text = vec!["Hello".to_string()];
    let translated_text = vec!["Bonjour".to_string()];
    let source_lang = Some("en".to_string());
    let target_lang = "fr".to_string();

    // Insert into cache
    into_cache(&source_text, &translated_text, &source_lang, &target_lang).unwrap();

    // Search in cache
    let result = search_cache(&source_text, &source_lang, &target_lang).unwrap();
    assert_eq!(result, Some(translated_text.join("\n").trim().to_string()));
    // Clear cache
    cache::get_cache_data(CACHE_NAME).map_err(|e| RuntimeError::CacheError(e)).unwrap()
        .clear_cache().map_err(|e| RuntimeError::CacheError(e)).unwrap();
    // Check if cache is empty
    let cache_data_elements = search_cache(&source_text, &source_lang, &target_lang).unwrap();
    assert_eq!(cache_data_elements, None);
}

#[test]
fn backend_format_translation_result_test() {
    // some Arabic text
    let translated_text = "مرحبا بك في ديبل";
    let formatted_text = format_translation_result(translated_text);
    assert_eq!(formatted_text, "لبيد يف كب ابحرم");     // Arabic text is right-to-left
    // some Japanese text
    let translated_text = "こんにちは、DeepLへようこそ";
    let formatted_text = format_translation_result(translated_text);
    assert_eq!(formatted_text, "こんにちは、DeepLへようこそ");
    // some English text
    let translated_text = "Hello, welcome to DeepL";
    let formatted_text = format_translation_result(translated_text);
    assert_eq!(formatted_text, "Hello, welcome to DeepL");
}

#[test]
fn backend_create_and_append_file_test() {
    let file_path = "test_file.txt";
    let text = "Hello, world!";
    let ofile = create_file(file_path).unwrap();
    append_to_file(&ofile, text).unwrap();
    std::fs::remove_file(file_path).unwrap(); // Clean up
}

#[test]
fn backend_get_usage_test() {
    impl_backend_get_usage(0);
}
