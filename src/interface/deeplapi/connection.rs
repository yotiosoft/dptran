//! curlを用いたDeepL APIとの通信

use std::str;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use curl::easy::Easy;

#[derive(Debug)]
pub enum ConnectionError {
    BadRequest,
    Forbidden,
    NotFound,
    RequestEntityTooLarge,
    TooManyRequests,
    UnprocessableEntity,
    ServiceUnavailable,
    UnknownError,
}

impl Display for ConnectionError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            ConnectionError::BadRequest => write!(f, "400 Bad Request"),
            ConnectionError::Forbidden => write!(f, "403 Forbidden"),
            ConnectionError::NotFound => write!(f, "404 Not Found"),
            ConnectionError::RequestEntityTooLarge => write!(f, "413 Request Entity Too Large"),
            ConnectionError::TooManyRequests => write!(f, "429 Too Many Requests"),
            ConnectionError::UnprocessableEntity => write!(f, "456 Unprocessable Entity"),
            ConnectionError::ServiceUnavailable => write!(f, "503 Service Unavailable"),
            ConnectionError::UnknownError => write!(f, "Unknown Error"),
        }
    }
}

/// curl::easyの準備
fn make_session(url: String, post_data: String) -> Result<Easy, String> {
    let mut easy = Easy::new();
    easy.url(url.as_str()).map_err(|e| e.to_string())?;
    easy.post(true).map_err(|e| e.to_string())?;
    easy.post_fields_copy(post_data.as_bytes()).map_err(|e| e.to_string())?;
    Ok(easy)
}

/// 送受信
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

/// エラー文生成
fn handle_error(response_code: u32) -> ConnectionError {
    match response_code {
        400 => ConnectionError::BadRequest,
        403 => ConnectionError::Forbidden,
        404 => ConnectionError::NotFound,
        413 => ConnectionError::RequestEntityTooLarge,
        429 => ConnectionError::TooManyRequests,
        456 => ConnectionError::UnprocessableEntity,
        503 => ConnectionError::ServiceUnavailable,
        _ => ConnectionError::UnknownError,
    }
}

/// DeepL APIとの通信を行う
pub fn send_and_get(url: String, post_data: String) -> Result<String, String> {
    let easy = make_session(url, post_data)?;
    let (dst, response_code) = transfer(easy)?;

    if dst.len() > 0 {
        let s = str::from_utf8(&dst).expect("Invalid UTF-8");
        Ok(s.to_string())
    } else {
        // HTTPエラー処理
        Err(handle_error(response_code).to_string())
    }
}
