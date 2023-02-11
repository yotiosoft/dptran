use std::io;

mod configure;
pub mod deeplapi;

/// APIキーの設定  
/// 設定ファイルconfig.jsonにAPIキーを設定する。
pub fn set_api_key(api_key: String) -> Result<(), io::Error> {
    configure::set_api_key(api_key).expect("Failed to set API key");
    Ok(())
}

/// デフォルトの翻訳先言語の設定  
/// 設定ファイルconfig.jsonにデフォルトの翻訳先言語を設定する。
pub fn set_default_target_language(arg_default_target_language: String) -> Result<(), io::Error> {
    // EN, PTは変換
    let default_target_language = match arg_default_target_language.to_ascii_uppercase().as_str() {
        "EN" => "EN-US".to_string(),
        "PT" => "PT-PT".to_string(),
        _ => arg_default_target_language.to_ascii_uppercase(),
    };

    // 言語コードが正しいか確認
    match deeplapi::check_language_code(&get_api_key()?, &default_target_language, "target".to_string()) {
        true => {
            configure::set_default_target_language(&default_target_language).expect("Failed to set default target language");
            println!("Default target language has been set to {}.", default_target_language);
            Ok(())
        }
        false => Err(io::Error::new(io::ErrorKind::Other, "Invalid language code")),
    }
}

/// 設定の初期化
pub fn clear_settings() -> Result<(), io::Error> {
    // 今一度確認
    println!("Are you sure you want to clear all settings? (y/N)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    // yが入力されたら設定を初期化する
    if input.trim().to_ascii_lowercase() == "y" {
        configure::clear_settings().expect("Failed to clear settings");
        println!("All settings have been cleared.");
        println!("Note: You need to set the API key again to use dptran.");
    }
    Ok(())
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
