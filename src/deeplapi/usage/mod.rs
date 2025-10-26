use super::DpTran;
use super::DeeplAPIError;

pub mod api;

/// For the usage API.  
/// Get the number of characters remaining to be translated.  
/// Retrieved from <https://api-free.deepl.com/v2/usage>.  
/// Returns an error if acquisition fails.  
pub fn get_usage(api: &DpTran) -> Result<(u64, u64), DeeplAPIError> {
    api::get_usage(api)
}

/// To run these tests, you need to set the environment variable `DPTRAN_DEEPL_API_KEY` to your DeepL API key.  
/// You should run these tests with ``cargo test -- --test-threads=1`` because the DeepL API has a limit on the number of requests per second.
/// And also, you need to run the dummy server for the DeepL API to test the API endpoints.
///   $ pip3 install -r requirements.txt
///   $ uvicorn dummy_api_server.main:app --reload
#[cfg(test)]
pub mod tests {
    use super::*;

    fn do_api_usage_test(times: u8) {
        // usage test
        let (api_key, api_key_type) = super::super::tests::get_api_key();
        let api = DpTran::with_endpoint(&api_key, &api_key_type, super::super::tests::get_endpoint());
        let res = get_usage(&api);
        if res.is_err() {
            if super::super::tests::retry_or_panic_for_api_tests(&res.err().unwrap(), times) {
                // retry
                do_api_usage_test(times + 1);
                return;
            }
        }
    }

    #[test]
    fn api_usage_test() {
        // usage test
        do_api_usage_test(0);
    }
}
