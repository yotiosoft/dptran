use super::DpTran;

use super::DeeplAPIError;
use super::connection;
use super::ApiKeyType;

pub mod api;

/// For the translation API.  
/// Translate with detailed options.  
/// Return detailed translation results.
pub fn translate(api: &DpTran, request: &api::TranslateRequest) -> Result<api::TranslateResult, DeeplAPIError> {
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
        let translations = api::TranslateRequest {
            text: text.clone(),
            source_lang: source_lang.clone(),
            target_lang: target_lang.clone(),
            ..Default::default()
        };
        let res = translate(&api, &translations);
        match res {
            Ok(res) => {
                let res: Vec<String> = res.translations.iter().map(|t| t.text.clone()).collect();
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

    #[test]
    fn api_translate_with_options_test() {
        // translate_with_options test
        let (api_key, api_key_type) = super::super::tests::get_api_key();
        let api = DpTran::with_endpoint(&api_key, &api_key_type, super::super::tests::get_endpoint());
        let request = api::TranslateRequest {
            text: vec!["Good morning".to_string()],
            source_lang: Some("EN".to_string()),
            target_lang: "FR".to_string(),
            formality: Some(api::Formality::More),
            ..Default::default()
        };
        let res = translate(&api, &request);
        match res {
            Ok(res) => {
                assert_eq!(res.translations[0].detected_source_language.to_ascii_uppercase(), "EN");
            },
            Err(e) => {
                if super::super::tests::retry_or_panic_for_api_tests(&e, 0) {
                    // retry
                    api_translate_with_options_test();
                    return;
                }
            }
        }
    }
}
