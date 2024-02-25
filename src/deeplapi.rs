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

/// 翻訳  
/// 失敗したらエラーを返す
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

/// json形式で渡された翻訳結果をパースし、ベクタに翻訳文を格納して返す
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

/// 翻訳結果の表示  
/// json形式の翻訳結果を受け取り、翻訳結果を表示する  
/// jsonのパースに失敗したらエラーを返す
pub fn translate(api_key: &String, text: Vec<String>, target_lang: &String, source_lang: &Option<String>) -> Result<Vec<String>, DeeplAPIError> {
    let auth_key = api_key;

    // request_translate()で翻訳結果のjsonを取得
    let res = request_translate(&auth_key, text, target_lang, source_lang);
    match res {
        Ok(res) => {
            json_to_vec(&res)
        },
        // 翻訳結果が失敗ならエラー表示
        // DeepL APIが特有の意味を持つエラーコードであればここで検知
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

/// 翻訳可能な残り文字数の取得
/// <https://api-free.deepl.com/v2/usage>より取得する  
/// 取得に失敗したらエラーを返す
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

/// 言語コード一覧の取得  
/// <https://api-free.deepl.com/v2/languages>から取得する
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
