use std::fmt;
use serde::{Deserialize, Serialize};
use confy;
use confy::ConfyError;
use std::path::PathBuf;

/// Configure properties
#[derive(Serialize, Deserialize, Debug)]
struct Configure {
    pub settings_version: String,
    pub api_key: String,
    pub default_target_language: String,
    pub cache_max_entries: usize,
    pub editor_command: Option<String>,
    pub cache_enabled: bool,
}
impl Default for Configure {
    fn default() -> Self {
        Self {
            settings_version: env!("CARGO_PKG_VERSION").to_string(),
            api_key: String::new(),
            default_target_language: "EN-US".to_string(),
            cache_max_entries: 100,
            editor_command: None,
            cache_enabled: true,
        }
    }
}

/// Configuration error
#[derive(Debug, PartialEq)]
pub enum ConfigError {
    FailToGetSettings(String),
    FailToSetApiKey(String),
    FailToSetDefaultTargetLanguage(String),
    FailToSetCacheMaxEntries(String),
    FailToSetEditor(String),
    FailToClearSettings(String),
    FailToFixSettings,
    FailToSetCacheEnabled(String),
}
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigError::FailToGetSettings(ref e) => write!(f, "Failed to get settings: {}", e),
            ConfigError::FailToSetApiKey(ref e) => write!(f, "Failed to set API key: {}", e),
            ConfigError::FailToSetDefaultTargetLanguage(ref e) => write!(f, "Failed to set default target language: {}", e),
            ConfigError::FailToSetCacheMaxEntries(ref e) => write!(f, "Failed to set cache max entries: {}", e),
            ConfigError::FailToSetEditor(ref e) => write!(f, "Failed to set editor: {}", e),
            ConfigError::FailToClearSettings(ref e) => write!(f, "Failed to clear settings: {}", e),
            ConfigError::FailToFixSettings => write!(f, "Failed to fix settings"),
            ConfigError::FailToSetCacheEnabled(ref e) => write!(f, "Failed to set cache enabled: {}", e),
        }
    }
}

/// Reading configuration files and extracting values
/// Get the API key and default target language for translation from the configuration file.
/// If none exists, create a new one with a default value.
fn get_settings() -> Result<Configure, ConfigError> {
    let result = confy::load::<Configure>("dptran", "configure");
    match result {
        Ok(settings) => Ok(settings),
        Err(e) => {
            if let ConfyError::BadTomlData(_) = e {
                let settings = fix_settings()?;
                Ok(settings)
            } else {
                Err(ConfigError::FailToGetSettings(e.to_string()))
            }
        }
    }
}

/// Set API key
/// Set the API key in the configuration file.
pub fn set_api_key(api_key: String) -> Result<(), ConfigError> {
    let mut settings = get_settings()?;
    settings.api_key = api_key;
    confy::store("dptran", "configure", settings).map_err(|e| ConfigError::FailToSetApiKey(e.to_string()))?;
    Ok(())
}

/// Set default destination language
/// Set the default target language for translation in the configuration file.
pub fn set_default_target_language(default_target_language: &String) -> Result<(), ConfigError> {
    let mut settings = get_settings()?;
    settings.default_target_language = default_target_language.to_string();
    confy::store("dptran", "configure", settings).map_err(|e| ConfigError::FailToSetDefaultTargetLanguage(e.to_string()))?;
    Ok(())
}

/// Set cache maximum entries
/// Set the maximum entries of the cache in the configuration file.
pub fn set_cache_max_entries(cache_max_entries: usize) -> Result<(), ConfigError> {
    let mut settings = get_settings()?;
    settings.cache_max_entries = cache_max_entries;
    confy::store("dptran", "configure", settings).map_err(|e| ConfigError::FailToSetCacheMaxEntries(e.to_string()))?;
    Ok(())
}

/// Set default editor
pub fn set_editor_command(editor_command: String) -> Result<(), ConfigError> {
    let mut settings = get_settings()?;
    settings.editor_command = Some(editor_command);
    confy::store("dptran", "configure", settings).map_err(|e| ConfigError::FailToSetEditor(e.to_string()))?;
    Ok(())
}

/// Set cache enabled
pub fn set_cache_enabled(cache_enabled: bool) -> Result<(), ConfigError> {
    let mut settings = get_settings()?;
    settings.cache_enabled = cache_enabled;
    confy::store("dptran", "configure", settings).map_err(|e| ConfigError::FailToSetCacheEnabled(e.to_string()))?;
    Ok(())
}

/// Initialize settings
pub fn clear_settings() -> Result<(), ConfigError> {
    let settings = Configure::default();
    confy::store("dptran", "configure", settings).map_err(|e| ConfigError::FailToClearSettings(e.to_string()))?;
    Ok(())
}

/// Get the configured default target language code for translation
pub fn get_default_target_language_code() -> Result<String, ConfigError> {
    let settings = get_settings()?;
    Ok(settings.default_target_language)
}

/// Get API key
pub fn get_api_key() -> Result<Option<String>, ConfigError> {
    let settings = get_settings()?;
    if settings.api_key.is_empty() {
        return Ok(None);
    }
    Ok(Some(settings.api_key))
}

/// Get cache maximum entries
pub fn get_cache_max_entries() -> Result<usize, ConfigError> {
    let settings = get_settings()?;
    Ok(settings.cache_max_entries)
}

/// Get default editor
pub fn get_editor_command() -> Result<Option<String>, ConfigError> {
    let settings = get_settings()?;
    Ok(settings.editor_command)
}

/// Get cache enabled
pub fn get_cache_enabled() -> Result<bool, ConfigError> {
    let settings = get_settings()?;
    Ok(settings.cache_enabled)
}

/// Get configuration file path
pub fn get_config_file_path() -> Result<PathBuf, ConfigError> {
    confy::get_configuration_file_path("dptran", "configure").map_err(|e| ConfigError::FailToGetSettings(e.to_string()))
}

/// Configure properties
#[derive(Serialize, Deserialize, Debug)]
struct ConfigureBeforeV200 {
    pub api_key: String,
    pub default_target_language: String,
}
impl Default for ConfigureBeforeV200 {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            default_target_language: "EN-US".to_string(),
        }
    }
}

/// If the configuration file is older, update it.
fn fix_settings() -> Result<Configure, ConfigError> {
    // from ver.2.0.0
    let config_v2_0_0 = confy::load::<ConfigureBeforeV200>("dptran", "configure");
    if config_v2_0_0.is_ok() {
        let config = config_v2_0_0.unwrap();
        let settings = Configure {
            settings_version: env!("CARGO_PKG_VERSION").to_string(),
            api_key: config.api_key,
            default_target_language: config.default_target_language,
            cache_max_entries: 100,
            editor_command: None,
            cache_enabled: true,
        };
        confy::store("dptran", "configure", &settings).map_err(|e| ConfigError::FailToGetSettings(e.to_string()))?;
        return Ok(settings);
    }
    Err(ConfigError::FailToFixSettings)
}
