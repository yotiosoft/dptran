pub mod parse;
pub mod configure;
pub mod cache;

use std::fmt::Debug;
use configure::ConfigError;
use cache::CacheError;
pub use parse::ExecutionMode;

use dptran::{DpTranError, DpTranUsage};

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

/// Get the number of characters remaining to be translated
/// Retrieved from <https://api-free.deepl.com/v2/usage>
/// Returns an error if acquisition fails
pub fn get_usage() -> Result<DpTranUsage, RuntimeError> {
    let api_key = get_api_key()?;
    if let Some(api_key) = api_key {
        let dptran = dptran::DpTran::with(&api_key);
        dptran.get_usage().map_err(|e| RuntimeError::DeeplApiError(e))
    } else {
        Err(RuntimeError::DeeplApiError(DpTranError::ApiKeyIsNotSet))
    }
}

/// Set API key (using confy crate).
/// Set the API key in the configuration file config.json.
pub fn set_api_key(api_key: String) -> Result<(), RuntimeError> {
    configure::set_api_key(api_key).map_err(|e| RuntimeError::ConfigError(e))?;
    Ok(())
}

/// Set default destination language.
/// Set the default target language for translation in the configuration file config.json.
pub fn set_default_target_language(arg_default_target_language: &String) -> Result<String, RuntimeError> {
    let api_key = match get_api_key()? {
        Some(api_key) => api_key,
        None => return Err(RuntimeError::DeeplApiError(DpTranError::ApiKeyIsNotSet)),
    };
    let dptran = dptran::DpTran::with(&api_key);

    // Check if the language code is correct
    if let Ok(validated_language_code) = dptran.correct_target_language_code(arg_default_target_language) {
        configure::set_default_target_language(&validated_language_code).map_err(|e| RuntimeError::ConfigError(e))?;
        Ok(validated_language_code)
    } else {
        Err(RuntimeError::DeeplApiError(DpTranError::InvalidLanguageCode))
    }
}

/// Set the editor command.
pub fn set_editor_command(editor_command: String) -> Result<(), RuntimeError> {
    configure::set_editor_command(editor_command).map_err(|e| RuntimeError::ConfigError(e))?;
    Ok(())
}

/// Clear the settings.
pub fn clear_settings() -> Result<(), RuntimeError> {
    configure::clear_settings().map_err(|e| RuntimeError::ConfigError(e))?;
    Ok(())
}

/// Get the configured default destination language code.
pub fn get_default_target_language_code() -> Result<String, RuntimeError> {
    let default_target_lang = configure::get_default_target_language_code().map_err(|e| RuntimeError::ConfigError(e))?;
    Ok(default_target_lang)
}

/// Load the API key from the configuration file.
pub fn get_api_key() -> Result<Option<String>, RuntimeError> {
    let api_key = configure::get_api_key().map_err(|e| RuntimeError::ConfigError(e))?;
    // If the API key is not set, check environment variables
    if api_key.is_none() {
        let env_api_key = std::env::var("DPTRAN_DEEPL_API_KEY").ok();
        return Ok(env_api_key);
    }
    Ok(api_key)
}

/// Get the maximum number of cache entries.
pub fn get_cache_max_entries() -> Result<usize, RuntimeError> {
    let cache_max_entries = configure::get_cache_max_entries().map_err(|e| RuntimeError::ConfigError(e))?;
    Ok(cache_max_entries)
}

/// Load the editor command from the configuration file.
pub fn get_editor_command_str() -> Result<Option<String>, RuntimeError> {
    let editor_command = configure::get_editor_command().map_err(|e| RuntimeError::ConfigError(e))?;
    Ok(editor_command)
}

/// Get the cache enabled status.
pub fn get_cache_enabled() -> Result<bool, RuntimeError> {
    let cache_enabled = configure::get_cache_enabled().map_err(|e| RuntimeError::ConfigError(e))?;
    Ok(cache_enabled)
}

/// Search the cache
pub fn search_cache(query: &Vec<String>, source_lang:&Option<String>, target_lang: &String) -> Result<Option<String>, RuntimeError> {
    let cache_enabled = configure::get_cache_enabled().map_err(|e| RuntimeError::ConfigError(e))?;
    let cache_str = query.join("\n").trim().to_string();
    let cache_result = if cache_enabled {
        cache::search_cache(&cache_str, source_lang, target_lang).map_err(|e| RuntimeError::CacheError(e))?
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
    cache::into_cache_element(&before_translate_str, &after_translate_str, source_lang, target_lang, max_entries).map_err(|e| RuntimeError::CacheError(e))?;
    Ok(())
}
