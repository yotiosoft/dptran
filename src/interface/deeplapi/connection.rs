//! curlを用いたDeepL APIとの通信

use std::str;
use curl::easy::Easy;

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
fn handle_error(response_code: u32) -> String {
    match response_code {
        400 => "400 Bad Request".to_string(),
        403 => "403 Forbidden".to_string(),
        404 => "404 Not Found".to_string(),
        413 => "413 Request Entity Too Large".to_string(),
        429 => "429 Too Many Requests".to_string(),
        456 => "456 Unprocessable Entity".to_string(),
        503 => "503 Service Unavailable".to_string(),
        529 => "529 Too Many Requests".to_string(),
        _ => "Unknown Error: No response".to_string(),
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
        Err(handle_error(response_code))
    }
}
