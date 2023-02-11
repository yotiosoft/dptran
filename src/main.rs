use std::io::{self, Write, stdout};
use std::time::Duration;
use async_std::io as async_io;

mod interfaces;
mod parse;

/// 対話と翻訳  
/// 対話モードであれば繰り返し入力を行う  
/// 通常モードであれば一回で終了する
async fn process(mut mode: parse::ExecutionMode, source_lang: String, target_lang: String, multilines: bool, mut text: Vec<String>) -> core::result::Result<(), io::Error> {
    // 翻訳
    // 対話モードならループする; 通常モードでは1回で抜ける
    let stdin = async_io::stdin();
    let init_input = async_io::timeout(Duration::from_millis(50), async {
        let mut init_input = Vec::<String>::new();
        let mut buf = String::new();
        while stdin.read_line(&mut buf).await.unwrap() > 0 {
            init_input.push(buf.clone());
            buf.clear();
        }
        Ok(init_input)
    })
    .await;
    if let Ok(init_input) = init_input {
        text = init_input;
        mode = parse::ExecutionMode::TranslateNormal;
    }

    // 対話モードなら終了方法を表示
    if mode == parse::ExecutionMode::TranslateInteractive {
        if source_lang.len() == 0 {
            println!("Now translating from detected language to {}.", target_lang);
        } else {
            println!("Now translating from {} to {}.", source_lang, target_lang);
        }
        println!("To quit, type \"quit\".");
    }

    let mut stdout = stdout();

    loop {
        // 対話モードなら標準入力から取得
        // 通常モードでは引数から取得
        let input = match mode {
            parse::ExecutionMode::TranslateInteractive => {
                print!("> ");
                stdout.flush()?;

                let mut input_vec = Vec::<String>::new();
                let mut input = String::new();
                while stdin.read_line(&mut input).await? > 0 {
                    if input.clone().trim_end() == "quit" {
                        input_vec.push(input.clone());
                        break;
                    }

                    // multilineモードなら改行を含む入力を受け付ける
                    if multilines {
                        if input == "\r\n" || input == "\n" {
                            break;
                        }
                    }
                    // multilineモードでない場合、\\ + 改行で改行を含む入力を受け付ける
                    else {
                        if input.ends_with("\n") && !input.ends_with("\\\r\n") && !input.ends_with("\\\n") {
                            input_vec.push(input.clone());
                            break;
                        }
                    }

                    input_vec.push(input.clone());
                    input.clear();

                    print!("..");
                    stdout.flush()?;
                }
                input_vec
            }
            parse::ExecutionMode::TranslateNormal => {
                text.clone()
            }
            _ => {
                panic!("Invalid mode.");
            }
        };

        // 対話モード："quit"で終了
        if mode == parse::ExecutionMode::TranslateInteractive {
            if input[0].clone().trim_end() == "quit" {
                break;
            }
            if input[0].clone().trim_end().is_empty() {
                continue;
            }
        }
        // 通常モード：空文字列なら終了
        if mode == parse::ExecutionMode::TranslateNormal && input[0].clone().trim_end().is_empty() {
            break;
        }

        // 翻訳
        let translated_texts = interfaces::translate(input, &target_lang, &source_lang);
        match translated_texts {
            Ok(s) => {
                for translated_text in s {
                    println!("{}", translated_text);
                }
            }
            Err(e) => {
                Err(e)?
            }
        }
        // 通常モードの場合、一回でループを抜ける
        if mode == parse::ExecutionMode::TranslateNormal {
            break;
        }
    }

    Ok(())
}

/// メイン関数
/// 引数の取得と翻訳処理の呼び出し
#[async_std::main]
async fn main() {
    // 引数を解析
    let arg_struct = parse::parser();
    let mode = arg_struct.execution_mode;
    let mut source_lang = String::new();
    let mut target_lang = String::new();
    let mut text = String::new();
    let mut multilines = false;
    match mode {
        parse::ExecutionMode::PrintUsage => {
            match interfaces::show_usage() {
                Ok(_) => return,
                Err(e) => Err(e).unwrap(),
            }
        }
        parse::ExecutionMode::SetApiKey => {
            match interfaces::set_api_key(arg_struct.api_key) {
                Ok(_) => return,
                Err(e) => Err(e).unwrap(),
            }
        }
        parse::ExecutionMode::SetDefaultTargetLang => {
            match interfaces::set_default_target_language(arg_struct.default_target_lang) {
                Ok(_) => return,
                Err(e) => Err(e).unwrap(),
            }
        }
        parse::ExecutionMode::ClearSettings => {
            match interfaces::clear_settings() {
                Ok(_) => return,
                Err(e) => Err(e).unwrap(),
            }
        }
        parse::ExecutionMode::ListSourceLangs => {
            match interfaces::show_source_language_codes() {
                Ok(_) => return,
                Err(e) => Err(e).unwrap(),
            }
        }
        parse::ExecutionMode::ListTargetLangs => {
            match interfaces::show_target_language_codes() {
                Ok(_) => return,
                Err(e) => Err(e).unwrap(),
            }
        }
        _ => {
            source_lang = arg_struct.translate_from;
            target_lang = arg_struct.translate_to;
            text = arg_struct.source_text;
            multilines = arg_struct.multilines;

            if target_lang.len() == 0 {
                target_lang = match interfaces::get_default_target_language_code() {
                    Ok(s) => s,
                    Err(e) => Err(e).unwrap(),
                }
            }
            // EN, PT は EN-US, PT-PT に変換
            if target_lang == "EN" {
                target_lang = "EN-US".to_string();
            }
            if target_lang == "PT" {
                target_lang = "PT-PT".to_string();
            }
        }
    };

    // APIキーの確認
    if interfaces::get_api_key().unwrap_or_default().is_empty() {
        println!("Welcome to dptran!\nFirst, please set your DeepL API-key:\n  $ dptran -c api-key [YOUR_API_KEY]\nYou can get DeepL API-key for free here:\n  https://www.deepl.com/ja/pro-api?cta=header-pro-api/");
        return;
    }

    // 言語コードのチェック
    if source_lang.len() > 0 && interfaces::check_language_code(&source_lang, "source".to_string()) == false {
        println!("Invalid source language code: {}", source_lang);
        return;
    }
    if target_lang.len() > 0 && interfaces::check_language_code(&target_lang, "target".to_string()) == false {
        println!("Invalid target language code: {}", target_lang);
        return;
    }

    // (対話＆)翻訳
    let text_vec = vec![text.to_string()];
    match process(mode, source_lang, target_lang, multilines, text_vec).await {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
