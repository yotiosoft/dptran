use serde_json::Value;

use super::DpTran;

use super::connection;
use super::DeeplAPIError;
use super::ApiKeyType;

/// Language code and language name
pub type LangCodeName = (String, String);

#[derive(Debug, PartialEq)]
pub enum LangType {
    Source,
    Target,
}

/// API endpoints for getting language codes
pub const DEEPL_API_LANGUAGES: &str = "https://api-free.deepl.com/v2/languages";
pub const DEEPL_API_LANGUAGES_PRO: &str = "https://api.deepl.com/v2/languages";

/// Extended language codes and names.  
/// Because DeepL API's ``/languages`` endpoint returns only the language codes that support document translation,
/// although only text translation is supported. Additionally, if the language code is unspecified variant, it is not returned.  
/// Therefore, dptran adds the following language codes and names manually.  
/// This constants must be updated when the DeepL API is updated.  
/// See <https://developers.deepl.com/docs/resources/supported-languages>.
pub static EXTENDED_LANG_CODES: [(&str, &str, LangType); 2] = [
    ("EN", "English", LangType::Target),
    ("PT", "Portuguese", LangType::Target),
];

/// Parses the translation results passed in json format,
/// stores the translation in a vector, and returns it.
fn json_to_vec(json: &String, type_name: &String) -> Result<Vec<LangCodeName>, DeeplAPIError> {
    let v: Value = serde_json::from_str(&json).map_err(|e| DeeplAPIError::JsonError(e.to_string(), json.clone()))?;

    let lang_type = if type_name == "source" { 
        LangType::Source 
    } else { 
        LangType::Target 
    };

    let mut lang_codes: Vec<LangCodeName> = Vec::new();
    let v_array = v.as_array();
    if let None = v_array {
        if v.to_string().contains("Wrong endpoint") {
            return Err(DeeplAPIError::WrongEndPointError(v.to_string()));
        }
        return Err(DeeplAPIError::JsonError(v.to_string(), json.clone()));
    }
    // Add got language codes
    for value in v_array.unwrap() {
        value.get("language").ok_or(format!("Invalid response: {}", value)).map_err(|e| DeeplAPIError::JsonError(e.to_string(), json.clone()))?;
        // Remove quotation marks
        let lang_code_with_quote = value["language"].to_string();
        let lang_code = &lang_code_with_quote[1..lang_code_with_quote.len()-1];
        let lang_name_with_quote = value["name"].to_string();
        let lang_name = &lang_name_with_quote[1..lang_name_with_quote.len()-1];
        let lang_code_pair = (lang_code.to_string(), lang_name.to_string());
        lang_codes.push(lang_code_pair);
    }
    // Add extended language codes
    for i in 0..EXTENDED_LANG_CODES.len() {
        if EXTENDED_LANG_CODES[i].2 == lang_type {
            // Check: if the language code is already in the list
            if lang_codes.iter().any(|x| x.0 == EXTENDED_LANG_CODES[i].0 && x.1 == EXTENDED_LANG_CODES[i].1) {
                // If it is already in the list, skip it
                continue;
            }
            lang_codes.push((EXTENDED_LANG_CODES[i].0.to_string(), EXTENDED_LANG_CODES[i].1.to_string()));
        }
    }
    Ok(lang_codes)
}

/// Get language code list  
/// Retrieved from <https://api-free.deepl.com/v2/languages>.  
pub fn get_language_codes(api: &DpTran, type_name: String) -> Result<Vec<LangCodeName>, DeeplAPIError> {
    let url = if api.api_key_type == ApiKeyType::Free {
        api.api_urls.languages_for_free.clone()
    } else {
        api.api_urls.languages_for_pro.clone()
    };
    let query = format!("type={}&auth_key={}", type_name, api.api_key);
    let res = connection::post(url, query).map_err(|e| DeeplAPIError::ConnectionError(e))?;

    let mut lang_codes = json_to_vec(&res, &type_name)?;    

    // Sort by language code
    lang_codes.sort_by(|a, b| a.0.cmp(&b.0));
    // return
    if lang_codes.len() == 0 {
        Err(DeeplAPIError::GetLanguageCodesError)
    } else {
        Ok(lang_codes)
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
    fn impl_json_to_vec() {
        let json = r#"[{"language":"EN","name":"English"},{"language":"DE","name":"German"}]"#.to_string();
        let res = json_to_vec(&json, &"source".to_string());
        match res {
            Ok(res) => {
                assert_eq!(res[0], ("EN".to_string(), "English".to_string()));
                assert_eq!(res[1], ("DE".to_string(), "German".to_string()));
            },
            Err(e) => {
                panic!("Error: {}", e);
            }
        }
    }
}
