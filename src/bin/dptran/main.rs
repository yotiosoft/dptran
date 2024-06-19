use std::io::{self, Write, stdin, stdout, BufWriter};
use std::fs::OpenOptions;
use std::fmt::Debug;

mod parse;
mod configure;
mod cache;

use dptran::{DpTranError, DpTranUsage, LangType};
use configure::ConfigError;
use cache::CacheError;
use parse::ExecutionMode;

enum RuntimeError {
    DeeplApiError(dptran::DpTranError),
    ConfigError(ConfigError),
    StdIoError(String),
    FileIoError(String),
    EditorError(String),
    CacheError(CacheError),
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
            RuntimeError::ConfigError(e) => format!("Config error: {}", e),
            RuntimeError::StdIoError(e) => format!("Standard I/O error: {}", e),
            RuntimeError::FileIoError(e) => format!("File I/O error: {}", e),
            RuntimeError::EditorError(e) => format!("Editor error: {}", e),
            RuntimeError::CacheError(e) => format!("Cache error: {}", e),
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
fn get_usage() -> Result<DpTranUsage, RuntimeError> {
    let api_key = get_api_key()?;
    if let Some(api_key) = api_key {
        dptran::get_usage(&api_key).map_err(|e| RuntimeError::DeeplApiError(e))
    } else {
        Err(RuntimeError::DeeplApiError(DpTranError::ApiKeyIsNotSet))
    }
}

/// Display the number of characters remaining.
fn show_usage() -> Result<(), RuntimeError> {
    let usage = get_usage()?;
    if usage.unlimited {
        println!("usage: {} / unlimited", usage.character_count);
    }
    else {
        println!("usage: {} / {} ({}%)", usage.character_count, usage.character_limit, (usage.character_count as f64 / usage.character_limit as f64 * 100.0).round());
        println!("remaining: {}", usage.character_limit - usage.character_count);
    }
    Ok(())
}

/// Set API key (using confy crate).
/// Set the API key in the configuration file config.json.
fn set_api_key(api_key: String) -> Result<(), RuntimeError> {
    configure::set_api_key(api_key).map_err(|e| RuntimeError::ConfigError(e))?;
    Ok(())
}

/// Set default destination language.
/// Set the default target language for translation in the configuration file config.json.
fn set_default_target_language(arg_default_target_language: String) -> Result<(), RuntimeError> {
    let api_key = match get_api_key()? {
        Some(api_key) => api_key,
        None => return Err(RuntimeError::DeeplApiError(DpTranError::ApiKeyIsNotSet)),
    };

    // Check if the language code is correct
    if let Ok(validated_language_code) = dptran::correct_language_code(&api_key, &arg_default_target_language) {
        configure::set_default_target_language(&validated_language_code).map_err(|e| RuntimeError::ConfigError(e))?;
        println!("Default target language has been set to {}.", validated_language_code);
        Ok(())
    } else {
        Err(RuntimeError::DeeplApiError(DpTranError::InvalidLanguageCode))
    }
}

/// Set the editor command.
fn set_editor_command(editor_command: String) -> Result<(), RuntimeError> {
    configure::set_editor_command(editor_command).map_err(|e| RuntimeError::ConfigError(e))?;
    Ok(())
}

/// Initialization of settings.
fn clear_settings() -> Result<(), RuntimeError> {
    println!("Are you sure you want to clear all settings? (y/N)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    // Initialize settings when y is entered.
    if input.trim().to_ascii_lowercase() == "y" {
        configure::clear_settings().map_err(|e| RuntimeError::ConfigError(e))?;
        println!("All settings have been cleared.");
        println!("Note: You need to set the API key again to use dptran.");
    }
    Ok(())
}

/// Get the configured default destination language code.
fn get_default_target_language_code() -> Result<String, RuntimeError> {
    let default_target_lang = configure::get_default_target_language_code().map_err(|e| RuntimeError::ConfigError(e))?;
    Ok(default_target_lang)
}

/// Load the API key from the configuration file.
fn get_api_key() -> Result<Option<String>, RuntimeError> {
    let api_key = configure::get_api_key().map_err(|e| RuntimeError::ConfigError(e))?;
    Ok(api_key)
}

/// Load the editor command from the configuration file.
fn get_editor_command_str() -> Result<Option<String>, RuntimeError> {
    let editor_command = configure::get_editor_command().map_err(|e| RuntimeError::ConfigError(e))?;
    Ok(editor_command)
}

/// Display of settings.
fn display_settings() -> Result<(), RuntimeError> {
    let api_key = get_api_key()?;
    let default_target_lang = get_default_target_language_code()?;
    let editor_command = get_editor_command_str()?;

    if let Some(api_key) = api_key {
        println!("API key: {}", api_key);
    }
    else {
        println!("API key: not set");
    }

    println!("Default target language: {}", default_target_lang);

    if let Some(editor_command) = editor_command {
        println!("Editor command: {}", editor_command);
    }
    else {
        println!("Editor command: not set");
    }

    Ok(())
}

/// Display list of source language codes.
/// Retrieved from <https://api-free.deepl.com/v2/languages>
fn show_source_language_codes() -> Result<(), RuntimeError> {
    let api_key = match get_api_key()? {
        Some(api_key) => api_key,
        None => return Err(RuntimeError::DeeplApiError(DpTranError::ApiKeyIsNotSet)),
    };

    // List of source language codes.
    let source_lang_codes = dptran::get_language_codes(&api_key, LangType::Source).map_err(|e| RuntimeError::DeeplApiError(e))?;
    
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
    let api_key = match get_api_key()? {
        Some(api_key) => api_key,
        None => return Err(RuntimeError::DeeplApiError(DpTranError::ApiKeyIsNotSet)),
    };

    // List of Language Codes.
    let mut target_lang_codes = dptran::get_language_codes(&api_key, LangType::Target).map_err(|e| RuntimeError::DeeplApiError(e))?;

    // special case code conversion
    target_lang_codes.push(("EN".to_string(), "English".to_string()));
    target_lang_codes.push(("PT".to_string(), "Portuguese".to_string()));

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

/// Get source text from the stdin.
fn get_input(mode: &ExecutionMode, multilines: bool, text: &Option<String>) -> Option<Vec<String>> {
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
            Some(input_vec)
        }
        ExecutionMode::TranslateNormal => {
            match text {
                Some(text) => {
                    // Split strings containing newline codes.
                    let lines = text.lines();
                    Some(lines.map(|x| x.to_string()).collect())
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
fn process(api_key: &String, mode: ExecutionMode, source_lang: Option<String>, target_lang: String, 
            multilines: bool, text: Option<String>, mut ofile: Option<std::fs::File>) -> Result<(), RuntimeError> {
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
        let input = get_input(&mode, multilines, &text);
        if input.is_none() {
            return Err(RuntimeError::DeeplApiError(DpTranError::CouldNotGetInputText));
        }

        // Interactive mode: "quit" to exit
        if mode == ExecutionMode::TranslateInteractive {
            if let Some(input) = &input {
                if input[0].trim_end() == "quit" {
                    break;
                }
                if input[0].clone().trim_end().is_empty() {
                    continue;
                }
            }
        }
        // Normal mode: Exit if empty string
        if mode == ExecutionMode::TranslateNormal && input.is_none() {
            break;
        }

        // Check the cache
        let cache_result = cache::search_cache(&input.clone().unwrap().join("\n"), &target_lang).map_err(|e| RuntimeError::CacheError(e))?;
        let translated_texts = if let Some(cached_text) = cache_result {
            println!("Translated from cache.");
            vec![cached_text]
        // If not in cache, translate and store in cache
        } else {
            // translate
            let result = dptran::translate(&api_key, input.clone().unwrap(), &target_lang, &source_lang)
                .map_err(|e| RuntimeError::DeeplApiError(e))?;
            // store in cache
            cache::into_cache_element(&result.clone().join("\n"), &target_lang).map_err(|e| RuntimeError::FileIoError(e.to_string()))?;
            result
        };
        for translated_text in translated_texts {
            if let Some(ofile) = &mut ofile {
                // append to the file
                let mut buf_writer = BufWriter::new(ofile);
                writeln!(buf_writer, "{}", translated_text).map_err(|e| RuntimeError::FileIoError(e.to_string()))?;
                if mode == ExecutionMode::TranslateInteractive {
                    println!("{}", translated_text);
                }
            } else {
                println!("{}", translated_text);
            }
        }
        // In normal mode, exit the loop once.
        if mode == ExecutionMode::TranslateNormal {
            break;
        }
    }

    Ok(())
}

/// Obtaining arguments and calling the translation process
fn main() -> Result<(), RuntimeError> {
    // Parsing arguments.
    let arg_struct = parse::parser()?;
    let mode = arg_struct.execution_mode;
    match mode {
        ExecutionMode::PrintUsage => {
            show_usage()?;
            return Ok(());
        }
        ExecutionMode::SetApiKey => {
            if let Some(s) = arg_struct.api_key {
                set_api_key(s)?;
                return Ok(());
            } else {
                return Err(RuntimeError::StdIoError("API key is not specified.".to_string()));
            }
        }
        ExecutionMode::SetDefaultTargetLang => {
            if let Some(s) = arg_struct.default_target_lang {
                set_default_target_language(s)?;
                return Ok(());
            } else {
                return Err(RuntimeError::DeeplApiError(DpTranError::NoTargetLanguageSpecified));
            }
        }
        ExecutionMode::SetEditor => {
            if let Some(s) = arg_struct.editor_command {
                set_editor_command(s)?;
                return Ok(());
            } else {
                return Err(RuntimeError::StdIoError("Editor command is not specified.".to_string()));
            }
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
        _ => {}     // ExecutionMode::TranslateNormal, ExecutionMode::TranslateInteractive, ExecutionMode::FileInput
    };

    let mut source_lang = arg_struct.translate_from;
    let mut target_lang = arg_struct.translate_to;

    if target_lang.is_none() {
        target_lang = Some(get_default_target_language_code()?);
    }

    // API Key confirmation
    let api_key = match get_api_key()? {
        Some(api_key) => api_key,
        None => {
            println!("Welcome to dptran!\nFirst, please set your DeepL API-key:\n  $ dptran set --api-key <API_KEY>\nYou can get DeepL API-key for free here:\n  https://www.deepl.com/en/pro-api?cta=header-pro-api/");
            return Ok(());
        },
    };

    // Language code check and correction
    if let Some(sl) = source_lang {
        source_lang = Some(dptran::correct_language_code(&api_key, &sl.to_string()).map_err(|e| RuntimeError::DeeplApiError(e))?);
    }
    if let Some(tl) = target_lang {
        target_lang = Some(dptran::correct_language_code(&api_key, &tl.to_string()).map_err(|e| RuntimeError::DeeplApiError(e))?);
    }

    // Output filepath
    // If output file is specified, it will be created or overwritten.
    let ofile = if let Some(output_file) = arg_struct.ofile_path {
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
        Some(OpenOptions::new().create(true).write(true).truncate(true).open(&output_file).map_err(|e| RuntimeError::FileIoError(e.to_string()))?)
    }
    else {
        None
    };

    // (Dialogue &) Translation
    process(&api_key, mode, source_lang, target_lang.unwrap(), arg_struct.multilines, arg_struct.source_text, ofile)?;

    Ok(())
}
