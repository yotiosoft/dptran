use super::DpTran;

use super::DeeplAPIError;
use super::connection;
use super::ApiKeyType;

pub mod api;

/// For the languages API.  
/// Get language code list  
/// Retrieved from <https://api-free.deepl.com/v2/languages>.  
pub fn get_language_codes(api: &DpTran, type_name: String) -> Result<Vec<api::LangCodeName>, DeeplAPIError> {
    api::get_language_codes(api, type_name)
}

/// To run these tests, you need to set the environment variable `DPTRAN_DEEPL_API_KEY` to your DeepL API key.  
/// You should run these tests with ``cargo test -- --test-threads=1`` because the DeepL API has a limit on the number of requests per second.
/// And also, you need to run the dummy server for the DeepL API to test the API endpoints.
///   $ pip3 install -r requirements.txt
///   $ uvicorn dummy_api_server.main:app --reload
#[cfg(test)]
pub mod tests {
    use super::*;

    fn do_api_get_language_codes_test(times: u8) {
        // get_language_codes test
        let (api_key, api_key_type) = super::super::tests::get_api_key();
        let api = DpTran::with_endpoint(&api_key, &api_key_type, super::super::tests::get_endpoint());
        // Get language codes for source languages
        let res = get_language_codes(&api, "source".to_string());
        match res {
            Ok(res) => {
                if res.len() == 0 {
                    panic!("Error: language codes is empty");
                }

                // Are there extended language codes?
                let mut found = false;
                for i in 0..api::EXTENDED_LANG_CODES.len() {
                    if res.iter().any(|x| x.0 == api::EXTENDED_LANG_CODES[i].0 && x.1 == api::EXTENDED_LANG_CODES[i].1) {
                        found = true;
                        break;
                    }
                }
                if !found {
                    panic!("Error: extended language codes not found");
                }
            },
            Err(e) => {
                if super::super::tests::retry_or_panic(&e, times) {
                    // retry
                    do_api_get_language_codes_test(times + 1);
                    return;
                }
            }
        }
    }

    #[test]
    fn api_get_language_codes_test() {
        // get_language_codes test
        do_api_get_language_codes_test(0);
    }
}
