use std::io;

use super::DpTran;

use super::connection;
use connection::ConnectionError;
use super::DeeplAPIError;
use super::ApiKeyType;

pub const DEEPL_API_TRANSLATE: &str = "https://api-free.deepl.com/v2/translate";
pub const DEEPL_API_TRANSLATE_PRO: &str = "https://api.deepl.com/v2/translate";

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

/// Translation  
/// Returns an error if it fails.
fn request_translate(api: &DpTran, text: &Vec<String>, target_lang: &String, source_lang: &Option<String>) -> Result<String, ConnectionError> {
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
        let t =  urlencoding::encode(t);
        query = format!("{}&text={}", query, t);
    }
    
    connection::post(url, query)
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

/// To run these tests, you need to set the environment variable `DPTRAN_DEEPL_API_KEY` to your DeepL API key.  
/// You should run these tests with ``cargo test -- --test-threads=1`` because the DeepL API has a limit on the number of requests per second.
/// And also, you need to run the dummy server for the DeepL API to test the API endpoints.
///   $ pip3 install -r requirements.txt
///   $ uvicorn dummy-api-server:app --reload
#[cfg(test)]
pub mod tests {
    use super::*;

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
