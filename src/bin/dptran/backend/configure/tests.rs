use super::*;

#[test]
fn configure_set_api_key_test() {
    let mut config_wrapper = ConfigureWrapper::get("configure_test").unwrap();
    let new_api_key = "new_api_key".to_string();
    config_wrapper.set_api_key(new_api_key, ApiKeyType::Free).unwrap();
    let updated_config = ConfigureWrapper::get("configure_test").unwrap();
    assert_eq!(updated_config.configure.api_key, Some("new_api_key".to_string()));
}

#[test]
fn configure_set_default_target_language_test() {
    let mut config_wrapper = ConfigureWrapper::get("configure_test").unwrap();
    let new_target_language = "FR".to_string();
    config_wrapper.set_default_target_language(&new_target_language).unwrap();
    let updated_config = ConfigureWrapper::get("configure_test").unwrap();
    assert_eq!(updated_config.configure.default_target_language, "FR");
}

#[test]
fn configure_set_cache_max_entries_test() {
    let mut config_wrapper = ConfigureWrapper::get("configure_test").unwrap();
    let new_cache_max_entries = 200;
    config_wrapper.set_cache_max_entries(new_cache_max_entries).unwrap();
    let updated_config = ConfigureWrapper::get("configure_test").unwrap();
    assert_eq!(updated_config.configure.cache_max_entries, 200);
}

#[test]
fn configure_set_editor_command_test() {
    let mut config_wrapper = ConfigureWrapper::get("configure_test").unwrap();
    let new_editor_command = "vim".to_string();
    config_wrapper.set_editor_command(new_editor_command).unwrap();
    let updated_config = ConfigureWrapper::get("configure_test").unwrap();
    assert_eq!(updated_config.configure.editor_command, Some("vim".to_string()));
}

#[test]
fn configure_set_cache_enabled_test() {
    let mut config_wrapper = ConfigureWrapper::get("configure_test").unwrap();
    let new_cache_enabled = false;
    config_wrapper.set_cache_enabled(new_cache_enabled).unwrap();
    let updated_config = ConfigureWrapper::get("configure_test").unwrap();
    assert_eq!(updated_config.configure.cache_enabled, false);
}

#[test]
fn configure_clear_general_settings_test() {
    let mut config_wrapper = ConfigureWrapper::get("configure_test").unwrap();
    config_wrapper.clear_general_settings().unwrap();
    let cleared_config = ConfigureWrapper::get("configure_test").unwrap();
    assert_eq!(cleared_config.configure.default_target_language, "EN");
    assert_eq!(cleared_config.configure.cache_max_entries, 100);
    assert_eq!(cleared_config.configure.editor_command, None);
    assert_eq!(cleared_config.configure.cache_enabled, true);
}

#[test]
fn configure_clear_api_settings_test() {
    let mut config_wrapper = ConfigureWrapper::get("configure_test").unwrap();
    config_wrapper.clear_api_settings().unwrap();
    let cleared_config = ConfigureWrapper::get("configure_test").unwrap();
    assert_eq!(cleared_config.configure.api_key, None);
    assert_eq!(cleared_config.configure.api_key_pro, None);
    assert_eq!(cleared_config.configure.endpoint_of_translation, None);
    assert_eq!(cleared_config.configure.endpoint_of_usage, None);
    assert_eq!(cleared_config.configure.endpoint_of_languages, None);
}

#[test]
fn configure_get_default_target_language_code_test() {
    // set up a test configuration 
    let mut config_wrapper = ConfigureWrapper::get("configure_test").unwrap();
    config_wrapper.set_default_target_language(&"FR".to_string()).unwrap();
    let default_target_language = config_wrapper.get_default_target_language_code().unwrap();
    assert_eq!(default_target_language, "FR");
}

#[test]
fn configure_set_and_get_api_key_test() {
    // for ApiKeyType::Free
    let mut config_wrapper = ConfigureWrapper::get("configure_test").unwrap();
    config_wrapper.clear_api_settings().unwrap();
    let api_key_to_set = "configure_api_key".to_string();
    config_wrapper.set_api_key(api_key_to_set.clone(), ApiKeyType::Free).unwrap();
    // Reload the configuration to ensure the key is set correctly
    let config_wrapper = ConfigureWrapper::get("configure_test").unwrap();
    let api_key = config_wrapper.get_api_key().unwrap();
    assert_eq!(api_key.api_key, api_key_to_set);
    assert_eq!(api_key.api_key_type, ApiKeyType::Free);

    // for ApiKeyType::Pro
    let mut config_wrapper = ConfigureWrapper::get("configure_test").unwrap();
    let api_key_pro_to_set = "configure_api_key_pro".to_string();
    config_wrapper.set_api_key(api_key_pro_to_set.clone(), ApiKeyType::Pro).unwrap();
    let api_key_to_set = "configure_api_key".to_string();
    config_wrapper.set_api_key(api_key_to_set.clone(), ApiKeyType::Free).unwrap();
    // Reload the configuration to ensure the pro key is set correctly
    let config_wrapper = ConfigureWrapper::get("configure_test").unwrap();
    let api_key_pro = config_wrapper.get_api_key().unwrap();
    assert_eq!(api_key_pro.api_key, api_key_pro_to_set);
    assert_eq!(api_key_pro.api_key_type, ApiKeyType::Pro);  // If the pro key is set, it will be returned instead of the free key
}

