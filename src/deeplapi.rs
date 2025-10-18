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
            DeeplAPIError::LimitError => write!(f, "The translation limit of your account has been reached. Consider upgrading your subscription."),
            DeeplAPIError::GetLanguageCodesError => write!(f, "Could not get language codes"),
        }
    }
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
/// Send glossary to DeepL API and create a glossary.
pub fn send_glossary(api: &DpTran, glossary: &glossaries::Glossary) -> Result<glossaries::GlossaryResponseData, DeeplAPIError> {
    glossary.send(api).map_err(|e| DeeplAPIError::GlossaryError(e.to_string()))
}

/// For the glossary API.
/// Get supported languages for Glossaries API.
pub fn get_glossary_supported_languages(api: &DpTran) -> Result<glossaries::SupportedLanguages, DeeplAPIError> {
    glossaries::SupportedLanguages::get(api).map_err(|e| DeeplAPIError::GlossaryError(e.to_string()))
}

/// To run these tests, you need to set the environment variable `DPTRAN_DEEPL_API_KEY` to your DeepL API key.  
/// You should run these tests with ``cargo test -- --test-threads=1`` because the DeepL API has a limit on the number of requests per second.
/// And also, you need to run the dummy server for the DeepL API to test the API endpoints.
///   $ pip3 install -r requirements.txt
///   $ uvicorn dummy-api-server:app --reload
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

    fn impl_api_translate_test(times: u8) {
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
                    impl_api_translate_test(times + 1);
                    return;
                }
            }
        }
    }

    fn impl_api_usage_test(times: u8) {
        // usage test
        let (api_key, api_key_type) = get_api_key();
        let api = DpTran::with_endpoint(&api_key, &api_key_type, get_endpoint());
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
}

