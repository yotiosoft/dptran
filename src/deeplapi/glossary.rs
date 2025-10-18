use std::io;

use super::DpTran;

use super::connection;
use connection::ConnectionError;
use super::DeeplAPIError;
use super::ApiKeyType;

pub const DEEPL_API_GLOSSARIES: &str = "https://api-free.deepl.com/v3/glossaries";
pub const DEEPL_API_GLOSSARIES_PRO: &str = "https://api.deepl.com/v3/glossaries";

pub enum GlossaryFormat {
    Tsv,
    Csv,
}

pub struct Glossary {
    source_lang: String,
    target_lang: String,
    entries: Vec<(String, String)>,
}

#[derive(serde::Serialize)]
struct DictionaryPostData {
    source_lang: String,
    target_lang: String,
    entries: String,
    entries_format: String,
}

#[derive(serde::Serialize)]
pub struct GlossaryPostData {
    name: String,
    dictionaries: Vec<DictionaryPostData>,
}

#[derive(serde::Deserialize)]
struct DictionaryResponseData {
    source_lang: String,
    target_lang: String,
    entry_count: u64,
}

#[derive(serde::Deserialize)]
struct GlossaryResponseData {
    glossary_id: String,
    ready: bool,
    name: String,
    dictionaries: Vec<DictionaryResponseData>,
    creation_time: String,
}

impl Glossary {
    /// Create a glossary.
    pub fn new(source_lang: &String, target_lang: &String, entries: &Vec<(String, String)>) -> Self {
        Glossary {
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
    pub fn new(name: &String, glossaries: &Vec<Glossary>, entries_format: &GlossaryFormat) -> Self {
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
            name: name.clone(),
            dictionaries: dictionaries,
        };
        post_data
    }

    /// Create a curl session.
    pub fn send(&self, api: &DpTran) -> Result<String, ConnectionError> {
        let url = if api.api_key_type == ApiKeyType::Free {
            DEEPL_API_GLOSSARIES.to_string()
        } else {
            DEEPL_API_GLOSSARIES_PRO.to_string()
        };
        
        let header_auth_key = format!("Authorization: DeepL-Auth-Key {}", api.api_key);
        let header_content_type = "Content-Type: application/json";
        let headers = vec![header_auth_key, header_content_type.to_string()];
        let post_data_json = serde_json::to_string(self).unwrap();
        connection::post_with_headers(url, post_data_json, &headers)
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
