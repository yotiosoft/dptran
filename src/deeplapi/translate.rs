use super::DpTran;

use super::connection;
use connection::ConnectionError;
use super::DeeplAPIError;
use super::ApiKeyType;

pub const DEEPL_API_TRANSLATE: &str = "https://api-free.deepl.com/v2/translate";
pub const DEEPL_API_TRANSLATE_PRO: &str = "https://api.deepl.com/v2/translate";

/// Request translation structure
/// See also <https://developers.deepl.com/api-reference/translate/request-translation>
#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct TranslateRequest {
    /// Text to be translated. (Required)
    pub text: Vec<String>,                      // Required 
    /// Target language code. (Required)
    pub target_lang: String,                    // Required
    /// Source language code. (Optional)
    pub source_lang: Option<String>,
    /// Additional context for the translation. (Optional)
    pub context: Option<String>,
    /// Whether to show the number of billed characters in the response. (Optional)
    pub show_billed_characters: Option<bool>,
    /// Sets whether the translation engine should first split the input into sentences. (Optional)
    pub split_sentences: Option<SplitSentences>,// "0", "1" (default), "nonewlines". Default is "1"
    /// Sets whether the translation engine should respect the original formatting, even if it would usually correct some aspects. (Optional)
    pub preserve_formatting: Option<bool>,      // Default is false
    /// Formality level for the translation. (Optional)
    pub formality: Option<Formality>,           // "default", "more", "less", "prefer_more", "prefer_less". Default is "default"
    /// Model type to use for the translation. (Optional)
    pub model_type: Option<ModelType>,          // "quality_optimized", "prefer_quality_optimized", "latency_optimized"
    /// Glossary ID to be used for the translation. (Optional)
    pub glossary_id: Option<String>,
    /// Tag handling mode. (Optional)
    pub tag_handling: Option<TagHandling>,      // "xml", "html"
    /// Disable the automatic detection of XML structure by setting the outline_detection parameter to false. (Optional)
    pub outline_detection: Option<bool>,        // Default is true
    /// Comma-separated list of XML tags which never split sentences. (Optional)
    pub non_splitting_tags: Option<Vec<String>>,
    /// Comma-separated list of XML tags which always cause splits. (Optional)
    pub splitting_tags: Option<Vec<String>>,
    /// Comma-separated list of XML tags that indicate text not to be translated. (Optional)
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

/// Enum for split_sentences option
#[derive(Debug)]
pub enum SplitSentences {
    /// no splitting at all, whole input is treated as one sentence
    False,
    /// splits on punctuation and on newlines
    True,
    /// splits on punctuation only, no splitting at newlines
    NoNewLines,
}
impl serde::Serialize for SplitSentences {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match *self {
            SplitSentences::False => "0",
            SplitSentences::True => "1",
            SplitSentences::NoNewLines => "nonewlines",
        };
        serializer.serialize_str(s)
    }
}
impl<'de> serde::Deserialize<'de> for SplitSentences {
    fn deserialize<D>(deserializer: D) -> Result<SplitSentences, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        match s {
            "0" => Ok(SplitSentences::False),
            "1" => Ok(SplitSentences::True),
            "nonewlines" => Ok(SplitSentences::NoNewLines),
            _ => Err(serde::de::Error::custom(format!("Invalid value for SplitSentences: {}", s))),
        }
    }
}

/// Enum for formality option
#[derive(Debug)]
pub enum Formality {
    Default,
    More,
    Less,
    PreferMore,
    PreferLess,
}
impl serde::Serialize for Formality {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match *self {
            Formality::Default => "default",
            Formality::More => "more",
            Formality::Less => "less",
            Formality::PreferMore => "prefer_more",
            Formality::PreferLess => "prefer_less",
        };
        serializer.serialize_str(s)
    }
}
impl <'de> serde::Deserialize<'de> for Formality {
    fn deserialize<D>(deserializer: D) -> Result<Formality, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        match s {
            "default" => Ok(Formality::Default),
            "more" => Ok(Formality::More),
            "less" => Ok(Formality::Less),
            "prefer_more" => Ok(Formality::PreferMore),
            "prefer_less" => Ok(Formality::PreferLess),
            _ => Err(serde::de::Error::custom(format!("Invalid value for Formality: {}", s))),
        }
    }
}

