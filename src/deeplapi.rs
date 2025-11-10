use std::fmt;

pub mod translate;
pub mod usage;
pub mod languages;
pub mod glossaries;
mod connection;

pub use languages::api::LangCodeName;
pub use connection::ConnectionError;
pub use translate::api::{ TranslateRequest, TranslateResult };
pub use glossaries::api::{ GlossariesApiFormat, GlossariesApiPostData };

pub use translate::api::DEEPL_API_TRANSLATE;
pub use translate::api::DEEPL_API_TRANSLATE_PRO;
pub use usage::api::DEEPL_API_USAGE;
pub use usage::api::DEEPL_API_USAGE_PRO;
pub use languages::api::DEEPL_API_LANGUAGES;
pub use languages::api::DEEPL_API_LANGUAGES_PRO;
pub use glossaries::api::DEEPL_API_GLOSSARIES;
pub use glossaries::api::DEEPL_API_GLOSSARIES_PRO;

/// Errors that can occur in this library.  
/// ``DeeplApiError``: DeepL API error  
/// ``InvalidLanguageCode``: Invalid language code  
/// ``ApiKeyIsNotSet``: API key is not set  
/// ``NoTargetLanguageSpecified``: No target language specified  
/// ``CouldNotGetInputText``: Could not get input text  
#[derive(Debug, PartialEq)]
pub enum DpTranError {
    DeeplApiError(DeeplAPIError),
    InvalidLanguageCode,
    ApiKeyIsNotSet,
    NoTargetLanguageSpecified,
    CouldNotGetInputText,
    GlossaryLangPairNotSupported,
}
impl ToString for DpTranError {
    fn to_string(&self) -> String {
        match self {
            DpTranError::DeeplApiError(e) => format!("Deepl API error: {}", e.to_string()),
            DpTranError::InvalidLanguageCode => "Invalid language code".to_string(),
            DpTranError::ApiKeyIsNotSet => "API key is not set".to_string(),
            DpTranError::NoTargetLanguageSpecified => "No target language specified".to_string(),
            DpTranError::CouldNotGetInputText => "Could not get input text".to_string(),
            DpTranError::GlossaryLangPairNotSupported => "Glossary language pair not supported".to_string(),
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
/// If the API key is pro, the character_limit is None.
pub struct DpTranUsage {
    pub character_count: u64,
    pub character_limit: Option<u64>,
}

/// DeepL API URLs
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EndpointUrls {
    pub translate_for_free: String,
    pub translate_for_pro: String,
    pub usage_for_free: String,
    pub usage_for_pro: String,
    pub languages_for_free: String,
    pub languages_for_pro: String,
    pub glossaries_for_free: String,
    pub glossaries_for_pro: String,
    pub glossaries_language_pairs_for_free: String,
    pub glossaries_language_pairs_for_pro: String,
}
impl Default for EndpointUrls {
    fn default() -> Self {
        EndpointUrls {
            translate_for_free: translate::api::DEEPL_API_TRANSLATE.to_string(),
            translate_for_pro: translate::api::DEEPL_API_TRANSLATE_PRO.to_string(),
            usage_for_free: usage::api::DEEPL_API_USAGE.to_string(),
            usage_for_pro: usage::api::DEEPL_API_USAGE_PRO.to_string(),
            languages_for_free: languages::api::DEEPL_API_LANGUAGES.to_string(),
            languages_for_pro: languages::api::DEEPL_API_LANGUAGES_PRO.to_string(),
            glossaries_for_free: glossaries::api::DEEPL_API_GLOSSARIES.to_string(),
            glossaries_for_pro: glossaries::api::DEEPL_API_GLOSSARIES_PRO.to_string(),
            glossaries_language_pairs_for_free: glossaries::api::DEEPL_API_GLOSSARIES_PAIRS.to_string(),
            glossaries_language_pairs_for_pro: glossaries::api::DEEPL_API_GLOSSARIES_PRO_PAIRS.to_string(),
        }
    }
}

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
    InvalidLangTypeError(String),
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
            DeeplAPIError::InvalidLangTypeError(ref e) => write!(f, "Invalid language type: {}", e),
        }
    }
}

/// Error message from DeepL API for some reason.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct DeeplAPIMessage {
    pub message: String,
}

