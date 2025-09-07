use serde_json::Value;

use super::DpTran;

use super::connection;
use super::DeeplAPIError;
use super::ApiKeyType;

pub const DEEPL_API_USAGE: &str = "https://api-free.deepl.com/v2/usage";
pub const DEEPL_API_USAGE_PRO: &str = "https://api.deepl.com/v2/usage";

/// Parses the translation results passed in json format,
/// stores the translation in a vector, and returns it.
fn json_to_vec(json: &String) -> Result<(u64, u64), DeeplAPIError> {
    let v: Value = serde_json::from_str(&json).map_err(|e| DeeplAPIError::JsonError(e.to_string()))?;
    v.get("character_count").ok_or(format!("failed to get character_count: {}", v).to_string()).map_err(|e| DeeplAPIError::JsonError(e.to_string()))?;
    v.get("character_limit").ok_or(format!("failed to get character_limit: {}", v).to_string()).map_err(|e| DeeplAPIError::JsonError(e.to_string()))?;
    let character_count = v["character_count"].as_u64().expect("failed to get character_count");
    let character_limit = v["character_limit"].as_u64().expect("failed to get character_limit");
    Ok((character_count, character_limit))
}

/// Get the number of characters remaining to be translated.  
/// Retrieved from <https://api-free.deepl.com/v2/usage>.  
/// Returns an error if acquisition fails.  
pub fn get_usage(api: &DpTran) -> Result<(u64, u64), DeeplAPIError> {
    let url = if api.api_key_type == ApiKeyType::Free {
        api.api_urls.usage_for_free.clone()
    } else {
        api.api_urls.usage_for_pro.clone()
    };
    let query = format!("auth_key={}", api.api_key);
    let res = connection::post(url, query).map_err(|e| DeeplAPIError::ConnectionError(e))?;
    
    let (character_count, character_limit) = json_to_vec(&res)?;
    Ok((character_count, character_limit))
}

/// To run these tests, you need to set the environment variable `DPTRAN_DEEPL_API_KEY` to your DeepL API key.  
/// You should run these tests with ``cargo test -- --test-threads=1`` because the DeepL API has a limit on the number of requests per second.
/// And also, you need to run the dummy server for the DeepL API to test the API endpoints.
///   $ pip3 install -r requirements.txt
///   $ uvicorn dummy-api-server:app --reload
#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn api_json_to_vec_test() {
        let json = r#"{"character_count":12345,"character_limit":500000}"#.to_string();
        let res = json_to_vec(&json);
        assert!(res.is_ok());
        let (character_count, character_limit) = res.unwrap();
        assert_eq!(character_count, 12345);
        assert_eq!(character_limit, 500000);
    }
}
