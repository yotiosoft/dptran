use std::io;
use std::fmt;
use serde_json::Value;

use super::DpTran;

mod connection;
pub use connection::ConnectionError;

#[cfg(not(test))]
pub const DEEPL_API_TRANSLATE: &str = "https://api-free.deepl.com/v2/translate";
#[cfg(not(test))]
pub const DEEPL_API_TRANSLATE_PRO: &str = "https://api.deepl.com/v2/translate";
#[cfg(not(test))]
pub const DEEPL_API_USAGE: &str = "https://api-free.deepl.com/v2/usage";
#[cfg(not(test))]
pub const DEEPL_API_USAGE_PRO: &str = "https://api.deepl.com/v2/usage";
#[cfg(not(test))]
pub const DEEPL_API_LANGUAGES: &str = "https://api-free.deepl.com/v2/languages";
#[cfg(not(test))]
pub const DEEPL_API_LANGUAGES_PRO: &str = "https://api.deepl.com/v2/languages";

#[cfg(test)]
pub const DEEPL_API_TRANSLATE: &str = "http://localhost:8000/free/v2/translate";
#[cfg(test)]
pub const DEEPL_API_TRANSLATE_PRO: &str = "http://localhost:8000/pro/v2/translate";
#[cfg(test)]
pub const DEEPL_API_USAGE: &str = "http://localhost:8000/free/v2/usage";
#[cfg(test)]
pub const DEEPL_API_USAGE_PRO: &str = "http://localhost:8000/pro/v2/usage";
#[cfg(test)]
pub const DEEPL_API_LANGUAGES: &str = "http://localhost:8000/free/v2/languages";
#[cfg(test)]
pub const DEEPL_API_LANGUAGES_PRO: &str = "http://localhost:8000/pro/v2/languages";

pub const UNLIMITED_CHARACTERS_NUMBER: u64 = 1000000000000;  // DeepL Pro API has no character limit, but the API returns a character limit of 1000000000000 characters as a default value.

/// Language code and language name
pub type LangCodeName = (String, String);

#[derive(Debug, PartialEq)]
enum LangType {
    Source,
    Target,
}

/// DeepL API key type.
/// DeepL API servers Free and Pro plans, but the endpoints are different.
/// So we need to distinguish between the two types of API keys.
/// ``Free``: Free API key, which has a character limit of 500,000 characters per month.
/// ``Pro``: Pro API key, which has no character limit.
#[derive(Debug, PartialEq, Clone)]
pub enum ApiKeyType {
    Free,
    Pro,
}

/// Extended language codes and names.  
/// Because DeepL API's ``/languages`` endpoint returns only the language codes that support document translation,
/// although only text translation is supported. Additionally, if the language code is unspecified variant, it is not returned.  
/// Therefore, dptran adds the following language codes and names manually.  
/// This constants must be updated when the DeepL API is updated.  
/// See <https://developers.deepl.com/docs/resources/supported-languages>.

static EXTENDED_LANG_CODES: [(&str, &str, LangType); 2] = [
    ("EN", "English", LangType::Target),
    ("PT", "Portuguese", LangType::Target),
];

/// DeepL API error.  
/// ``ConnectionError``: Connection error occurred in the process of sending and receiving data.  
/// ``JsonError``: Error occurred while parsing json.  
/// ``LimitError``: The translation limit of your account has been reached. Consider upgrading your subscription.  
/// ``GetLanguageCodesError``: Could not get language codes.  
#[derive(Debug, PartialEq)]
pub enum DeeplAPIError {
    ConnectionError(ConnectionError),
    JsonError(String),
    WrongEndPointError(String),
    LimitError,
    GetLanguageCodesError,
}
impl fmt::Display for DeeplAPIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeeplAPIError::ConnectionError(ref e) => write!(f, "Connection error: {}", e),
            DeeplAPIError::JsonError(ref e) => write!(f, "JSON error: {}", e),
            DeeplAPIError::WrongEndPointError(ref e) => write!(f, "Wrong endpoint error. Please check your API key type such as Free or Pro. {}", e),
            DeeplAPIError::LimitError => write!(f, "The translation limit of your account has been reached. Consider upgrading your subscription."),
            DeeplAPIError::GetLanguageCodesError => write!(f, "Could not get language codes"),
        }
    }
}

