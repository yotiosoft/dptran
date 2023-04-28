use std::io;

mod configure;
mod deeplapi;

use deeplapi::LangCode;

/// APIキーの設定  
/// 設定ファイルconfig.jsonにAPIキーを設定する。
pub fn set_api_key(api_key: String) -> Result<(), String> {
    configure::set_api_key(api_key).map_err(|e| e.to_string())?;
    Ok(())
}

/// デフォルトの翻訳先言語の設定  
/// 設定ファイルconfig.jsonにデフォルトの翻訳先言語を設定する。
pub fn set_default_target_language(arg_default_target_language: String) -> Result<(), String> {
    // 言語コードが正しいか確認
    if let Ok(validated_language_code) = correct_language_code(&arg_default_target_language.to_string()) {
        configure::set_default_target_language(&validated_language_code).map_err(|e| e.to_string())?;
        println!("Default target language has been set to {}.", validated_language_code);
        Ok(())
    } else {
        Err("Invalid language code".to_string())
    }
}

/// 設定の初期化
pub fn clear_settings() -> Result<(), String> {
    // 今一度確認
    println!("Are you sure you want to clear all settings? (y/N)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    // yが入力されたら設定を初期化する
    if input.trim().to_ascii_lowercase() == "y" {
        configure::clear_settings().map_err(|e| e.to_string())?;
        println!("All settings have been cleared.");
        println!("Note: You need to set the API key again to use dptran.");
    }
    Ok(())
}

/// 設定済みの既定の翻訳先言語コードを取得
pub fn get_default_target_language_code() -> Result<String, String> {
    let default_target_lang = configure::get_default_target_language_code().map_err(|e| e.to_string())?;
    Ok(default_target_lang)
}

/// APIキーを取得
pub fn get_api_key() -> Result<Option<String>, String> {
    let api_key = configure::get_api_key().map_err(|e| e.to_string())?;
    Ok(api_key)
}

/// 言語コード一覧の取得  
/// <https://api-free.deepl.com/v2/languages>から取得する
pub fn get_language_codes(type_name: String) -> Result<Vec<LangCode>, String> {
    let api_key = get_api_key().map_err(|e| e.to_string())?;
    if let Some(api_key) = api_key {
        let lang_codes = deeplapi::get_language_codes(&api_key, type_name)?;
        Ok(lang_codes)
    } else {
        Err("API key is not set.".to_string())
    }
}

/// 言語コードの有効性をチェック
fn check_language_code(lang_code: &String, type_name: String) -> Result<bool, String> {
    let lang_codes = get_language_codes(type_name.to_string())?;
    for lang in lang_codes {
        if lang.0.trim_matches('"') == lang_code.to_uppercase() {
            return Ok(true);
        }
    }
    Ok(false)
}

/// 正しい言語コードに変換
pub fn correct_language_code(language_code: &str) -> Result<String, String> {
    // EN, PTは変換
    let language_code_uppercase = match language_code.to_ascii_uppercase().as_str() {
        "EN" => "EN-US".to_string(),
        "PT" => "PT-PT".to_string(),
        _ => language_code.to_ascii_uppercase(),
    };

    match check_language_code(&language_code_uppercase, "target".to_string())? {
        true => Ok(language_code_uppercase),
        false => Err("Invalid language code".to_string()),
    }
}

/// 翻訳可能な残り文字数の取得
/// <https://api-free.deepl.com/v2/usage>より取得する  
/// 取得に失敗したらエラーを返す
pub fn get_usage() -> Result<(i64, i64), String> {
    let api_key = get_api_key()?;
    if let Some(api_key) = api_key {
        deeplapi::get_usage(&api_key)
    } else {
        Err("API key is not set.".to_string())
    }
}

/// 翻訳結果の表示  
/// json形式の翻訳結果を受け取り、翻訳結果を表示する  
/// jsonのパースに失敗したらエラーを返す
pub fn translate(text: Vec<String>, target_lang: &String, source_lang: &Option<String>) -> Result<Vec<String>, String> {
    let api_key = get_api_key()?;
    if let Some(api_key) = api_key {
        deeplapi::translate(&api_key, text, target_lang, source_lang)
    } else {
        Err("API key is not set.".to_string())
    }
}
