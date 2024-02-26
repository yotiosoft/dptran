use std::fmt;
use serde::{Deserialize, Serialize};
use confy;

/// Configure properties
#[derive(Serialize, Deserialize, Debug)]
struct Configure {
    pub api_key: String,
    pub default_target_language: String
}
impl Default for Configure {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            default_target_language: "JA".to_string()
        }
    }
}

/// Configuration error
#[derive(Debug, PartialEq)]
pub enum ConfigError {
    FailToGetSettings(String),
    FailToSetApiKey(String),
    FailToSetDefaultTargetLanguage(String),
    FailToClearSettings(String),
}
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigError::FailToGetSettings(ref e) => write!(f, "Failed to get settings: {}", e),
            ConfigError::FailToSetApiKey(ref e) => write!(f, "Failed to set API key: {}", e),
            ConfigError::FailToSetDefaultTargetLanguage(ref e) => write!(f, "Failed to set default target language: {}", e),
            ConfigError::FailToClearSettings(ref e) => write!(f, "Failed to clear settings: {}", e),
        }
    }
}

/// Reading configuration files and extracting values
/// Get the API key and default target language for translation from the configuration file.
/// If none exists, create a new one with a default value.
fn get_settings() -> Result<Configure, ConfigError> {
    confy::load::<Configure>("dptran", "configure").map_err(|e| ConfigError::FailToGetSettings(e.to_string()))
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
