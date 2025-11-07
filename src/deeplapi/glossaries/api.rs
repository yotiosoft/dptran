use core::fmt;

use crate::deeplapi::DeeplAPIMessage;

use super::DpTran;

use super::connection;
use connection::ConnectionError;
use super::DeeplAPIError;
use super::ApiKeyType;

use serde::{Deserialize, Serialize};

pub const DEEPL_API_GLOSSARIES: &str = "https://api-free.deepl.com/v3/glossaries";
pub const DEEPL_API_GLOSSARIES_PRO: &str = "https://api.deepl.com/v3/glossaries";
pub const DEEPL_API_GLOSSARIES_PAIRS: &str = "https://api-free.deepl.com/v2/glossary-language-pairs";
pub const DEEPL_API_GLOSSARIES_PRO_PAIRS: &str = "https://api.deepl.com/v2/glossary-language-pairs";

#[derive(Debug, PartialEq)]
pub enum GlossaryError {
    ConnectionError(ConnectionError),
    JsonError(String, String),
    CouldNotCreateGlossary,
    QuotaExceeded,
}
impl fmt::Display for GlossaryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GlossaryError::ConnectionError(e) => write!(f, "Connection error: {}", e),
            GlossaryError::JsonError(e, j) => write!(f, "JSON error: {}\nJSON: {}", e, j),
            GlossaryError::CouldNotCreateGlossary => write!(f, "Could not create glossary on DeepL API by some reason"),
            GlossaryError::QuotaExceeded => write!(f, "Glossary quota exceeded on DeepL API"),
        }
    }
}

/// Glossary file format  
/// TSV: Tab-Separated Values  
/// CSV: Comma-Separated Values
#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq)]
pub enum GlossariesApiFormat {
    Tsv,
    Csv,
}
impl fmt::Display for GlossariesApiFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GlossariesApiFormat::Tsv => write!(f, "tsv"),
            GlossariesApiFormat::Csv => write!(f, "csv"),
        }
    }
}

/// Data structures for glossary API.
#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct GlossariesApiDictionaryPostData {
    source_lang: String,
    target_lang: String,
    entries: String,
    entries_format: String,
}

/// Response data structures for glossary API.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct GlossariesApiDictionaryResponseData {
    pub source_lang: String,
    pub target_lang: String,
    pub entry_count: u64,
}

/// Glossary post data structure.  
/// Used to create a glossary via the DeepL API.  
/// You need to create an instance of this struct to send a glossary creation request.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct GlossariesApiPostData {
    name: String,
    dictionaries: Vec<GlossariesApiDictionaryPostData>,
}

/// Response data structure for glossary creation via DeepL API.  
/// Returned when a glossary is successfully created.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct GlossariesApiResponseData {
    pub glossary_id: String,
    pub name: String,
    pub dictionaries: Vec<GlossariesApiDictionaryResponseData>,
    pub creation_time: String,
}

/// List of glossaries registered in the DeepL API.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct GlossariesApiList {
    pub glossaries: Vec<GlossariesApiResponseData>,
}

/// An entry of supported languages by Glossaries API.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct GlossariesApiSupportedLanguageEntry {
    pub source_lang: String,
    pub target_lang: String,
}

/// List of supported languages by Glossaries API.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct GlossariesApiSupportedLanguages {
    pub supported_languages: Vec<GlossariesApiSupportedLanguageEntry>,
}

impl GlossariesApiPostData {
    /// Create a glossary post data.
    pub fn new(glossary_name: String, dictionaries: Vec<GlossariesApiDictionaryPostData>) -> Self {
        let post_data = GlossariesApiPostData {
            name: glossary_name,
            dictionaries,
        };
        post_data
    }