/// Translation  
/// Returns an error if it fails.
fn request_translate(api: &DpTran, text: &Vec<String>, target_lang: &String, source_lang: &Option<String>) -> Result<String, connection::ConnectionError> {
    let url = if api.api_key_type == ApiKeyType::Free {
        api.api_urls.translate_for_free.clone()
    } else {
        api.api_urls.translate_for_pro.clone()
    };
    let mut query = if source_lang.is_none() {
        format!("auth_key={}&target_lang={}", api.api_key, target_lang)
    } else {
        format!("auth_key={}&target_lang={}&source_lang={}", api.api_key, target_lang, source_lang.as_ref().unwrap())
    };
    
    for t in text {
        query = format!("{}&text={}", query, t);
    }
    
    connection::post(url, query)
}

/// Return translation results.  
/// Receive translation results in json format and display translation results.  
/// Return error if json parsing fails.
pub fn translate(api: &DpTran, text: &Vec<String>, target_lang: &String, source_lang: &Option<String>) -> Result<Vec<String>, DeeplAPIError> {
    // Get json of translation result with request_translate().
    let res = request_translate(api, text, target_lang, source_lang);
    match res {
        Ok(res) => {
            json_to_vec(&res)
        },
        // Error message if translation result is not successful
        // DeepL If the API is an error code with a specific meaning, detect it here
        // https://www.deepl.com/en/docs-api/api-access/error-handling/
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

/// Parses the translation results passed in json format,
/// stores the translation in a vector, and returns it.
fn json_to_vec(json: &String) -> Result<Vec<String>, DeeplAPIError> {
    let json: serde_json::Value = serde_json::from_str(&json).map_err(|e| DeeplAPIError::JsonError(e.to_string()))?;
    json.get("translations").ok_or(io::Error::new(io::ErrorKind::Other, format!("Invalid response: {}", json))).map_err(|e| DeeplAPIError::JsonError(e.to_string()))?;
    let translations = &json["translations"];

    let mut translated_texts = Vec::new();
    for translation in translations.as_array().expect("failed to get array") {
        let len = translation["text"].to_string().len();
        let translation_trimmed= translation["text"].to_string()[1..len-1].to_string();
        translated_texts.push(translation_trimmed);
    }

    Ok(translated_texts)
}

/// Get the number of characters remaining to be translated.  
/// Retrieved from <https://api-free.deepl.com/v2/usage>.  
/// Returns an error if acquisition fails.  
pub fn get_usage(api: &DpTran) -> Result<(u64, u64), DeeplAPIError> {
    let url = if api.api_key_type == ApiKeyType::Free {
        api.api_urls.usage_for_free.clone()
    } else {
        api.api_urls.usage_for_pro.clone()
    };
    let query = format!("auth_key={}", api.api_key);
    let res = connection::post(url, query).map_err(|e| DeeplAPIError::ConnectionError(e))?;
    let v: Value = serde_json::from_str(&res).map_err(|e| DeeplAPIError::JsonError(e.to_string()))?;

    v.get("character_count").ok_or(format!("failed to get character_count: {}", v).to_string()).map_err(|e| DeeplAPIError::JsonError(e.to_string()))?;
    v.get("character_limit").ok_or(format!("failed to get character_limit: {}", v).to_string()).map_err(|e| DeeplAPIError::JsonError(e.to_string()))?;

    let character_count = v["character_count"].as_u64().expect("failed to get character_count");
    let character_limit = v["character_limit"].as_u64().expect("failed to get character_limit");
    Ok((character_count, character_limit))
}

/// Get language code list  
/// Retrieved from <https://api-free.deepl.com/v2/languages>.  
pub fn get_language_codes(api: &DpTran, type_name: String) -> Result<Vec<LangCodeName>, DeeplAPIError> {
    let url = if api.api_key_type == ApiKeyType::Free {
        api.api_urls.languages_for_free.clone()
    } else {
        api.api_urls.languages_for_pro.clone()
    };
    let query = format!("type={}&auth_key={}", type_name, api.api_key);
    let res = connection::post(url, query).map_err(|e| DeeplAPIError::ConnectionError(e))?;
    let v: Value = serde_json::from_str(&res).map_err(|e| DeeplAPIError::JsonError(e.to_string()))?;

    let lang_type = if type_name == "source" { LangType::Source } else { LangType::Target };

    let mut lang_codes: Vec<LangCodeName> = Vec::new();
    let v_array = v.as_array();
    if let None = v_array {
        if v.to_string().contains("Wrong endpoint") {
            return Err(DeeplAPIError::WrongEndPointError(v.to_string()));
        }
        return Err(DeeplAPIError::JsonError(v.to_string()));
    }
    // Add got language codes
    for value in v_array.unwrap() {
        value.get("language").ok_or(format!("Invalid response: {}", value)).map_err(|e| DeeplAPIError::JsonError(e.to_string()))?;
        // Remove quotation marks
        let lang_code_with_quote = value["language"].to_string();
        let lang_code = &lang_code_with_quote[1..lang_code_with_quote.len()-1];
        let lang_name_with_quote = value["name"].to_string();
        let lang_name = &lang_name_with_quote[1..lang_name_with_quote.len()-1];
        let lang_code_pair = (lang_code.to_string(), lang_name.to_string());
        lang_codes.push(lang_code_pair);
    }
    // Add extended language codes
    for i in 0..EXTENDED_LANG_CODES.len() {
        if EXTENDED_LANG_CODES[i].2 == lang_type {
            // Check: if the language code is already in the list
            if lang_codes.iter().any(|x| x.0 == EXTENDED_LANG_CODES[i].0 && x.1 == EXTENDED_LANG_CODES[i].1) {
                // If it is already in the list, skip it
                continue;
            }
            lang_codes.push((EXTENDED_LANG_CODES[i].0.to_string(), EXTENDED_LANG_CODES[i].1.to_string()));
        }
    }
    // Sort by language code
    lang_codes.sort_by(|a, b| a.0.cmp(&b.0));
    // return
    if lang_codes.len() == 0 {
        Err(DeeplAPIError::GetLanguageCodesError)
    } else {
        Ok(lang_codes)
    }
}

/// To run these tests, you need to set the environment variable `DPTRAN_DEEPL_API_KEY` to your DeepL API key.  
/// You should run these tests with ``cargo test -- --test-threads=1`` because the DeepL API has a limit on the number of requests per second.
/// And also, you need to run the dummy server for the DeepL API to test the API endpoints.
///   $ pip3 install -r requirements.txt
///   $ uvicorn dummy-api-server:app --reload
#[cfg(test)]
pub mod tests {
    use core::panic;

    use super::*;

    fn retry_or_panic(e: &DeeplAPIError, times: u8) -> bool {
        if e == &DeeplAPIError::ConnectionError(ConnectionError::TooManyRequests) && times < 3 {
            // Because the DeepL API has a limit on the number of requests per second, retry after 2 seconds if the error is TooManyRequests.
            std::thread::sleep(std::time::Duration::from_secs(2));
            return true;
        }
        else {
            panic!("Error: {}", e.to_string());
        }
    }

    pub fn get_api_key() -> (String, ApiKeyType) {
        let api_key_free = std::env::var("DPTRAN_DEEPL_API_KEY");
        let api_key_pro = std::env::var("DPTRAN_DEEPL_API_KEY_PRO");
        if api_key_free.is_ok() {
            return (api_key_free.unwrap(), ApiKeyType::Free);
        }
        else if api_key_pro.is_ok() {
            return (api_key_pro.unwrap(), ApiKeyType::Pro);
        }
        panic!("To run this test, you need to set the environment variable `DPTRAN_DEEPL_API_KEY` or `DPTRAN_DEEPL_API_KEY_PRO` to your DeepL API key.");
    }

    fn impl_api_translate_test(times: u8) {
        // translate test
        let (api_key, api_key_type) = get_api_key();
        let api = DpTran::with(&api_key, api_key_type);
        let text = vec!["Hello, World!".to_string()];
        let target_lang = "JA".to_string();
        let source_lang = None;
        let res = translate(&api, &text, &target_lang, &source_lang);
        match res {
            Ok(res) => {
                assert_eq!(res[0], "ハロー、ワールド！");
            },
            Err(e) => {
                if retry_or_panic(&e, 0) {
                    // retry
                    impl_api_translate_test(times + 1);
                    return;
                }
            }
        }
    }

    fn impl_api_usage_test(times: u8) {
        // usage test
        let (api_key, api_key_type) = get_api_key();
        let api = DpTran::with(&api_key, api_key_type);
        let res = get_usage(&api);
        if res.is_err() {
            if retry_or_panic(&res.err().unwrap(), times) {
                // retry
                impl_api_usage_test(times + 1);
                return;
            }
        }
    }

    fn impl_api_get_language_codes_test(times: u8) {
        // get_language_codes test
        let (api_key, api_key_type) = get_api_key();
        let api = DpTran::with(&api_key, api_key_type);
        // Get language codes for source languages
        let res = get_language_codes(&api, "source".to_string());
        match res {
            Ok(res) => {
                if res.len() == 0 {
                    panic!("Error: language codes is empty");
                }

                // Are there extended language codes?
                let mut found = false;
                for i in 0..EXTENDED_LANG_CODES.len() {
                    if res.iter().any(|x| x.0 == EXTENDED_LANG_CODES[i].0 && x.1 == EXTENDED_LANG_CODES[i].1) {
                        found = true;
                        break;
                    }
                }
                if !found {
                    panic!("Error: extended language codes not found");
                }
            },
            Err(e) => {
                if retry_or_panic(&e, times) {
                    // retry
                    impl_api_get_language_codes_test(times + 1);
                    return;
                }
            }
        }
    }

    #[test]
    fn api_translate_test() {
        // translate test
        impl_api_translate_test(0);
    }

    #[test]
    fn api_usage_test() {
        // usage test
        impl_api_usage_test(0);
    }

    #[test]
    fn api_get_language_codes_test() {
        // get_language_codes test
        impl_api_get_language_codes_test(0);
    }

    #[test]
    fn api_get_api_key_test() {
        // If the environment variable `DPTRAN_DEEPL_API_KEY` or `DPTRAN_DEEPL_API_KEY_PRO` is not set, panic.
        let free_api_key = std::env::var("DPTRAN_DEEPL_API_KEY");
        let pro_api_key = std::env::var("DPTRAN_DEEPL_API_KEY_PRO");

        // First, remove the pro API key from the environment variable to test the free API key.
        if pro_api_key.is_ok() {
            std::env::remove_var("DPTRAN_DEEPL_API_KEY_PRO");
        }
        // If both environment variables are not set, panic.
        if free_api_key.is_err() && pro_api_key.is_err() {
            panic!("To run this test, you need to set the environment variable `DPTRAN_DEEPL_API_KEY` or `DPTRAN_DEEPL_API_KEY_PRO` to your DeepL API key.");
        }
        let (api_key, api_key_type) = get_api_key();
        assert!(free_api_key.is_ok());
        assert_eq!(api_key, free_api_key.as_ref().unwrap().clone());
        assert!(api_key_type == ApiKeyType::Free);

        // Recover the pro API key to test the Pro API key.
        if free_api_key.is_ok() {
            std::env::remove_var("DPTRAN_DEEPL_API_KEY");
        }
        if pro_api_key.is_ok() {
            std::env::set_var("DPTRAN_DEEPL_API_KEY_PRO", pro_api_key.as_ref().unwrap().clone());
            let (api_key, api_key_type) = get_api_key();
            assert!(pro_api_key.is_ok());
            assert_eq!(api_key, pro_api_key.unwrap().clone());
            assert!(api_key_type == ApiKeyType::Pro);
        }
        // Recover the free API key to test the Free API key.
        if free_api_key.is_ok() {
            std::env::set_var("DPTRAN_DEEPL_API_KEY", free_api_key.unwrap().clone());
        }
    }

    #[test]
    fn api_json_to_vec_test() {
        let json = r#"{"translations":[{"detected_source_language":"EN","text":"ハロー、ワールド！"}]}"#.to_string();
        let res = json_to_vec(&json);
        match res {
            Ok(res) => {
                assert_eq!(res[0], "ハロー、ワールド！");
            },
            Err(e) => {
                panic!("Error: {}", e);
            }
        }
    }
}

