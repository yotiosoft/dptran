//! curlを用いたDeepL APIとの通信

use std::str;
use std::io;
use curl::easy::Easy;

/// curl::easyの準備
fn make_session(url: String, post_data: String) -> Result<Easy, io::Error> {
    let mut easy = Easy::new();
    easy.url(url.as_str())?;
    easy.post(true)?;
    easy.post_fields_copy(post_data.as_bytes())?;
    Ok(easy)
}

/// 送受信
fn transfer(mut easy: Easy) -> Result<(Vec<u8>, u32), io::Error> {
    let mut dst = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            dst.extend_from_slice(data);
            Ok(data.len())
        })?;
        transfer.perform()?;
    }
    let response_code = easy.response_code()?;
    Ok((dst, response_code))
}

/// エラー文生成
fn handle_error(response_code: u32) -> io::Error {
    match response_code {
        400 => io::Error::new(io::ErrorKind::Other, "400 Bad Request"),
        403 => io::Error::new(io::ErrorKind::Other, "403 Forbidden"),
        404 => io::Error::new(io::ErrorKind::Other, "404 Not Found"),
        413 => io::Error::new(io::ErrorKind::Other, "413 Request Entity Too Large"),
        429 => io::Error::new(io::ErrorKind::Other, "429 Too Many Requests"),
        456 => io::Error::new(io::ErrorKind::Other, "456 Unprocessable Entity"),
        503 => io::Error::new(io::ErrorKind::Other, "503 Service Unavailable"),
        529 => io::Error::new(io::ErrorKind::Other, "529 Too Many Requests"),
        _ => io::Error::new(io::ErrorKind::Other, "Unknown Error: No response"),
    }
}

/// DeepL APIとの通信を行う
pub fn send_and_get(url: String, post_data: String) -> Result<String, io::Error> {
    let easy = make_session(url, post_data)?;
    let (dst, response_code) = transfer(easy)?;

    if dst.len() > 0 {
        let s = str::from_utf8(&dst).expect("Invalid UTF-8");
        Ok(s.to_string())
    } else {
        // HTTPエラー処理
        Err(handle_error(response_code))
    }
}
