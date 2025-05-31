//! curlを用いたDeepL APIとの通信

use std::str;
use std::fmt;
use curl::easy::Easy;

/// ConnectionErrors.  
/// It is an error that occurs when communicating with the DeepL API.  
/// ``BadRequest``: 400 Bad Request  
/// ``Forbidden``: 403 Forbidden  
/// ``NotFound``: 404 Not Found  
/// ``RequestEntityTooLarge``: 413 Request Entity Too Large  
/// ``TooManyRequests``: 429 Too Many Requests  
/// ``UnprocessableEntity``: 456 Unprocessable Entity  
/// ``ServiceUnavailable``: 503 Service Unavailable  
/// ``CurlError``: Curl Error  
/// ``UnknownError``: Unknown Error  
#[derive(Debug, PartialEq)]
pub enum ConnectionError {
    BadRequest,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    RequestEntityTooLarge,
    TooManyRequests,
    UnprocessableEntity,
    ServiceUnavailable,
    CurlError(String),
    UnknownError,
}
impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConnectionError::BadRequest => write!(f, "400 Bad Request"),
            ConnectionError::Forbidden => write!(f, "403 Forbidden"),
            ConnectionError::NotFound => write!(f, "404 Not Found"),
            ConnectionError::MethodNotAllowed => write!(f, "405 Method Not Allowed"),
            ConnectionError::RequestEntityTooLarge => write!(f, "413 Request Entity Too Large"),
            ConnectionError::TooManyRequests => write!(f, "429 Too Many Requests"),
            ConnectionError::UnprocessableEntity => write!(f, "456 Unprocessable Entity"),
            ConnectionError::ServiceUnavailable => write!(f, "503 Service Unavailable"),
            ConnectionError::CurlError(ref e) => write!(f, "Curl Error: {}", e),
            ConnectionError::UnknownError => write!(f, "Unknown Error"),
        }
    }
}

/// Preparing curl::easy
fn make_session(url: String, post_data: String) -> Result<Easy, String> {
    let mut easy = Easy::new();
    easy.url(url.as_str()).map_err(|e| e.to_string())?;
    easy.post(true).map_err(|e| e.to_string())?;
    easy.post_fields_copy(post_data.as_bytes()).map_err(|e| e.to_string())?;
    Ok(easy)
}

/// Sending and Receiving
fn transfer(mut easy: Easy) -> Result<(Vec<u8>, u32), String> {
    let mut dst = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            dst.extend_from_slice(data);
            Ok(data.len())
        }).map_err(|e| e.to_string())?;
        transfer.perform().map_err(|e| e.to_string())?;
    }
    let response_code = easy.response_code().map_err(|e| e.to_string())?;
    Ok((dst, response_code))
}

/// Error statement generation
fn handle_error(response_code: u32) -> ConnectionError {
    match response_code {
        400 => ConnectionError::BadRequest,
        403 => ConnectionError::Forbidden,
        404 => ConnectionError::NotFound,
        405 => ConnectionError::MethodNotAllowed,
        413 => ConnectionError::RequestEntityTooLarge,
        429 => ConnectionError::TooManyRequests,
        456 => ConnectionError::UnprocessableEntity,
        503 => ConnectionError::ServiceUnavailable,
        _ => ConnectionError::UnknownError,
    }
}

/// Communicate with the DeepL API.
pub fn post(url: String, post_data: String) -> Result<String, ConnectionError> {
    let easy = match make_session(url, post_data) {
        Ok(easy) => easy,
        Err(e) => return Err(ConnectionError::CurlError(e)),
    };
    let (dst, response_code) = match transfer(easy) {
        Ok((dst, response_code)) => (dst, response_code),
        Err(e) => return Err(ConnectionError::CurlError(e)),
    };

    if dst.len() > 0 {
        let s = str::from_utf8(&dst).expect("Invalid UTF-8");
        Ok(s.to_string())
    } else {
        // HTTP Error Handling
        Err(handle_error(response_code))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn send_and_get_test() {
        let url = "https://api-free.deepl.com/v2/".to_string();
        let post_data = "text=Hello&target_lang=FR&auth_key=".to_string();
        let api_key = std::env::var("DPTRAN_DEEPL_API_KEY").unwrap();
        let url = format!("{}translate?auth_key={}", url, api_key);
        let result = post(url, post_data);
        assert!(result.is_ok());
    }

    #[test]
    fn handling_error_test() {
        let error_code = 400;
        let error = handle_error(error_code);
        assert_eq!(error, ConnectionError::BadRequest);

        let error_code = 403;
        let error = handle_error(error_code);
        assert_eq!(error, ConnectionError::Forbidden);

        let error_code = 404;
        let error = handle_error(error_code);
        assert_eq!(error, ConnectionError::NotFound);

        let error_code = 413;
        let error = handle_error(error_code);
        assert_eq!(error, ConnectionError::RequestEntityTooLarge);

        let error_code = 429;
        let error = handle_error(error_code);
        assert_eq!(error, ConnectionError::TooManyRequests);

        let error_code = 456;
        let error = handle_error(error_code);
        assert_eq!(error, ConnectionError::UnprocessableEntity);

        let error_code = 503;
        let error = handle_error(error_code);
        assert_eq!(error, ConnectionError::ServiceUnavailable);

        let error_code = 999;
        let error = handle_error(error_code);
        assert_eq!(error, ConnectionError::UnknownError);
    }
}
