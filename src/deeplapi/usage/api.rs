use super::DpTran;

use super::super::connection;
use super::DeeplAPIError;
use super::super::ApiKeyType;

pub const DEEPL_API_USAGE: &str = "https://api-free.deepl.com/v2/usage";
pub const DEEPL_API_USAGE_PRO: &str = "https://api.deepl.com/v2/usage";

/// Usage struct
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Usage {
    pub character_count: u64,
    pub character_limit: u64,
}

/// Get the number of characters remaining to be translated.  
/// Retrieved from <https://api-free.deepl.com/v2/usage>.  
/// Returns an error if acquisition fails.  
pub fn get_usage(api: &DpTran) -> Result<(u64, u64), DeeplAPIError> {
    let usage = get_usage_as_struct(api)?;
    Ok((usage.character_count, usage.character_limit))
}

/// Get the number of characters remaining to be translated and return as Usage struct.
pub fn get_usage_as_struct(api: &DpTran) -> Result<Usage, DeeplAPIError> {
    let url = if api.api_key_type == ApiKeyType::Free {
        api.api_urls.usage_for_free.clone()
    } else {
        api.api_urls.usage_for_pro.clone()
    };
    let header_auth_key = format!("Authorization: DeepL-Auth-Key {}", api.api_key);
    let header_content_type = "Content-Type: application/json";
    let headers = vec![header_auth_key, header_content_type.to_string()];
    let res = connection::get_with_headers(url, &headers).map_err(|e| DeeplAPIError::ConnectionError(e))?;
    let usage: Usage = serde_json::from_str(&res).map_err(|e| DeeplAPIError::JsonError(e.to_string(), res.clone()))?;
    Ok(usage)
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
    fn impl_deserialize_usage() {
        let json = r#"{"character_count":12345,"character_limit":1000000}"#.to_string();
        let usage: Usage = serde_json::from_str(&json).unwrap();
        assert_eq!(usage.character_count, 12345);
        assert_eq!(usage.character_limit, 1000000);
    }
}
