use std::io;
use std::fmt;
use serde_json::Value;

mod connection;

const DEEPL_API_TRANSLATE: &str = "https://api-free.deepl.com/v2/translate";
const DEEPL_API_USAGE: &str = "https://api-free.deepl.com/v2/usage";
const DEEPL_API_LANGUAGES: &str = "https://api-free.deepl.com/v2/languages";

/// Language code and language name
pub type LangCodeName = (String, String);

#[derive(Debug, PartialEq)]
pub enum DeeplAPIError {
    ConnectionError(connection::ConnectionError),
    JsonError(String),
    LimitError,
    GetLanguageCodesError,
}
impl fmt::Display for DeeplAPIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeeplAPIError::ConnectionError(ref e) => write!(f, "Connection error: {}", e),
            DeeplAPIError::JsonError(ref e) => write!(f, "JSON error: {}", e),
            DeeplAPIError::LimitError => write!(f, "The translation limit of your account has been reached. Consider upgrading your subscription."),
            DeeplAPIError::GetLanguageCodesError => write!(f, "Could not get language codes"),
        }
    }
}

/// Translation
/// Returns an error if it fails
fn request_translate(auth_key: &String, text: Vec<String>, target_lang: &String, source_lang: &Option<String>) -> Result<String, connection::ConnectionError> {
    let url = DEEPL_API_TRANSLATE.to_string();
    let mut query = if source_lang.is_none() {
        format!("auth_key={}&target_lang={}", auth_key, target_lang)
    } else {
        format!("auth_key={}&target_lang={}&source_lang={}", auth_key, target_lang, source_lang.as_ref().unwrap())
    };

    for t in text {
        query = format!("{}&text={}", query, t);
    }
    
    connection::send_and_get(url, query)
}

/// Parses the translation results passed in json format,
///   stores the translation in a vector, and returns it.
fn json_to_vec(json: &String) -> Result<Vec<String>, DeeplAPIError> {
    let json: serde_json::Value = serde_json::from_str(&json).map_err(|e| DeeplAPIError::JsonError(e.to_string()))?;
    json.get("translations").ok_or(io::Error::new(io::ErrorKind::Other, "Invalid response")).map_err(|e| DeeplAPIError::JsonError(e.to_string()))?;
    let translations = &json["translations"];

    let mut translated_texts = Vec::new();
    for translation in translations.as_array().expect("failed to get array") {
        let len = translation["text"].to_string().len();
        let translation_trimmed= translation["text"].to_string()[1..len-1].to_string();
        translated_texts.push(translation_trimmed);
    }

    Ok(translated_texts)
}

/// Return translation results.
/// Receive translation results in json format and display translation results.
/// Return error if json parsing fails.
pub fn translate(api_key: &String, text: Vec<String>, target_lang: &String, source_lang: &Option<String>) -> Result<Vec<String>, DeeplAPIError> {
    let auth_key = api_key;

    // Get json of translation result with request_translate().
    let res = request_translate(&auth_key, text, target_lang, source_lang);
    match res {
        Ok(res) => {
            json_to_vec(&res)
        },
        // Error message if translation result is not successful
        // DeepL If the API is an error code with a specific meaning, detect it here
        // https://www.deepl.com/ja/docs-api/api-access/error-handling/
        Err(e) => {
            if e == connection::ConnectionError::UnprocessableEntity {  // 456 Unprocessable Entity -> limit reached
                Err(DeeplAPIError::LimitError)
            }
            else {
                Err(DeeplAPIError::ConnectionError(e))
            }
        }
    }
}

/// Get the number of characters remaining to be translated.
/// Retrieved from <https://api-free.deepl.com/v2/usage>.
/// Returns an error if acquisition fails.
pub fn get_usage(api_key: &String) -> Result<(u64, u64), DeeplAPIError> {
    let url = DEEPL_API_USAGE.to_string();
    let query = format!("auth_key={}", api_key);
    let res = connection::send_and_get(url, query).map_err(|e| DeeplAPIError::ConnectionError(e))?;
    let v: Value = serde_json::from_str(&res).map_err(|e| DeeplAPIError::JsonError(e.to_string()))?;

    v.get("character_count").ok_or("failed to get character_count".to_string()).map_err(|e| DeeplAPIError::JsonError(e.to_string()))?;
    v.get("character_limit").ok_or("failed to get character_limit".to_string()).map_err(|e| DeeplAPIError::JsonError(e.to_string()))?;

    let character_count = v["character_count"].as_u64().expect("failed to get character_count");
    let character_limit = v["character_limit"].as_u64().expect("failed to get character_limit");
    Ok((character_count, character_limit))
}

/// Get language code list
/// Retrieved from <https://api-free.deepl.com/v2/languages>.
pub fn get_language_codes(api_key: &String, type_name: String) -> Result<Vec<LangCodeName>, DeeplAPIError> {
    let url = DEEPL_API_LANGUAGES.to_string();
    let query = format!("type={}&auth_key={}", type_name, api_key);
    let res = connection::send_and_get(url, query).map_err(|e| DeeplAPIError::ConnectionError(e))?;
    let v: Value = serde_json::from_str(&res).map_err(|e| DeeplAPIError::JsonError(e.to_string()))?;

    let mut lang_codes: Vec<LangCodeName> = Vec::new();
    for value in v.as_array().expect("Invalid response at get_language_codes") {
        value.get("language").ok_or("Invalid response".to_string()).map_err(|e| DeeplAPIError::JsonError(e.to_string()))?;
        let lang_code = (value["language"].to_string(), value["name"].to_string());
        lang_codes.push(lang_code);
    }
    if lang_codes.len() == 0 {
        Err(DeeplAPIError::GetLanguageCodesError)
    } else {
        Ok(lang_codes)
    }
}
