use serde::{Deserialize, Serialize};
use confy;
use confy::ConfyError;

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
fn get_settings() -> Result<Configure, ConfyError> {
    confy::load::<Configure>("dptran", "configure")
}

/// APIキーの設定  
/// 設定ファイルにAPIキーを設定する。
pub fn set_apikey(api_key: String) -> Result<(), ConfyError> {
    let mut settings = get_settings()?;
    settings.api_key = api_key;
    confy::store("dptran", "configure", settings)?;
    Ok(())
}

/// デフォルトの翻訳先言語の設定  
/// 設定ファイルにデフォルトの翻訳先言語を設定する。
pub fn set_default_target_language(default_target_language: String) -> Result<(), ConfyError> {
    let mut settings = get_settings()?;
    settings.default_target_language = default_target_language;
    confy::store("dptran", "configure", settings)?;
    Ok(())
}

/// 設定の初期化
pub fn clear_settings() -> Result<(), ConfyError> {
    let settings = Configure::default();
    confy::store("dptran", "configure", settings)?;
    Ok(())
}

/// 設定済みの既定の翻訳先言語コードを取得
pub fn get_default_target_language_code() -> core::result::Result<String, ConfyError> {
    let settings = get_settings()?;
    Ok(settings.default_target_language)
}

/// APIキーを取得
pub fn get_api_key() -> core::result::Result<String, ConfyError> {
    let settings = get_settings()?;
    Ok(settings.api_key)
}
