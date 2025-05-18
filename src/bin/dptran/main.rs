use std::io::{self, Write, stdin, stdout};

mod backend;
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
        println!("API key: {}", api_key);
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

    let config_filepath = backend::configure::get_config_file_path().map_err(|e| RuntimeError::ConfigError(e))?;
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
    let dptran = dptran::DpTran::with(&api_key);

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
    let dptran = dptran::DpTran::with(&api_key);

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
    if usage.unlimited {
        println!("usage: {} / unlimited", usage.character_count);
    }
    else {
        println!("usage: {} / {} ({}%)", usage.character_count, usage.character_limit, (usage.character_count as f64 / usage.character_limit as f64 * 100.0).round());
        println!("remaining: {}", usage.character_limit - usage.character_count);
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
        ExecutionMode::SetApiKey => {
            if let Some(s) = arg_struct.api_key {
                backend::set_api_key(s)?;
                return Ok(());
            } else {
                return Err(RuntimeError::ApiKeyIsNotSet);
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
                backend::configure::set_cache_max_entries(s).map_err(|e| RuntimeError::ConfigError(e))?;
                return Ok(());
            } else {
                return Err(RuntimeError::CacheMaxEntriesIsNotSet);
            }
        }
        ExecutionMode::ClearCache => {
            backend::cache::clear_cache().map_err(|e| RuntimeError::CacheError(e))?;
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
            backend::configure::set_cache_enabled(true).map_err(|e| RuntimeError::ConfigError(e))?;
            return Ok(());
        }
        ExecutionMode::DisableCache => {
            backend::configure::set_cache_enabled(false).map_err(|e| RuntimeError::ConfigError(e))?;
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
        _ => {}     // ExecutionMode::TranslateNormal, ExecutionMode::TranslateInteractive, ExecutionMode::FileInput
    };

    let mut source_lang = arg_struct.translate_from;
    let mut target_lang = arg_struct.translate_to;

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
            println!("\t$ export DPTRAN_DEEPL_API_KEY=<API_KEY>");
            println!();
            println!("If you don't have an API-key, please sign up for a free/pro account at DeepL.");
            println!("You can get DeepL API-key for free here:");
            println!("\thttps://www.deepl.com/en/pro-api?cta=header-pro-api/");
            return Ok(());
        },
    };
    let dptran = dptran::DpTran::with(&api_key);

    // Language code check and correction
    if let Some(sl) = source_lang {
        source_lang = Some(dptran.correct_source_language_code(&sl.to_string()).map_err(|e| RuntimeError::DeeplApiError(e))?);
    }
    if let Some(tl) = target_lang {
        target_lang = Some(dptran.correct_target_language_code(&tl.to_string()).map_err(|e| RuntimeError::DeeplApiError(e))?);
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
        Some(backend::create_file(&output_file)?)
    }
    else {
        None
    };

    // (Dialogue &) Translation
    process(&dptran, mode, source_lang, target_lang.unwrap(), 
            arg_struct.multilines, arg_struct.remove_line_breaks, arg_struct.source_text, ofile)?;

    Ok(())
}
