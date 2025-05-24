pub mod parse;
pub mod configure;
pub mod cache;
use configure::ConfigError;
pub use configure::ApiKey;
use cache::CacheError;
pub use parse::ExecutionMode;

use dptran::{DpTranError, DpTranUsage};

use std::fmt::Debug;
use std::io::{Write, BufWriter};
use std::fs::OpenOptions;
use unicode_bidi::BidiInfo;

#[derive(PartialEq)]
pub enum RuntimeError {
    DeeplApiError(dptran::DpTranError),
    ApiKeyIsNotSet,
    ConfigError(ConfigError),
    StdIoError(String),
    FileIoError(String),
    EditorError(String),
    EditorCommandIsNotSet,
    CacheError(CacheError),
    CacheMaxEntriesIsNotSet,
}
impl ToString for RuntimeError {
    fn to_string(&self) -> String {
        match self {
            RuntimeError::DeeplApiError(e) => {
                match e {
                    dptran::DpTranError::DeeplApiError(e) => {
                        match e {
                            dptran::DeeplAPIError::ConnectionError(e) => {
                                match e {
                                    dptran::ConnectionError::Forbidden => "403 Forbidden Error. Maybe the API key is invalid.".to_string(),
                                    dptran::ConnectionError::NotFound => "404 Not Found Error. Make sure the internet connection is working.".to_string(),
                                    e => format!("Connection error: {}", e),
                                }
                            },
                            e => format!("Deepl API error: {}", e.to_string()),
                        }
                    },
                    e => format!("Deepl API error: {}", e.to_string()),
                }
            }
            RuntimeError::ApiKeyIsNotSet => "API key is not set.".to_string(),
            RuntimeError::ConfigError(e) => format!("Config error: {}", e),
            RuntimeError::StdIoError(e) => format!("Standard I/O error: {}", e),
            RuntimeError::FileIoError(e) => format!("File I/O error: {}", e),
            RuntimeError::EditorError(e) => format!("Editor error: {}", e),
            RuntimeError::EditorCommandIsNotSet => "Editor command is not specified.".to_string(),
            RuntimeError::CacheError(e) => format!("Cache error: {}", e),
            RuntimeError::CacheMaxEntriesIsNotSet => "Cache max entries is not specified.".to_string(),
        }
    }
}
impl Debug for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
static CONFIG_NAME: &str = "configure_test";
#[cfg(not(test))]
static CONFIG_NAME: &str = "configure";

#[cfg(test)]
static CACHE_NAME: &str = "cache_test";
#[cfg(not(test))]
static CACHE_NAME: &str = "cache";

/// Get the number of characters remaining to be translated
/// Retrieved from <https://api-free.deepl.com/v2/usage>
/// Returns an error if acquisition fails
pub fn get_usage() -> Result<DpTranUsage, RuntimeError> {
    let api_key = get_api_key()?;
    if let Some(api_key) = api_key {
        let dptran = dptran::DpTran::with(&api_key.api_key, api_key.api_key_type);
        dptran.get_usage().map_err(|e| RuntimeError::DeeplApiError(e))
    } else {
        Err(RuntimeError::DeeplApiError(DpTranError::ApiKeyIsNotSet))
    }
}

/// Set API key (using confy crate).
/// Set the API key in the configuration file config.json.
pub fn set_api_key(api_key: ApiKey) -> Result<(), RuntimeError> {
    configure::ConfigureWrapper::get(CONFIG_NAME).map_err(|e| RuntimeError::ConfigError(e))?
        .set_api_key(api_key.api_key, api_key.api_key_type).map_err(|e| RuntimeError::ConfigError(e))?;
    Ok(())
}

/// Set default destination language.
/// Set the default target language for translation in the configuration file config.json.
pub fn set_default_target_language(arg_default_target_language: &String) -> Result<String, RuntimeError> {
    let api_key = match get_api_key()? {
        Some(api_key) => api_key,
        None => return Err(RuntimeError::DeeplApiError(DpTranError::ApiKeyIsNotSet)),
    };
    let dptran = dptran::DpTran::with(&api_key.api_key, api_key.api_key_type);

    // Check if the language code is correct
    match dptran.correct_target_language_code(arg_default_target_language) {
        Ok(validated_language_code) => {
            configure::ConfigureWrapper::get(CONFIG_NAME).map_err(|e| RuntimeError::ConfigError(e))?
                .set_default_target_language(&validated_language_code).map_err(|e| RuntimeError::ConfigError(e))?;
            Ok(validated_language_code)
        }
        Err(e) => {
            Err(RuntimeError::DeeplApiError(e))
        }
    }
}

/// Set the editor command.
pub fn set_editor_command(editor_command: String) -> Result<(), RuntimeError> {
    configure::ConfigureWrapper::get(CONFIG_NAME).map_err(|e| RuntimeError::ConfigError(e))?
        .set_editor_command(editor_command).map_err(|e| RuntimeError::ConfigError(e))?;
    Ok(())
}

/// Clear the settings.
pub fn clear_settings() -> Result<(), RuntimeError> {
    configure::ConfigureWrapper::get(CONFIG_NAME).map_err(|e| RuntimeError::ConfigError(e))?
        .clear_settings().map_err(|e| RuntimeError::ConfigError(e))?;
    Ok(())
}

