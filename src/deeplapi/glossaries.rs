use core::fmt;

use crate::deeplapi::DeeplAPIMessage;

use super::DpTran;

use super::connection;
use connection::ConnectionError;
use super::DeeplAPIError;
use super::ApiKeyType;

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
pub enum GlossaryFormat {
    Tsv,
    Csv,
}

/// Glossary dictionary.  
/// A glossary consists of one or more pairs of words.
pub struct Dictionary {
    source_lang: String,
    target_lang: String,
    entries: Vec<(String, String)>,
}

/// Data structures for glossary API.
#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct DictionaryPostData {
    source_lang: String,
    target_lang: String,
    entries: String,
    entries_format: String,
}

/// Response data structures for glossary API.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct DictionaryResponseData {
    pub source_lang: String,
    pub target_lang: String,
    pub entry_count: u64,
}

/// Glossary post data structure.  
/// Used to create a glossary via the DeepL API.  
/// You need to create an instance of this struct to send a glossary creation request.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct GlossaryPostData {
    name: String,
    dictionaries: Vec<DictionaryPostData>,
}

/// Response data structure for glossary creation via DeepL API.  
/// Returned when a glossary is successfully created.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct GlossaryResponseData {
    pub glossary_id: String,
    pub name: String,
    pub dictionaries: Vec<DictionaryResponseData>,
    pub creation_time: String,
}

/// List of glossaries registered in the DeepL API.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct GlossariesList {
    pub glossaries: Vec<GlossaryResponseData>,
}

/// An entry of supported languages by Glossaries API.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct SupportedLanguageEntry {
    pub source_lang: String,
    pub target_lang: String,
}

/// List of supported languages by Glossaries API.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct SupportedLanguages {
    pub supported_languages: Vec<SupportedLanguageEntry>,
}

impl Dictionary {
    /// Create a glossary.
    pub fn new(source_lang: &String, target_lang: &String, entries: &Vec<(String, String)>) -> Self {
        Dictionary {
            source_lang: source_lang.clone(),
            target_lang: target_lang.clone(),
            entries: entries.clone(),
        }
    }

    /// Add an entry to the glossary.
    pub fn add_entry(&mut self, source_term: &String, target_term: &String) {
        self.entries.push((source_term.clone(), target_term.clone()));
    }
}

impl GlossaryPostData {
    /// Create a glossary post data.
    pub fn new(glossary_name: &String, glossaries: &Vec<Dictionary>, entries_format: &GlossaryFormat) -> Self {
        // Prepare post data
        let mut dictionaries: Vec<DictionaryPostData> = Vec::new();
        for g in glossaries {
            // Currently, we use only TSV format.
            let mut entries = String::new();
            for (source, target) in &g.entries {
                let source = source.replace("\t", " ");  // Replace tab characters in the source
                let target = target.replace("\t", " ");  // Replace tab characters in the target
                entries = match entries_format {
                    GlossaryFormat::Tsv => format!("{}{}\t{}\n", entries, source, target),
                    GlossaryFormat::Csv => format!("{}\"{}\",\"{}\"\n", entries, source.replace("\"", "\"\""), target.replace("\"", "\"\"")),
                };
            }

            // Create dictionary post data
            let entries_format_str = match entries_format {
                GlossaryFormat::Tsv => "tsv",
                GlossaryFormat::Csv => "csv",
            }.to_string();
            let dict_post_data = DictionaryPostData {
                source_lang: g.source_lang.clone(),
                target_lang: g.target_lang.clone(),
                entries: entries,
                entries_format: entries_format_str,
            };
            dictionaries.push(dict_post_data);
        }
        let post_data = GlossaryPostData {
            name: glossary_name.clone(),
            dictionaries: dictionaries,
        };
        post_data
    }

    /// Create a curl session.
    pub fn send(&self, api: &DpTran) -> Result<GlossaryResponseData, GlossaryError> {
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
                let ret: GlossaryResponseData = serde_json::from_str(&res).map_err(|e| GlossaryError::JsonError(e.to_string(), res.clone()))?;
                Ok(ret)
            },
            Err(e) => Err(GlossaryError::ConnectionError(e)),
        }
    }

    /// Get glossary dictionaries.
    pub fn get_dictionaries(&self) -> Vec<DictionaryPostData> {
        self.dictionaries.clone()
    }
}

impl DictionaryPostData {
    /// Get source language.
    pub fn get_source_lang(&self) -> &String {
        &self.source_lang
    }

    /// Get target language.
    pub fn get_target_lang(&self) -> &String {
        &self.target_lang
    }
}

