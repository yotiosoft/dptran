use std::{io, env};
use serde_json::Value;

mod connection;
pub mod settings;

/// バージョン情報の表示  
/// CARGO_PKG_VERSIONから取得する
pub fn show_version() {
    println!("dptran version {}", env!("CARGO_PKG_VERSION"));
}

/// 翻訳可能な残り文字数の表示  
/// <https://api-free.deepl.com/v2/usage>より取得する  
/// 取得に失敗したらエラーを返す
pub fn get_remain() -> core::result::Result<(i32, i32), io::Error> {
    let url = "https://api-free.deepl.com/v2/usage".to_string();
    let query = format!("auth_key={}", settings::get_settings().api_key);
    let res = connection::send_and_get(url, query)?;
    let v: Value = serde_json::from_str(&res)?;

    v.get("character_count").ok_or(io::Error::new(io::ErrorKind::Other, "failed to get character_count"))?;
    v.get("character_limit").ok_or(io::Error::new(io::ErrorKind::Other, "failed to get character_limit"))?;

    let character_count = v["character_count"].as_i64().expect("failed to get character_count") as i32;
    let character_limit = v["character_limit"].as_i64().expect("failed to get character_limit") as i32;
    Ok((character_count, character_limit))
}

/// ヘルプの表示
pub fn show_help() {
    println!("To translate with optional languages, usage: deepl [options] [sentence]");
    println!("Options:");
    println!("  -f or --from\t\t\tSet source language");
    println!("  -t or --to\t\t\tSet target language");
    println!("If -f is not specified, the source language is automatically inferred by DeepL.");
    println!("If -t is not specified, the translation is done into the configured default target language.");
    println!("");
    println!("To setup setting options, usage: deepl -s [setting options]");
    println!("Setting options:");
    println!("  -s default-lang\t\tSetup default target language");
    println!("  -s api-key\t\t\tSetup your DeepL API key");
    println!("  -s clear\t\t\tClear all settings");
    println!("");
    println!("For other options, usage: deepl [options]");
    println!("Options:");
    println!("  -h or --help\t\t\tShow this help message");
    println!("  -lt\t\t\t\tShow all supported target language codes");
    println!("  -ls\t\t\t\tShow all supported source language codes");
    println!("  -v or --version\t\tShow version");
}

/// 翻訳  
/// 失敗したらエラーを返す
pub fn translate(auth_key: &String, text: String, target_lang: &String, source_lang: &String) -> core::result::Result<String, io::Error> {
    let url = "https://api-free.deepl.com/v2/translate".to_string();
    let query = if source_lang.trim_matches('"').is_empty() {
        format!("auth_key={}&text={}&target_lang={}", auth_key, text, target_lang)
    } else {
        format!("auth_key={}&text={}&target_lang={}&source_lang={}", auth_key, text, target_lang, source_lang)
    };
    
    connection::send_and_get(url, query)
}

/// 翻訳結果の表示  
/// json形式の翻訳結果を受け取り、翻訳結果を表示する  
/// jsonのパースに失敗したらエラーを返す
pub fn show_translated_text(json_str: &String) -> core::result::Result<(), io::Error> {
    let json: serde_json::Value = serde_json::from_str(json_str)?;
    json.get("translations").ok_or(io::Error::new(io::ErrorKind::Other, "Invalid response"))?;
    let translations = &json["translations"];
    for translation in translations.as_array().expect("failed to get array") {
        let len = translation["text"].to_string().len();
        let translation_trimmed= translation["text"].to_string()[1..len-1].to_string();
        println!("{}", translation_trimmed);
    }

    Ok(())
}

pub type LangCode = (String, String);
/// 言語コード一覧の取得  
/// <https://api-free.deepl.com/v2/languages>から取得する
pub fn get_language_codes(type_name: String) -> core::result::Result<Vec<LangCode>, io::Error> {
    let url = "https://api-free.deepl.com/v2/languages".to_string();
    let query = format!("type={}&auth_key={}", type_name, settings::get_settings().api_key);
    let res = connection::send_and_get(url, query)?;
    let v: Value = serde_json::from_str(&res)?;

    let mut lang_codes: Vec<LangCode> = Vec::new();
    for value in v.as_array().expect("Invalid response at get_language_codes") {
        value.get("language").ok_or(io::Error::new(io::ErrorKind::Other, "Invalid response"))?;
        let lang_code = (value["language"].to_string(), value["name"].to_string());
        lang_codes.push(lang_code);
    }

    Ok(lang_codes)
}
/// 翻訳元言語コード一覧の表示  
/// <https://api-free.deepl.com/v2/languages>から取得する
pub fn show_source_language_codes() -> core::result::Result<(), io::Error> {
    // 翻訳元言語コード一覧
    let source_lang_codes = get_language_codes("source".to_string())?;
    println!("Source language codes:");
    for lang_code in source_lang_codes {
        println!("{}: {}", lang_code.0.trim_matches('"'), lang_code.1.trim_matches('"'));
    }

    Ok(())
}
/// 翻訳先言語コード一覧の表示
pub fn show_target_language_codes() -> core::result::Result<(), io::Error> {
    // 翻訳先言語コード一覧
    let mut target_lang_codes = get_language_codes("target".to_string())?;

    // 特例コード変換
    target_lang_codes.push(("EN".to_string(), "English".to_string()));
    target_lang_codes.push(("PT".to_string(), "Portuguese".to_string()));

    println!("Target languages:");
    for lang_code in target_lang_codes {
        println!("{}: {}", lang_code.0.trim_matches('"'), lang_code.1.trim_matches('"'));
    }

    Ok(())
}