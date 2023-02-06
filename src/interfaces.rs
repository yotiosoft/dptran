use std::{io, env};
use serde_json::Value;

mod connection;
mod configure;

/// バージョン情報の表示  
/// CARGO_PKG_VERSIONから取得する
pub fn show_version() {
    println!("dptran version {}", env!("CARGO_PKG_VERSION"));
}

/// 翻訳可能な残り文字数の表示  
/// <https://api-free.deepl.com/v2/usage>より取得する  
/// 取得に失敗したらエラーを返す
pub fn get_usage() -> core::result::Result<(i64, i64), io::Error> {
    let url = "https://api-free.deepl.com/v2/usage".to_string();
    let query = format!("auth_key={}", get_api_key()?);
    let res = connection::send_and_get(url, query)?;
    let v: Value = serde_json::from_str(&res)?;

    v.get("character_count").ok_or(io::Error::new(io::ErrorKind::Other, "failed to get character_count"))?;
    v.get("character_limit").ok_or(io::Error::new(io::ErrorKind::Other, "failed to get character_limit"))?;

    let character_count = v["character_count"].as_i64().expect("failed to get character_count");
    let character_limit = v["character_limit"].as_i64().expect("failed to get character_limit");
    Ok((character_count, character_limit))
}

/// ヘルプの表示
pub fn show_help() {
    let default_lang = get_default_target_language_code().unwrap_or("JA".to_string());
    
    println!("To translate with optional languages, usage: dptran [options] [sentence]");
    println!("Translation options:");
    println!("  -f or --from\t\tSet source language (default: auto)");
    println!("  -t or --to\t\tSet target language (default: {})", default_lang);
    println!("");
    println!("To setup setting options, usage: dptran -c [setting options] (or --config [setting options])");
    println!("Setting options:");
    println!("  -c default-lang\tSetup default target language");
    println!("  -c api-key\t\tSetup your DeepL API key");
    println!("  -c clear\t\tClear all settings");
    println!("");
    println!("For other options, usage: dptran [options]");
    println!("Options:");
    println!("  -h or --help\t\tShow this help message");
    println!("  -lt\t\t\tShow all supported target language codes");
    println!("  -ls\t\t\tShow all supported source language codes");
    println!("  -u or --usage\t\tShow usage and remaining characters available for translation with your API key");
    println!("  -v or --version\tShow version");
}

/// APIキーの設定  
/// 設定ファイルconfig.jsonにAPIキーを設定する。
pub fn set_apikey(api_key: String) -> Result<(), confy::ConfyError> {
    configure::set_apikey(api_key)
}

/// デフォルトの翻訳先言語の設定  
/// 設定ファイルconfig.jsonにデフォルトの翻訳先言語を設定する。
pub fn set_default_target_language(default_target_language: String) -> Result<(), confy::ConfyError> {
    configure::set_default_target_language(default_target_language)
}

