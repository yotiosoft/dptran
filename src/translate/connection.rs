//! curlを用いたDeepL APIとの通信

use std::str;
use curl::easy::Easy;

pub fn send_and_get(url: String, post_data: String) -> String {
    let mut dst = Vec::new();

    let mut easy = Easy::new();
    easy.url(url.as_str()).unwrap();
    easy.post(true).unwrap();
    easy.post_fields_copy(post_data.as_bytes()).unwrap();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            dst.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    str::from_utf8(&dst).unwrap().to_string()
}
