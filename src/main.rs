use std::io::{self, Write, stdin, stdout};

mod parse;
mod configure;

use dptran::{DpTranError, DpTranUsage, LangCode};
use parse::ExecutionMode;

/// 翻訳可能な残り文字数の取得
/// <https://api-free.deepl.com/v2/usage>より取得する  
/// 取得に失敗したらエラーを返す
fn get_usage() -> Result<DpTranUsage, DpTranError> {
    let api_key = get_api_key()?;
    if let Some(api_key) = api_key {
        dptran::get_usage(&api_key).map_err(|e| DpTranError::DeeplApiError(e.to_string()))
    } else {
        Err(DpTranError::ApiKeyIsNotSet)
    }
}

/// 残り文字数を表示
fn show_usage() -> Result<(), DpTranError> {
    let usage = get_usage()?;
    if usage.character_limit == 0 {
        println!("usage: {} / unlimited", usage.character_count);
    }
    else {
        println!("usage: {} / {} ({}%)", usage.character_count, usage.character_limit, (usage.character_count as f64 / usage.character_limit as f64 * 100.0).round());
        println!("remaining: {}", usage.character_limit - usage.character_count);
    }
    Ok(())
}

/// APIキーの設定  
/// 設定ファイルconfig.jsonにAPIキーを設定する。
fn set_api_key(api_key: String) -> Result<(), DpTranError> {
    configure::set_api_key(api_key).map_err(|e| DpTranError::ConfigError(e.to_string()))?;
    Ok(())
}

/// デフォルトの翻訳先言語の設定  
/// 設定ファイルconfig.jsonにデフォルトの翻訳先言語を設定する。
fn set_default_target_language(arg_default_target_language: String) -> Result<(), DpTranError> {
    let api_key = match get_api_key()? {
        Some(api_key) => api_key,
        None => return Err(DpTranError::ApiKeyIsNotSet),
    };

    // 言語コードが正しいか確認
    if let Ok(validated_language_code) = dptran::correct_language_code(&api_key, &arg_default_target_language) {
        configure::set_default_target_language(&validated_language_code).map_err(|e| DpTranError::ConfigError(e.to_string()))?;
        println!("Default target language has been set to {}.", validated_language_code);
        Ok(())
    } else {
        Err(DpTranError::InvalidLanguageCode)
    }
}