/// Get the configured default destination language code.
pub fn get_default_target_language_code() -> Result<String, RuntimeError> {
    let default_target_lang = configure::ConfigureWrapper::get(CONFIG_NAME).map_err(|e| RuntimeError::ConfigError(e))?
        .get_default_target_language_code().map_err(|e| RuntimeError::ConfigError(e))?;
    Ok(default_target_lang)
}

/// Load the API key from the configuration file.
pub fn get_api_key() -> Result<Option<ApiKey>, RuntimeError> {
    let api_key = configure::ConfigureWrapper::get(CONFIG_NAME).map_err(|e| RuntimeError::ConfigError(e))?
        .get_api_key();
    // If the API key is not set, check environment variables.
    // If there is a pro key, return it.
    // If there is no pro key, return the free key.
    if api_key.is_none() {
        let env_api_key = std::env::var("DPTRAN_DEEPL_API_KEY_PRO").ok();
        if let Some(env_api_key) = env_api_key {
            let api_key = ApiKey {
                api_key: env_api_key,
                api_key_type: dptran::ApiKeyType::Pro,
            };
            return Ok(Some(api_key));
        }

        let env_api_key = std::env::var("DPTRAN_DEEPL_API_KEY").ok();
        if let Some(env_api_key) = env_api_key {
            let api_key = ApiKey {
                api_key: env_api_key,
                api_key_type: dptran::ApiKeyType::Free,
            };
            return Ok(Some(api_key));
        }
    }
    Ok(api_key)
}

/// Get the maximum number of cache entries.
pub fn get_cache_max_entries() -> Result<usize, RuntimeError> {
    let cache_max_entries = configure::ConfigureWrapper::get(CONFIG_NAME).map_err(|e| RuntimeError::ConfigError(e))?
        .get_cache_max_entries().map_err(|e| RuntimeError::ConfigError(e))?;
    Ok(cache_max_entries)
}

/// Load the editor command from the configuration file.
pub fn get_editor_command_str() -> Result<Option<String>, RuntimeError> {
    let editor_command = configure::ConfigureWrapper::get(CONFIG_NAME).map_err(|e| RuntimeError::ConfigError(e))?
        .get_editor_command().map_err(|e| RuntimeError::ConfigError(e))?;
    Ok(editor_command)
}

/// Get the cache enabled status.
pub fn get_cache_enabled() -> Result<bool, RuntimeError> {
    let cache_enabled = configure::ConfigureWrapper::get(CONFIG_NAME).map_err(|e| RuntimeError::ConfigError(e))?
        .get_cache_enabled().map_err(|e| RuntimeError::ConfigError(e))?;
    Ok(cache_enabled)
}

/// Search the cache
pub fn search_cache(query: &Vec<String>, source_lang:&Option<String>, target_lang: &String) -> Result<Option<String>, RuntimeError> {
    let cache_enabled = configure::ConfigureWrapper::get(CONFIG_NAME).map_err(|e| RuntimeError::ConfigError(e))?
        .get_cache_enabled().map_err(|e| RuntimeError::ConfigError(e))?;
    let cache_str = query.join("\n").trim().to_string();
    let cache_result = if cache_enabled {
        cache::get_cache_data(CACHE_NAME).map_err(|e| RuntimeError::CacheError(e))?
            .search_cache(&cache_str, source_lang, target_lang).map_err(|e| RuntimeError::CacheError(e))?
    } else {
        None
    };
    Ok(cache_result)
}

/// Into the cache
pub fn into_cache(before_translate_str: &Vec<String>, after_translate_str: &Vec<String>, source_lang:&Option<String>, target_lang: &String) -> Result<(), RuntimeError> {
    let before_translate_str = before_translate_str.clone().join("\n").trim().to_string();
    let after_translate_str = after_translate_str.clone().join("\n").trim().to_string();
    let max_entries = get_cache_max_entries()?;
    cache::get_cache_data(CACHE_NAME).map_err(|e| RuntimeError::CacheError(e))?
            .into_cache_element(&before_translate_str, &after_translate_str, source_lang, target_lang, max_entries).map_err(|e| RuntimeError::CacheError(e))?;
    Ok(())
}

/// Return a formatted string of the translation result.
/// Use the unicode_bidi crate to handle bidirectional text.
pub fn format_translation_result(translated_text: &str) -> String {
    let bidi = BidiInfo::new(translated_text, None);
    let mut formatted_text = String::new();
    for para in &bidi.paragraphs {
        let line = para.range.clone();
        let display = bidi.reorder_line(para, line);
        formatted_text.push_str(&format!("{}", display));
    }
    formatted_text
}

/// Create a new file.
pub fn create_file(file_path: &str) -> Result<std::fs::File, RuntimeError> {
    let ofile = OpenOptions::new().create(true).write(true).truncate(true).open(&file_path).map_err(|e| RuntimeError::FileIoError(e.to_string()))?;
    Ok(ofile)
}

/// Append to the file
pub fn append_to_file(ofile: &std::fs::File, text: &str) -> Result<(), RuntimeError> {
    let mut buf_writer = BufWriter::new(ofile);
    writeln!(buf_writer, "{}", text).map_err(|e| RuntimeError::FileIoError(e.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use dptran::DeeplAPIError;

    use super::*;

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
}
