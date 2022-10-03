use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct Configure {
    pub api_key: String,
    pub default_target_language: String
}

const SETTING_FILEPATH: &str = "config.json";
/// 設定ファイルの読み込みと値の抽出  
/// 設定ファイルconfig.jsonからAPIキーとデフォルトの翻訳先言語を取得する。  
/// 存在しない場合、既定値を指定して新規作成する。
pub fn get_settings() -> Configure {
    // ファイルが存在しない場合は新規作成＆初期化
    if Path::new(SETTING_FILEPATH).exists() == false {
        let mut f = File::create(SETTING_FILEPATH).expect("failed to create config.json");
        let config = Configure {
            api_key: String::new(),
            default_target_language: "JA".to_string()
        };
        let json_str = serde_json::to_string(&config).expect("failed to serialize settings");
        f.write_all(json_str.as_bytes()).expect("failed to write config.json");
    }

    // 設定ファイル読み込み
    let mut f = File::open(SETTING_FILEPATH).expect("config.json has not been set");
    let mut s = String::new();
    f.read_to_string(&mut s).expect(format!("{} is empty", SETTING_FILEPATH).as_str());
    let v: Value = serde_json::from_str(&s).expect("failed to deserialize config.json");

    // 各設定項目の取得
    Configure {
        api_key: v["api_key"].to_string().trim_matches('"').to_string(),
        default_target_language: v["default_target_language"].to_string().trim_matches('"').to_string()
    }
}

/// APIキーの設定  
/// 設定ファイルconfig.jsonにAPIキーを設定する。
pub fn set_apikey(api_key: String) -> Result<(), io::Error> {
    let mut settings = get_settings();
    settings.api_key = api_key;
    let json_str = serde_json::to_string(&settings)?;
    let mut f = File::create(SETTING_FILEPATH)?;
    f.write_all(json_str.as_bytes())?;

    Ok(())
}

/// デフォルトの翻訳先言語の設定  
/// 設定ファイルconfig.jsonにデフォルトの翻訳先言語を設定する。
pub fn set_default_target_language(default_target_language: String) -> Result<(), io::Error> {
    let mut settings = get_settings();
    settings.default_target_language = default_target_language;
    let json_str = serde_json::to_string(&settings)?;
    let mut f = File::create(SETTING_FILEPATH)?;
    f.write_all(json_str.as_bytes())?;

    Ok(())
}

/// 設定の初期化
pub fn clear_settings() -> Result<(), io::Error> {
    let mut f = File::create(SETTING_FILEPATH)?;
    let settings = Configure {
        api_key: String::new(),
        default_target_language: "JA".to_string()
    };
    let json_str = serde_json::to_string(&settings)?;
    f.write_all(json_str.as_bytes())?;

    Ok(())
}
