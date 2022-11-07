//! curlを用いたDeepL APIとの通信

use std::str;
use std::io;
use curl::easy::Easy;

/// DeepL APIとの通信を行う
pub fn send_and_get(url: String, post_data: String) -> Result<String, io::Error> {
    let mut dst = Vec::new();

    let mut easy = Easy::new();
    easy.url(url.as_str())?;
    easy.post(true)?;
    easy.post_fields_copy(post_data.as_bytes())?;
    println!("post_data: {}", post_data);
    println!("post_data as_bytes: {:?}", post_data.as_bytes());
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            dst.extend_from_slice(data);
            println!("post_data data: {:?}", data);
            Ok(data.len())
        })?;
        transfer.perform()?;
    }

    if dst.len() > 0 {
        let s = str::from_utf8(&dst).expect("Invalid UTF-8");
        Ok(s.to_string())
    } else {
        // HTTPエラー処理
        match easy.response_code().expect("failed to get response code") {
            400 => Err(io::Error::new(io::ErrorKind::Other, "400 Bad Request")),
            403 => Err(io::Error::new(io::ErrorKind::Other, "403 Forbidden")),
            404 => Err(io::Error::new(io::ErrorKind::Other, "404 Not Found")),
            413 => Err(io::Error::new(io::ErrorKind::Other, "413 Request Entity Too Large")),
            429 => Err(io::Error::new(io::ErrorKind::Other, "429 Too Many Requests")),
            456 => Err(io::Error::new(io::ErrorKind::Other, "456 Unprocessable Entity")),
            503 => Err(io::Error::new(io::ErrorKind::Other, "503 Service Unavailable")),
            529 => Err(io::Error::new(io::ErrorKind::Other, "529 Too Many Requests")),
            _ => Err(io::Error::new(io::ErrorKind::Other, "Unknown Error: No response")),
        }
    }
}
