//! 入力文章の原文の送信と翻訳文の受信
use std::io;
mod connection;

/// 翻訳
pub fn translate(auth_key: String, text: String, target_lang: String, source_lang: String) -> Result<String, io::Error> {
    let url = "https://api-free.deepl.com/v2/translate".to_string();
    let d = format!("auth_key={}&text={}&target_lang={}&source_lang={}", auth_key, text, target_lang, source_lang);
    
    connection::send_and_get(url, d)
}

// 対話モード
fn interactive_mode(auth_key: String, target_lang: String, source_lang: String) -> Result<(), io::Error> {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line.");

        if input == "exit" {
            Ok(())
        }

        let translated_sentence = translate(auth_key.clone(), input, target_lang.clone(), source_lang.clone());
        match translated_sentence {
            Ok(s) => {
                println!("translated: {}", s);
                let j_translate: Result<Value, io::Error> = serde_json::from_str(&s);
                match j_translate {
                    Ok(v) => {
                        println!("translated sentence: {}", v["translations"][0]["text"]);
                    }
                    Err(e) => {
                        println!("json parse error: {}", e);
                    }
                }
            }
            Err(e) => {
                translated_sentence
            }
        }
    }
}
