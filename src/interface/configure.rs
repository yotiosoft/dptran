use serde::{Deserialize, Serialize};
use confy;

#[derive(Serialize, Deserialize, Debug)]
struct Configure {
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
/// 設定ファイルからAPIキーとデフォルトの翻訳先言語を取得する。  
/// 存在しない場合、既定値を指定して新規作成する。
fn get_settings() -> Result<Configure, String> {
    confy::load::<Configure>("dptran", "configure").map_err(|e| format!("failed to get settings: {}", e))
}

/// APIキーの設定  
/// 設定ファイルにAPIキーを設定する。
pub fn set_api_key(api_key: String) -> Result<(), String> {
    let mut settings = get_settings()?;
    settings.api_key = api_key;
    confy::store("dptran", "configure", settings).map_err(|e| format!("Failed to set API key: {}", e))?;
    Ok(())
}

/// デフォルトの翻訳先言語の設定  
/// 設定ファイルにデフォルトの翻訳先言語を設定する。
pub fn set_default_target_language(default_target_language: &String) -> Result<(), String> {
    let mut settings = get_settings()?;
    settings.default_target_language = default_target_language.to_string();
    confy::store("dptran", "configure", settings).map_err(|e| format!("Failed to set default target language: {}", e))?;
    Ok(())
}

/// 設定の初期化
pub fn clear_settings() -> Result<(), String> {
    let settings = Configure::default();
    confy::store("dptran", "configure", settings).map_err(|e| format!("Failed to clear settings: {}", e))?;
    Ok(())
}

/// 設定済みの既定の翻訳先言語コードを取得
pub fn get_default_target_language_code() -> Result<String, String> {
    let settings = get_settings()?;
    Ok(settings.default_target_language)
}

/// APIキーを取得
pub fn get_api_key() -> Result<Option<String>, String> {
    let settings = get_settings()?;
    if settings.api_key.is_empty() {
        return Ok(None);
    }
    Ok(Some(settings.api_key))
}