/// 設定の初期化
fn clear_settings() -> Result<(), DpTranError> {
    // 今一度確認
    println!("Are you sure you want to clear all settings? (y/N)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    // yが入力されたら設定を初期化する
    if input.trim().to_ascii_lowercase() == "y" {
        configure::clear_settings().map_err(|e| DpTranError::ConfigError(e.to_string()))?;
        println!("All settings have been cleared.");
        println!("Note: You need to set the API key again to use dptran.");
    }
    Ok(())
}

/// 設定済みの既定の翻訳先言語コードを取得
fn get_default_target_language_code() -> Result<String, DpTranError> {
    let default_target_lang = configure::get_default_target_language_code().map_err(|e| DpTranError::ConfigError(e.to_string()))?;
    Ok(default_target_lang)
}

/// APIキーを取得
fn get_api_key() -> Result<Option<String>, DpTranError> {
    let api_key = configure::get_api_key().map_err(|e| DpTranError::ConfigError(e.to_string()))?;
    Ok(api_key)
}

/// 設定内容の表示
fn display_settings() -> Result<(), DpTranError> {
    let api_key = get_api_key()?;
    let default_target_lang = get_default_target_language_code()?;
    if let Some(api_key) = api_key {
        println!("API key: {}", api_key);
    }
    else {
        println!("API key: not set");
    }
    println!("Default target language: {}", default_target_lang);
    Ok(())
}

/// 翻訳元言語コード一覧の表示  
/// <https://api-free.deepl.com/v2/languages>から取得する
fn show_source_language_codes() -> Result<(),  DpTranError> {
    let api_key = match get_api_key()? {
        Some(api_key) => api_key,
        None => return Err(DpTranError::ApiKeyIsNotSet),
    };

    // 翻訳元言語コード一覧
    let source_lang_codes = dptran::get_language_codes(&api_key, "source".to_string())?;
    
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
/// 翻訳先言語コード一覧の表示
fn show_target_language_codes() -> Result<(), DpTranError> {
    let api_key = match get_api_key()? {
        Some(api_key) => api_key,
        None => return Err(DpTranError::ApiKeyIsNotSet),
    };

    // 翻訳先言語コード一覧
    let mut target_lang_codes = dptran::get_language_codes(&api_key, "target".to_string())?;

    // 特例コード変換
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

/// 標準入力より原文取得
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

                // multilineモードなら改行を含む入力を受け付ける
                if multilines {
                    if input == "\r\n" || input == "\n" {
                        break;
                    }
                }
                // multilineモードでない場合、\\ + 改行で改行を含む入力を受け付ける
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
                    // 改行コードを含む文字列を分割
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

/// 対話と翻訳  
/// 対話モードであれば繰り返し入力を行う  
/// 通常モードであれば一回で終了する
fn process(api_key: &String, mode: ExecutionMode, source_lang: Option<String>, target_lang: String, multilines: bool, text: Option<String>) -> Result<(), DpTranError> {
    // 翻訳
    // 対話モードならループする; 通常モードでは1回で抜ける

    // 対話モードなら終了方法を表示
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
        // 対話モードなら標準入力から取得
        // 通常モードでは引数から取得
        let input = get_input(&mode, multilines, &text);
        if input.is_none() {
            return Err(DpTranError::CouldNotGetInputText);
        }

        // 対話モード："quit"で終了
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
        // 通常モード：空文字列なら終了
        if mode == ExecutionMode::TranslateNormal && input.is_none() {
            break;
        }

        // 翻訳
        let translated_texts = dptran::translate(&api_key, input.unwrap(), &target_lang, &source_lang);
        match translated_texts {
            Ok(s) => {
                for translated_text in s {
                    println!("{}", translated_text);
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
        // 通常モードの場合、一回でループを抜ける
        if mode == ExecutionMode::TranslateNormal {
            break;
        }
    }

    Ok(())
}

/// メイン関数
/// 引数の取得と翻訳処理の呼び出し
fn main() -> Result<(), DpTranError> {
    // 引数を解析
    let arg_struct = parse::parser().map_err(|e| DpTranError::StdIoError(e))?;
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
                return Err(DpTranError::ApiKeyIsNotSet);
            }
        }
        ExecutionMode::SetDefaultTargetLang => {
            if let Some(s) = arg_struct.default_target_lang {
                set_default_target_language(s)?;
                return Ok(());
            } else {
                return Err(DpTranError::NoTargetLanguageSpecified);
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
        _ => {}     // ExecutionMode::TranslateNormal, ExecutionMode::TranslateInteractive
    };

    let mut source_lang = arg_struct.translate_from;
    let mut target_lang = arg_struct.translate_to;

    if target_lang.is_none() {
        target_lang = Some(get_default_target_language_code()?);
    }

    // APIキーの確認
    let api_key = get_api_key()?;
    if api_key.is_none() {
        println!("Welcome to dptran!\nFirst, please set your DeepL API-key:\n  $ dptran set --api-key <API_KEY>\nYou can get DeepL API-key for free here:\n  https://www.deepl.com/ja/pro-api?cta=header-pro-api/");
        return Ok(());
    }

    // 言語コードのチェック & 正しい言語コードに変換
    if let Some(sl) = source_lang {
        source_lang = Some(dptran::correct_language_code(&api_key, &sl.to_string())?);
    }
    if let Some(tl) = target_lang {
        target_lang = Some(dptran::correct_language_code(&api_key, &tl.to_string())?);
    }

    // (対話＆)翻訳
    process(mode, source_lang, target_lang.unwrap(), arg_struct.multilines, arg_struct.source_text)?;

    Ok(())
}
