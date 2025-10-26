use super::DpTran;

use super::DeeplAPIError;
use super::connection;
use super::ApiKeyType;

pub mod api;

/// For the translation API.  
/// Return translation results.  
/// Receive translation results in json format and display translation results.  
/// Return error if json parsing fails.
pub fn translate(api: &DpTran, text: &Vec<String>, target_lang: &String, source_lang: &Option<String>) -> Result<Vec<String>, DeeplAPIError> {
    let translations = api::TranslateRequest {
        text: text.clone(),
        target_lang: target_lang.clone(),
        source_lang: source_lang.clone(),
        ..Default::default()
    };
    let results = translations.translate(api)?;
    let translated_texts = results.get_translation_strings()?;
    Ok(translated_texts)
}

/// For the translation API.  
/// Translate with detailed options.  
/// Return detailed translation results.
pub fn translate_with_options(api: &DpTran, request: &api::TranslateRequest) -> Result<api::TranslateResult, DeeplAPIError> {
    request.translate(api)
}

/// To run these tests, you need to set the environment variable `DPTRAN_DEEPL_API_KEY` to your DeepL API key.  
/// You should run these tests with ``cargo test -- --test-threads=1``
/// because the DeepL API has a limit on the number of requests per second.
/// And also, you need to run the dummy server for the DeepL API to test the API endpoints.
///   $ pip3 install -r requirements.txt
///   $ uvicorn dummy_api_server.main:app --reload
#[cfg(test)]
pub mod tests {
    use super::*;

    fn do_api_translate_test(times: u8) {
        // translate test
        let (api_key, api_key_type) = super::super::tests::get_api_key();
        let api = DpTran::with_endpoint(&api_key, &api_key_type, super::super::tests::get_endpoint());
        let text = vec!["Hello, World!".to_string()];
        let target_lang = "JA".to_string();
        let source_lang = None;
        let res = translate(&api, &text, &target_lang, &source_lang);
        match res {
            Ok(res) => {
                assert_eq!(res[0], "ハロー、ワールド！");
            },
            Err(e) => {
                if super::super::tests::retry_or_panic_for_api_tests(&e, 0) {
                    // retry
                    do_api_translate_test(times + 1);
                    return;
                }
            }
        }
    }

    #[test]
    fn api_translate_test() {
        // translate test
        do_api_translate_test(0);
    }
}
