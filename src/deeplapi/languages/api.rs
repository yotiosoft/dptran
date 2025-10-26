use serde_json::Value;

use super::DpTran;

use super::connection;
use super::DeeplAPIError;
use super::ApiKeyType;

/// Language information
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Language {
    pub language: String,
    pub name: String,
    pub supports_formality: bool,
}

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
fn get_languages_list(json: &String) -> Result<Vec<Language>, DeeplAPIError> {
    // Add got language codes
    let langs: Vec<Language> = serde_json::from_str(&json).map_err(|e| DeeplAPIError::JsonError(e.to_string(), json.clone()))?;
    Ok(langs)
}

/// Parses the translation results passed in json format,
/// stores the translation in a vector, and returns it.
fn json_to_vec(json: &String, type_name: &String) -> Result<Vec<LangCodeName>, DeeplAPIError> {
    let v: Value = serde_json::from_str(&json).map_err(|e| DeeplAPIError::JsonError(e.to_string(), json.clone()))?;

    let lang_type = if type_name == "source" { 
        LangType::Source 
    } else if type_name == "target" {
        LangType::Target 
    } else {
        return Err(DeeplAPIError::InvalidLangTypeError(type_name.clone()));
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
    let langs = get_languages_list(&json)?;
    for lang in langs.iter() {
        match lang_type {
            LangType::Source => {
                lang_codes.push((lang.language.clone(), lang.name.clone()));
            },
            LangType::Target => {
                lang_codes.push((lang.language.clone(), lang.name.clone()));
            },
        }
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
    let url = format!("{}?type={}", url, type_name);
    let header_auth_key = format!("Authorization: DeepL-Auth-Key {}", api.api_key);
    let header_content_type = "Content-Type: application/json";
    let headers = vec![header_auth_key, header_content_type.to_string()];
    let res = connection::get_with_headers(url, &headers).map_err(|e| DeeplAPIError::ConnectionError(e))?;
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

/// Get language code list and return as Language struct vector.
/// Retrieved from <https://api-free.deepl.com/v2/languages>.
pub fn get_languages_as_struct(api: &DpTran, type_name: String) -> Result<Vec<Language>, DeeplAPIError> {
    let url = if api.api_key_type == ApiKeyType::Free {
        api.api_urls.languages_for_free.clone()
    } else {
        api.api_urls.languages_for_pro.clone()
    };
    let url = format!("{}?type={}", url, type_name);
    let header_auth_key = format!("Authorization: DeepL-Auth-Key {}", api.api_key);
    let header_content_type = "Content-Type: application/json";
    let headers = vec![header_auth_key, header_content_type.to_string()];
    let res = connection::get_with_headers(url, &headers).map_err(|e| DeeplAPIError::ConnectionError(e))?;

    let result = get_languages_list(&res)?;

    Ok(result)
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

    #[test]
    fn api_sorted_languages_test() {
        // get_language_codes test
        let (api_key, api_key_type) = super::super::super::tests::get_api_key();
        let api = DpTran::with_endpoint(&api_key, &api_key_type, super::super::super::tests::get_endpoint());
        // Get language codes for source languages
        let res = get_language_codes(&api, "source".to_string());
        match res {
            Ok(res) => {
                if res.len() == 0 {
                    panic!("Error: language codes is empty");
                }

                // Check if the language codes are sorted
                for i in 0..res.len()-1 {
                    if res[i].0 > res[i+1].0 {
                        panic!("Error: language codes are not sorted");
                    }
                }
            },
            Err(e) => {
                panic!("Error: {}", e);
            }
        }
        // Are there extended language codes?
        let res = get_language_codes(&api, "source".to_string());
        match res {
            Ok(res) => {
                let mut found = false;
                for i in 0..EXTENDED_LANG_CODES.len() {
                    if res.iter().any(|x| x.0 == EXTENDED_LANG_CODES[i].0 && x.1 == EXTENDED_LANG_CODES[i].1) {
                        found = true;
                        break;
                    }
                }
                if !found {
                    panic!("Error: extended language codes not found");
                }
            },
            Err(e) => {
                panic!("Error: {}", e);
            }
        }
        // Are they sorted?
        let res = get_language_codes(&api, "source".to_string());
        match res {
            Ok(res) => {
                // Check if the language codes are sorted
                for i in 0..res.len()-1 {
                    if res[i].0 > res[i+1].0 {
                        panic!("Error: language codes are not sorted");
                    }
                }
            },
            Err(e) => {
                panic!("Error: {}", e);
            }
        }
        
        // Get language codes for target languages
        let res = get_language_codes(&api, "target".to_string());
        match res {
            Ok(res) => {
                if res.len() == 0 {
                    panic!("Error: language codes is empty");
                }

                // Check if the language codes are sorted
                for i in 0..res.len()-1 {
                    if res[i].0 > res[i+1].0 {
                        panic!("Error: language codes are not sorted");
                    }
                }
            },
            Err(e) => {
                panic!("Error: {}", e);
            }
        }
        // Are there extended language codes?
        let res = get_language_codes(&api, "target".to_string());
        match res {
            Ok(res) => {
                let mut found = false;
                for i in 0..EXTENDED_LANG_CODES.len() {
                    if res.iter().any(|x| x.0 == EXTENDED_LANG_CODES[i].0 && x.1 == EXTENDED_LANG_CODES[i].1) {
                        found = true;
                        break;
                    }
                }
                if !found {
                    panic!("Error: extended language codes not found");
                }
            },
            Err(e) => {
                panic!("Error: {}", e);
            }
        }
        // Are they sorted?
        let res = get_language_codes(&api, "target".to_string());
        match res {
            Ok(res) => {
                // Check if the language codes are sorted
                for i in 0..res.len()-1 {
                    if res[i].0 > res[i+1].0 {
                        panic!("Error: language codes are not sorted");
                    }
                }
            },
            Err(e) => {
                panic!("Error: {}", e);
            }
        }
    }

    #[test]
    fn api_get_languages_as_struct_test() {
        // get_languages_as_struct test
        let (api_key, api_key_type) = super::super::super::tests::get_api_key();
        let api = DpTran::with_endpoint(&api_key, &api_key_type, super::super::super::tests::get_endpoint());
        let res = get_languages_as_struct(&api, "source".to_string());
        match res {
            Ok(res) => {
                if res.len() == 0 {
                    panic!("Error: languages is empty");
                }
            },
            Err(e) => {
                panic!("Error: {}", e);
            }
        }
    }
}
