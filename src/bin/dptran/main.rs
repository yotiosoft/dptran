use std::io::{Write, stdin, stdout};

mod backend;
use backend::RuntimeError;
use backend::ExecutionMode;

use dptran::{DpTranError, LangType};
use crate::backend::parse::CacheTarget;
use crate::backend::parse::SettingTarget;

/// Initialization of settings.
fn clear_settings() -> Result<(), RuntimeError> {
    print!("Are you sure you want to clear all settings? (y/N) ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    // Initialize settings when y is entered.
    if input.trim().to_ascii_lowercase() == "y" {
        let mut config = backend::configure::ConfigureWrapper::get("configure").map_err(|e| RuntimeError::ConfigError(e))?;
        config.clear_settings().map_err(|e| RuntimeError::ConfigError(e))?;
        println!("All settings have been cleared.");
        println!("Note: You need to set the API key again to use dptran.");
    }
    Ok(())
}

/// Display of settings.
fn display_settings() -> Result<(), RuntimeError> {
    let config = backend::configure::ConfigureWrapper::get("configure").map_err(|e| RuntimeError::ConfigError(e))?;
    let api_key = backend::get_api_key()?;
    let default_target_lang = config.get_default_target_language_code().map_err(|e| RuntimeError::ConfigError(e))?;
    let cache_max_entries = config.get_cache_max_entries().map_err(|e| RuntimeError::ConfigError(e))?;
    let editor_command = config.get_editor_command().map_err(|e| RuntimeError::ConfigError(e))?;
    let cache_enabled = config.get_cache_enabled().map_err(|e| RuntimeError::ConfigError(e))?;

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

/// Display list of source language codes.
/// Retrieved from <https://api-free.deepl.com/v2/languages>
fn show_source_language_codes() -> Result<(), RuntimeError> {
    let api_key = match backend::get_api_key()? {
        Some(api_key) => api_key,
        None => return Err(RuntimeError::DeeplApiError(DpTranError::ApiKeyIsNotSet)),
    };
    let dptran = dptran::DpTran::with(&api_key.api_key, &api_key.api_key_type);

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
    let dptran = dptran::DpTran::with(&api_key.api_key, &api_key.api_key_type);

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
pub fn handle_show_usage() -> Result<(), RuntimeError> {
    let usage = backend::get_usage()?;
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
                if input.trim_end() == "quit" || input.trim_end() == "exit" {
                    input_vec.push(input);
                    break;
                }

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

fn handle_general_settings(setting_struct: backend::parse::ArgSettingStruct) -> Result<(), RuntimeError> {
    let setting_target = setting_struct.setting_target.clone();
    if let None = setting_target {
        return Err(RuntimeError::ArgInvalidTarget);
    }
    let mut config = backend::configure::ConfigureWrapper::get("configure").map_err(|e| RuntimeError::ConfigError(e))?;
    match setting_target.unwrap() {
        SettingTarget::FreeApiKey => {
            if let Some(s) = setting_struct.api_key {
                config.set_api_key(s, dptran::ApiKeyType::Free).map_err(|e| RuntimeError::ConfigError(e))?;
                return Ok(());
            } else {
                backend::clear_api_key(dptran::ApiKeyType::Free)?;
                return Ok(());
            }
        }
        SettingTarget::ProApiKey => {
            if let Some(s) = setting_struct.api_key {
                config.set_api_key(s, dptran::ApiKeyType::Pro).map_err(|e| RuntimeError::ConfigError(e))?;
                return Ok(());
            } else {
                backend::clear_api_key(dptran::ApiKeyType::Pro)?;
                return Ok(());
            }
        }
        SettingTarget::DefaultTargetLang => {
            if let Some(s) = setting_struct.default_target_lang {
                let validated_language_code = backend::set_default_target_language(&s)?;
                println!("Default target language has been set to {}.", validated_language_code);
                return Ok(());
            } else {
                return Err(RuntimeError::DeeplApiError(DpTranError::NoTargetLanguageSpecified));
            }
        }
        SettingTarget::EditorCommand => {
            if let Some(s) = setting_struct.editor_command {
                config.set_editor_command(s).map_err(|e| RuntimeError::ConfigError(e))?;
                return Ok(());
            } else {
                return Err(RuntimeError::EditorCommandIsNotSet);
            }
        }
        SettingTarget::EnableCache => {
            config.set_cache_enabled(true).map_err(|e| RuntimeError::ConfigError(e))?;
            return Ok(());
        }
        SettingTarget::DisableCache => {
            config.set_cache_enabled(false).map_err(|e| RuntimeError::ConfigError(e))?;
            return Ok(());
        }
        SettingTarget::EndpointOfTranslation => {
            if let Some(s) = setting_struct.endpoint_of_translation {
                config.set_endpoint_of_translation(s).map_err(|e| RuntimeError::ConfigError(e))?;
                return Ok(());
            } else {
                config.reset_endpoint_of_translation().map_err(|e| RuntimeError::ConfigError(e))?;
                println!("Endpoint of translation has been reset to the default value.");
                return Ok(());
            }
        }
        SettingTarget::EndpointOfUsage => {
            if let Some(s) = setting_struct.endpoint_of_usage {
                config.set_endpoint_of_usage(s).map_err(|e| RuntimeError::ConfigError(e))?;
                return Ok(());
            } else {
                config.reset_endpoint_of_usage().map_err(|e| RuntimeError::ConfigError(e))?;
                println!("Endpoint of usage has been reset to the default value.");
                return Ok(());
            }
        }
        SettingTarget::EndpointOfLangs => {
            if let Some(s) = setting_struct.endpoint_of_langs {
                config.set_endpoint_of_languages(s).map_err(|e| RuntimeError::ConfigError(e))?;
                return Ok(());
            } else {
                config.reset_endpoint_of_languages().map_err(|e| RuntimeError::ConfigError(e))?;
                println!("Endpoint of languages has been reset to the default value.");
                return Ok(());
            }
        }
        SettingTarget::DisplaySettings => {
            display_settings()?;
            return Ok(());
        }
        SettingTarget::ClearSettings => {
            clear_settings()?;
            return Ok(());
        }
    }
}

fn handle_cache_settings(cache_setting_struct: backend::parse::CacheSettingStruct) -> Result<(), RuntimeError> {
    let cache_target = cache_setting_struct.cache_target;
    if let None = cache_target {
        return Err(RuntimeError::ArgInvalidTarget);
    }
    match cache_target.unwrap() {
        CacheTarget::MaxEntries => {
            if let Some(s) = cache_setting_struct.max_entries {
                backend::configure::ConfigureWrapper::get("configure").map_err(|e| RuntimeError::ConfigError(e))?
                    .set_cache_max_entries(s).map_err(|e| RuntimeError::ConfigError(e))?;
                return Ok(());
            } else {
                return Err(RuntimeError::CacheMaxEntriesIsNotSet);
            }
        }
        CacheTarget::Clear => {
            // Clear the cache
            backend::cache::get_cache_data("cache").map_err(|e| RuntimeError::CacheError(e))?
                .clear_cache().map_err(|e| RuntimeError::CacheError(e))?;
            println!("Cache has been cleared.");
        }
    }
    Ok(())
}

fn handle_show_list(list_target_langs: backend::parse::ListTargetLangs) -> Result<(), RuntimeError> {
    match list_target_langs {
        backend::parse::ListTargetLangs::SourceLangs => {
            show_source_language_codes()?;
        }
        backend::parse::ListTargetLangs::TargetLangs => {
            show_target_language_codes()?;
        }
    }
    Ok(())
}

fn print_api_key_error() {
    println!("Welcome to dptran!");
    println!("First, please set your DeepL API-key:");
    println!("\t$ dptran set --api-key <API_KEY>");
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

/// Core function to handle the translation process.
/// If in interactive mode, it will loop until "quit" or "exit" is entered.
/// In normal mode, it will exit once after translation.
/// Returns true if it continues the interactive mode, false if it exits.
fn do_translation(dptran: &dptran::DpTran, mode: ExecutionMode, source_lang: Option<String>, target_lang: String, 
                    multilines: bool, rm_line_breaks: bool, text: Option<String>, mut ofile: &Option<std::fs::File>) -> Result<bool, RuntimeError> {
    // If in interactive mode, get from standard input
    // In normal mode, get from argument
    let input = get_input(&mode, multilines, rm_line_breaks, &text);
    if input.is_none() {
        return Err(RuntimeError::DeeplApiError(DpTranError::CouldNotGetInputText));
    }
    let input = input.unwrap();

    // Interactive mode: "exit" or "quit" to exit
    if mode == ExecutionMode::TranslateInteractive {
        if input.len() == 0 {
            return Ok(true);    // Continue the interactive mode
        }
        if input[0].trim_end() == "quit" || input[0].trim_end() == "exit" {
            return Ok(false);   // Exit the interactive mode
        }
        if input[0].clone().trim_end().is_empty() {
            return Ok(true);    // Continue the interactive mode
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
        return Ok(false);   // Exit the normal mode
    }
    
    Ok(true)    // Continue the interactive mode
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
        println!("Type \"exit\" or \"quit\" to exit dptran.");
    }

    loop {
        if let Ok(false) = do_translation(dptran, mode, source_lang.clone(), target_lang.clone(), 
                                        multilines, rm_line_breaks, text.clone(), &ofile) {
            break;  // Exit the loop if in normal mode or if "quit" or "exit" is entered in interactive mode
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
    let dptran = dptran::DpTran::with(&api_key.api_key, &api_key.api_key_type);

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
    let arg_struct = backend::parse::parser()?;
    let mode = arg_struct.execution_mode;
    match mode {
        ExecutionMode::PrintUsage => {
            handle_show_usage()?;
            return Ok(());
        }
        ExecutionMode::Setting => {
            // Handle settings
            handle_general_settings(arg_struct.setting.unwrap())?;
            return Ok(());
        }
        ExecutionMode::Cache => {
            // Handle cache settings
            handle_cache_settings(arg_struct.cache_setting.unwrap())?;
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
///   $ uvicorn dummy-api-server:app --reload
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

    fn impl_app_show_source_language_codes_test(times: u8) {
        let result = show_source_language_codes();
        if let Err(e) = &result {
            if retry_or_panic(e, 1) {
                return impl_app_show_source_language_codes_test(times + 1);
            }
        }
        assert!(result.is_ok());
    }

    fn impl_app_show_target_language_codes_test(times: u8) {
        let result = show_target_language_codes();
        if let Err(e) = &result {
            if retry_or_panic(e, 1) {
                return impl_app_show_target_language_codes_test(times + 1);
            }
        }
        assert!(result.is_ok());
    }

    fn impl_app_show_usage_test(times: u8) {
        let result = handle_show_usage();
        if let Err(e) = &result {
            if retry_or_panic(e, 1) {
                return impl_app_show_usage_test(times + 1);
            }
        }
        assert!(result.is_ok());
    }

    fn impl_app_process_test(times: u8) {
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
                return impl_app_process_test(times + 1);
            }
        }
        assert!(result.is_ok());
    }

    #[test]
    fn app_get_langcodes_maxlen_test() {
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
    fn app_display_settings_test() {
        let result = display_settings();
        assert!(result.is_ok());
    }

    #[test]
    fn app_show_source_language_codes_test() {
        impl_app_show_source_language_codes_test(0);
    }

    #[test]
    fn app_show_target_language_codes_test() {
        impl_app_show_target_language_codes_test(0);
    }

    #[test]
    fn app_show_usage_test() {
        impl_app_show_usage_test(0);
    }

    #[test]
    fn app_get_input_test() {
        let mode = ExecutionMode::TranslateNormal;
        let multilines = false;
        let rm_line_breaks = false;
        let text = "Hello, world!".to_string();

        let result = get_input(&mode, multilines, rm_line_breaks, &Some(text));
        assert!(result.is_some());
    }

    #[test]
    fn app_process_test() {
        impl_app_process_test(0);
    }
}

#[cfg(test)]
mod runtime_tests {
    use std::{io::Write, process::Command, process::Stdio};

    #[test]
    fn runtime_test() {
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
    fn runtime_with_file_test() {
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
    fn runtime_with_cache_test() {
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
    fn runtime_interactive_mode_test() {
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
        let input = "Hello, world!\nquit\n";
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
    fn runtime_from_pipe_test() {
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
}
