use std::fmt;

use super::DpTran;

pub mod connection;
pub use connection::ConnectionError;

pub mod translate;
pub mod usage;
pub mod languages;
pub mod glossaries;

pub const UNLIMITED_CHARACTERS_NUMBER: u64 = 1000000000000;  // DeepL Pro API has no character limit, but the API returns a character limit of 1000000000000 characters as a default value.

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

/// string as language code
pub type LangCode = String;

/// DeepL API error.  
/// ``ConnectionError``: Connection error occurred in the process of sending and receiving data.  
/// ``JsonError``: Error occurred while parsing json.  
/// ``LimitError``: The translation limit of your account has been reached. Consider upgrading your subscription.  
/// ``GetLanguageCodesError``: Could not get language codes.  
#[derive(Debug, PartialEq)]
pub enum DeeplAPIError {
    ConnectionError(ConnectionError),
    JsonError(String, String),
    WrongEndPointError(String),
    GlossaryError(String),
    GlossaryIsNotRegisteredError,
    LimitError,
    GetLanguageCodesError,
}
impl fmt::Display for DeeplAPIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeeplAPIError::ConnectionError(ref e) => write!(f, "Connection error: {}", e),
            DeeplAPIError::JsonError(ref e, ref json) => write!(f, "JSON error: {}\nContent: {}", e, json),
            DeeplAPIError::WrongEndPointError(ref e) => write!(f, "Wrong endpoint error. Please check your API key type such as Free or Pro. {}", e),
            DeeplAPIError::GlossaryError(ref e) => write!(f, "Glossary error: {}", e),
            DeeplAPIError::GlossaryIsNotRegisteredError => write!(f, "The specified glossary is not registered."),
            DeeplAPIError::LimitError => write!(f, "The translation limit of your account has been reached. Consider upgrading your subscription."),
            DeeplAPIError::GetLanguageCodesError => write!(f, "Could not get language codes"),
        }
    }
}

/// Glossary ID
pub type GlossaryID = String;

/// Glossaries dictionary struct
pub struct GlossaryDictionary {
    pub source_lang: LangCode,
    pub target_lang: LangCode,
    pub entries: Vec<(String, String)>,
    pub entries_format: glossaries::GlossariesApiFormat,
    pub entry_count: usize,
}

/// Glossary struct
pub struct Glossary {
    pub name: String,
    pub id: Option<GlossaryID>,
    pub dictionaries: Vec<GlossaryDictionary>,
}

/// Error message from DeepL API for some reason.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct DeeplAPIMessage {
    pub message: String,
}

/// For the translation API.  
/// Return translation results.  
/// Receive translation results in json format and display translation results.  
/// Return error if json parsing fails.
pub fn translate(api: &DpTran, text: &Vec<String>, target_lang: &String, source_lang: &Option<String>) -> Result<Vec<String>, DeeplAPIError> {
    let translations = translate::TranslateRequest {
        text: text.clone(),
        target_lang: target_lang.clone(),
        source_lang: source_lang.clone(),
        ..Default::default()
    };
    let results = translations.translate(api)?;
    let translated_texts = results.get_translation_strings()?;
    Ok(translated_texts)
}

/// For the translation API.  
/// Translate with detailed options.  
/// Return detailed translation results.
pub fn translate_with_options(api: &DpTran, request: &translate::TranslateRequest) -> Result<translate::TranslateResponse, DeeplAPIError> {
    request.translate(api)
}

/// For the usage API.  
/// Get the number of characters remaining to be translated.  
/// Retrieved from <https://api-free.deepl.com/v2/usage>.  
/// Returns an error if acquisition fails.  
pub fn get_usage(api: &DpTran) -> Result<(u64, u64), DeeplAPIError> {
    usage::get_usage(api)
}

/// For the languages API.  
/// Get language code list  
/// Retrieved from <https://api-free.deepl.com/v2/languages>.  
pub fn get_language_codes(api: &DpTran, type_name: String) -> Result<Vec<languages::LangCodeName>, DeeplAPIError> {
    languages::get_language_codes(api, type_name)
}

/// For the glossary API.  
/// Get a list of registered glossaries.
pub fn get_registered_glossaries(api: &DpTran) -> Result<Vec<Glossary>, DeeplAPIError> {
    let glossaries_list = glossaries::GlossariesApiList::get_registered_dictionaries(api).map_err(|e| DeeplAPIError::GlossaryError(e.to_string()))?;
    let mut result: Vec<Glossary> = Vec::new();
    for glossary_data in glossaries_list.glossaries.iter() {
        // Convert GlossariesApiResponseData to Glossary
        let dictionaries: Vec<GlossaryDictionary> = glossary_data.dictionaries.iter().map(|dict_data| {
            GlossaryDictionary {
                source_lang: dict_data.source_lang.clone(),
                target_lang: dict_data.target_lang.clone(),
                entries: Vec::new(),  // Entries are not included in the list API response
                entries_format: glossaries::GlossariesApiFormat::Tsv,  // Default to Tsv
                entry_count: dict_data.entry_count as usize,
            }
        }).collect();

        let glossary = Glossary {
            name: glossary_data.name.clone(),
            dictionaries,
            id: Some(glossary_data.glossary_id.clone()),
        };
        result.push(glossary);
    }

    Ok(result)
}

