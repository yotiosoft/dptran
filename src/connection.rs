//! curlを用いたDeepL APIとの通信

use std::str;
use std::io;
use curl::easy::Easy;

pub fn send_and_get(url: String, post_data: String) -> Result<String, io::Error> {
    let mut dst = Vec::new();

    let mut easy = Easy::new();
    easy.url(url.as_str())?;
    easy.post(true)?;
    easy.post_fields_copy(post_data.as_bytes())?;
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            dst.extend_from_slice(data);
            Ok(data.len())
        })?;
        transfer.perform()?;
    }

    if dst.len() > 0 {
        let s = str::from_utf8(&dst).expect("Invalid UTF-8");
        Ok(s.to_string())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "No response"))
    }
}
