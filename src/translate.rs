//! 入力文章の原文の送信と翻訳文の受信
use std::io;
mod connection;

/// 翻訳
pub fn translate(auth_key:String, text: String, target_lang: String, source_lang: String) -> Result<String, io::Error> {
    let url = "https://api-free.deepl.com/v2/translate".to_string();
    let d = format!("auth_key={}&text={}&target_lang={}&source_lang={}", auth_key, text, target_lang, source_lang);
    
    connection::send_and_get(url, d)
}
