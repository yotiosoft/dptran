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
    // 言語コードが正しいか確認
    if let Ok(validated_language_code) = correct_language_code(&arg_default_target_language.to_string()) {
        configure::set_default_target_language(&validated_language_code).expect("Failed to set default target language");
        println!("Default target language has been set to {}.", validated_language_code);
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Invalid language code"))
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
pub fn get_default_target_language_code() -> Result<String, io::Error> {
    let default_target_lang = configure::get_default_target_language_code().expect("failed to get default target language code");
    Ok(default_target_lang)
}

/// APIキーを取得
pub fn get_api_key() -> Result<String, io::Error> {
    let api_key = configure::get_api_key().expect("failed to get api key");
    Ok(api_key)
}

/// 正しい言語コードに変換
pub fn correct_language_code(language_code: &str) -> Result<String, io::Error> {
    // EN, PTは変換
    let language_code_uppercase = match language_code.to_ascii_uppercase().as_str() {
        "EN" => "EN-US".to_string(),
        "PT" => "PT-PT".to_string(),
        _ => language_code.to_ascii_uppercase(),
    };

    match deeplapi::check_language_code(&get_api_key()?, &language_code_uppercase, "target".to_string()) {
        true => Ok(language_code_uppercase),
        false => Err(io::Error::new(io::ErrorKind::Other, "Invalid language code")),
    }
}
