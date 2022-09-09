//! 入力文章の原文の送信と翻訳文の受信

use std::str;
use curl::easy::Easy;

/// 翻訳
pub fn translate(auth_key:String, text: String, target_lang: String, source_lang: String) -> String {
    let d = format!("auth_key={}&text={}&target_lang={}&source_lang={}", auth_key, text, target_lang, source_lang);
    let mut dst = Vec::new();
    println!("{}", d);

    let mut easy = Easy::new();
    easy.url("https://api-free.deepl.com/v2/translate").unwrap();
    easy.post(true).unwrap();
    easy.post_fields_copy(d.as_bytes()).unwrap();
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