impl GlossariesList {
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
                let ret: GlossariesList = serde_json::from_str(&res).map_err(|e| DeeplAPIError::JsonError(e.to_string(), res.clone()))?;
                Ok(ret)
            },
            Err(e) => Err(DeeplAPIError::ConnectionError(e)),
        }
    }
}

impl GlossaryResponseData {
    /// Delete the glossary via DeepL API.
    pub fn delete(&self, api: &DpTran) -> Result<(), GlossaryError> {
        let url = if api.api_key_type == ApiKeyType::Free {
            format!("{}/{}", DEEPL_API_GLOSSARIES, self.glossary_id)
        } else {
            format!("{}/{}", DEEPL_API_GLOSSARIES_PRO, self.glossary_id)
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
}

impl SupportedLanguages {
    /// Get supported languages for Glossaries API.
    pub fn get(api: &DpTran) -> Result<SupportedLanguages, GlossaryError> {
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
                let ret: SupportedLanguages = serde_json::from_str(&res).map_err(|e| GlossaryError::JsonError(e.to_string(), res.clone()))?;
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
///   $ uvicorn dummy-api-server:app --reload
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_glossaries_dictionary() {
        let mut dict = Dictionary::new(&"EN".to_string(), &"FR".to_string(), &vec![]);
        dict.add_entry(&"Hello".to_string(), &"Bonjour".to_string());
        dict.add_entry(&"Goodbye".to_string(), &"Au revoir".to_string());
        assert_eq!(dict.source_lang, "EN".to_string());
        assert_eq!(dict.target_lang, "FR".to_string());
        assert_eq!(dict.entries.len(), 2);
        assert_eq!(dict.entries[0], ("Hello".to_string(), "Bonjour".to_string()));
        assert_eq!(dict.entries[1], ("Goodbye".to_string(), "Au revoir".to_string()));
    }

    #[test]
    fn api_glossaries_post_data() {
        let dict1 = Dictionary::new(&"EN".to_string(), &"FR".to_string(), &vec![
            ("Hello".to_string(), "Bonjour".to_string()),
            ("Goodbye".to_string(), "Au revoir".to_string()),
        ]);
        let dict2 = Dictionary::new(&"DE".to_string(), &"EN".to_string(), &vec![
            ("Hallo".to_string(), "Hello".to_string()),
            ("Tschüss".to_string(), "Goodbye".to_string()),
        ]);
        let glossaries = vec![dict1, dict2];
        let post_data = GlossaryPostData::new(&"MyGlossary".to_string(), &glossaries, &GlossaryFormat::Tsv);
        assert_eq!(post_data.name, "MyGlossary".to_string());
        assert_eq!(post_data.dictionaries.len(), 2);
    }

    #[test]
    fn api_glossaries_post_send() {
        // Send glossary creation request to DeepL API
        let api_key = std::env::var("DPTRAN_DEEPL_API_KEY").unwrap();
        let api = DpTran::with(&api_key, &ApiKeyType::Free);
        let dict = Dictionary::new(&"EN".to_string(), &"FR".to_string(), &vec![
            ("Hello".to_string(), "Bonjour".to_string()),
            ("Goodbye".to_string(), "Au revoir".to_string()),
        ]);
        let glossaries = vec![dict];
        let post_data = GlossaryPostData::new(&"MyGlossary".to_string(), &glossaries, &GlossaryFormat::Tsv);
        let res = post_data.send(&api);
        assert!(res.is_ok());
        
        // Retrieve response data
        let glossary_response = GlossariesList::get_registered_dictionaries(&api);
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
        let delete_res = created_glossary.delete(&api);
        assert!(delete_res.is_ok());
    }

    #[test]
    fn api_glossaries_get_registered_dictionaries() {
        let api_key = std::env::var("DPTRAN_DEEPL_API_KEY").unwrap();
        let api = DpTran::with(&api_key, &ApiKeyType::Free);
        let res = GlossariesList::get_registered_dictionaries(&api);
        assert!(res.is_ok());
    }

    #[test]
    fn api_glossaries_supported_languages() {
        let api_key = std::env::var("DPTRAN_DEEPL_API_KEY").unwrap();
        let api = DpTran::with(&api_key, &ApiKeyType::Free);
        let res = SupportedLanguages::get(&api);
        assert!(res.is_ok());
        let supported_languages = res.unwrap();
        assert!(supported_languages.is_lang_pair_supported(&"EN".to_string(), &"FR".to_string()));
        assert!(!supported_languages.is_lang_pair_supported(&"EN".to_string(), &"XX".to_string()));
    }
}
