//! 入力文章の原文の送信と翻訳文の受信

use curl::easy::Easy;

/// 翻訳
pub fn translate(auth_key:String, text: String, target_lang: String, source_lang: String) -> String {
    let data = format!("auth_key={}&text={}&target_lang={}&source_lang={}", auth_key, text, target_lang, source_lang).as_bytes();

    let mut easy = Easy::new();
    easy.url("https://api-free.deepl.com/v2/translate").unwrap();
    easy.post(true).unwrap();
    easy.post_field_size(data.len() as u64).unwrap();

    let mut transfer = easy.transfer();
    transfer.read_function(|buf| {
        Ok(data.as_bytes().read(buf).unwrap_or(0))
    }).unwrap();
    transfer.perform().unwrap();

    "あいうえお".to_string()
}