/// 設定の初期化
pub fn clear_settings() -> Result<(), confy::ConfyError> {
    // 今一度確認
    println!("Are you sure you want to clear all settings? (y/N)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    // yが入力されたら設定を初期化する
    if input.trim().to_ascii_lowercase() == "y" {
        configure::clear_settings()?;
        println!("All settings have been cleared.");
        println!("Note: You need to set the API key again to use dptran.");
    }
    Ok(())
}

/// 翻訳  
/// 失敗したらエラーを返す
fn request_translate(auth_key: &String, text: String, target_lang: &String, source_lang: &String) -> Result<String, io::Error> {
    let url = "https://api-free.deepl.com/v2/translate".to_string();
    let query = if source_lang.trim_matches('"').is_empty() {
        format!("auth_key={}&text={}&target_lang={}", auth_key, text, target_lang)
    } else {
        format!("auth_key={}&text={}&target_lang={}&source_lang={}", auth_key, text, target_lang, source_lang)
    };
    
    connection::send_and_get(url, query)
}

/// json形式で渡された翻訳結果をパースし、ベクタに翻訳文を格納して返す
fn json_to_vec(json: &String) -> Result<Vec<String>, io::Error> {
    let json: serde_json::Value = serde_json::from_str(&json)?;
    json.get("translations").ok_or(io::Error::new(io::ErrorKind::Other, "Invalid response"))?;
    let translations = &json["translations"];

    let mut translated_texts = Vec::new();
    for translation in translations.as_array().expect("failed to get array") {
        let len = translation["text"].to_string().len();
        let translation_trimmed= translation["text"].to_string()[1..len-1].to_string();
        translated_texts.push(translation_trimmed);
    }

    Ok(translated_texts)
}

/// 翻訳結果の表示  
/// json形式の翻訳結果を受け取り、翻訳結果を表示する  
/// jsonのパースに失敗したらエラーを返す
pub fn translate(text: Vec<String>, target_lang: &String, source_lang: &String) -> Result<Vec<String>, io::Error> {
    let auth_key = get_api_key()?;
    let send_text = text.join("<dpbr>");

    // request_translate()で翻訳結果のjsonを取得
    let res = request_translate(&auth_key, send_text, target_lang, source_lang);
    if let Err(e) = res {
        return Err(e);
    }

    match res {
        Ok(res) => {
            let replace = |t: String| t.replace("<dpbr>", "\n");
            let vec = json_to_vec(&res)?;
            let ret: Vec<_> = vec.into_iter().map(replace).collect();
            Ok(ret)
        },
        // 翻訳結果が失敗ならエラー表示
        // DeepL APIが特有の意味を持つエラーコードであればここで検知
        // https://www.deepl.com/ja/docs-api/api-access/error-handling/
        Err(e) => {
            if e.to_string().contains("456") {  // 456 Unprocessable Entity
                Err(io::Error::new(io::ErrorKind::Other, 
                    "The translation limit of your account has been reached. Consider upgrading your subscription."))?
            }
            else {
                Err(e)?
            }
        }
    }
}

type LangCode = (String, String);
/// 言語コード一覧の取得  
/// <https://api-free.deepl.com/v2/languages>から取得する
fn get_language_codes(type_name: String) -> core::result::Result<Vec<LangCode>, io::Error> {
    let url = "https://api-free.deepl.com/v2/languages".to_string();
    let query = format!("type={}&auth_key={}", type_name, get_api_key()?);
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
    
    let mut i = 0;
    let len = source_lang_codes.len();
    let max_code_len = source_lang_codes.iter().map(|x| x.0.len()).max().unwrap();
    let max_str_len = source_lang_codes.iter().map(|x| x.1.len()).max().unwrap();

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
pub fn show_target_language_codes() -> core::result::Result<(), io::Error> {
    // 翻訳先言語コード一覧
    let mut target_lang_codes = get_language_codes("target".to_string())?;

    // 特例コード変換
    target_lang_codes.push(("EN".to_string(), "English".to_string()));
    target_lang_codes.push(("PT".to_string(), "Portuguese".to_string()));

    let mut i = 0;
    let len = target_lang_codes.len();
    let max_code_len = target_lang_codes.iter().map(|x| x.0.len()).max().unwrap();
    let max_str_len = target_lang_codes.iter().map(|x| x.1.len()).max().unwrap();

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
/// 言語コードの有効性をチェック
pub fn check_language_code(lang_code: &String, type_name: String) -> bool {
    let lang_codes = get_language_codes(type_name.to_string()).expect("failed to get language codes");
    for lang in lang_codes {
        if lang.0.trim_matches('"') == lang_code.to_uppercase() {
            return true;
        }
    }
    false
}

/// 設定済みの既定の翻訳先言語コードを取得
pub fn get_default_target_language_code() -> core::result::Result<String, io::Error> {
    let default_target_lang = configure::get_default_target_language_code().expect("failed to get default target language code");
    Ok(default_target_lang)
}

/// APIキーを取得
pub fn get_api_key() -> core::result::Result<String, io::Error> {
    let api_key = configure::get_api_key().expect("failed to get api key");
    Ok(api_key)
}