/// DeepL translation library.
/// Create a new instance of DpTran with the API key.
/// Use the translate() method to translate the text.
/// Use the get_usage() method to get the number of characters remaining to be translated.
/// Use the get_language_codes() method to get the language code list.
/// Use the check_language_code() method to check the validity of the language code.
/// Use the correct_source_language_code() method to convert the source language code to the correct language code.
/// Use the correct_target_language_code() method to convert the target language code to the correct language code.
#[derive(Clone, Debug)]
pub struct DpTran {
    api_key: String,
    api_key_type: ApiKeyType,
    api_urls: EndpointUrls,
}
impl DpTran {
    /// Create a new instance of DpTran.  
    /// api_key: DeepL API key
    /// api_key_type: Type of API key (ApiKeyType::Free or ApiKeyType::Pro)
    pub fn with(api_key: &str, api_key_type: &ApiKeyType) -> DpTran {
        DpTran {
            api_key: api_key.to_string(),
            api_key_type: api_key_type.clone(),
            api_urls: EndpointUrls::default(),
        }
    }

    /// Create a new instance of DpTran with an API endpoint.
    /// api_key: DeepL API key
    /// api_key_type: Type of API key (ApiKeyType::Free or ApiKeyType::Pro)
    /// endpoint_urls: API endpoint URLs
    pub fn with_endpoint(api_key: &str, api_key_type: &ApiKeyType, endpoint_urls: EndpointUrls) -> DpTran {
        DpTran {
            api_key: api_key.to_string(),
            api_key_type: api_key_type.clone(),
            api_urls: endpoint_urls,
        }
    }

    /// Set the API key.  
    /// api_key: DeepL API key
    pub fn set_api_key(&mut self, api_key: &String, api_key_type: ApiKeyType) {
        self.api_key = api_key.clone();
        self.api_key_type = api_key_type;
    }

    /// Get the API key.
    pub fn get_api_key(&self) -> String {
        self.api_key.clone()
    }

    /// Get the API key type.
    pub fn get_api_key_type(&self) -> ApiKeyType {
        self.api_key_type.clone()
    }

    /// Get the DeepL API URLs.
    pub fn get_api_urls(&self) -> EndpointUrls {
        self.api_urls.clone()
    }

    /// Set the DeepL API URLs.
    /// api_urls: DeepL API URLs
    pub fn set_api_urls(&mut self, api_urls: EndpointUrls) {
        self.api_urls = api_urls;
    }

    /// Get language code list. Using DeepL API.  
    /// Retrieved from <https://api-free.deepl.com/v2/languages>.  
    /// lang_type: Target or Source  
    pub fn get_language_codes(&self, lang_type: LangType) -> Result<Vec<LangCodeName>, DpTranError> {
        let type_name = match lang_type {
            LangType::Target => "target".to_string(),
            LangType::Source => "source".to_string(),
        };
        let lang_codes = languages::get_language_codes(&self, type_name).map_err(|e| DpTranError::DeeplApiError(e))?;
        Ok(lang_codes)
    }

