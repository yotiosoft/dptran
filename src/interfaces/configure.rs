use std::io;
use serde::{Deserialize, Serialize};
use confy;
use confy::ConfyError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Configure {
    pub api_key: String,
    pub default_target_language: String
}
impl Default for Configure {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            default_target_language: "JA".to_string()
        }
    }
}

/// 設定ファイルの読み込みと値の抽出  
/// 設定ファイルconfig.jsonからAPIキーとデフォルトの翻訳先言語を取得する。  
/// 存在しない場合、既定値を指定して新規作成する。
pub fn get_settings() -> Result<Configure, ConfyError> {
    confy::load::<Configure>("dptran", "configure")
}

/// APIキーの設定  
/// 設定ファイルconfig.jsonにAPIキーを設定する。
pub fn set_apikey(api_key: String) -> Result<(), io::Error> {
    let mut settings = get_settings();
    match settings {
        Ok(ref mut settings) => {
            settings.api_key = api_key;
            confy::store("dptran", "configure", settings).expect("Failed to save configure");
        }
        Err(e) => {
            return Err(io::Error::new(io::ErrorKind::Other, e));
        }
    }
    Ok(())
}

/// デフォルトの翻訳先言語の設定  
/// 設定ファイルconfig.jsonにデフォルトの翻訳先言語を設定する。
pub fn set_default_target_language(default_target_language: String) -> Result<(), io::Error> {
    let mut settings = get_settings();
    match settings {
        Ok(ref mut settings) => {
            settings.default_target_language = default_target_language;
            confy::store("dptran", "configure", settings).expect("Failed to save configure");
        }
        Err(e) => {
            return Err(io::Error::new(io::ErrorKind::Other, e));
        }
    }
    Ok(())
}

/// 設定の初期化
pub fn clear_settings() -> Result<(), io::Error> {
    let mut settings = get_settings();
    match settings {
        Ok(ref mut settings) => {
            *settings = Configure::default();
            confy::store("dptran", "configure", settings).expect("Failed to save configure");
        }
        Err(e) => {
            return Err(io::Error::new(io::ErrorKind::Other, e));
        }
    }
    Ok(())
}
