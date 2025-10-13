use std::io;

use super::DpTran;

use super::connection;
use connection::ConnectionError;
use super::DeeplAPIError;
use super::ApiKeyType;

pub const DEEPL_API_GLOSARRIES: &str = "https://api-free.deepl.com/v3/glossaries";
pub const DEEPL_API_GLOSARRIES_PRO: &str = "https://api.deepl.com/v3/glossaries";

pub enum GlosarryFormat {
    Tsv,
    Csv,
}

pub struct Glosarry {
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
struct GlossariesPostData {
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
struct GlossariesResponseData {
    glossary_id: String,
    ready: bool,
    name: String,
    dictionaries: Vec<DictionaryResponseData>,
    creation_time: String,
}

/// Create a glosarry post data.
pub fn create_glosarry_post(api: &DpTran, name: &String, glosarries: &Vec<Glosarry>) -> GlossariesPostData {
    // Prepare post data
    let mut dictionaries: Vec<DictionaryPostData> = Vec::new();
    for g in glosarries {
        // Currently, we use only TSV format.
        let mut entries = String::new();
        for (source, target) in &g.entries {
            let source = source.replace("\t", " ");  // Replace tab characters in the source
            let target = target.replace("\t", " ");  // Replace tab characters in the target
            entries = format!("{}\n{}\t{}", entries, source, target);
        }
        let dict_post_data = DictionaryPostData {
            source_lang: g.source_lang.clone(),
            target_lang: g.target_lang.clone(),
            entries: entries.trim().to_string(),
            entries_format: "tsv".to_string(),
        };
        dictionaries.push(dict_post_data);
    }
    let post_data = GlossariesPostData {
        name: name.clone(),
        dictionaries: dictionaries,
    };
    post_data
}

/// Create a curl session.
pub fn post_glosarry(api: &DpTran, post_data: &GlossariesPostData) -> Result<String, ConnectionError> {
    let url = if api.api_key_type == ApiKeyType::Free {
        DEEPL_API_GLOSARRIES.to_string()
    } else {
        DEEPL_API_GLOSARRIES_PRO.to_string()
    };
    
    let header_auth_key = format!("Authorization: DeepL-Auth-Key {}", api.api_key);
    let header_content_type = "Content-Type: application/json";
    let headers = vec![header_auth_key, header_content_type.to_string()];
    let post_data_json = serde_json::to_string(post_data).unwrap();
    connection::post_with_headers(url, post_data_json, &headers)
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
