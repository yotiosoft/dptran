use std::fmt;
use dptran::ApiKeyType;
use serde::{Deserialize, Serialize};
use confy;
use confy::ConfyError;
use std::path::PathBuf;

/// Configure properties
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigureWrapper {
    config_name: String,
    pub configure: Configure,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configure {
    pub settings_version: String,
    pub api_key: Option<String>,
    pub api_key_pro: Option<String>,
    pub default_target_language: String,
    pub cache_max_entries: usize,
    pub editor_command: Option<String>,
    pub cache_enabled: bool,
}
impl Default for Configure {
    fn default() -> Self {
        Self {
            settings_version: env!("CARGO_PKG_VERSION").to_string(),
            api_key: None,
            api_key_pro: None,
            default_target_language: "EN".to_string(),
            cache_max_entries: 100,
            editor_command: None,
            cache_enabled: true,
        }
    }
}

pub struct ApiKey {
    pub api_key: String,
    pub api_key_type: dptran::ApiKeyType,
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

impl ConfigureWrapper {
    /// Reading configuration files and extracting values
    /// Get the API key and default target language for translation from the configuration file.
    /// If none exists, create a new one with a default value.
    pub fn get(configure_name: &str) -> Result<Self, ConfigError> {
        let result = confy::load::<Configure>("dptran", configure_name);
        match result {
            Ok(settings) => {
                Ok(ConfigureWrapper { config_name: configure_name.to_string(), configure: settings })
            },
            Err(e) => {
                if let ConfyError::BadTomlData(_) = e {
                    let settings = older_configure::fix_settings_from_v2_0_0(configure_name)?;
                    Ok(ConfigureWrapper { config_name: configure_name.to_string(), configure: settings })
                } else {
                    Err(ConfigError::FailToGetSettings(e.to_string()))
                }
            }
        }
    }

    /// Save configuration file
    fn save(&self) -> Result<(), ConfyError> {
        confy::store("dptran", self.config_name.clone().as_str(), self.configure.clone())
    }

    /// Set API key
    /// Set the API key in the configuration file.
    pub fn set_api_key(&mut self, api_key: String, api_key_type: ApiKeyType) -> Result<(), ConfigError> {
        let api_key = if api_key.len() == 0 {
            None
        } else {
            Some(api_key)
        };
        if api_key_type == ApiKeyType::Pro {
            self.configure.api_key_pro = api_key;
        }
        else {
            self.configure.api_key = api_key;
        }
        self.save().map_err(|e| ConfigError::FailToSetApiKey(e.to_string()))?;
        Ok(())
    }

    /// Set default destination language
    /// Set the default target language for translation in the configuration file.
    pub fn set_default_target_language(&mut self, default_target_language: &String) -> Result<(), ConfigError> {
        self.configure.default_target_language = default_target_language.to_string();
        self.save().map_err(|e| ConfigError::FailToSetDefaultTargetLanguage(e.to_string()))?;
        Ok(())
    }

    /// Set cache maximum entries
    /// Set the maximum entries of the cache in the configuration file.
    pub fn set_cache_max_entries(&mut self, cache_max_entries: usize) -> Result<(), ConfigError> {
        self.configure.cache_max_entries = cache_max_entries;
        self.save().map_err(|e| ConfigError::FailToSetCacheMaxEntries(e.to_string()))?;
        Ok(())
    }

    /// Set default editor
    pub fn set_editor_command(&mut self, editor_command: String) -> Result<(), ConfigError> {
        self.configure.editor_command = Some(editor_command);
        self.save().map_err(|e| ConfigError::FailToSetEditor(e.to_string()))?;
        Ok(())
    }

    /// Set cache enabled
    pub fn set_cache_enabled(&mut self, cache_enabled: bool) -> Result<(), ConfigError> {
        self.configure.cache_enabled = cache_enabled;
        self.save().map_err(|e| ConfigError::FailToSetCacheEnabled(e.to_string()))?;
        Ok(())
    }

    /// Initialize settings
    pub fn clear_settings(&self) -> Result<(), ConfigError> {
        let cleared_settings = Configure::default();
        confy::store("dptran", self.config_name.clone().as_str(), cleared_settings).map_err(|e| ConfigError::FailToClearSettings(e.to_string()))?;
        Ok(())
    }

    /// Get the configured default target language code for translation
    pub fn get_default_target_language_code(&self) -> Result<String, ConfigError> {
        Ok(self.configure.default_target_language.clone())
    }

    /// Get API key
    /// If there is a pro API key, return it.
    /// Otherwise, return the free API key.
    pub fn get_api_key(&self) -> Option<ApiKey> {
        if self.configure.api_key_pro.is_none() && self.configure.api_key.is_none() {
            None
        }
        else if self.configure.api_key_pro.is_none() {
            Some(ApiKey {
                api_key: self.configure.api_key.clone().unwrap(),
                api_key_type: dptran::ApiKeyType::Free,
            })
        }
        else {
            Some(ApiKey {
                api_key: self.configure.api_key_pro.clone().unwrap(),
                api_key_type: dptran::ApiKeyType::Pro,
            })
        }
    }

    /// Get cache maximum entries
    pub fn get_cache_max_entries(&self) -> Result<usize, ConfigError> {
        Ok(self.configure.cache_max_entries)
    }

    /// Get default editor
    pub fn get_editor_command(&self) -> Result<Option<String>, ConfigError> {
        Ok(self.configure.editor_command.clone())
    }

    /// Get cache enabled
    pub fn get_cache_enabled(&self) -> Result<bool, ConfigError> {
        Ok(self.configure.cache_enabled)
    }

    /// Get configuration file path
    pub fn get_config_file_path(&self) -> Result<PathBuf, ConfigError> {
        confy::get_configuration_file_path("dptran", self.config_name.clone().as_str()).map_err(|e| ConfigError::FailToGetSettings(e.to_string()))
    }
}

mod older_configure {
    use super::*;

    /// Old configure properties
    #[derive(Serialize, Deserialize, Debug)]
    pub struct ConfigureBeforeV2_0_0 {
        pub api_key: String,
        pub default_target_language: String,
    }
    impl Default for ConfigureBeforeV2_0_0 {
        fn default() -> Self {
            Self {
                api_key: String::new(),
                default_target_language: "EN".to_string(),
            }
        }
    }

    /// If the configuration file is older, update it.
    pub fn fix_settings_from_v2_0_0(configure_name: &str) -> Result<Configure, ConfigError> {
        // from ver.2.0.0
        let config_v2_0_0 = confy::load::<ConfigureBeforeV2_0_0>("dptran", configure_name);
        if config_v2_0_0.is_ok() {
            let config = config_v2_0_0.unwrap();
            let settings = Configure {
                settings_version: env!("CARGO_PKG_VERSION").to_string(),
                api_key: Some(config.api_key),
                api_key_pro: None,
                default_target_language: config.default_target_language,
                cache_max_entries: 100,
                editor_command: None,
                cache_enabled: true,
            };
            confy::store("dptran", configure_name, &settings).map_err(|e| ConfigError::FailToGetSettings(e.to_string()))?;
            return Ok(settings);
        }
        Err(ConfigError::FailToFixSettings)
    }
}


#[cfg(test)]
include!("./tests.rs");