/// For the glossary API.  
/// Get supported languages for Glossaries API.
pub fn get_glossary_supported_languages(api: &DpTran) -> Result<glossaries::GlossariesApiSupportedLanguages, DeeplAPIError> {
    glossaries::GlossariesApiSupportedLanguages::get(api).map_err(|e| DeeplAPIError::GlossaryError(e.to_string()))
}

/// For the glossary API.
/// Delete a glossary.
pub fn delete_glossary(api: &DpTran, glossary: &Glossary) -> Result<(), DeeplAPIError> {
    if let Some(glossary_id) = &glossary.id {
        glossaries::delete_glossary(api, glossary_id).map_err(|e| DeeplAPIError::GlossaryError(e.to_string()))
    } else {
        Err(DeeplAPIError::GlossaryIsNotRegisteredError)
    }
}

impl GlossaryDictionary {
    /// Make a new GlossaryDictionary instance.
    pub fn new(source_lang: String, target_lang: String, entries: Vec<(String, String)>, entries_format: glossaries::GlossariesApiFormat) -> Self {
        let entry_count = entries.len();
        GlossaryDictionary {
            source_lang,
            target_lang,
            entries,
            entries_format,
            entry_count: entry_count,
        }
    }
}

impl Glossary {
    /// Make a new Glossary instance.
    /// 
    /// `name`: Glossary name
    /// `dictionaries`: Vec<GlossaryDictionary>
    pub fn new(name: String, dictionaries: Vec<GlossaryDictionary>) -> Self {
        Glossary {
            name,
            dictionaries,
            id: None,
        }
    }

    /// Send glossary to DeepL API and create a glossary.  
    /// 
    /// `api`: DpTran instance
    pub fn send(&mut self, api: &DpTran) -> Result<GlossaryID, DeeplAPIError> {
        // Make Vec<GlossariesApiDictionaryPostData>
        let dictionaries: Vec<glossaries::GlossariesApiDictionaryPostData> = self.dictionaries.iter().map(|dict| {
            // Prepare entries
            let entries = match dict.entries_format {
                glossaries::GlossariesApiFormat::Tsv => {
                    dict.entries.iter().map(|(source, target)| format!("{}\t{}", source, target)).collect::<Vec<String>>().join("\n")
                },
                glossaries::GlossariesApiFormat::Csv => {
                    dict.entries.iter().map(|(source, target)| format!("\"{}\",\"{}\"", source.replace("\"", "\"\""), target.replace("\"", "\"\""))).collect::<Vec<String>>().join("\n")
                },
            };

            glossaries::GlossariesApiDictionaryPostData::new(
                &dict.source_lang,
                &dict.target_lang,
                &entries,
                &dict.entries_format.to_string(),
            )
        }).collect();

        // Make a new GlossariesApiPostData instance
        let glossary = glossaries::GlossariesApiPostData::new(
            self.name.clone(),
            dictionaries,
        );

        // Send glossary to DeepL API
        let res = glossary.send(api).map_err(|e| DeeplAPIError::GlossaryError(e.to_string()))?;

        // Set and return glossary ID
        self.id = Some(res.glossary_id);

        Ok(self.id.as_ref().unwrap().clone())
    }

    /// Edit glossary content and update it on DeepL API.
    /// 
    /// `api`: DpTran instance
    pub fn update(&self, api: &DpTran) -> Result<(), DeeplAPIError> {
        if let Some(glossary_id) = &self.id {
            // Make Vec<GlossariesApiDictionaryPostData>
            let dictionaries: Vec<glossaries::GlossariesApiDictionaryPostData> = self.dictionaries.iter().map(|dict| {
                // Prepare entries
                let entries = match dict.entries_format {
                    glossaries::GlossariesApiFormat::Tsv => {
                        dict.entries.iter().map(|(source, target)| format!("{}\t{}", source, target)).collect::<Vec<String>>().join("\n")
                    },
                    glossaries::GlossariesApiFormat::Csv => {
                        dict.entries.iter().map(|(source, target)| format!("\"{}\",\"{}\"", source.replace("\"", "\"\""), target.replace("\"", "\"\""))).collect::<Vec<String>>().join("\n")
                    },
                };

                glossaries::GlossariesApiDictionaryPostData::new(
                    &dict.source_lang,
                    &dict.target_lang,
                    &entries,
                    &dict.entries_format.to_string(),
                )
            }).collect();

            // Make a new GlossariesApiPostData instance
            let glossary = glossaries::GlossariesApiPostData::new(
                self.name.clone(),
                dictionaries,
            );

            // Update glossary on DeepL API
            glossaries::patch_glossary(api, glossary_id, &glossary).map_err(|e| DeeplAPIError::GlossaryError(e.to_string()))
        } else {
            Err(DeeplAPIError::GlossaryIsNotRegisteredError)
        }
    }
}

