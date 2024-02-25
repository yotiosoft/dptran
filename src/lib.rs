use std::io;

mod deeplapi;

pub use deeplapi::LangCode;

#[derive(Debug)]
pub enum DpTranError {
    ConfigError(String),
    DeeplApiError(String),
    InvalidLanguageCode,
    ApiKeyIsNotSet,
    NoTargetLanguageSpecified,
    CouldNotGetInputText,
}
impl ToString for DpTranError {
    fn to_string(&self) -> String {
        match self {
            DpTranError::ConfigError(e) => format!("Config error: {}", e),
            DpTranError::DeeplApiError(e) => format!("Deepl API error: {}", e),
            DpTranError::InvalidLanguageCode => "Invalid language code".to_string(),
            DpTranError::ApiKeyIsNotSet => "API key is not set".to_string(),
            DpTranError::NoTargetLanguageSpecified => "No target language specified".to_string(),
            DpTranError::CouldNotGetInputText => "Could not get input text".to_string(),
        }
    }
}

pub struct DpTranUsage {
    pub character_count: u64,
    pub character_limit: u64,
}

/// 言語コード一覧の取得  
/// <https://api-free.deepl.com/v2/languages>から取得する
pub fn get_language_codes(api_key: &String, type_name: String) -> Result<Vec<LangCode>, DpTranError> {
    let lang_codes = deeplapi::get_language_codes(&api_key, type_name).map_err(|e| DpTranError::DeeplApiError(e.to_string()))?;
    Ok(lang_codes)
}

/// 言語コードの有効性をチェック
pub fn check_language_code(api_key: &String, lang_code: &String, type_name: String) -> Result<bool, DpTranError> {
    let lang_codes = get_language_codes(api_key, type_name)?;
    for lang in lang_codes {
        if lang.0.trim_matches('"') == lang_code.to_uppercase() {
            return Ok(true);
        }
    }
    Ok(false)
}

/// 正しい言語コードに変換
pub fn correct_language_code(api_key: &String, language_code: &str) -> Result<String, DpTranError> {
    // EN, PTは変換
    let language_code_uppercase = match language_code.to_ascii_uppercase().as_str() {
        "EN" => "EN-US".to_string(),
        "PT" => "PT-PT".to_string(),
        _ => language_code.to_ascii_uppercase(),
    };

    match check_language_code(api_key, &language_code_uppercase, "target".to_string())? {
        true => Ok(language_code_uppercase),
        false => Err(DpTranError::InvalidLanguageCode),
    }
}

/// 翻訳可能な残り文字数の取得
/// <https://api-free.deepl.com/v2/usage>より取得する  
/// 取得に失敗したらエラーを返す
pub fn get_usage(api_key: &String) -> Result<DpTranUsage, DpTranError> {
    let (count, limit) = deeplapi::get_usage(&api_key).map_err(|e| DpTranError::DeeplApiError(e.to_string()))?;
    Ok(DpTranUsage {
        character_count: count,
        character_limit: limit,
    })
}

/// 翻訳結果の表示  
/// json形式の翻訳結果を受け取り、翻訳結果を表示する  
/// jsonのパースに失敗したらエラーを返す
pub fn translate(api_key: &String, text: Vec<String>, target_lang: &String, source_lang: &Option<String>) -> Result<Vec<String>, DpTranError> {
    deeplapi::translate(&api_key, text, target_lang, source_lang).map_err(|e| DpTranError::DeeplApiError(e.to_string()))
}
