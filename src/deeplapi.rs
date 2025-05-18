use std::io;
use std::fmt;
use serde_json::Value;

mod connection;
pub use connection::ConnectionError;

const DEEPL_API_TRANSLATE: &str = "https://api-free.deepl.com/v2/translate";
const DEEPL_API_USAGE: &str = "https://api-free.deepl.com/v2/usage";
const DEEPL_API_LANGUAGES: &str = "https://api-free.deepl.com/v2/languages";

/// Language code and language name
pub type LangCodeName = (String, String);

#[derive(Debug, PartialEq)]
enum LangType {
    Source,
    Target,
}

/// Extended language codes and names.
/// Because DeepL API's ``/languages`` endpoint returns only the language codes that support document translation,
/// although only text translation is supported. Additionally, if the language code is unspecified variant, it is not returned.
/// Therefore, dptran adds the following language codes and names manually.
/// This constants must be updated when the DeepL API is updated.
/// See <https://developers.deepl.com/docs/resources/supported-languages>.

static EXTENDED_LANG_CODES: [(&str, &str, LangType); 3] = [
    ("EN", "English", LangType::Target),
    ("PT", "Portuguese", LangType::Target),
    ("ZH-HANT", "Chinese (traditional)", LangType::Target)
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
fn request_translate(auth_key: &String, text: &Vec<String>, target_lang: &String, source_lang: &Option<String>) -> Result<String, connection::ConnectionError> {
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
pub fn translate(api_key: &String, text: &Vec<String>, target_lang: &String, source_lang: &Option<String>) -> Result<Vec<String>, DeeplAPIError> {
    let auth_key = api_key;

    // Get json of translation result with request_translate().
    let res = request_translate(&auth_key, text, target_lang, source_lang);
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

    let lang_type = if type_name == "source" { LangType::Source } else { LangType::Target };

    let mut lang_codes: Vec<LangCodeName> = Vec::new();
    // Add got language codes
    for value in v.as_array().expect("Invalid response at get_language_codes") {
        value.get("language").ok_or("Invalid response".to_string()).map_err(|e| DeeplAPIError::JsonError(e.to_string()))?;
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

#[test]
/// run with `cargo test api_tests -- <api_key> <DeepL API free = 0, DeepL API pro = 1>`
/// arg[2] : api_key
/// arg[3] : DeepL API free = 0, DeepL API pro = 1
fn api_tests() {
    if std::env::args().len() < 3 {
        panic!("Usage: cargo test api_tests -- <api_key> <DeepL API free = 0, DeepL API pro = 1>");
    }

    let mut args = Vec::new();
    for arg in std::env::args().skip(2) {
        println!("arg: {}", arg);
        args.push(arg);
    }

    // translate test
    let api_key = &args[0];
    let text = vec!["Hello, World!".to_string()];
    let target_lang = "JA".to_string();
    let source_lang = None;
    let res = translate(api_key, &text, &target_lang, &source_lang);
    match res {
        Ok(res) => {
            //assert_eq!(res[0], "ハロー、ワールド！");
            println!("res: {}", res[0]);
        },
        Err(e) => {
            panic!("Error: {}", e);
        }
    }

    // usage test
    let res = get_usage(api_key);
    match res {
        Ok(res) => {
            // If you have a pro account, it is not an error.
            if args[1] == "0" && res.1 != 500000 {
                panic!("Error: usage limit is not 50000");
            }
            if args[1] == "1" && res.1 != 0 {
                panic!("Error: usage limit is not 0");
            }
        },
        Err(e) => {
            panic!("Error: {}", e);
        }
    }

    // get_language_codes test
    let res = get_language_codes(api_key, "source".to_string());
    match res {
        Ok(res) => {
            if res.len() == 0 {
                panic!("Error: language codes is empty");
            }
        },
        Err(e) => {
            panic!("Error: {}", e);
        }
    }
}

#[test]
fn json_to_vec_test() {
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

#[test]
fn error_test() {
    // no api_key
    let text = vec!["Hello, World!".to_string()];
    let target_lang = "JA".to_string();
    let source_lang = None;
    let res = translate(&"".to_string(), &text, &target_lang, &source_lang);
    match res {
        Ok(_) => {
            panic!("Error: translation success");
        },
        Err(e) => {
            if e != DeeplAPIError::ConnectionError(connection::ConnectionError::Forbidden) {
                panic!("Error: wrong error: {}", e);
            }
        }
    }
}