/// To run these tests, you need to set the environment variable `DPTRAN_DEEPL_API_KEY` to your DeepL API key.  
/// You should run these tests with ``cargo test -- --test-threads=1`` because the DeepL API has a limit on the number of requests per second.
/// And also, you need to run the dummy server for the DeepL API to test the API endpoints.
///   $ pip3 install -r requirements.txt
///   $ uvicorn dummy_api_server.main:app --reload
#[cfg(test)]
pub mod tests {
    use core::panic;

    use crate::EndpointUrls;

    use super::*;
    use languages::EXTENDED_LANG_CODES;

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

    pub fn get_endpoint() -> EndpointUrls {
        pub const TEST_DEEPL_API_TRANSLATE: &str = "http://localhost:8000/free/v2/translate";
        pub const TEST_DEEPL_API_TRANSLATE_PRO: &str = "http://localhost:8000/pro/v2/translate";
        pub const TEST_DEEPL_API_USAGE: &str = "http://localhost:8000/free/v2/usage";
        pub const TEST_DEEPL_API_USAGE_PRO: &str = "http://localhost:8000/pro/v2/usage";
        pub const TEST_DEEPL_API_LANGUAGES: &str = "http://localhost:8000/free/v2/languages";
        pub const TEST_DEEPL_API_LANGUAGES_PRO: &str = "http://localhost:8000/pro/v2/languages";
        pub const TEST_DEEPL_API_GLOSSARIES: &str = "http://localhost:8000/free/v2/glossaries";
        pub const TEST_DEEPL_API_GLOSSARIES_PRO: &str = "http://localhost:8000/pro/v2/glossaries";
        pub const TEST_DEEPL_API_GLOSSARIES_LANGUAGE_PAIRS: &str = "http://localhost:8000/free/v2/glossaries/supported-languages";
        pub const TEST_DEEPL_API_GLOSSARIES_PRO_LANGUAGE_PAIRS: &str = "http://localhost:8000/pro/v2/glossaries/supported-languages";

        EndpointUrls {
            translate_for_free: TEST_DEEPL_API_TRANSLATE.to_string(),
            translate_for_pro: TEST_DEEPL_API_TRANSLATE_PRO.to_string(),
            usage_for_free: TEST_DEEPL_API_USAGE.to_string(),
            usage_for_pro: TEST_DEEPL_API_USAGE_PRO.to_string(),
            languages_for_free: TEST_DEEPL_API_LANGUAGES.to_string(),
            languages_for_pro: TEST_DEEPL_API_LANGUAGES_PRO.to_string(),
            glossaries_for_free: TEST_DEEPL_API_GLOSSARIES.to_string(),
            glossaries_for_pro: TEST_DEEPL_API_GLOSSARIES_PRO.to_string(),
            glossaries_language_pairs_for_free: TEST_DEEPL_API_GLOSSARIES_LANGUAGE_PAIRS.to_string(),
            glossaries_language_pairs_for_pro: TEST_DEEPL_API_GLOSSARIES_PRO_LANGUAGE_PAIRS.to_string(),
        }
    }

    fn do_api_translate_test(times: u8) {
        // translate test
        let (api_key, api_key_type) = get_api_key();
        let api = DpTran::with_endpoint(&api_key, &api_key_type, get_endpoint());
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
                    do_api_translate_test(times + 1);
                    return;
                }
            }
        }
    }

    fn do_api_usage_test(times: u8) {
        // usage test
        let (api_key, api_key_type) = get_api_key();
        let api = DpTran::with_endpoint(&api_key, &api_key_type, get_endpoint());
        let res = get_usage(&api);
        if res.is_err() {
            if retry_or_panic(&res.err().unwrap(), times) {
                // retry
                do_api_usage_test(times + 1);
                return;
            }
        }
    }

    fn do_api_get_language_codes_test(times: u8) {
        // get_language_codes test
        let (api_key, api_key_type) = get_api_key();
        let api = DpTran::with_endpoint(&api_key, &api_key_type, get_endpoint());
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
        do_api_translate_test(0);
    }

    #[test]
    fn api_usage_test() {
        // usage test
        do_api_usage_test(0);
    }

    #[test]
    fn api_get_language_codes_test() {
        // get_language_codes test
        do_api_get_language_codes_test(0);
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
}