    /// Check the validity of language codes. Using DeepL API.  
    /// lang_code: Language code to check  
    /// lang_type: Target or Source  
    pub fn check_language_code(&self, lang_code: &String, lang_type: LangType) -> Result<bool, DpTranError> {
        let lang_codes = self.get_language_codes(lang_type)?;
        for lang in lang_codes {
            if lang.0.trim_matches('"') == lang_code.to_uppercase() {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Convert to correct language code from input source language code string. Using DeepL API.  
    /// language_code: Language code to convert  
    /// Caution: EN, PT are not automatically converted to EN-US, PT-PT from version 2.1.0.
    pub fn correct_source_language_code(&self, language_code: &str) -> Result<LangCode, DpTranError> {
        let source_language = language_code.to_ascii_uppercase().to_string();
        match self.check_language_code(&source_language, LangType::Source)? {
            true => Ok(source_language),
            false => Err(DpTranError::InvalidLanguageCode),
        }
    }

    /// Convert to correct language code from input target language code string. Using DeepL API.  
    /// language_code: Language code to convert  
    /// Caution: EN, PT are not automatically converted to EN-US, PT-PT from version 2.1.0.  
    pub fn correct_target_language_code(&self, language_code: &str) -> Result<LangCode, DpTranError> {
        let target_language = language_code.to_ascii_uppercase().to_string();
        match self.check_language_code(&target_language, LangType::Target)? {
            true => Ok(target_language),
            false => Err(DpTranError::InvalidLanguageCode),
        }
    }

    /// Get the number of characters remaining to be translated. Using DeepL API.  
    /// Retrieved from <https://api-free.deepl.com/v2/usage>.  
    /// Returns an error if acquisition fails.  
    /// api_key: DeepL API key  
    pub fn get_usage(&self) -> Result<DpTranUsage, DpTranError> {
        let (count, limit) = usage::get_usage(&self).map_err(|e| DpTranError::DeeplApiError(e))?;
        let limit = if limit == UNLIMITED_CHARACTERS_NUMBER {
            None
        } else {
            Some(limit)
        };
        Ok(DpTranUsage {
            character_count: count,
            character_limit: limit,
        })
    }

    /// Get translation results. Using DeepL API.  
    /// Receive translation results in json format and return them as a vector of strings.  
    /// Return error if json parsing fails.  
    /// text: Text to translate  
    /// target_lang: Target language  
    /// source_lang: Source language (optional)  
    pub fn translate(&self, text: &Vec<String>, target_lang: &String, source_lang: &Option<String>) -> Result<Vec<String>, DpTranError> {
        translate::translate(&self, text, target_lang, source_lang).map_err(|e| DpTranError::DeeplApiError(e))
    }

    /// Get translation results with glossary. Using DeepL API.
    /// Receive translation results in json format and return them as a vector of strings.  
    /// Return error if json parsing fails.  
    /// text: Text to translate  
    /// target_lang: Target language  
    /// source_lang: Source language (optional)  
    /// glossary: Glossary to use (optional)
    pub fn translate_with_glossary(&self, text: &Vec<String>, target_lang: &String, source_lang: &Option<String>, glossary_id: &glossaries::GlossaryID) -> Result<Vec<String>, DpTranError> {
        let translate_request = translate::api::TranslateRequest {
            text: text.clone(),
            target_lang: target_lang.clone(),
            source_lang: source_lang.clone(),
            glossary_id: Some(glossary_id.clone()),
            ..Default::default()
        };
        translate::translate_with_options(&self, &translate_request).map_err(|e| DpTranError::DeeplApiError(e))?
            .get_translation_strings().map_err(|e| DpTranError::DeeplApiError(e))
    }

    /// Translate with options. Using DeepL API.  
    /// You need to create a TranslateRequest instance first.
    pub fn translate_with_options(&self, request: &translate::api::TranslateRequest) -> Result<translate::api::TranslateResult, DpTranError> {
        translate::translate_with_options(&self, request).map_err(|e| DpTranError::DeeplApiError(e))
    }

    /// Send glossary to DeepL API and create a glossary.  
    /// You need to create a Glossary instance first.  
    /// Returns the glossary ID if successful.  
    /// glossary: Glossary instance
    pub fn send_glossary(&self, glossary: &mut glossaries::Glossary) -> Result<String, DpTranError> {
        glossary.send(&self).map_err(|e| DpTranError::DeeplApiError(e))
    }

    /// Get a list of registered glossaries.  
    /// Returns the list of registered glossaries.
    pub fn get_registered_glossaries(&self) -> Result<Vec<glossaries::Glossary>, DpTranError> {
        glossaries::get_registered_glossaries(&self).map_err(|e| DpTranError::DeeplApiError(e))
    }

    /// Get supported languages for Glossaries API.  
    /// Returns the supported languages.
    pub fn get_glossary_supported_languages(&self) -> Result<glossaries::api::GlossariesApiSupportedLanguages, DpTranError> {
        glossaries::get_glossary_supported_languages(&self).map_err(|e| DpTranError::DeeplApiError(e))
    }

    /// Delete a glossary.
    /// glossary: Glossary instance
    pub fn delete_glossary(&self, glossary: &glossaries::Glossary) -> Result<(), DpTranError> {
        glossaries::delete_glossary(&self, glossary).map_err(|e| DpTranError::DeeplApiError(e))
    }
}

/// To run these tests, you need to set the environment variable `DPTRAN_DEEPL_API_KEY` to your DeepL API key.  
/// You should run these tests with ``cargo test -- --test-threads=1`` because the DeepL API has a limit on the number of requests per second.  
/// And also, you need to run the dummy server for the DeepL API to test the API endpoints.
///   $ pip3 install -r requirements.txt
///   $ uvicorn dummy_api_server.main:app --reload
#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn retry_or_panic_for_api_tests(e: &DeeplAPIError, times: u8) -> bool {
        if e == &DeeplAPIError::ConnectionError(ConnectionError::TooManyRequests) && times < 3 {
            // Because the DeepL API has a limit on the number of requests per second, retry after 2 seconds if the error is TooManyRequests.
            std::thread::sleep(std::time::Duration::from_secs(2));
            return true;
        }
        else {
            panic!("Error: {}", e.to_string());
        }
    }

    pub fn retry_or_panic(e: &DpTranError, times: u8) -> bool {
        if e == &DpTranError::DeeplApiError(DeeplAPIError::ConnectionError(ConnectionError::TooManyRequests)) && times < 3 {
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

    fn do_lib_api_translate_test(times: u8) {
        // create instance test
        let (api_key, api_key_type) = get_api_key();
        let dptran = DpTran::with_endpoint(&api_key, &api_key_type, get_endpoint());

        // translate test
        let text = vec!["Hello, World!".to_string()];
        let target_lang = "JA".to_string();
        let source_lang = None;
        let res = dptran.translate(&text, &target_lang, &source_lang);
        match res {
            Ok(res) => {
                //assert_eq!(res[0], "ハロー、ワールド！");
                println!("res: {}", res[0]);
            },
            Err(e) => {
                if retry_or_panic(&e, times) {
                    // retry
                    do_lib_api_translate_test(times + 1);
                    return;
                }
            }
        }
    }

    fn do_lib_api_usage_test(times: u8) {
        // usage test
        let (api_key, api_key_type) = get_api_key();
        let dptran = DpTran::with_endpoint(&api_key, &api_key_type, get_endpoint());

        let res = dptran.get_usage();
        if res.is_err() {
            if retry_or_panic(&res.err().unwrap(), times) {
                // retry
                do_lib_api_usage_test(times + 1);
                return;
            }
        }
    }

    fn do_lib_api_get_language_code_test(times: u8) {
        // get_language_codes test
        let (api_key, api_key_type) = get_api_key();
        let dptran = DpTran::with_endpoint(&api_key, &api_key_type, get_endpoint());

        let res = dptran.get_language_codes(LangType::Source);
        match res {
            Ok(res) => {
                if res.len() == 0 {
                    panic!("Error: language codes is empty");
                }
            },
            Err(e) => {
                if retry_or_panic(&e, times) {
                    // retry
                    do_lib_api_get_language_code_test(times + 1);
                    return;
                }
            }
        }
    }

    fn do_lib_api_check_language_code_test(times: u8) {
        // check_language_code test
        let (api_key, api_key_type) = get_api_key();
        let dptran = DpTran::with_endpoint(&api_key, &api_key_type, get_endpoint());
        
        let res = dptran.check_language_code(&"EN-US".to_string(), LangType::Target);
        match res {
            Ok(res) => {
                assert_eq!(res, true);
            },
            Err(e) => {
                if retry_or_panic(&e, times) {
                    // retry
                    do_lib_api_check_language_code_test(times + 1);
                    return;
                }
            }
        }
        let res = dptran.check_language_code(&"XX".to_string(), LangType::Source);
        match res {
            Ok(res) => {
                assert_eq!(res, false);
            },
            Err(e) => {
                if retry_or_panic(&e, times) {
                    // retry
                    do_lib_api_check_language_code_test(times + 1);
                    return;
                }
            }
        }
    }

    fn do_lib_api_correct_source_language_code_test(times: u8) {
        // correct_source_language_code test
        let (api_key, api_key_type) = get_api_key();
        let dptran = DpTran::with_endpoint(&api_key, &api_key_type, get_endpoint());
        
        let valid_lang_code = "en";
        loop {
            let res = dptran.correct_source_language_code(valid_lang_code);
            match res {
                Ok(res) => {
                    assert_eq!(res, "EN".to_string());
                    break;
                },
                Err(e) => {
                    if retry_or_panic(&e, times) {
                        // retry
                        do_lib_api_correct_source_language_code_test(times + 1);
                        return;
                    }
                }
            }
        }

        let invalid_lang_code = "XX";
        loop {
            let res = dptran.correct_source_language_code(invalid_lang_code);
            match res {
                Ok(res) => {
                    panic!("Invalid language code but got: {}", res);
                },
                Err(e) => {
                    if e == DpTranError::DeeplApiError(DeeplAPIError::ConnectionError(ConnectionError::TooManyRequests)) && times < 3 {
                        // retry
                        std::thread::sleep(std::time::Duration::from_secs(2));
                    }
                    else if e != DpTranError::InvalidLanguageCode {
                        panic!("Error: {}", e.to_string());
                    }
                    else {
                        break;
                    }
                }
            }
        }
    }

    fn do_lib_api_correct_target_language_code_test(times: u8) {
        // correct_target_language_code test
        let (api_key, api_key_type) = get_api_key();
        let dptran = DpTran::with_endpoint(&api_key, &api_key_type, get_endpoint());
        
        let valid_lang_code = "ja";
        loop {
            let res = dptran.correct_target_language_code(valid_lang_code);
            match res {
                Ok(res) => {
                    assert_eq!(res, "JA".to_string());
                    break;
                },
                Err(e) => {
                    if retry_or_panic(&e, times) {
                        // retry
                        do_lib_api_correct_target_language_code_test(times + 1);
                        return;
                    }
                }
            }
        }

        let invalid_lang_code = "XX";
        loop {
            let res = dptran.correct_target_language_code(invalid_lang_code);
            match res {
                Ok(res) => {
                    panic!("Invalid language code but got: {}", res);
                },
                Err(e) => {
                    if e == DpTranError::DeeplApiError(DeeplAPIError::ConnectionError(ConnectionError::TooManyRequests)) && times < 3 {
                        // retry
                        std::thread::sleep(std::time::Duration::from_secs(2));
                    }
                    else if e != DpTranError::InvalidLanguageCode {
                        panic!("Error: {}", e.to_string());
                    }
                    else {
                        break;
                    }
                }
            }
        }
    }

    #[test]
    fn lib_api_translate_test() {
        do_lib_api_translate_test(0);
    }

    #[test]
    fn lib_api_usage_test() {
        // usage test
        do_lib_api_usage_test(0);
    }

    #[test]
    fn lib_api_get_language_code_test() {   
        // get_language_codes test
        do_lib_api_get_language_code_test(0);
    }

    #[test]
    fn lib_api_check_language_code_test() {
        // check_language_code test
        do_lib_api_check_language_code_test(0);
    }

    #[test]
    fn lib_impl_set_and_get_api_key_test() {
        // set_api_key test
        let (api_key, api_key_type) = get_api_key();
        let mut dptran = DpTran::with_endpoint(&api_key, &api_key_type, get_endpoint());
        assert_eq!(dptran.get_api_key(), api_key);
        dptran.set_api_key(&"test".to_string(), ApiKeyType::Free);
        assert_eq!(dptran.get_api_key(), "test".to_string());
        assert_eq!(dptran.get_api_key_type(), ApiKeyType::Free);
    }

    #[test]
    fn lib_impl_correct_source_language_code_test() {
        // correct_source_language_code test
        do_lib_api_correct_source_language_code_test(0);
    }

    #[test]
    fn lib_impl_correct_target_language_code_test() {
        // correct_target_language_code test
        do_lib_api_correct_target_language_code_test(0);
    }

    #[test]
    fn lib_impl_set_and_get_api_urls_test() {
        // set_api_urls test
        let (api_key, api_key_type) = get_api_key();
        let mut dptran = DpTran::with(&api_key, &api_key_type);

        // Default URLs
        let api_urls = EndpointUrls::default();
        dptran.set_api_urls(api_urls.clone());
        assert_eq!(dptran.get_api_urls(), api_urls);

        // Custom URLs
        let custom_urls = get_endpoint();
        dptran.set_api_urls(custom_urls.clone());
        assert_eq!(dptran.get_api_urls(), custom_urls);
    }

    #[test]
    fn lib_impl_with_endpoint_test() {
        // with_endpoint test
        let (api_key, api_key_type) = get_api_key();
        let dptran = DpTran::with(&api_key, &api_key_type);
        let before_urls = dptran.get_api_urls();
        let custom_urls = get_endpoint();
        let dptran_with_endpoint = DpTran::with_endpoint(&api_key, &api_key_type, custom_urls.clone());
        assert_eq!(dptran_with_endpoint.get_api_urls(), custom_urls);
        assert_ne!(dptran_with_endpoint.get_api_urls(), before_urls);
    }

    #[test]
    fn lib_impl_change_endpoint_urls_test() {
        // change_endpoint_urls test
        let (api_key, api_key_type) = get_api_key();
        let mut dptran = DpTran::with(&api_key, &api_key_type);
        let before_urls = dptran.get_api_urls();
        let custom_urls = EndpointUrls {
            translate_for_free: "http://localhost:8000/v2/translate".to_string(),
            translate_for_pro: "http://localhost:8000/v2/translate_pro".to_string(),
            usage_for_free: "http://localhost:8000/v2/usage".to_string(),
            usage_for_pro: "http://localhost:8000/v2/usage_pro".to_string(),
            languages_for_free: "http://localhost:8000/v2/languages".to_string(),
            languages_for_pro: "http://localhost:8000/v2/languages_pro".to_string(),
            glossaries_for_free: "http://localhost:8000/v2/glossaries".to_string(),
            glossaries_for_pro: "http://localhost:8000/v2/glossaries_pro".to_string(),
            glossaries_language_pairs_for_free: "http://localhost:8000/v2/glossaries/supported-languages".to_string(),
            glossaries_language_pairs_for_pro: "http://localhost:8000/v2/glossaries/supported-languages_pro".to_string(),
        };
        dptran.set_api_urls(custom_urls.clone());
        assert_eq!(dptran.get_api_urls(), custom_urls);
        assert_ne!(dptran.get_api_urls(), before_urls);
    }
}
