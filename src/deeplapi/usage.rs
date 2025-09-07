use serde_json::Value;

use super::DpTran;

use super::connection;
use super::DeeplAPIError;
use super::ApiKeyType;

pub const DEEPL_API_USAGE: &str = "https://api-free.deepl.com/v2/usage";
pub const DEEPL_API_USAGE_PRO: &str = "https://api.deepl.com/v2/usage";

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
    let v: Value = serde_json::from_str(&res).map_err(|e| DeeplAPIError::JsonError(e.to_string()))?;

    v.get("character_count").ok_or(format!("failed to get character_count: {}", v).to_string()).map_err(|e| DeeplAPIError::JsonError(e.to_string()))?;
    v.get("character_limit").ok_or(format!("failed to get character_limit: {}", v).to_string()).map_err(|e| DeeplAPIError::JsonError(e.to_string()))?;

    let character_count = v["character_count"].as_u64().expect("failed to get character_count");
    let character_limit = v["character_limit"].as_u64().expect("failed to get character_limit");
    Ok((character_count, character_limit))
}