#[test]
fn configure_get_cache_max_entries_test() {
    let mut config_wrapper = ConfigureWrapper::get("configure_test").unwrap();
    let cache_max_entries_to_set = 200;
    config_wrapper.set_cache_max_entries(cache_max_entries_to_set).unwrap();
    let cache_max_entries = config_wrapper.get_cache_max_entries().unwrap();
    assert_eq!(cache_max_entries, cache_max_entries_to_set);
}

#[test]
fn configure_get_editor_command_test() {
    let mut config_wrapper = ConfigureWrapper::get("configure_test").unwrap();
    let editor_command_to_set = "vim".to_string();
    config_wrapper.set_editor_command(editor_command_to_set.clone()).unwrap();
    let editor_command = config_wrapper.get_editor_command().unwrap();
    assert_eq!(editor_command, Some(editor_command_to_set));
}

#[test]
fn configure_get_cache_enabled_test() {
    let mut config_wrapper = ConfigureWrapper::get("configure_test").unwrap();
    let cache_enabled_to_set = false;
    config_wrapper.set_cache_enabled(cache_enabled_to_set).unwrap();
    let cache_enabled = config_wrapper.get_cache_enabled().unwrap();
    assert_eq!(cache_enabled, cache_enabled_to_set);
}

#[test]
fn configure_fix_settings_test() {
    // for ConfigureBeforeV2_0_0
    // Create a temporary configuration file with old settings
    let old_config = older_configure::ConfigureBeforeV2_0_0::default();
    confy::store("dptran", "configure_test", &old_config).unwrap();

    // Call the fix_settings function
    let fixed_config = older_configure::fix_settings_from_v2_0_0("configure_test").unwrap();

    // Check if the settings were updated correctly
    assert_eq!(fixed_config.settings_version, env!("CARGO_PKG_VERSION"));
    assert_eq!(fixed_config.default_target_language, old_config.default_target_language);
    assert_eq!(fixed_config.cache_max_entries, 100);
    assert_eq!(fixed_config.editor_command, None);
    assert_eq!(fixed_config.cache_enabled, true);
}

#[test]
fn configure_set_endpoints_test() {
    let mut config_wrapper = ConfigureWrapper::get("configure_test").unwrap();
    let new_endpoint_translation = "https://api-free.deepl.com/v2/translate".to_string();
    let new_endpoint_usage = "https://api-free.deepl.com/v2/usage".to_string();
    let new_endpoint_languages = "https://api-free.deepl.com/v2/languages".to_string();
    let new_endpoint_glossaries = "https://api-free.deepl.com/v2/glossaries".to_string();
    let new_endpoint_glossaries_langs = "https://api-free.deepl.com/v2/glossary-language-pairs".to_string();

    config_wrapper.set_endpoint_of_translation(new_endpoint_translation.clone()).unwrap();
    config_wrapper.set_endpoint_of_usage(new_endpoint_usage.clone()).unwrap();
    config_wrapper.set_endpoint_of_languages(new_endpoint_languages.clone()).unwrap();
    config_wrapper.set_endpoint_of_glossaries(new_endpoint_glossaries.clone()).unwrap();
    config_wrapper.set_endpoint_of_glossaries_langs(new_endpoint_glossaries_langs.clone()).unwrap();

    let updated_config = ConfigureWrapper::get("configure_test").unwrap();
    assert_eq!(updated_config.configure.endpoint_of_translation, Some(new_endpoint_translation));
    assert_eq!(updated_config.configure.endpoint_of_usage, Some(new_endpoint_usage));
    assert_eq!(updated_config.configure.endpoint_of_languages, Some(new_endpoint_languages));
    assert_eq!(updated_config.configure.endpoint_of_glossaries, Some(new_endpoint_glossaries));
    assert_eq!(updated_config.configure.endpoint_of_glossaries_langs, Some(new_endpoint_glossaries_langs));
}