/// Enum for model_type option
#[derive(Debug)]
pub enum ModelType {
    QualityOptimized,
    PreferQualityOptimized,
    LatencyOptimized,
}
impl serde::Serialize for ModelType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match *self {
            ModelType::QualityOptimized => "quality_optimized",
            ModelType::PreferQualityOptimized => "prefer_quality_optimized",
            ModelType::LatencyOptimized => "latency_optimized",
        };
        serializer.serialize_str(s)
    }
}
impl <'de> serde::Deserialize<'de> for ModelType {
    fn deserialize<D>(deserializer: D) -> Result<ModelType, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        match s {
            "quality_optimized" => Ok(ModelType::QualityOptimized),
            "prefer_quality_optimized" => Ok(ModelType::PreferQualityOptimized),
            "latency_optimized" => Ok(ModelType::LatencyOptimized),
            _ => Err(serde::de::Error::custom(format!("Invalid value for ModelType: {}", s))),
        }
    }
}

/// Enum for tag_handling option
#[derive(Debug)]
pub enum TagHandling {
    Xml,
    Html,
}
impl serde::Serialize for TagHandling {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match *self {
            TagHandling::Xml => "xml",
            TagHandling::Html => "html",
        };
        serializer.serialize_str(s)
    }
}
impl <'de> serde::Deserialize<'de> for TagHandling {
    fn deserialize<D>(deserializer: D) -> Result<TagHandling, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        match s {
            "xml" => Ok(TagHandling::Xml),
            "html" => Ok(TagHandling::Html),
            _ => Err(serde::de::Error::custom(format!("Invalid value for TagHandling: {}", s))),
        }
    }
}

