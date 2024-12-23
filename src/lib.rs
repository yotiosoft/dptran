mod deeplapi;

pub use deeplapi::LangCodeName;
pub use deeplapi::DeeplAPIError;
pub use deeplapi::ConnectionError;

/// string as language code
pub type LangCode = String;

/// Errors that can occur in this library.  
/// ``DeeplApiError``: DeepL API error  
/// ``InvalidLanguageCode``: Invalid language code  
/// ``ApiKeyIsNotSet``: API key is not set  
/// ``NoTargetLanguageSpecified``: No target language specified  
/// ``CouldNotGetInputText``: Could not get input text  
#[derive(Debug, PartialEq)]
pub enum DpTranError {
    DeeplApiError(DeeplAPIError),
    InvalidLanguageCode,
    ApiKeyIsNotSet,
    NoTargetLanguageSpecified,
    CouldNotGetInputText,
}
impl ToString for DpTranError {
    fn to_string(&self) -> String {
        match self {
            DpTranError::DeeplApiError(e) => format!("Deepl API error: {}", e.to_string()),
            DpTranError::InvalidLanguageCode => "Invalid language code".to_string(),
            DpTranError::ApiKeyIsNotSet => "API key is not set".to_string(),
            DpTranError::NoTargetLanguageSpecified => "No target language specified".to_string(),
            DpTranError::CouldNotGetInputText => "Could not get input text".to_string(),
        }
    }
}

/// Target / Source language types  
/// used in get_language_codes()  
pub enum LangType {
    Target,
    Source,
}

/// DeepL API usage information  
/// character_count: Number of characters translated this month  
/// character_limit: Maximum number of characters that can be translated this month  
/// If character_limit is 0, it is unlimited  
pub struct DpTranUsage {
    pub character_count: u64,
    pub character_limit: u64,
    pub unlimited: bool,
}

/// Get language code list. Using DeepL API.  
/// Retrieved from <https://api-free.deepl.com/v2/languages>.  
/// api_key: DeepL API key  
/// lang_type: Target or Source  
pub fn get_language_codes(api_key: &String, lang_type: LangType) -> Result<Vec<LangCodeName>, DpTranError> {
    let type_name = match lang_type {
        LangType::Target => "target".to_string(),
        LangType::Source => "source".to_string(),
    };
    let lang_codes = deeplapi::get_language_codes(&api_key, type_name).map_err(|e| DpTranError::DeeplApiError(e))?;
    Ok(lang_codes)
}

/// Check the validity of language codes. Using DeepL API.  
/// api_key: DeepL API key  
/// lang_code: Language code to check  
/// lang_type: Target or Source  
pub fn check_language_code(api_key: &String, lang_code: &String, lang_type: LangType) -> Result<bool, DpTranError> {
    let lang_codes = get_language_codes(api_key, lang_type)?;
    for lang in lang_codes {
        if lang.0.trim_matches('"') == lang_code.to_uppercase() {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Convert to correct language code from input source language code string. Using DeepL API.  
/// api_key: DeepL API key  
/// language_code: Language code to convert  
/// Caution: EN, PT are not automatically converted to EN-US, PT-PT from version 2.1.0.
pub fn correct_source_language_code(api_key: &String, language_code: &str) -> Result<LangCode, DpTranError> {
    let source_language = language_code.to_ascii_uppercase().to_string();
    match check_language_code(api_key, &source_language, LangType::Source)? {
        true => Ok(source_language),
        false => Err(DpTranError::InvalidLanguageCode),
    }
}

/// Convert to correct language code from input target language code string. Using DeepL API.
/// api_key: DeepL API key
/// language_code: Language code to convert
/// Caution: EN, PT are not automatically converted to EN-US, PT-PT from version 2.1.0.
pub fn correct_target_language_code(api_key: &String, language_code: &str) -> Result<LangCode, DpTranError> {
    let target_language = language_code.to_ascii_uppercase().to_string();
    match check_language_code(api_key, &target_language, LangType::Target)? {
        true => Ok(target_language),
        false => Err(DpTranError::InvalidLanguageCode),
    }
}

/// Get the number of characters remaining to be translated. Using DeepL API.  
/// Retrieved from <https://api-free.deepl.com/v2/usage>.  
/// Returns an error if acquisition fails.  
/// api_key: DeepL API key  
pub fn get_usage(api_key: &String) -> Result<DpTranUsage, DpTranError> {
    let (count, limit) = deeplapi::get_usage(&api_key).map_err(|e| DpTranError::DeeplApiError(e))?;
    Ok(DpTranUsage {
        character_count: count,
        character_limit: limit,
        unlimited: limit == 0,
    })
}

/// Display translation results. Using DeepL API.  
/// Receive translation results in json format and display translation results.  
/// Return error if json parsing fails.  
/// api_key: DeepL API key  
/// text: Text to translate  
/// target_lang: Target language  
/// source_lang: Source language (optional)  
pub fn translate(api_key: &String, text: Vec<String>, target_lang: &String, source_lang: &Option<String>) -> Result<Vec<String>, DpTranError> {
    deeplapi::translate(&api_key, text, target_lang, source_lang).map_err(|e| DpTranError::DeeplApiError(e))
}

#[test]
/// run with `cargo test api_tests -- <api_key> <DeepL API free = 0, DeepL API pro = 1>`
/// arg[2] : api_key
/// arg[3] : DeepL API free = 0, DeepL API pro = 1
fn lib_tests() {
    if std::env::args().len() < 3 {
        panic!("Usage: cargo test lib_tests -- <api_key> <DeepL API free = 0, DeepL API pro = 1>");
    }

    let mut args = Vec::new();
    for arg in std::env::args().skip(2) {
        println!("arg: {}", arg);
        args.push(arg);
    }

    // translate test
    let api_key = &args[0];
    let text = vec!["Hello, World!".to_string()];
    let target_lang = "JA".to_string();
    let source_lang = None;
    let res = translate(api_key, text, &target_lang, &source_lang);
    match res {
        Ok(res) => {
            //assert_eq!(res[0], "ハロー、ワールド！");
            println!("res: {}", res[0]);
        },
        Err(e) => {
            panic!("Error: {}", e.to_string());
        }
    }

    // usage test
    let res = get_usage(api_key);
    match res {
        Ok(res) => {
            // If you have a pro account, it is not an error.
            if args[1] == "0" && res.character_limit != 500000 {
                panic!("Error: usage limit is not 50000");
            }
            if args[1] == "1" && res.character_limit != 0 {
                panic!("Error: usage limit is not 0");
            }
        },
        Err(e) => {
            panic!("Error: {}", e.to_string());
        }
    }

    // get_language_codes test
    let res = get_language_codes(api_key, LangType::Source);
    match res {
        Ok(res) => {
            if res.len() == 0 {
                panic!("Error: language codes is empty");
            }
        },
        Err(e) => {
            panic!("Error: {}", e.to_string());
        }
    }

    // check_language_code test
    let res = check_language_code(api_key, &"EN-US".to_string(), LangType::Target);
    match res {
        Ok(res) => {
            assert_eq!(res, true);
        },
        Err(e) => {
            panic!("Error: {}", e.to_string());
        }
    }
    let res = check_language_code(api_key, &"XX".to_string(), LangType::Source);
    match res {
        Ok(res) => {
            assert_eq!(res, false);
        },
        Err(e) => {
            panic!("Error: {}", e.to_string());
        }
    }
}
