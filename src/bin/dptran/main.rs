use std::io::{Write, stdin, stdout};

mod backend;
use backend::RuntimeError;
use backend::ExecutionMode;

use dptran::{DpTranError, LangType};
use crate::backend::args::CacheSettingsTarget;
use crate::backend::args::GeneralSettingTarget;

/// Initialization of settings.
fn clear_general_settings() -> Result<(), RuntimeError> {
    print!("Are you sure you want to clear all settings? (y/N) ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    // Initialize settings when y is entered.
    if input.trim().to_ascii_lowercase() == "y" {
        let mut config = backend::configure::ConfigureWrapper::get("configure").map_err(|e| RuntimeError::ConfigError(e))?;
        config.clear_general_settings().map_err(|e| RuntimeError::ConfigError(e))?;
        println!("All settings have been cleared.");
        println!("Note: You need to set the API key again to use dptran.");
    }
    Ok(())
}

/// Display of general settings.
fn display_general_settings() -> Result<(), RuntimeError> {
    let config = backend::configure::ConfigureWrapper::get("configure").map_err(|e| RuntimeError::ConfigError(e))?;
    let default_target_lang = config.get_default_target_language_code().map_err(|e| RuntimeError::ConfigError(e))?;
    let cache_max_entries = config.get_cache_max_entries().map_err(|e| RuntimeError::ConfigError(e))?;
    let editor_command = config.get_editor_command().map_err(|e| RuntimeError::ConfigError(e))?;
    let cache_enabled = config.get_cache_enabled().map_err(|e| RuntimeError::ConfigError(e))?;

    println!("Default target language: {}", default_target_lang);

    println!("Cache max entries: {}", cache_max_entries);

    if let Some(editor_command) = editor_command {
        println!("Editor command: {}", editor_command);
    }
    else {
        println!("Editor command: not set");
    }

    println!("Cache enabled: {}", cache_enabled);

    let config_filepath = backend::configure::ConfigureWrapper::get("configure").map_err(|e| RuntimeError::ConfigError(e))?
        .get_config_file_path().map_err(|e| RuntimeError::ConfigError(e))?;
    println!("Configuration file path: {}", config_filepath.to_str().unwrap());

    Ok(())
}

/// Initialization of settings.
fn clear_api_settings() -> Result<(), RuntimeError> {
    print!("Are you sure you want to clear all settings? (y/N) ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    // Initialize settings when y is entered.
    if input.trim().to_ascii_lowercase() == "y" {
        backend::clear_api_key(dptran::ApiKeyType::Free)?;
        backend::clear_api_key(dptran::ApiKeyType::Pro)?;

        let mut config = backend::configure::ConfigureWrapper::get("configure").map_err(|e| RuntimeError::ConfigError(e))?;
        config.clear_api_settings().map_err(|e| RuntimeError::ConfigError(e))?;
    }
    Ok(())   
}

/// Display of API settings.
fn display_api_settings() -> Result<(), RuntimeError> {
    let config = backend::configure::ConfigureWrapper::get("configure").map_err(|e| RuntimeError::ConfigError(e))?;
    let api_key = backend::get_api_key()?;

    if let Some(api_key) = api_key {
        if api_key.api_key_type == dptran::ApiKeyType::Free {
            println!("API key (free): {}", api_key.api_key);
        }
        else {
            println!("API key (pro): {}", api_key.api_key);
        }
    }
    else {
        println!("API key: not set");
    }

    if let Some(endpoint_of_translation) = config.get_endpoint_of_translation().map_err(|e| RuntimeError::ConfigError(e))? {
        println!("Endpoint of translation: {}", endpoint_of_translation);
    } else {
        println!("Endpoint of translation: not set");
    }

    if let Some(endpoint_of_usage) = config.get_endpoint_of_usage().map_err(|e| RuntimeError::ConfigError(e))? {
        println!("Endpoint of usage: {}", endpoint_of_usage);
    } else {
        println!("Endpoint of usage: not set");
    }

    if let Some(endpoint_of_langs) = config.get_endpoint_of_languages().map_err(|e| RuntimeError::ConfigError(e))? {
        println!("Endpoint of languages: {}", endpoint_of_langs);
    } else {
        println!("Endpoint of languages: not set");
    }

    Ok(())
}

/// Display list of source language codes.
/// Retrieved from <https://api-free.deepl.com/v2/languages>
fn show_source_language_codes() -> Result<(), RuntimeError> {
    let api_key = match backend::get_api_key()? {
        Some(api_key) => api_key,
        None => return Err(RuntimeError::DeeplApiError(DpTranError::ApiKeyIsNotSet)),
    };
    let mut dptran = dptran::DpTran::with(&api_key.api_key, &api_key.api_key_type);

    // Reflect the endpoint settings in the configuration file.
    backend::reflect_endpoints(&mut dptran)?;

    // List of source language codes.
    let source_lang_codes = dptran.get_language_codes(LangType::Source).map_err(|e| RuntimeError::DeeplApiError(e))?;
    
    let mut i = 0;
    let (len, max_code_len, max_str_len) = get_langcodes_maxlen(&source_lang_codes);

    println!("Source language codes:");
    for lang_code in source_lang_codes {
        print!(" {lc:<cl$}: {ls:<sl$}", lc=lang_code.0.trim_matches('"'), ls=lang_code.1.trim_matches('"'), cl=max_code_len, sl=max_str_len);
        i += 1;
        if (i % 3) == 0 || i == len {
            println!();
        }
    }

    Ok(())
}
/// Display of list of language codes to be translated.
fn show_target_language_codes() -> Result<(), RuntimeError> {
    let api_key = match backend::get_api_key()? {
        Some(api_key) => api_key,
        None => return Err(RuntimeError::DeeplApiError(DpTranError::ApiKeyIsNotSet)),
    };
    let mut dptran = dptran::DpTran::with(&api_key.api_key, &api_key.api_key_type);

    // Reflect the endpoint settings in the configuration file.
    backend::reflect_endpoints(&mut dptran)?;

    // List of Language Codes.
    let target_lang_codes = dptran.get_language_codes(LangType::Target).map_err(|e| RuntimeError::DeeplApiError(e))?;

    let mut i = 0;
    let (len, max_code_len, max_str_len) = get_langcodes_maxlen(&target_lang_codes);

    println!("Target languages:");
    for lang_code in target_lang_codes {
        print!(" {lc:<cl$}: {ls:<sl$}", lc=lang_code.0.trim_matches('"'), ls=lang_code.1.trim_matches('"'), cl=max_code_len, sl=max_str_len);
        i += 1;
        if (i % 2) == 0 || i == len {
            println!();
        }
    }

    Ok(())
}
fn get_langcodes_maxlen(lang_codes: &Vec<(String, String)>) -> (usize, usize, usize) {
    let len = lang_codes.len();
    let max_code_len = lang_codes.iter().map(|x| x.0.len()).max().unwrap();
    let max_str_len = lang_codes.iter().map(|x| x.1.len()).max().unwrap();
    (len, max_code_len, max_str_len)
}

/// Display the number of characters remaining.
/// Retrieved from <https://api-free.deepl.com/v2/usage>
/// Returns an error if acquisition fails
pub fn handle_show_usage() -> Result<(), RuntimeError> {
    let api_key = backend::get_api_key()?;
    let usage = if let Some(api_key) = api_key {
        let mut dptran = dptran::DpTran::with(&api_key.api_key, &api_key.api_key_type);
        // Reflect the endpoint settings in the configuration file.
        backend::reflect_endpoints(&mut dptran)?;
        dptran.get_usage()
            .map_err(|e| RuntimeError::DeeplApiError(e))?
    } else {
        return Err(RuntimeError::DeeplApiError(DpTranError::ApiKeyIsNotSet));
    };
    if usage.character_limit.is_none() {
        println!("usage: {} / unlimited", usage.character_count);
    }
    else {
        println!("usage: {} / {} ({}%)", usage.character_count, usage.character_limit.unwrap(), (usage.character_count as f64 / usage.character_limit.unwrap() as f64 * 100.0).round());
        println!("remaining: {}", usage.character_limit.unwrap() - usage.character_count);
    }
    Ok(())
}

/// Get source text from the stdin.
fn get_input(mode: &backend::ExecutionMode, multilines: bool, rm_line_breaks: bool, text: &Option<String>) -> Option<Vec<String>> {
    let stdin = stdin();
    let mut stdout = stdout();

    match mode {
        ExecutionMode::TranslateInteractive => {
            print!("> ");
            stdout.flush().unwrap();

            let mut input_vec = Vec::<String>::new();
            let mut input = String::new();
            while stdin.read_line(&mut input).unwrap() > 0 {
                // If in multiline mode, it accepts input including newlines.
                if multilines {
                    if input == "\r\n" || input == "\n" {
                        break;
                    }
                }
                // If not in multiline mode, accepts input containing line feeds with [\\ + newline].
                else {
                    if input.ends_with("\n") && !input.ends_with("\\\r\n") && !input.ends_with("\\\n") {
                        input_vec.push(input.trim_end().to_string());
                        break;
                    }
                }

                input_vec.push(input.trim_end().to_string());
                input.clear();

                print!("..");
                stdout.flush().unwrap();
            }
            if rm_line_breaks {
                let input_vec = vec![input_vec.join(" ")];
                Some(input_vec)
            } else {
                Some(input_vec)
            }
        }
        ExecutionMode::TranslateNormal => {
            match text {
                Some(text) => {
                    if rm_line_breaks {
                        // Remove line breaks
                        let text = text.lines().collect::<Vec<&str>>().join(" ");
                        Some(vec![text])
                    } else {
                        // Split strings containing newline codes.
                        let lines = text.lines();
                        Some(lines.map(|x| x.to_string()).collect())
                    }
                },
                None => None
            }
        }
        _ => {
            panic!("Invalid mode.");
        }
    }
}

fn handle_general_settings(setting_struct: backend::args::GeneralSettingsStruct) -> Result<(), RuntimeError> {
    let setting_target = setting_struct.setting_target.clone();
    if let None = setting_target {
        return Err(RuntimeError::ArgInvalidTarget);
    }
    let mut config = backend::configure::ConfigureWrapper::get("configure").map_err(|e| RuntimeError::ConfigError(e))?;
    match setting_target.unwrap() {
        GeneralSettingTarget::DefaultTargetLang => {
            if let Some(s) = setting_struct.default_target_lang {
                let validated_language_code = backend::set_default_target_language(&s)?;
                println!("Default target language has been set to {}.", validated_language_code);
                return Ok(());
            } else {
                return Err(RuntimeError::DeeplApiError(DpTranError::NoTargetLanguageSpecified));
            }
        }
        GeneralSettingTarget::EditorCommand => {
            if let Some(s) = setting_struct.editor_command {
                config.set_editor_command(s).map_err(|e| RuntimeError::ConfigError(e))?;
                return Ok(());
            } else {
                return Err(RuntimeError::EditorCommandIsNotSet);
            }
        }
        GeneralSettingTarget::ShowSettings => {
            display_general_settings()?;
            return Ok(());
        }
        GeneralSettingTarget::ClearSettings => {
            clear_general_settings()?;
            return Ok(());
        }
    }
}

fn handle_api_settings(api_setting_struct: backend::args::ApiSettingsStruct) -> Result<(), RuntimeError> {
    let api_setting_target = api_setting_struct.setting_target;
    if let None = api_setting_target {
        return Err(RuntimeError::ArgInvalidTarget);
    }
    let mut config = backend::configure::ConfigureWrapper::get("configure").map_err(|e| RuntimeError::ConfigError(e))?;
    match api_setting_target.unwrap() {
        backend::args::ApiSettingsTarget::FreeApiKey => {
            if let Some(s) = api_setting_struct.api_key_free {
                config.set_api_key(s, dptran::ApiKeyType::Free).map_err(|e| RuntimeError::ConfigError(e))?;
                return Ok(());
            } else {
                backend::clear_api_key(dptran::ApiKeyType::Free)?;
                return Ok(());
            }
        }
        backend::args::ApiSettingsTarget::ProApiKey => {
            if let Some(s) = api_setting_struct.api_key_pro {
                config.set_api_key(s, dptran::ApiKeyType::Pro).map_err(|e| RuntimeError::ConfigError(e))?;
                return Ok(());
            } else {
                backend::clear_api_key(dptran::ApiKeyType::Pro)?;
                return Ok(());
            }
        }
        backend::args::ApiSettingsTarget::ClearFreeApiKey => {
            backend::clear_api_key(dptran::ApiKeyType::Free)?;
            return Ok(());
        }
        backend::args::ApiSettingsTarget::ClearProApiKey => {
            backend::clear_api_key(dptran::ApiKeyType::Pro)?;
            return Ok(());
        }
        backend::args::ApiSettingsTarget::EndpointOfTranslation => {
            if let Some(s) = api_setting_struct.endpoint_of_translation {
                if s.len() == 0 {
                    config.reset_endpoint_of_translation().map_err(|e| RuntimeError::ConfigError(e))?;
                }
                else {
                    config.set_endpoint_of_translation(s).map_err(|e| RuntimeError::ConfigError(e))?;
                }
                return Ok(());
            } else {
                config.reset_endpoint_of_translation().map_err(|e| RuntimeError::ConfigError(e))?;
                return Ok(());
            }
        }
        backend::args::ApiSettingsTarget::EndpointOfUsage => {
            if let Some(s) = api_setting_struct.endpoint_of_usage {
                if s.len() == 0 {
                    config.reset_endpoint_of_usage().map_err(|e| RuntimeError::ConfigError(e))?;
                }
                else {
                    config.set_endpoint_of_usage(s).map_err(|e| RuntimeError::ConfigError(e))?;
                }
                return Ok(());
            } else {
                config.reset_endpoint_of_usage().map_err(|e| RuntimeError::ConfigError(e))?;
                return Ok(());
            }
        }
        backend::args::ApiSettingsTarget::EndpointOfLangs => {
            if let Some(s) = api_setting_struct.endpoint_of_langs {
                if s.len() == 0 {
                    config.reset_endpoint_of_languages().map_err(|e| RuntimeError::ConfigError(e))?;
                }
                else {
                    config.set_endpoint_of_languages(s).map_err(|e| RuntimeError::ConfigError(e))?;
                }
                return Ok(());
            } else {
                config.reset_endpoint_of_languages().map_err(|e| RuntimeError::ConfigError(e))?;
                return Ok(());
            }
        }
        backend::args::ApiSettingsTarget::EndpointOfGlossaries => {
            if let Some(s) = api_setting_struct.endpoint_of_glossaries {
                if s.len() == 0 {
                    config.reset_endpoint_of_glossaries().map_err(|e| RuntimeError::ConfigError(e))?;
                }
                else {
                    config.set_endpoint_of_glossaries(s).map_err(|e| RuntimeError::ConfigError(e))?;
                }
                return Ok(());
            } else {
                config.reset_endpoint_of_glossaries().map_err(|e| RuntimeError::ConfigError(e))?;
                return Ok(());
            }
        }
        backend::args::ApiSettingsTarget::EndpointOfGlossariesLangs => {
            if let Some(s) = api_setting_struct.endpoint_of_glossaries_langs {
                if s.len() == 0 {
                    config.reset_endpoint_of_glossaries_langs().map_err(|e| RuntimeError::ConfigError(e))?;
                }
                else {
                    config.set_endpoint_of_glossaries_langs(s).map_err(|e| RuntimeError::ConfigError(e))?;
                }
                return Ok(());
            } else {
                config.reset_endpoint_of_glossaries_langs().map_err(|e| RuntimeError::ConfigError(e))?;
                return Ok(());
            }
        }
        backend::args::ApiSettingsTarget::ClearEndpoints => {
            config.reset_endpoint_of_translation().map_err(|e| RuntimeError::ConfigError(e))?;
            config.reset_endpoint_of_usage().map_err(|e| RuntimeError::ConfigError(e))?;
            config.reset_endpoint_of_languages().map_err(|e| RuntimeError::ConfigError(e))?;
            return Ok(());
        }
        backend::args::ApiSettingsTarget::ShowSettings => {
            display_api_settings()?;
            return Ok(());
        }
        backend::args::ApiSettingsTarget::ClearSettings => {
            clear_api_settings()?;
            return Ok(());
        }
    }
}

fn handle_cache_settings(cache_setting_struct: backend::args::CacheSettingsStruct) -> Result<(), RuntimeError> {
    let cache_target = cache_setting_struct.setting_target;
    let mut config = backend::get_config()?;
    if let None = cache_target {
        return Err(RuntimeError::ArgInvalidTarget);
    }
    match cache_target.unwrap() {
        CacheSettingsTarget::EnableCache => {
            config.set_cache_enabled(true).map_err(|e| RuntimeError::ConfigError(e))?;
            return Ok(());
        }
        CacheSettingsTarget::DisableCache => {
            config.set_cache_enabled(false).map_err(|e| RuntimeError::ConfigError(e))?;
            return Ok(());
        }
        CacheSettingsTarget::MaxEntries => {
            if let Some(s) = cache_setting_struct.max_entries {
                config.set_cache_max_entries(s).map_err(|e| RuntimeError::ConfigError(e))?;
                return Ok(());
            } else {
                return Err(RuntimeError::CacheMaxEntriesIsNotSet);
            }
        }
        CacheSettingsTarget::Clear => {
            // Clear the cache
            backend::cache::get_cache_data("cache").map_err(|e| RuntimeError::CacheError(e))?
                .clear_cache().map_err(|e| RuntimeError::CacheError(e))?;
            println!("Cache has been cleared.");
        }
    }
    Ok(())
}

fn handle_glossary_settings(glossary_setting_struct: backend::args::GlossarySettingsStruct) -> Result<(), RuntimeError> {
    let glossary_setting_target = glossary_setting_struct.setting_target;
    
    // API Key confirmation
    let api_key = match backend::get_api_key()? {
        Some(api_key) => api_key,
        None => {
            print_api_key_error();
            return Ok(());
        },
    };
    let dptran = dptran::DpTran::with(&api_key.api_key, &api_key.api_key_type);
    
    match glossary_setting_target.unwrap() {
        backend::args::GlossarySettingsTarget::ShowSupportedLanguages => {
            let list = backend::get_glossary_supported_languages(&dptran)?;
            println!("Supported languages for Glossaries API (source_lang: target_lang):");
            // Print in a table format with 6 columns
            let mut i = 0;
            for lang in list.supported_languages {
                print!(" {lc:<cl$}: {ln:<lnl$}", lc=lang.source_lang.trim_matches('"'), ln=lang.target_lang.trim_matches('"'), cl=2, lnl=5);
                i += 1;
                if (i % 6) == 0 {
                    println!();
                }
            }
            return Ok(());
        },
        backend::args::GlossarySettingsTarget::CreateGlossary => {
            return backend::create_glossary(&dptran, &glossary_setting_struct.target_glossary,
                &glossary_setting_struct.add_word_pairs,
                &glossary_setting_struct
        },
    }
    
    Ok(())  /* Placeholder */
}

fn handle_show_list(list_target_langs: backend::args::ListTargetLangs) -> Result<(), RuntimeError> {
    match list_target_langs {
        backend::args::ListTargetLangs::SourceLangs => {
            show_source_language_codes()?;
        }
        backend::args::ListTargetLangs::TargetLangs => {
            show_target_language_codes()?;
        }
    }
    Ok(())
}

fn print_api_key_error() {
    println!("Welcome to dptran!");
    println!("First, please set your DeepL API-key:");
    println!("\t$ dptran api --api-key-free <YOUR-FREE-API-KEY>");
    println!("or");
    println!("\t$ dptran api --api-key-pro <YOUR-PRO-API-KEY>");
    println!();

    println!("Or, you can set it in the environment variable DPTRAN_DEEPL_API_KEY.");
    if cfg!(target_os = "windows") {
        // for Windows
        println!("\nFor Windows (PowerShell):");
        println!("\t$env:DPTRAN_DEEPL_API_KEY = \"<API_KEY>\"");
        println!("To make it persistent, use the System Environment Variables:");
        println!("\t1. Open 'System Properties' > 'Environment Variables'");
        println!("\t2. Add a new user or system variable named 'DPTRAN_DEEPL_API_KEY' with your API key.");
        println!("(Alternatively, for Command Prompt):");
        println!("\t> set DPTRAN_DEEPL_API_KEY=<API_KEY>");
        println!("Note: This is temporary and will be lost when the window is closed.");
    } else {
        // for macOS/Linux
        println!("\nFor Linux/macOS:");
        println!("\t$ export DPTRAN_DEEPL_API_KEY=<API_KEY>");
        println!("To make it persistent, add the above line to your shell config file:");
        println!("\t~/.bashrc, ~/.zshrc, or ~/.bash_profile (depending on your shell)");
    }
    println!();

    println!("If you don't have an API-key, please sign up for a free/pro account at DeepL.");
    println!("You can get DeepL API-key for free here:");
    println!("\thttps://www.deepl.com/en/pro-api?cta=header-pro-api/");
}

fn create_or_open_file(output_file: &str) -> Result<Option<std::fs::File>, RuntimeError> {
    // is the file exists?
    if std::path::Path::new(&output_file).exists() {
        print!("The file {} already exists. Overwrite? (y/N) ", output_file);
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_ascii_lowercase() != "y" {
            return Ok(None);  // Do not overwrite
        }
    }
    Ok(Some(backend::create_file(&output_file)?))
}

/// Commands prefixed with "/" in interactive mode.
#[derive(Debug, Clone, PartialEq, Eq)]
enum InteractiveCommand {
    Exit,
    SetSourceLang(String),
    SetTargetLang(String),
    GetSourceLangs,
    GetTargetLangs,
    Help,
    Continue,
}
/// Handle commands prefixed with "/" in interactive mode.
/// Returns Some(InteractiveCommand) if a command is recognized, otherwise None.
fn determine_interactive_commands_in_text(text: &str) -> Option<InteractiveCommand> {
    let trimmed_text = text.trim();
    if trimmed_text == "/exit" || trimmed_text == "/quit" {
        return Some(InteractiveCommand::Exit);
    }
    if trimmed_text.starts_with("/from") {
        let parts: Vec<&str> = trimmed_text.split_whitespace().collect();
        if parts.len() == 2 {
            return Some(InteractiveCommand::SetSourceLang(parts[1].to_string()));
        }
        else {
            println!("Invalid command. Usage: /from <language_code>");
            return None;
        }
    }
    if trimmed_text.starts_with("/to") {
        let parts: Vec<&str> = trimmed_text.split_whitespace().collect();
        if parts.len() == 2 {
            return Some(InteractiveCommand::SetTargetLang(parts[1].to_string()));
        }
        else {
            println!("Invalid command. Usage: /to <language_code>");
            return None;
        }
    }
    if trimmed_text == "/source-langs" {
        return Some(InteractiveCommand::GetSourceLangs);
    }
    if trimmed_text == "/target-langs" {
        return Some(InteractiveCommand::GetTargetLangs);
    }
    if trimmed_text == "/help" {
        return Some(InteractiveCommand::Help);
    }
    None
}

fn handle_interactive_commands_in_text(dptran: &dptran::DpTran, text: &str) -> Option<InteractiveCommand> {
    let cmd = determine_interactive_commands_in_text(text);
    if cmd.is_none() {
        return None;
    }
    let cmd = cmd.unwrap();
    match cmd {
        InteractiveCommand::Exit => {
            return Some(InteractiveCommand::Exit);
        }
        InteractiveCommand::SetSourceLang(lang_code) => {
            let corrected_lang_code = dptran.correct_source_language_code(&lang_code);
            if let Err(e) = &corrected_lang_code {
                println!("Invalid source language code: {}.", e.to_string());
                return Some(InteractiveCommand::Continue);    // Continue the interactive mode
            }
            else if let Ok(corrected_lang_code) = corrected_lang_code {
                println!("Source language set to {}.", corrected_lang_code);
                return Some(InteractiveCommand::SetSourceLang(corrected_lang_code)); // Continue the interactive mode
            }
        }
        InteractiveCommand::SetTargetLang(lang_code) => {
            let corrected_lang_code = dptran.correct_target_language_code(&lang_code);
            if let Err(e) = &corrected_lang_code {
                println!("Invalid target language code: {}.", e.to_string());
                return Some(InteractiveCommand::Continue);    // Continue the interactive mode
            }
            else if let Ok(corrected_lang_code) = corrected_lang_code {
                println!("Target language set to {}.", corrected_lang_code);
                return Some(InteractiveCommand::SetTargetLang(corrected_lang_code)); // Continue the interactive mode
            }
        }
        InteractiveCommand::GetSourceLangs => {
            if let Err(e) = show_source_language_codes() {
                println!("Could not retrieve source language codes: {}.", e.to_string());
            }
            return Some(InteractiveCommand::Continue);    // Continue the interactive mode
        }
        InteractiveCommand::GetTargetLangs => {
            if let Err(e) = show_target_language_codes() {
                println!("Could not retrieve target language codes: {}.", e.to_string());
            }
            return Some(InteractiveCommand::Continue);    // Continue the interactive mode
        }
        InteractiveCommand::Help => {
            println!("Available commands:");
            println!("/exit or /quit               : Exit dptran");
            println!("/from <language_code>        : Change the source language");
            println!("/to <language_code>          : Change the target language");
            println!("/source-langs                : Show available source language codes");
            println!("/target-langs                : Show available target language codes");
            println!("/help                        : Show this help message");
            return Some(InteractiveCommand::Continue);    // Continue the interactive mode
        }
        InteractiveCommand::Continue => {
            return Some(InteractiveCommand::Continue);    // Continue the interactive mode
        }
    }
    None
}

/// Core function to handle the translation process.
/// If in interactive mode, it will loop until "/quit" or "/exit" is entered.
/// In normal mode, it will exit once after translation.
/// Returns true if it continues the interactive mode, false if it exits.
fn do_translation(dptran: &dptran::DpTran, mode: ExecutionMode, source_lang: &Option<String>, target_lang: &String, 
                    multilines: bool, rm_line_breaks: bool, text: &Option<String>, mut ofile: &Option<std::fs::File>) -> Result<InteractiveCommand, RuntimeError> {
    // If in interactive mode, get from standard input
    // In normal mode, get from argument
    let input = get_input(&mode, multilines, rm_line_breaks, &text);
    if input.is_none() {
        return Err(RuntimeError::DeeplApiError(DpTranError::CouldNotGetInputText));
    }
    let input = input.unwrap();

    // Interactive mode: "/exit" or "/quit" to exit
    if mode == ExecutionMode::TranslateInteractive {
        if input.len() == 0 {
            return Ok(InteractiveCommand::Continue);
        }
        if input[0].starts_with("/") {
            if let Some(cmd) = handle_interactive_commands_in_text(dptran, &input[0]) {
                return Ok(cmd);
            } else {
                println!("Invalid command. To check available commands, type \"/help\".");
                return Ok(InteractiveCommand::Continue);    // Continue the interactive mode
            }
        }
        if input[0].clone().trim_end().is_empty() {
            return Ok(InteractiveCommand::Continue);    // Continue the interactive mode
        }
    }

    // Check the cache
    let cache_result = backend::search_cache(&input, &source_lang, &target_lang)?;
    let translated_texts = if let Some(cached_text) = cache_result {
        vec![cached_text]
    // If not in cache, translate and store in cache
    } else {
        // translate
        let result = dptran.translate(&input, &target_lang, &source_lang)
            .map_err(|e| RuntimeError::DeeplApiError(e))?;
        // replace \" with "
        let result = result.iter().map(|x| x.replace(r#"\""#, "\"")).collect::<Vec<String>>();
        // store in cache
        if backend::get_config()?.get_cache_enabled().map_err(|e| RuntimeError::ConfigError(e))? {
            backend::into_cache(&input, &result, &source_lang, &target_lang)?;
        }
        result
    };
    for translated_text in translated_texts {
        let formatted_text = backend::format_translation_result(&translated_text);
        if let Some(ofile) = &mut ofile {
            // append to the file
            backend::append_to_file(ofile, &formatted_text)?;
            if mode == ExecutionMode::TranslateInteractive {
                println!("{}", formatted_text);
            }
        } else {
            // print to stdout
            println!("{}", formatted_text);
        }
    }

    // In normal mode, exit the loop once.
    if mode == ExecutionMode::TranslateNormal {
        return Ok(InteractiveCommand::Exit);   // Exit the normal mode
    }

    Ok(InteractiveCommand::Continue)    // Continue the interactive mode
}

/// Dialogue and Translation.
/// Repeat input if in interactive mode
/// In normal mode, it will be finished once
fn translation_loop(dptran: &dptran::DpTran, mode: ExecutionMode, source_lang: Option<String>, target_lang: String, 
            multilines: bool, rm_line_breaks: bool, text: Option<String>, ofile: &Option<std::fs::File>) -> Result<(), RuntimeError> {
    // Translation
    // loop if in interactive mode; exit once in normal mode

    // If it is interactive mode, it shows how to exit.
    if mode == ExecutionMode::TranslateInteractive {
        if source_lang.is_none() {
            println!("Now translating from detected language to {}.", target_lang);
        } else {
            println!("Now translating from {} to {}.", source_lang.as_ref().unwrap(), target_lang);
        }
        if multilines {
            println!("Multiline mode: Enter a blank line to send the input.");
        }
        println!("Type \"/exit\" or \"/quit\" to exit dptran.");
        println!("Type \"/from <language_code>\" to change the source language and \"/to <language_code>\" to change the target language.");
        println!("To see more commands, type \"/help\".");
    }

    let mut source_lang = source_lang;
    let mut target_lang = target_lang;

    loop {
        match do_translation(dptran, mode, &source_lang, &target_lang, multilines, rm_line_breaks, &text, ofile) {
            Ok(continue_mode) => {
                match continue_mode {
                    InteractiveCommand::Continue => {
                        // Continue the interactive mode
                    }
                    InteractiveCommand::Exit => {
                        // Exit the interactive mode
                        break;
                    }
                    InteractiveCommand::SetSourceLang(new_source_lang) => {
                        // Change source language
                        source_lang = Some(new_source_lang);
                    }
                    InteractiveCommand::SetTargetLang(new_target_lang) => {
                        // Change target language
                        target_lang = new_target_lang;
                    }
                    _ => {
                        // Should not reach here
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(())
}

/// Start translation process.
fn handle_translation(mode: ExecutionMode, translate_from: Option<String>, translate_to: Option<String>, 
                            multilines: bool, remove_line_breaks: bool, source_text: Option<String>, ofile_path: Option<String>) -> Result<(), RuntimeError> {
    let mut source_lang = translate_from;
    let mut target_lang = translate_to;

    if target_lang.is_none() {
        let config = backend::get_config()?;
        target_lang = Some(config.get_default_target_language_code().map_err(|e| RuntimeError::ConfigError(e))?);
    }

    // API Key confirmation
    let api_key = match backend::get_api_key()? {
        Some(api_key) => api_key,
        None => {
            print_api_key_error();
            return Ok(());
        },
    };

    let mut dptran = dptran::DpTran::with(&api_key.api_key, &api_key.api_key_type);

    // Reflect the endpoint settings in the configuration file.
    backend::reflect_endpoints(&mut dptran)?;

    // Language code check and correction
    if let Some(sl) = source_lang {
        source_lang = Some(dptran.correct_source_language_code(&sl.to_string()).map_err(|e| RuntimeError::DeeplApiError(e))?);
    }
    if let Some(tl) = target_lang {
        target_lang = Some(dptran.correct_target_language_code(&tl.to_string()).map_err(|e| RuntimeError::DeeplApiError(e))?);
    }

    // Output filepath
    // If output file is specified, it will be created or overwritten.
    let ofile = if let Some(output_file) = ofile_path {
        create_or_open_file(&output_file)?  
    }
    else {
        None
    };

    // (Dialogue &) Translation
    translation_loop(&dptran, mode, source_lang, target_lang.unwrap(), 
            multilines, remove_line_breaks, source_text, &ofile)?;

    Ok(())
}

/// Obtaining arguments and calling the translation process
fn main() -> Result<(), RuntimeError> {
    // Parsing arguments.
    let arg_struct = backend::args::parser()?;
    let mode = arg_struct.execution_mode;
    match mode {
        ExecutionMode::PrintUsage => {
            handle_show_usage()?;
            return Ok(());
        }
        ExecutionMode::GeneralSettings => {
            // Handle settings
            handle_general_settings(arg_struct.general_setting.unwrap())?;
            return Ok(());
        }
        ExecutionMode::ApiSettings => {
            // Handle API settings
            handle_api_settings(arg_struct.api_setting.unwrap())?;
            return Ok(());
        }
        ExecutionMode::CacheSettings => {
            // Handle cache settings
            handle_cache_settings(arg_struct.cache_setting.unwrap())?;
            return Ok(());
        }
        ExecutionMode::GlossarySettings => {
            // Handle glossary settings
            handle_glossary_settings(arg_struct.glossary_setting.unwrap())?;
            return Ok(());
        }
        ExecutionMode::List => {
            // Handle list of language codes
            handle_show_list(arg_struct.list_target_langs.unwrap())?;
            return Ok(());
        }
        ExecutionMode::TranslateNormal | ExecutionMode::TranslateInteractive => {
            handle_translation(mode, arg_struct.translate_from, 
                                arg_struct.translate_to, 
                                arg_struct.multilines, 
                                arg_struct.remove_line_breaks, 
                                arg_struct.source_text, 
                                arg_struct.ofile_path)?;
            return Ok(());
        }
    };
}

/// To run these tests, you need to set the environment variable `DPTRAN_DEEPL_API_KEY` to your DeepL API key.  
/// You should run these tests with ``cargo test -- --test-threads=1`` because the DeepL API has a limit on the number of requests per second.  
/// And also, you need to run the dummy server for the DeepL API to test the API endpoints.
///   $ pip3 install -r requirements.txt
///   $ uvicorn dummy_api_server.main:app --reload
#[cfg(test)]
mod func_tests {
    use super::*;

    fn retry_or_panic(e: &RuntimeError, times: u8) -> bool {
        if e == &RuntimeError::DeeplApiError(DpTranError::DeeplApiError(dptran::DeeplAPIError::ConnectionError(dptran::ConnectionError::TooManyRequests))) && times < 3 {
            // Because the DeepL API has a limit on the number of requests per second, retry after 2 seconds if the error is TooManyRequests.
            std::thread::sleep(std::time::Duration::from_secs(2));
            return true;
        }
        else {
            panic!("Error: {}", e.to_string());
        }
    }

    fn do_app_deeplapi_show_source_language_codes_test(times: u8) {
        let result = show_source_language_codes();
        if let Err(e) = &result {
            if retry_or_panic(e, 1) {
                return do_app_deeplapi_show_source_language_codes_test(times + 1);
            }
        }
        assert!(result.is_ok());
    }

    fn do_app_deeplapi_show_target_language_codes_test(times: u8) {
        let result = show_target_language_codes();
        if let Err(e) = &result {
            if retry_or_panic(e, 1) {
                return do_app_deeplapi_show_target_language_codes_test(times + 1);
            }
        }
        assert!(result.is_ok());
    }

    fn do_app_deeplapi_show_usage_test(times: u8) {
        let result = handle_show_usage();
        if let Err(e) = &result {
            if retry_or_panic(e, 1) {
                return do_app_deeplapi_show_usage_test(times + 1);
            }
        }
        assert!(result.is_ok());
    }

    fn do_app_deeplapi_process_test(times: u8) {
        let api_key = backend::get_api_key().unwrap().unwrap();
        let dptran = dptran::DpTran::with(&api_key.api_key, &api_key.api_key_type);
        let mode = ExecutionMode::TranslateNormal;
        let multilines = false;
        let rm_line_breaks = false;
        let text = Some("Hello, world!".to_string());
        let source_lang = Some("en".to_string());
        let target_lang = "fr".to_string();
        let ofile = None;
        let result = translation_loop(&dptran, mode, source_lang, target_lang, multilines, rm_line_breaks, text, &ofile);
        if let Err(e) = &result {
            if retry_or_panic(e, 1) {
                return do_app_deeplapi_process_test(times + 1);
            }
        }
        assert!(result.is_ok());
    }

    #[test]
    fn app_impl_get_langcodes_maxlen_test() {
        let lang_codes = vec![
            ("en".to_string(), "English".to_string()),
            ("fr".to_string(), "French".to_string()),
            ("de".to_string(), "German".to_string()),
        ];
        let (len, max_code_len, max_str_len) = get_langcodes_maxlen(&lang_codes);
        assert_eq!(len, 3);
        assert_eq!(max_code_len, 2);
        assert_eq!(max_str_len, 7);
    }

    #[test]
    fn app_impl_display_general_settings_test() {
        let result = display_general_settings();
        assert!(result.is_ok());
    }

    #[test]
    fn app_impl_display_api_settings_test() {
        let result = display_api_settings();
        assert!(result.is_ok());
    }

    #[test]
    fn app_deeplapi_show_source_language_codes_test() {
        do_app_deeplapi_show_source_language_codes_test(0);
    }

    #[test]
    fn app_deeplapi_show_target_language_codes_test() {
        do_app_deeplapi_show_target_language_codes_test(0);
    }

    #[test]
    fn app_deeplapi_show_usage_test() {
        do_app_deeplapi_show_usage_test(0);
    }

    #[test]
    fn app_impl_get_input_test() {
        let mode = ExecutionMode::TranslateNormal;
        let multilines = false;
        let rm_line_breaks = false;
        let text = "Hello, world!".to_string();

        let result = get_input(&mode, multilines, rm_line_breaks, &Some(text));
        assert!(result.is_some());
    }

    #[test]
    fn app_deeplapi_process_test() {
        do_app_deeplapi_process_test(0);
    }
}

#[cfg(test)]
mod runtime_tests {
    use std::{io::Write, process::Command, process::Stdio};

    fn reset_general_settings() {
        // Reset configuration.
        let mut cmd = Command::new("cargo");
        let text = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("config")
            .arg("--clear-all")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn();
        
        let mut child = text.unwrap();
        std::thread::sleep(std::time::Duration::from_secs(2));
        let input = "y\n"; // Confirm clearing settings
        if let Some(stdin) = child.stdin.as_mut() {
            stdin.write_all(input.as_bytes()).unwrap();
        }
        let output = child.wait_with_output().unwrap();
        if !output.status.success() {
            panic!("Failed to reset settings: {}", String::from_utf8_lossy(&output.stderr));
        }
    }

    fn reset_api_settings() {
        // Reset configuration.
        let mut cmd = Command::new("cargo");
        let text = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("api")
            .arg("--clear-all")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn();
        
        let mut child = text.unwrap();
        std::thread::sleep(std::time::Duration::from_secs(2));
        let input = "y\n"; // Confirm clearing settings
        if let Some(stdin) = child.stdin.as_mut() {
            stdin.write_all(input.as_bytes()).unwrap();
        }
        let output = child.wait_with_output().unwrap();
        if !output.status.success() {
            panic!("Failed to reset API settings: {}", String::from_utf8_lossy(&output.stderr));
        }
    }

    #[test]
    fn runtime_deeplapi_test() {
        // Reset configuration.
        reset_general_settings();
        reset_api_settings();
        
        let mut cmd = Command::new("cargo");
        std::thread::sleep(std::time::Duration::from_secs(2));
        let text = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("Hello, world!")
            .arg("-t")
            .arg("ja")
            .output();

        assert!(text.is_ok());
        let text = text.unwrap();
        if text.status.success() != true {
            panic!("Error: {}", String::from_utf8_lossy(&text.stderr));
        }
        assert!(text.stdout != b"Hello\n");

        let mut cmd = Command::new("cargo");
        std::thread::sleep(std::time::Duration::from_secs(2));
        let text = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("Hello")
            .arg("-t")
            .arg("en")
            .output();

        assert!(text.is_ok());
        let text = text.unwrap();
        if text.status.success() != true {
            panic!("Error: {}", String::from_utf8_lossy(&text.stderr));
        }
        assert!(text.stdout == b"Hello\n");
    }

    #[test]
    fn runtime_deeplapi_with_file_test() {
        // Reset configuration.
        reset_general_settings();
        reset_api_settings();

        let mut cmd = Command::new("cargo");
        std::thread::sleep(std::time::Duration::from_secs(2));
        let text = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("-t")
            .arg("en")
            .arg("Hello")
            .arg("-o")
            .arg("test.txt")
            .output();

        assert!(text.is_ok());
        let text = text.unwrap();
        if text.status.success() != true {
            panic!("Error: {}", String::from_utf8_lossy(&text.stderr));
        }

        // Check if the file exists
        let file_path = std::path::Path::new("test.txt");
        assert!(file_path.exists(), "File test.txt does not exist.");
        // Check if the file is not empty
        let metadata = std::fs::metadata(file_path).unwrap();
        assert!(metadata.len() > 0, "File test.txt is empty.");
        let file_content = std::fs::read_to_string(file_path).unwrap();
        assert!(file_content.contains("Hello"), "File test.txt does not contain the expected content.");
        // Clean up the file
        std::fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn runtime_deeplapi_with_cache_test() {
        // Reset configuration.
        reset_general_settings();
        reset_api_settings();

        // 1st run..
        let mut cmd = Command::new("cargo");
        std::thread::sleep(std::time::Duration::from_secs(2));
        let text = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("Hello, world!")
            .arg("-t")
            .arg("ja")
            .output();
        let text = text.unwrap();
        if text.status.success() != true {
            panic!("Error: {}", String::from_utf8_lossy(&text.stderr));
        }

        // Get usage..
        let mut cmd = Command::new("cargo");
        std::thread::sleep(std::time::Duration::from_secs(2));
        let usage = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("-u")
            .output();
        let usage = usage.unwrap();
        if usage.status.success() != true {
            panic!("Error: {}", String::from_utf8_lossy(&usage.stderr));
        }
        // to u32
        let usage_before = usage.stdout
            .split(|&b| b == b' ')
            .filter_map(|s| std::str::from_utf8(s).ok())
            .filter_map(|s| s.parse::<u32>().ok())
            .next()
            .unwrap();

        // 2nd run.
        std::thread::sleep(std::time::Duration::from_secs(2));
        let mut cmd = Command::new("cargo");
        let text = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("Hello, world!")
            .arg("-t")
            .arg("ja")
            .output();
        let text = text.unwrap();
        if text.status.success() != true {
            panic!("Error: {}", String::from_utf8_lossy(&text.stderr));
        }

        // Get usage once again.
        std::thread::sleep(std::time::Duration::from_secs(2));
        let mut cmd = Command::new("cargo");
        let usage = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("-u")
            .output();
        let usage = usage.unwrap();
        if usage.status.success() != true {
            panic!("Error: {}", String::from_utf8_lossy(&usage.stderr));
        }

        // to u32
        let usage_after = usage.stdout
            .split(|&b| b == b' ')
            .filter_map(|s| std::str::from_utf8(s).ok())
            .filter_map(|s| s.parse::<u32>().ok())
            .next()
            .unwrap();

        // Check if the usage has not changed.
        assert!(usage_after == usage_before);
    }

    /// Test for the interactive mode.
    #[test]
    fn runtime_deeplapi_interactive_mode_test() {
        // Reset configuration.
        reset_general_settings();
        reset_api_settings();

        let mut cmd = Command::new("cargo");
        std::thread::sleep(std::time::Duration::from_secs(2));
        let text = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("-t")
            .arg("en")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn();

        let mut child = text.unwrap();
        std::thread::sleep(std::time::Duration::from_secs(2));
        let input = "Hello, world!\n/quit\n";
        let output = child.stdin.as_mut().unwrap().write_all(input.as_bytes());
        if let Err(e) = output {
            panic!("Error: {}", e);
        }
        let output = child.wait_with_output();
        if let Err(e) = output {
            panic!("Error: {}", e);
        }
        let output = output.unwrap();
        if output.status.success() != true {
            panic!("Error: {}", String::from_utf8_lossy(&output.stderr));
        }
        // Check if the output contains "Hello, world!"
        let output_str = String::from_utf8_lossy(&output.stdout);
        assert!(output_str.contains("Hello, world!"), "Output does not contain the expected text.");
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn runtime_deeplapi_from_pipe_test() {
        // Reset configuration.
        reset_general_settings();
        reset_api_settings();

        std::thread::sleep(std::time::Duration::from_secs(2));
        let mut echo_cmd = Command::new("echo")
            .arg("Hello, world!")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start echo command");

        let dptran_cmd = Command::new("cargo")
            .arg("run")
            .arg("--release")
            .arg("--")
            .arg("-t")
            .arg("en")
            .stdin(Stdio::from(echo_cmd.stdout.take().unwrap()))
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start dptran command");
        
        let output = dptran_cmd.wait_with_output().expect("Failed to read dptran output");
        if output.status.success() != true {
            panic!("Error: {}", String::from_utf8_lossy(&output.stderr));
        }
        // Check if the output contains "Hello, world!"
        let output_str = String::from_utf8_lossy(&output.stdout);
        assert!(output_str.contains("Hello, world!"), "Output does not contain the expected text.");
    }

    #[test]
    fn runtime_impl_change_endpoints_test() {
        // Reset configuration.
        reset_general_settings();
        reset_api_settings();

        // Set the endpoints to the real DeepL API endpoints.
        let mut cmd = Command::new("cargo");
        let _ = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("api")
            .arg("--endpoint-of-translation")
            .arg("http://localhost:8000/free/v2/translate")
            .output();
        
        let mut cmd = Command::new("cargo");
        let _ = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("api")
            .arg("--endpoint-of-usage")
            .arg("http://localhost:8000/free/v2/usage")
            .output();

        let mut cmd = Command::new("cargo");
        let _ = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("api")
            .arg("--endpoint-of-langs")
            .arg("http://localhost:8000/free/v2/languages")
            .output();

        let mut cmd = Command::new("cargo");
        let _ = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("api")
            .arg("--endpoint-of-glossaries")
            .arg("http://localhost:8000/free/v2/glossaries")
            .output();

        let mut cmd = Command::new("cargo");
        let _ = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("api")
            .arg("--endpoint-of-glossaries-langs")
            .arg("http://localhost:8000/free/v2/glossary-language-pairs")
            .output();

        // Now, test the translation with the changed endpoints.
        let mut cmd = Command::new("cargo");
        std::thread::sleep(std::time::Duration::from_secs(2));
        let text = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("Hello, world!")
            .arg("-t")
            .arg("ja")
            .output();
        
        assert!(text.is_ok());
        let text = text.unwrap();
        if text.status.success() != true {
            panic!("Error: {}", String::from_utf8_lossy(&text.stderr));
        }
    }

    #[test]
    fn runtime_impl_change_endpoints_and_clear_test() {
        // Reset configuration.
        reset_general_settings();
        reset_api_settings();

        // Set the endpoints to the real DeepL API endpoints.
        let mut cmd = Command::new("cargo");
        let _ = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("api")
            .arg("--endpoint-of-translation")
            .arg("http://localhost:8000/free/v2/translate")
            .output();

        let mut cmd = Command::new("cargo");
        let _ = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("api")
            .arg("--clear-endpoints")
            .output();

        // Check the settings
        let mut cmd = Command::new("cargo");
        std::thread::sleep(std::time::Duration::from_secs(2));
        let text = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("api")
            .arg("-s")
            .output();

        assert!(text.is_ok());
        // If there is 'http://localhost:8000/free/v2/translate' in the output, the test fails.
        let text = text.unwrap();
        if text.status.success() != true {
            panic!("Error: {}", String::from_utf8_lossy(&text.stderr));
        }
        let output_str = String::from_utf8_lossy(&text.stdout);
        assert!(!output_str.contains("http://localhost:8000/free/v2/translate"), "Endpoints were not cleared.");
    }

    #[test]
    fn runtime_impl_config_helper_test() {
        let mut cmd = Command::new("cargo");
        std::thread::sleep(std::time::Duration::from_secs(2));
        let text = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("--help")
            .output();
        assert!(text.is_ok());
        let text = text.unwrap();
        if text.status.success() != true {
            panic!("Error: {}", String::from_utf8_lossy(&text.stderr));
        }

        let mut cmd = Command::new("cargo");
        std::thread::sleep(std::time::Duration::from_secs(2));
        let text = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("list")
            .arg("--help")
            .output();
        assert!(text.is_ok());
        let text = text.unwrap();
        if text.status.success() != true {
            panic!("Error: {}", String::from_utf8_lossy(&text.stderr));
        }

        let mut cmd = Command::new("cargo");
        std::thread::sleep(std::time::Duration::from_secs(2));
        let text = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("config")
            .arg("--help")
            .output();
        assert!(text.is_ok());
        let text = text.unwrap();
        if text.status.success() != true {
            panic!("Error: {}", String::from_utf8_lossy(&text.stderr));
        }

        let mut cmd = Command::new("cargo");
        std::thread::sleep(std::time::Duration::from_secs(2));
        let text = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("api")
            .arg("--help")
            .output();
        assert!(text.is_ok());
        let text = text.unwrap();
        if text.status.success() != true {
            panic!("Error: {}", String::from_utf8_lossy(&text.stderr));
        }

        let mut cmd = Command::new("cargo");
        std::thread::sleep(std::time::Duration::from_secs(2));
        let text = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("cache")
            .arg("--help")
            .output();
        assert!(text.is_ok());
        let text = text.unwrap();
        if text.status.success() != true {
            panic!("Error: {}", String::from_utf8_lossy(&text.stderr));
        }
    }
}
