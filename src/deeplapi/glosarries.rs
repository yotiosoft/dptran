use std::io;

use super::DpTran;

use super::connection;
use connection::ConnectionError;
use super::DeeplAPIError;
use super::ApiKeyType;

pub const DEEPL_API_GLOSARRIES: &str = "https://api-free.deepl.com/v3/glossaries";
pub const DEEPL_API_GLOSARRIES_PRO: &str = "https://api.deepl.com/v3/glossaries";

/// Register a dictionary (glossary).
/// Returns true if successful, false if not.
pub fn create_glosarry(api: &DpTran, name: &String, source_lang: &String, target_lang: &String, entries: &Vec<(String, String)>) -> Result<(), DeeplAPIError> {
    // Get json of translation result with request_translate().

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