impl TranslateRequest {
    /// Return translation results.  
    /// Receive translation results in json format and display translation results.  
    /// Return error if json parsing fails.
    pub fn translate(&self, api: &DpTran) -> Result<TranslateResponse, DeeplAPIError> {
        // Get json of translation result with request_translate().
        let res = self.request_translate(api);
        match res {
            Ok(res) => {
                // Parse json to TranslateResponse struct
                let translate_response: TranslateResponse = serde_json::from_str(&res)
                    .map_err(|e| DeeplAPIError::JsonError(e.to_string(), res.clone()))?;
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
        
        // Prepare headers
        let header_auth_key = format!("Authorization: DeepL-Auth-Key {}", api.api_key);
        let header_content_type = "Content-Type: application/json";
        let headers = vec![header_auth_key, header_content_type.to_string()];
        let post_data_json = serde_json::to_string(self).unwrap();

        // Send request
        connection::post_with_headers(url, post_data_json, &headers)
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
/// You should run these tests with ``cargo test -- --test-threads=1``
/// because the DeepL API has a limit on the number of requests per second.
/// And also, you need to run the dummy server for the DeepL API to test the API endpoints.
///   $ pip3 install -r requirements.txt
///   $ uvicorn dummy_api_server.main:app --reload
#[cfg(test)]
pub mod tests {
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

    #[test]
    fn api_split_sentences_serialize() {
        let s1 = SplitSentences::False;
        let s2 = SplitSentences::True;
        let s3 = SplitSentences::NoNewLines;
        let res1 = serde_json::to_string(&s1).unwrap();
        let res2 = serde_json::to_string(&s2).unwrap();
        let res3 = serde_json::to_string(&s3).unwrap();
        assert_eq!(res1, r#""0""#);
        assert_eq!(res2, r#""1""#);
        assert_eq!(res3, r#""nonewlines""#);
    }

    #[test]
    fn api_split_sentences_deserialize() {
        let json1 = r#""0""#;
        let json2 = r#""1""#;
        let json3 = r#""nonewlines""#;
        let res1: SplitSentences = serde_json::from_str(json1).unwrap();
        let res2: SplitSentences = serde_json::from_str(json2).unwrap();
        let res3: SplitSentences = serde_json::from_str(json3).unwrap();
        match res1 {
            SplitSentences::False => {},
            _ => panic!("Expected SplitSentences::False"),
        }
        match res2 {
            SplitSentences::True => {},
            _ => panic!("Expected SplitSentences::True"),
        }
        match res3 {
            SplitSentences::NoNewLines => {},
            _ => panic!("Expected SplitSentences::NoNewLines"),
        }
    }

    #[test]
    fn api_formality_serialize() {
        let f1 = Formality::Default;
        let f2 = Formality::More;
        let f3 = Formality::Less;
        let f4 = Formality::PreferMore;
        let f5 = Formality::PreferLess;
        let res1 = serde_json::to_string(&f1).unwrap();
        let res2 = serde_json::to_string(&f2).unwrap();
        let res3 = serde_json::to_string(&f3).unwrap();
        let res4 = serde_json::to_string(&f4).unwrap();
        let res5 = serde_json::to_string(&f5).unwrap();
        assert_eq!(res1, r#""default""#);
        assert_eq!(res2, r#""more""#);
        assert_eq!(res3, r#""less""#);
        assert_eq!(res4, r#""prefer_more""#);
        assert_eq!(res5, r#""prefer_less""#);
    }

    #[test]
    fn api_formality_deserialize() {
        let json1 = r#""default""#;
        let json2 = r#""more""#;
        let json3 = r#""less""#;
        let json4 = r#""prefer_more""#;
        let json5 = r#""prefer_less""#;
        let res1: Formality = serde_json::from_str(json1).unwrap();
        let res2: Formality = serde_json::from_str(json2).unwrap();
        let res3: Formality = serde_json::from_str(json3).unwrap();
        let res4: Formality = serde_json::from_str(json4).unwrap();
        let res5: Formality = serde_json::from_str(json5).unwrap();
        match res1 {
            Formality::Default => {},
            _ => panic!("Expected Formality::Default"),
        }
        match res2 {
            Formality::More => {},
            _ => panic!("Expected Formality::More"),
        }
        match res3 {
            Formality::Less => {},
            _ => panic!("Expected Formality::Less"),
        }
        match res4 {
            Formality::PreferMore => {},
            _ => panic!("Expected Formality::PreferMore"),
        }
        match res5 {
            Formality::PreferLess => {},
            _ => panic!("Expected Formality::PreferLess"),
        }
    }

    #[test]
    fn api_model_type_serialize() {
        let m1 = ModelType::QualityOptimized;
        let m2 = ModelType::PreferQualityOptimized;
        let m3 = ModelType::LatencyOptimized;
        let res1 = serde_json::to_string(&m1).unwrap();
        let res2 = serde_json::to_string(&m2).unwrap();
        let res3 = serde_json::to_string(&m3).unwrap();
        assert_eq!(res1, r#""quality_optimized""#);
        assert_eq!(res2, r#""prefer_quality_optimized""#);
        assert_eq!(res3, r#""latency_optimized""#);
    }

    #[test]
    fn api_model_type_deserialize() {
        let json1 = r#""quality_optimized""#;
        let json2 = r#""prefer_quality_optimized""#;
        let json3 = r#""latency_optimized""#;
        let res1: ModelType = serde_json::from_str(json1).unwrap();
        let res2: ModelType = serde_json::from_str(json2).unwrap();
        let res3: ModelType = serde_json::from_str(json3).unwrap();
        match res1 {
            ModelType::QualityOptimized => {},
            _ => panic!("Expected ModelType::QualityOptimized"),
        }
        match res2 {
            ModelType::PreferQualityOptimized => {},
            _ => panic!("Expected ModelType::PreferQualityOptimized"),
        }
        match res3 {
            ModelType::LatencyOptimized => {},
            _ => panic!("Expected ModelType::LatencyOptimized"),
        }
    }

    #[test]
    fn api_tag_handling_serialize() {
        let t1 = TagHandling::Xml;
        let t2 = TagHandling::Html;
        let res1 = serde_json::to_string(&t1).unwrap();
        let res2 = serde_json::to_string(&t2).unwrap();
        assert_eq!(res1, r#""xml""#);
        assert_eq!(res2, r#""html""#);
    }

    #[test]
    fn api_tag_handling_deserialize() {
        let json1 = r#""xml""#;
        let json2 = r#""html""#;
        let res1: TagHandling = serde_json::from_str(json1).unwrap();
        let res2: TagHandling = serde_json::from_str(json2).unwrap();
        match res1 {
            TagHandling::Xml => {},
            _ => panic!("Expected TagHandling::Xml"),
        }
        match res2 {
            TagHandling::Html => {},
            _ => panic!("Expected TagHandling::Html"),
        }
    }

    #[test]
    fn api_translate_request_serialize() {
        let request = TranslateRequest {
            text: vec!["Hello, world!".to_string()],
            target_lang: "JA".to_string(),
            source_lang: Some("EN".to_string()),
            context: Some("greeting".to_string()),
            show_billed_characters: Some(true),
            split_sentences: Some(SplitSentences::True),
            preserve_formatting: Some(false),
            formality: Some(Formality::Default),
            model_type: Some(ModelType::QualityOptimized),
            glossary_id: Some("glossary-id-123".to_string()),
            tag_handling: Some(TagHandling::Xml),
            outline_detection: Some(true),
            non_splitting_tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
            splitting_tags: Some(vec!["tag3".to_string()]),
            ignore_tags: Some(vec!["tag4".to_string()]),
        };
        let res = serde_json::to_string(&request).unwrap();
        let expected = r#"{"text":["Hello, world!"],"target_lang":"JA","source_lang":"EN","context":"greeting","show_billed_characters":true,"split_sentences":"1","preserve_formatting":false,"formality":"default","model_type":"quality_optimized","glossary_id":"glossary-id-123","tag_handling":"xml","outline_detection":true,"non_splitting_tags":["tag1","tag2"],"splitting_tags":["tag3"],"ignore_tags":["tag4"]}"#;
        assert_eq!(res, expected);
    }

    #[test]
    fn api_translate_request_deserialize() {
        let json = r#"{"text":["Hello, world!"],"target_lang":"JA","source_lang":"EN","context":"greeting","show_billed_characters":true,"split_sentences":"1","preserve_formatting":false,"formality":"default","model_type":"quality_optimized","glossary_id":"glossary-id-123","tag_handling":"xml","outline_detection":true,"non_splitting_tags":["tag1","tag2"],"splitting_tags":["tag3"],"ignore_tags":["tag4"]}"#;
        let res: TranslateRequest = serde_json::from_str(json).unwrap();
        assert_eq!(res.text[0], "Hello, world!");
        assert_eq!(res.target_lang, "JA");
        assert_eq!(res.source_lang.unwrap(), "EN");
        assert_eq!(res.context.unwrap(), "greeting");
        assert_eq!(res.show_billed_characters.unwrap(), true);
        match res.split_sentences.unwrap() {
            SplitSentences::True => {},
            _ => panic!("Expected SplitSentences::True"),
        }
        assert_eq!(res.preserve_formatting.unwrap(), false);
        match res.formality.unwrap() {
            Formality::Default => {},
            _ => panic!("Expected Formality::Default"),
        }
        match res.model_type.unwrap() {
            ModelType::QualityOptimized => {},
            _ => panic!("Expected ModelType::QualityOptimized"),
        }
        assert_eq!(res.glossary_id.unwrap(), "glossary-id-123");
        match res.tag_handling.unwrap() {
            TagHandling::Xml => {},
            _ => panic!("Expected TagHandling::Xml"),
        }
        assert_eq!(res.outline_detection.unwrap(), true);
        assert_eq!(res.non_splitting_tags.unwrap(), vec!["tag1".to_string(), "tag2".to_string()]);
        assert_eq!(res.splitting_tags.unwrap(), vec!["tag3".to_string()]);
        assert_eq!(res.ignore_tags.unwrap(), vec!["tag4".to_string()]);
    }

    #[test]
    fn api_translate_request_default() {
        let request = TranslateRequest {
            text: vec!["Hello, world!".to_string()],
            target_lang: "JA".to_string(),
            ..Default::default()
        };
        assert_eq!(request.text[0], "Hello, world!");
        assert_eq!(request.target_lang, "JA");
        assert!(request.source_lang.is_none());
        assert!(request.context.is_none());
        assert!(request.show_billed_characters.is_none());
        assert!(request.split_sentences.is_none());
        assert!(request.preserve_formatting.is_none());
        assert!(request.formality.is_none());
        assert!(request.model_type.is_none());
        assert!(request.glossary_id.is_none());
        assert!(request.tag_handling.is_none());
        assert!(request.outline_detection.is_none());
        assert!(request.non_splitting_tags.is_none());
        assert!(request.splitting_tags.is_none());
        assert!(request.ignore_tags.is_none());
    }
}