    /// Create a curl session.
    pub fn send(&self, api: &DpTran) -> Result<GlossariesApiResponseData, GlossaryError> {
        let url = if api.api_key_type == ApiKeyType::Free {
            DEEPL_API_GLOSSARIES.to_string()
        } else {
            DEEPL_API_GLOSSARIES_PRO.to_string()
        };
        
        // Prepare headers
        let header_auth_key = format!("Authorization: DeepL-Auth-Key {}", api.api_key);
        let header_content_type = "Content-Type: application/json";
        let headers = vec![header_auth_key, header_content_type.to_string()];
        let post_data_json = serde_json::to_string(self).unwrap();
        
        // Send request
        let ret = connection::post_with_headers(url, post_data_json, &headers);

        // Handle response
        match ret {
            Ok(res) => {
                // Error check
                let ret: Result<DeeplAPIMessage, serde_json::Error> = serde_json::from_str(&res);
                if let Ok(ret) = ret {
                    if ret.message == "Quota exceeded" {
                        return Err(GlossaryError::QuotaExceeded);
                    } else {
                        return Err(GlossaryError::CouldNotCreateGlossary);
                    }
                }

                // If there is no error, the response is GlossaryResponseData.
                // Parse response
                let ret: GlossariesApiResponseData = serde_json::from_str(&res).map_err(|e| GlossaryError::JsonError(e.to_string(), res.clone()))?;
                Ok(ret)
            },
            Err(e) => Err(GlossaryError::ConnectionError(e)),
        }
    }

    /// Get glossary dictionaries.
    pub fn get_dictionaries(&self) -> Vec<GlossariesApiDictionaryPostData> {
        self.dictionaries.clone()
    }
}

impl GlossariesApiDictionaryPostData {
    /// Make a new GlossariesApiDictionaryPostData.
    pub fn new(source_lang: &String, target_lang: &String, entries: &String, entries_format: &String) -> Self {
        GlossariesApiDictionaryPostData {
            source_lang: source_lang.clone(),
            target_lang: target_lang.clone(),
            entries: entries.clone(),
            entries_format: entries_format.clone(),
        }
    }

    /// Get source language.
    pub fn get_source_lang(&self) -> &String {
        &self.source_lang
    }

    /// Get target language.
    pub fn get_target_lang(&self) -> &String {
        &self.target_lang
    }
}

impl GlossariesApiList {
    /// Get a list of glossaries from the API server.
    pub fn get_registered_dictionaries(api: &DpTran) -> Result<Self, DeeplAPIError> {
        let url = if api.api_key_type == ApiKeyType::Free {
            DEEPL_API_GLOSSARIES.to_string()
        } else {
            DEEPL_API_GLOSSARIES_PRO.to_string()
        };

        // Prepare headers
        let header_auth_key = format!("Authorization: DeepL-Auth-Key {}", api.api_key);
        let headers = vec![header_auth_key];

        // Send request
        let ret = connection::get_with_headers(url, &headers);

        // Handle response
        match ret {
            Ok(res) => {
                let ret: GlossariesApiList = serde_json::from_str(&res).map_err(|e| DeeplAPIError::JsonError(e.to_string(), res.clone()))?;
                Ok(ret)
            },
            Err(e) => Err(DeeplAPIError::ConnectionError(e)),
        }
    }
}

/// Delete the glossary via DeepL API.
pub fn delete_glossary(api: &DpTran, glossary_id: &String) -> Result<(), GlossaryError> {
    let url = if api.api_key_type == ApiKeyType::Free {
        format!("{}/{}", DEEPL_API_GLOSSARIES, glossary_id)
    } else {
        format!("{}/{}", DEEPL_API_GLOSSARIES_PRO, glossary_id)
    };

    // Prepare headers
    let header_auth_key = format!("Authorization: DeepL-Auth-Key {}", api.api_key);
    let headers = vec![header_auth_key];

    // Send request
    let ret = connection::delete_with_headers(url, &headers);
    match ret {
        Ok(_) => Ok(()),
        Err(e) => Err(GlossaryError::ConnectionError(e)),
    }
}

/// Patch the glossary via DeepL API.
pub fn patch_glossary(api: &DpTran, glossary_id: &String, patch_data: &GlossariesApiPostData) -> Result<(), GlossaryError> {
    let url = if api.api_key_type == ApiKeyType::Free {
        format!("{}/{}", DEEPL_API_GLOSSARIES, glossary_id)
    } else {
        format!("{}/{}", DEEPL_API_GLOSSARIES_PRO, glossary_id)
    };

    // Prepare headers
    let header_auth_key = format!("Authorization: DeepL-Auth-Key {}", api.api_key);
    let header_content_type = "Content-Type: application/json";
    let headers = vec![header_auth_key, header_content_type.to_string()];
    let patch_data_json = serde_json::to_string(patch_data).unwrap();

    // Send request
    let ret = connection::patch_with_headers(url, patch_data_json, &headers);
    match ret {
        Ok(_) => Ok(()),
        Err(e) => Err(GlossaryError::ConnectionError(e)),
    }
}

