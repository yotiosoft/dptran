use super::DpTran;

use super::connection;
use connection::ConnectionError;
use super::DeeplAPIError;
use super::ApiKeyType;

pub const DEEPL_API_TRANSLATE: &str = "https://api-free.deepl.com/v2/translate";
pub const DEEPL_API_TRANSLATE_PRO: &str = "https://api.deepl.com/v2/translate";

/// Request translation structure
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TranslateRequest {
    pub text: Vec<String>,                      // Required 
    pub target_lang: String,                    // Required
    pub source_lang: Option<String>,
    pub context: Option<String>,
    pub show_billed_characters: Option<bool>,
    pub split_sentences: Option<String>,        // "0", "1" (default), "nonewlines". Default is "1"
    pub preserve_formatting: Option<bool>,      // Default is false
    pub formality: Option<String>,              // "default", "more", "less", "prefer_more", "prefer_less". Default is "default"
    pub model_type: Option<String>,             // "quality_optimized", "prefer_quality_optimized", "latency_optimized"
    pub glossary_id: Option<String>,
    pub tag_handling: Option<String>,           // "xml", "html"
    pub outline_detection: Option<bool>,        // Default is true
    pub non_splitting_tags: Option<Vec<String>>,
    pub splitting_tags: Option<Vec<String>>,
    pub ignore_tags: Option<Vec<String>>,
}

/// Translation item structure
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Translation {
    pub detected_source_language: String,
    pub text: String,
    pub billed_characters: Option<u64>,
    pub model_type_used: Option<String>,
}

/// Translation response structure
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TranslateResponse {
    pub translations: Vec<Translation>,
}

impl TranslateRequest {
    /// Create a new TranslateRequest.
    pub fn new(text: &Vec<String>, target_lang: &String, source_lang: &Option<String>) -> TranslateRequest {
        TranslateRequest {
            text: text.clone(),
            target_lang: target_lang.clone(),
            source_lang: source_lang.clone(),
            context: None,
            show_billed_characters: None,
            split_sentences: None,
            preserve_formatting: None,
            formality: None,
            model_type: None,
            glossary_id: None,
            tag_handling: None,
            outline_detection: None,
            non_splitting_tags: None,
            splitting_tags: None,
            ignore_tags: None,
        }
    }

    /// Return translation results.  
    /// Receive translation results in json format and display translation results.  
    /// Return error if json parsing fails.
    pub fn translate(&self, api: &DpTran) -> Result<TranslateResponse, DeeplAPIError> {
        // Get json of translation result with request_translate().
        let res = self.request_translate(api);
        match res {
            Ok(res) => {
                // Parse json to TranslateResponse struct
                let translate_response: TranslateResponse = serde_json::from_str(&res).map_err(|e| DeeplAPIError::JsonError(e.to_string()))?;
                Ok(translate_response)
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
    fn request_translate(&self, api: &DpTran) -> Result<String, ConnectionError> {
        let url = if api.api_key_type == ApiKeyType::Free {
            api.api_urls.translate_for_free.clone()
        } else {
            api.api_urls.translate_for_pro.clone()
        };
        let mut query = if self.source_lang.is_none() {
            format!("auth_key={}&target_lang={}", api.api_key, self.target_lang)
        } else {
            format!("auth_key={}&target_lang={}&source_lang={}", api.api_key, self.target_lang, self.source_lang.as_ref().unwrap())
        };

        for t in &self.text {
            let t =  urlencoding::encode(t);
            query = format!("{}&text={}", query, t);
        }
        
        connection::post(url, query)
    }
}

impl TranslateResponse {
    /// Parses the translation results passed in json format,
    /// stores the translation in a vector, and returns it.
    pub fn get_translation_strings(&self) -> Result<Vec<String>, DeeplAPIError> {
        let mut translated_texts: Vec<String> = Vec::new();
        for t in &self.translations {
            translated_texts.push(t.text.clone());
        }
        Ok(translated_texts)
    }
}

/// To run these tests, you need to set the environment variable `DPTRAN_DEEPL_API_KEY` to your DeepL API key.  
/// You should run these tests with ``cargo test -- --test-threads=1`` because the DeepL API has a limit on the number of requests per second.
/// And also, you need to run the dummy server for the DeepL API to test the API endpoints.
///   $ pip3 install -r requirements.txt
///   $ uvicorn dummy-api-server:app --reload
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_get_translation_strings() {
        let json = TranslateResponse {
            translations: vec![
                Translation {
                    detected_source_language: "EN".to_string(),
                    text: "ハロー、ワールド！".to_string(),
                    billed_characters: None,
                    model_type_used: None,
                }
            ]
        };
        let res = json.get_translation_strings();
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
