use std::io::{self, Write, stdin, stdout};

mod backend;
use backend::ApiKey;
use backend::RuntimeError;
use backend::ExecutionMode;

use dptran::{DpTranError, LangType};

/// Initialization of settings.
fn clear_settings() -> Result<(), RuntimeError> {
    print!("Are you sure you want to clear all settings? (y/N) ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    // Initialize settings when y is entered.
    if input.trim().to_ascii_lowercase() == "y" {
        backend::clear_settings()?;
        println!("All settings have been cleared.");
        println!("Note: You need to set the API key again to use dptran.");
    }
    Ok(())
}

/// Display of settings.
fn display_settings() -> Result<(), RuntimeError> {
    let api_key = backend::get_api_key()?;
    let default_target_lang = backend::get_default_target_language_code()?;
    let cache_max_entries = backend::get_cache_max_entries()?;
    let editor_command = backend::get_editor_command_str()?;
    let cache_enabled = backend::get_cache_enabled()?;

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

/// Set default destination language.
/// Set the default target language for translation in the configuration file config.json.
fn set_default_target_language(arg_default_target_language: &String) -> Result<(), RuntimeError> {
    let validated_language_code = backend::set_default_target_language(arg_default_target_language)?;
    println!("Default target language has been set to {}.", validated_language_code);
    Ok(())
}

/// Display list of source language codes.
/// Retrieved from <https://api-free.deepl.com/v2/languages>
fn show_source_language_codes() -> Result<(), RuntimeError> {
    let api_key = match backend::get_api_key()? {
        Some(api_key) => api_key,
        None => return Err(RuntimeError::DeeplApiError(DpTranError::ApiKeyIsNotSet)),
    };
    let dptran = dptran::DpTran::with(&api_key.api_key, api_key.api_key_type);

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
    let dptran = dptran::DpTran::with(&api_key.api_key, api_key.api_key_type);

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
pub fn show_usage() -> Result<(), RuntimeError> {
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
                if input.trim_end() == "quit" {
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

/// Dialogue and Translation.
/// Repeat input if in interactive mode
/// In normal mode, it will be finished once
fn process(dptran: &dptran::DpTran, mode: ExecutionMode, source_lang: Option<String>, target_lang: String, 
            multilines: bool, rm_line_breaks: bool, text: Option<String>, mut ofile: Option<std::fs::File>) -> Result<(), RuntimeError> {
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
        println!("Type \"quit\" to exit dptran.");
    }

    loop {
        // If in interactive mode, get from standard input
        // In normal mode, get from argument
        let input = get_input(&mode, multilines, rm_line_breaks, &text);
        if input.is_none() {
            return Err(RuntimeError::DeeplApiError(DpTranError::CouldNotGetInputText));
        }
        let input = input.unwrap();

        // Interactive mode: "quit" to exit
        if mode == ExecutionMode::TranslateInteractive {
            if input.len() == 0 {
                continue;
            }
            if input[0].trim_end() == "quit" {
                break;
            }
            if input[0].clone().trim_end().is_empty() {
                continue;
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
            if backend::get_cache_enabled()? {
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
            break;
        }
    }

    Ok(())
}

/// Start translation process.
fn start_translation_process(mode: ExecutionMode, translate_from: Option<String>, translate_to: Option<String>, 
                            multilines: bool, remove_line_breaks: bool, source_text: Option<String>, ofile_path: Option<String>) -> Result<(), RuntimeError> {
    let mut source_lang = translate_from;
    let mut target_lang = translate_to;

    if target_lang.is_none() {
        target_lang = Some(backend::get_default_target_language_code()?);
    }

    // API Key confirmation
    let api_key = match backend::get_api_key()? {
        Some(api_key) => api_key,
        None => {
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
            return Ok(());
        },
    };
    let dptran = dptran::DpTran::with(&api_key.api_key, api_key.api_key_type);

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
        // is the file exists?
        if std::path::Path::new(&output_file).exists() {
            print!("The file {} already exists. Overwrite? (y/N) ", output_file);
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            if input.trim().to_ascii_lowercase() != "y" {
                return Ok(());  // Do not overwrite
            }
        }
        Some(backend::create_file(&output_file)?)
    }
    else {
        None
    };

    // (Dialogue &) Translation
    process(&dptran, mode, source_lang, target_lang.unwrap(), 
            multilines, remove_line_breaks, source_text, ofile)?;

    Ok(())
}

/// Obtaining arguments and calling the translation process
fn main() -> Result<(), RuntimeError> {
    // Parsing arguments.
    let arg_struct = backend::parse::parser()?;
    let mode = arg_struct.execution_mode;
    match mode {
        ExecutionMode::PrintUsage => {
            show_usage()?;
            return Ok(());
        }
        ExecutionMode::SetFreeApiKey => {
            if let Some(s) = arg_struct.api_key {
                backend::set_api_key(ApiKey {
                    api_key: s,
                    api_key_type: dptran::ApiKeyType::Free,
                })?;
                return Ok(());
            } else {
                backend::clear_api_key(dptran::ApiKeyType::Free)?;
                return Ok(());
            }
        }
        ExecutionMode::SetProApiKey => {
            if let Some(s) = arg_struct.api_key {
                backend::set_api_key(ApiKey {
                    api_key: s,
                    api_key_type: dptran::ApiKeyType::Pro,
                })?;
                return Ok(());
            } else {
                backend::clear_api_key(dptran::ApiKeyType::Pro)?;
                return Ok(());
            }
        }
        ExecutionMode::SetDefaultTargetLang => {
            if let Some(s) = arg_struct.default_target_lang {
                set_default_target_language(&s)?;
                return Ok(());
            } else {
                return Err(RuntimeError::DeeplApiError(DpTranError::NoTargetLanguageSpecified));
            }
        }
        ExecutionMode::SetCacheMaxEntries => {
            if let Some(s) = arg_struct.cache_max_entries {
                backend::configure::ConfigureWrapper::get("configure").map_err(|e| RuntimeError::ConfigError(e))?
                    .set_cache_max_entries(s).map_err(|e| RuntimeError::ConfigError(e))?;
                return Ok(());
            } else {
                return Err(RuntimeError::CacheMaxEntriesIsNotSet);
            }
        }
        ExecutionMode::ClearCache => {
            backend::cache::get_cache_data("cache").map_err(|e| RuntimeError::CacheError(e))?
                .clear_cache().map_err(|e| RuntimeError::CacheError(e))?;
            return Ok(());
        }
        ExecutionMode::SetEditor => {
            if let Some(s) = arg_struct.editor_command {
                backend::set_editor_command(s)?;
                return Ok(());
            } else {
                return Err(RuntimeError::EditorCommandIsNotSet);
            }
        }
        ExecutionMode::EnableCache => {
            backend::configure::ConfigureWrapper::get("configure").map_err(|e| RuntimeError::ConfigError(e))?
                .set_cache_enabled(true).map_err(|e| RuntimeError::ConfigError(e))?;
            return Ok(());
        }
        ExecutionMode::DisableCache => {
            backend::configure::ConfigureWrapper::get("configure").map_err(|e| RuntimeError::ConfigError(e))?
                .set_cache_enabled(false).map_err(|e| RuntimeError::ConfigError(e))?;
            return Ok(());
        }
        ExecutionMode::DisplaySettings => {
            display_settings()?;
            return Ok(());
        }
        ExecutionMode::ClearSettings => {
            clear_settings()?;
            return Ok(());
        }
        ExecutionMode::ListSourceLangs => {
            show_source_language_codes()?;
            return Ok(());
        }
        ExecutionMode::ListTargetLangs => {
            show_target_language_codes()?;
            return Ok(());
        }
        ExecutionMode::TranslateNormal | ExecutionMode::TranslateInteractive => {
            // Normal or interactive translation mode
            let source_lang = arg_struct.translate_from;
            let target_lang = arg_struct.translate_to;
            let multilines = arg_struct.multilines;
            let rm_line_breaks = arg_struct.remove_line_breaks;
            let source_text = arg_struct.source_text;
            let ofile_path = arg_struct.ofile_path;

            // Start translation process
            start_translation_process(mode, source_lang, target_lang,
                                    multilines, rm_line_breaks, source_text, ofile_path)?;
            return Ok(());
        }
    };
}

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
        let result = show_usage();
        if let Err(e) = &result {
            if retry_or_panic(e, 1) {
                return impl_app_show_usage_test(times + 1);
            }
        }
        assert!(result.is_ok());
    }

    fn impl_app_process_test(times: u8) {
        let api_key = backend::get_api_key().unwrap().unwrap();
        let dptran = dptran::DpTran::with(&api_key.api_key, api_key.api_key_type);
        let mode = ExecutionMode::TranslateNormal;
        let multilines = false;
        let rm_line_breaks = false;
        let text = Some("Hello, world!".to_string());
        let source_lang = Some("en".to_string());
        let target_lang = "fr".to_string();
        let ofile = None;

        let result = process(&dptran, mode, source_lang, target_lang, multilines, rm_line_breaks, text, ofile);
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
