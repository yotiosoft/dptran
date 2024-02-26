use std::io;

mod deeplapi;

pub use deeplapi::LangCodeName;

/// string as language code
pub type LangCode = String;

/// Errors that can occur in this library.
/// ConfigError: Configuration error
/// DeeplApiError: DeepL API error
/// StdIoError: Standard I/O error
/// InvalidLanguageCode: Invalid language code
/// ApiKeyIsNotSet: API key is not set
/// NoTargetLanguageSpecified: No target language specified
/// CouldNotGetInputText: Could not get input text
#[derive(Debug)]
pub enum DpTranError {
    ConfigError(String),
    DeeplApiError(String),
    StdIoError(io::Error),
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
            DpTranError::StdIoError(e) => format!("Standard I/O error: {}", e),
            DpTranError::InvalidLanguageCode => "Invalid language code".to_string(),
            DpTranError::ApiKeyIsNotSet => "API key is not set".to_string(),
            DpTranError::NoTargetLanguageSpecified => "No target language specified".to_string(),
            DpTranError::CouldNotGetInputText => "Could not get input text".to_string(),
        }
    }
}

/// Target / Source language types
/// used in get_language_codes()
pub enum LangType {
    Target,
    Source,
}

/// DeepL API usage information
/// character_count: Number of characters translated this month
/// character_limit: Maximum number of characters that can be translated this month
/// If character_limit is 0, it is unlimited
pub struct DpTranUsage {
    pub character_count: u64,
    pub character_limit: u64,
    pub unlimited: bool,
}

/// Get language code list. Using DeepL API.
/// Retrieved from <https://api-free.deepl.com/v2/languages>.
/// api_key: DeepL API key
/// lang_type: Target or Source
pub fn get_language_codes(api_key: &String, lang_type: LangType) -> Result<Vec<LangCodeName>, DpTranError> {
    let type_name = match lang_type {
        LangType::Target => "target".to_string(),
        LangType::Source => "source".to_string(),
    };
    let lang_codes = deeplapi::get_language_codes(&api_key, type_name).map_err(|e| DpTranError::DeeplApiError(e.to_string()))?;
    Ok(lang_codes)
}

/// Check the validity of language codes. Using DeepL API.
/// api_key: DeepL API key
/// lang_code: Language code to check
/// lang_type: Target or Source
pub fn check_language_code(api_key: &String, lang_code: &String, lang_type: LangType) -> Result<bool, DpTranError> {
    let lang_codes = get_language_codes(api_key, lang_type)?;
    for lang in lang_codes {
        if lang.0.trim_matches('"') == lang_code.to_uppercase() {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Convert to correct language code from input language code string. Using DeepL API.
/// api_key: DeepL API key
/// language_code: Language code to convert
pub fn correct_language_code(api_key: &String, language_code: &str) -> Result<LangCode, DpTranError> {
    // EN, PTは変換
    let language_code_uppercase = match language_code.to_ascii_uppercase().as_str() {
        "EN" => "EN-US".to_string(),
        "PT" => "PT-PT".to_string(),
        _ => language_code.to_ascii_uppercase(),
    };

    match check_language_code(api_key, &language_code_uppercase, LangType::Target)? {
        true => Ok(language_code_uppercase),
        false => Err(DpTranError::InvalidLanguageCode),
    }
}

/// Get the number of characters remaining to be translated. Using DeepL API.
/// Retrieved from <https://api-free.deepl.com/v2/usage>.
/// Returns an error if acquisition fails.
/// api_key: DeepL API key
pub fn get_usage(api_key: &String) -> Result<DpTranUsage, DpTranError> {
    let (count, limit) = deeplapi::get_usage(&api_key).map_err(|e| DpTranError::DeeplApiError(e.to_string()))?;
    Ok(DpTranUsage {
        character_count: count,
        character_limit: limit,
        unlimited: limit == 0,
    })
}

/// Display translation results. Using DeepL API.
/// Receive translation results in json format and display translation results.
/// Return error if json parsing fails.
/// api_key: DeepL API key
/// text: Text to translate
/// target_lang: Target language
/// source_lang: Source language (optional)
pub fn translate(api_key: &String, text: Vec<String>, target_lang: &String, source_lang: &Option<String>) -> Result<Vec<String>, DpTranError> {
    deeplapi::translate(&api_key, text, target_lang, source_lang).map_err(|e| DpTranError::DeeplApiError(e.to_string()))
}
