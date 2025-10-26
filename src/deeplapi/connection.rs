//! Connection with DeepL API using curl

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
    NoContent,
    BadRequest,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    RequestEntityTooLarge,
    TooManyRequests,
    UnprocessableEntity,
    ServiceUnavailable,
    CurlError(String),
    UnknownError(u32),
}
impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConnectionError::NoContent => write!(f, "204 No Content"),
            ConnectionError::BadRequest => write!(f, "400 Bad Request"),
            ConnectionError::Forbidden => write!(f, "403 Forbidden"),
            ConnectionError::NotFound => write!(f, "404 Not Found"),
            ConnectionError::MethodNotAllowed => write!(f, "405 Method Not Allowed"),
            ConnectionError::RequestEntityTooLarge => write!(f, "413 Request Entity Too Large"),
            ConnectionError::TooManyRequests => write!(f, "429 Too Many Requests"),
            ConnectionError::UnprocessableEntity => write!(f, "456 Unprocessable Entity"),
            ConnectionError::ServiceUnavailable => write!(f, "503 Service Unavailable"),
            ConnectionError::CurlError(ref e) => write!(f, "Curl Error: {}", e),
            ConnectionError::UnknownError(code) => write!(f, "Unknown Error: {}", code),
        }
    }
}

/// Create a POST session
fn make_post_session(url: String, post_data: String) -> Result<Easy, String> {
    let mut easy = Easy::new();
    easy.url(url.as_str()).map_err(|e| e.to_string())?;
    easy.post(true).map_err(|e| e.to_string())?;
    easy.post_fields_copy(post_data.as_bytes()).map_err(|e| e.to_string())?;
    Ok(easy)
}

/// Create a GET session
fn make_get_session(url: String) -> Result<Easy, String> {
    let mut easy = Easy::new();
    easy.url(url.as_str()).map_err(|e| e.to_string())?;
    Ok(easy)
}

/// Create a DELETE session
fn make_delete_session(url: String) -> Result<Easy, String> {
    let mut easy = Easy::new();
    easy.url(url.as_str()).map_err(|e| e.to_string())?;
    easy.custom_request("DELETE").map_err(|e| e.to_string())?;
    Ok(easy)
}

/// Create a PATCH session
fn make_patch_session(url: String, patch_data: String) -> Result<Easy, String> {
    let mut easy = Easy::new();
    easy.url(url.as_str()).map_err(|e| e.to_string())?;
    easy.custom_request("PATCH").map_err(|e| e.to_string())?;
    easy.post_fields_copy(patch_data.as_bytes()).map_err(|e| e.to_string())?;
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
        204 => ConnectionError::NoContent,
        400 => ConnectionError::BadRequest,
        403 => ConnectionError::Forbidden,
        404 => ConnectionError::NotFound,
        405 => ConnectionError::MethodNotAllowed,
        413 => ConnectionError::RequestEntityTooLarge,
        429 => ConnectionError::TooManyRequests,
        456 => ConnectionError::UnprocessableEntity,
        503 => ConnectionError::ServiceUnavailable,
        code => ConnectionError::UnknownError(code),
    }
}

/// Communicate with the DeepL API with header
pub fn post_with_headers(url: String, post_data: String, header: &Vec<String>) -> Result<String, ConnectionError> {
    let mut easy = match make_post_session(url, post_data) {
        Ok(easy) => easy,
        Err(e) => return Err(ConnectionError::CurlError(e)),
    };
    {
        let mut list = curl::easy::List::new();
        for h in header {
            list.append(h).map_err(|e| ConnectionError::CurlError(e.to_string()))?;
        }
        easy.http_headers(list).map_err(|e| ConnectionError::CurlError(e.to_string()))?;
    }
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

/// Get method with headers
pub fn get_with_headers(url: String, header: &Vec<String>) -> Result<String, ConnectionError> {
    let mut easy = match make_get_session(url) {
        Ok(easy) => easy,
        Err(e) => return Err(ConnectionError::CurlError(e)),
    };
    {
        let mut list = curl::easy::List::new();
        for h in header {
            list.append(h).map_err(|e| ConnectionError::CurlError(e.to_string()))?;
        }
        easy.http_headers(list).map_err(|e| ConnectionError::CurlError(e.to_string()))?;
    }
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

/// Delete method with headers
pub fn delete_with_headers(url: String, header: &Vec<String>) -> Result<(), ConnectionError> {
    let mut easy = match make_delete_session(url) {
        Ok(easy) => easy,
        Err(e) => return Err(ConnectionError::CurlError(e)),
    };
    {
        let mut list = curl::easy::List::new();
        for h in header {
            list.append(h).map_err(|e| ConnectionError::CurlError(e.to_string()))?;
        }
        easy.http_headers(list).map_err(|e| ConnectionError::CurlError(e.to_string()))?;
    }
    let response_code = match transfer(easy) {
        Ok((_, response_code)) => response_code,
        Err(e) => return Err(ConnectionError::CurlError(e)),
    };

    if response_code == 200 || response_code == 204 {
        Ok(())
    } else {
        // HTTP Error Handling
        Err(handle_error(response_code))
    }
}

/// Patch method with headers
pub fn patch_with_headers(url: String, patch_data: String, header: &Vec<String>) -> Result<String, ConnectionError> {
    let mut easy = match make_patch_session(url, patch_data) {
        Ok(easy) => easy,
        Err(e) => return Err(ConnectionError::CurlError(e)),
    };
    {
        let mut list = curl::easy::List::new();
        for h in header {
            list.append(h).map_err(|e| ConnectionError::CurlError(e.to_string()))?;
        }
        easy.http_headers(list).map_err(|e| ConnectionError::CurlError(e.to_string()))?;
    }
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
pub mod tests {
    use super::*;

    #[test]
    fn access_post_with_headers_test() {
        let url = "http://localhost:8000/".to_string();
        let post_data = "test".to_string();
        let headers = vec!["Content-Type: application/json".to_string()];
        let result = post_with_headers(url, post_data, &headers);
        assert!(result.is_ok());
    }

    #[test]
    fn access_delete_test() {
        let url = "http://localhost:8000/".to_string();
        let headers = vec!["Content-Type: application/json".to_string()];
        let result = delete_with_headers(url, &headers);
        assert!(result.is_ok());
    }

    #[test]
    fn access_patch_with_headers_test() {
        let url = "http://localhost:8000/".to_string();
        let patch_data = "test".to_string();
        let headers = vec!["Content-Type: application/json".to_string()];
        let result = patch_with_headers(url, patch_data, &headers);
        assert!(result.is_ok());
    }

    #[test]
    fn impl_handling_error_test() {
        let error_code = 204;
        let error = handle_error(error_code);
        assert_eq!(error, ConnectionError::NoContent);

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
        assert_eq!(error, ConnectionError::UnknownError(999));
    }
}