impl GlossariesApiSupportedLanguages {
    /// Get supported languages for Glossaries API.
    pub fn get(api: &DpTran) -> Result<GlossariesApiSupportedLanguages, GlossaryError> {
        let url = if api.api_key_type == ApiKeyType::Free {
            DEEPL_API_GLOSSARIES_PAIRS.to_string()
        } else {
            DEEPL_API_GLOSSARIES_PRO_PAIRS.to_string()
        };
        
        // Prepare headers
        let header_auth_key = format!("Authorization: DeepL-Auth-Key {}", api.api_key);
        let headers = vec![header_auth_key];
        
        // Send request
        let ret = connection::get_with_headers(url, &headers);

        // Handle response
        match ret {
            Ok(res) => {
                let ret: GlossariesApiSupportedLanguages = serde_json::from_str(&res).map_err(|e| GlossaryError::JsonError(e.to_string(), res.clone()))?;
                Ok(ret)
            },
            Err(e) => Err(GlossaryError::ConnectionError(e)),
        }
    }

    /// Is the language pair supported?
    pub fn is_lang_pair_supported(&self, source_lang: &String, target_lang: &String) -> bool {
        for pair in &self.supported_languages {
            if pair.source_lang.to_ascii_uppercase() == source_lang.to_ascii_uppercase() 
                && pair.target_lang.to_ascii_uppercase() == target_lang.to_ascii_uppercase() {
                return true;
            }
        }
        false
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

    #[test]
    fn impl_glossaries_dictionary() {
        let dict = GlossariesApiDictionaryPostData::new(&"EN".to_string(), &"FR".to_string(), &"Hello\tBonjour\nGoodbye\tAu revoir".to_string(), &"tsv".to_string());
        assert_eq!(dict.get_source_lang(), &"EN".to_string());
    }

    #[test]
    fn impl_glossaries_post_data() {
        let dict1 = GlossariesApiDictionaryPostData::new(&"EN".to_string(), &"FR".to_string(), &"Hello\tBonjour\nGoodbye\tAu revoir".to_string(), &"tsv".to_string());
        let dict2 = GlossariesApiDictionaryPostData::new(&"DE".to_string(), &"EN".to_string(), &"Hallo\tHello\nTschüss\tGoodbye".to_string(), &"tsv".to_string());
        let glossaries_post_data = GlossariesApiPostData::new("MyGlossary".to_string(), vec![dict1.clone(), dict2.clone()]);
        let dictionaries = glossaries_post_data.get_dictionaries();
        assert_eq!(dictionaries.len(), 2);
        assert_eq!(dictionaries[0].get_source_lang(), &"EN".to_string());
    }

    #[test]
    fn api_glossaries_post_send() {
        // Send glossary creation request to DeepL API
        let api_key = std::env::var("DPTRAN_DEEPL_API_KEY").unwrap();
        let api = DpTran::with_endpoint(&api_key, &ApiKeyType::Free, super::super::super::tests::get_endpoint());
        let dict = GlossariesApiDictionaryPostData::new(&"EN".to_string(), &"FR".to_string(), &"Hello\tBonjour\nGoodbye\tAu revoir".to_string(), &"tsv".to_string());
        let glossaries = vec![dict];
        let post_data = GlossariesApiPostData::new("MyGlossary".to_string(), glossaries);
        let res = post_data.send(&api);
        assert!(res.is_ok());
        
        // Retrieve response data
        let glossary_response = GlossariesApiList::get_registered_dictionaries(&api);
        assert!(glossary_response.is_ok());

        let glossary_response = glossary_response.unwrap();
        assert!(glossary_response.glossaries.len() > 0);

        let created_glossary = &glossary_response.glossaries[0];
        assert_eq!(created_glossary.name, "MyGlossary".to_string());
        assert_eq!(created_glossary.dictionaries.len(), 1);
        assert_eq!(created_glossary.dictionaries[0].source_lang.to_uppercase(), "EN".to_string());
        assert_eq!(created_glossary.dictionaries[0].target_lang.to_uppercase(), "FR".to_string());
        assert_eq!(created_glossary.dictionaries[0].entry_count, 2);

        // Delete the created glossary
        let delete_res = delete_glossary(&api, &created_glossary.glossary_id);
        assert!(delete_res.is_ok());
    }

    #[test]
    fn api_glossaries_get_registered_dictionaries() {
        let api_key = std::env::var("DPTRAN_DEEPL_API_KEY").unwrap();
        let api = DpTran::with_endpoint(&api_key, &ApiKeyType::Free, super::super::super::tests::get_endpoint());
        let res = GlossariesApiList::get_registered_dictionaries(&api);
        assert!(res.is_ok());
    }

    #[test]
    fn api_glossaries_supported_languages() {
        let api_key = std::env::var("DPTRAN_DEEPL_API_KEY").unwrap();
        let api = DpTran::with_endpoint(&api_key, &ApiKeyType::Free, super::super::super::tests::get_endpoint());
        let res = GlossariesApiSupportedLanguages::get(&api);
        assert!(res.is_ok());
        let supported_languages = res.unwrap();
        assert!(supported_languages.is_lang_pair_supported(&"EN".to_string(), &"FR".to_string()));
        assert!(!supported_languages.is_lang_pair_supported(&"EN".to_string(), &"XX".to_string()));
    }

    #[test]
    fn api_glossaries_patch_glossary() {
        // Send glossary creation request to DeepL API
        let api_key = std::env::var("DPTRAN_DEEPL_API_KEY").unwrap();
        let api = DpTran::with_endpoint(&api_key, &ApiKeyType::Free, super::super::super::tests::get_endpoint());
        let dict = GlossariesApiDictionaryPostData::new(&"EN".to_string(), &"FR".to_string(), &"Hello\tBonjour\nGoodbye\tAu revoir".to_string(), &"tsv".to_string());
        let glossaries = vec![dict];
        let post_data = GlossariesApiPostData::new("MyGlossary".to_string(), glossaries);
        let res = post_data.send(&api);
        assert!(res.is_ok());

        // Retrieve response data
        let glossary_response = GlossariesApiList::get_registered_dictionaries(&api);
        assert!(glossary_response.is_ok());
        let glossary_response = glossary_response.unwrap();
        let created_glossary = &glossary_response.glossaries[0];
        
        // Patch the created glossary
        let new_dict = GlossariesApiDictionaryPostData::new(&"EN".to_string(), &"FR".to_string(), &"Hello\tBonjour!\nGoodbye\tAu revoir!".to_string(), &"tsv".to_string());
        let patch_data = GlossariesApiPostData::new("MyGlossaryUpdated".to_string(), vec![new_dict]);
        let patch_res = patch_glossary(&api, &created_glossary.glossary_id, &patch_data);
        assert!(patch_res.is_ok());

        // Retrieve updated glossary data
        let updated_glossary_response = GlossariesApiList::get_registered_dictionaries(&api);
        assert!(updated_glossary_response.is_ok());
        let updated_glossary_response = updated_glossary_response.unwrap();
        let updated_glossary = &updated_glossary_response.glossaries[0];
        assert_eq!(updated_glossary.name, "MyGlossaryUpdated".to_string());
        assert_eq!(updated_glossary.dictionaries.len(), 1);
        assert_eq!(updated_glossary.dictionaries[0].source_lang.to_uppercase(), "EN".to_string());
        assert_eq!(updated_glossary.dictionaries[0].target_lang.to_uppercase(), "FR".to_string());
        assert_eq!(updated_glossary.dictionaries[0].entry_count, 2);

        // Delete the created glossary
        let delete_res = delete_glossary(&api, &created_glossary.glossary_id);
        assert!(delete_res.is_ok());
    }

    #[test]
    fn impl_glossaries_api_format_debug() {
        let format_tsv = GlossariesApiFormat::Tsv;
        let format_csv = GlossariesApiFormat::Csv;
        assert_eq!(format!("{:?}", format_tsv), "Tsv");
        assert_eq!(format!("{:?}", format_csv), "Csv");
    }
}
