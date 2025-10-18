use core::fmt;
use std::io;
use serde_json::Value;

use super::DpTran;

use super::connection;
use connection::ConnectionError;
use super::DeeplAPIError;
use super::ApiKeyType;

pub const DEEPL_API_GLOSSARIES: &str = "https://api-free.deepl.com/v3/glossaries";
pub const DEEPL_API_GLOSSARIES_PRO: &str = "https://api.deepl.com/v3/glossaries";

#[derive(Debug, PartialEq)]
pub enum GlossaryError {
    ConnectionError(ConnectionError),
    JsonError(String),
    CouldNotCreateGlossary,
}
impl fmt::Display for GlossaryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GlossaryError::ConnectionError(e) => write!(f, "Connection error: {}", e),
            GlossaryError::JsonError(e) => write!(f, "JSON error: {}", e),
            GlossaryError::CouldNotCreateGlossary => write!(f, "Could not create glossary on DeepL API by some reason"),
        }
    }
}

pub enum GlossaryFormat {
    Tsv,
    Csv,
}

pub struct Dictionary {
    source_lang: String,
    target_lang: String,
    entries: Vec<(String, String)>,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct DictionaryPostData {
    source_lang: String,
    target_lang: String,
    entries: String,
    entries_format: String,
}

#[derive(serde::Serialize)]
struct DictionaryResponseData {
    source_lang: String,
    target_lang: String,
    entry_count: u64,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Glossary {
    name: String,
    dictionaries: Vec<DictionaryPostData>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct GlossaryResponseData {
    pub glossary_id: String,
    pub ready: bool,
    pub name: String,
    pub source_lang: String,
    pub target_lang: String,
    pub creation_time: String,
    pub entry_count: u64,
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

impl Glossary {
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
        let post_data = Glossary {
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
                let ret: GlossaryResponseData = serde_json::from_str(&res).map_err(|e| GlossaryError::JsonError(e.to_string()))?;
                Ok(ret)
            },
            Err(e) => Err(GlossaryError::ConnectionError(e)),
        }
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

}
