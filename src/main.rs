use std::io::{self, Write, stdin, stdout};

mod interfaces;
mod parse;

/// 残り文字数を表示
fn show_usage(api_key: &String) -> core::result::Result<(), io::Error> {
    let (character_count, character_limit) = interfaces::deeplapi::get_usage(api_key)?;
    if character_limit == 0 {
        println!("usage: {} / unlimited", character_count);
    }
    else {
        println!("usage: {} / {} ({}%)", character_count, character_limit, (character_count as f64 / character_limit as f64 * 100.0).round());
        println!("remaining: {}", character_limit - character_count);
    }
    Ok(())
}

/// 翻訳元言語コード一覧の表示  
/// <https://api-free.deepl.com/v2/languages>から取得する
fn show_source_language_codes(api_key: &String) -> core::result::Result<(), io::Error> {
    // 翻訳元言語コード一覧
    let source_lang_codes = interfaces::deeplapi::get_language_codes(api_key, "source".to_string())?;
    
    let mut i = 0;
    let (len, max_code_len, max_str_len) = get_langcodes_maxlen(&source_lang_codes);

    println!("Source language codes:");
    for lang_code in source_lang_codes {
        print!(" {lc:<cl$}: {ls:<sl$}", lc=lang_code.0.trim_matches('"'), ls=lang_code.1.trim_matches('"'), cl=max_code_len, sl=max_str_len);
        i += 1;
        if (i % 3) == 0 || i == len {
            println!();
        }
    }

    Ok(())
}
/// 翻訳先言語コード一覧の表示
fn show_target_language_codes(api_key: &String) -> core::result::Result<(), io::Error> {
    // 翻訳先言語コード一覧
    let mut target_lang_codes = interfaces::deeplapi::get_language_codes(api_key, "target".to_string())?;

    // 特例コード変換
    target_lang_codes.push(("EN".to_string(), "English".to_string()));
    target_lang_codes.push(("PT".to_string(), "Portuguese".to_string()));

    let mut i = 0;
    let (len, max_code_len, max_str_len) = get_langcodes_maxlen(&target_lang_codes);

    println!("Target languages:");
    for lang_code in target_lang_codes {
        print!(" {lc:<cl$}: {ls:<sl$}", lc=lang_code.0.trim_matches('"'), ls=lang_code.1.trim_matches('"'), cl=max_code_len, sl=max_str_len);
        i += 1;
        if (i % 2) == 0 || i == len {
            println!();
        }
    }

    Ok(())
}
fn get_langcodes_maxlen(lang_codes: &Vec<(String, String)>) -> (usize, usize, usize) {
    let len = lang_codes.len();
    let max_code_len = lang_codes.iter().map(|x| x.0.len()).max().unwrap();
    let max_str_len = lang_codes.iter().map(|x| x.1.len()).max().unwrap();
    (len, max_code_len, max_str_len)
}

/// 対話と翻訳  
/// 対話モードであれば繰り返し入力を行う  
/// 通常モードであれば一回で終了する
fn process(api_key: String, mode: parse::ExecutionMode, source_lang: String, target_lang: String, multilines: bool, text: Vec<String>) -> Result<(), io::Error> {
    // 翻訳
    // 対話モードならループする; 通常モードでは1回で抜ける
    let stdin = stdin();

    // 対話モードなら終了方法を表示
    if mode == parse::ExecutionMode::TranslateInteractive {
        if source_lang.len() == 0 {
            println!("Now translating from detected language to {}.", target_lang);
        } else {
            println!("Now translating from {} to {}.", source_lang, target_lang);
        }
        if multilines {
            println!("Multiline mode: Enter a blank line to send the input.");
        }
        println!("Type \"quit\" to exit dptran.");
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
                while stdin.read_line(&mut input)? > 0 {
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
        let translated_texts = interfaces::deeplapi::translate(&api_key, input, &target_lang, &source_lang);
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
fn main() {
    // API keyの取得
    let api_key = interfaces::get_api_key().expect("Failed to get API key.");

    // 引数を解析
    let arg_struct = parse::parser();
    let mode = arg_struct.execution_mode;
    let mut source_lang = String::new();
    let mut target_lang = String::new();
    let mut text = String::new();
    let mut multilines = false;
    let mode_switch = match mode {
        parse::ExecutionMode::PrintUsage => {
            show_usage(&api_key)
        }
        parse::ExecutionMode::SetApiKey => {
            interfaces::set_api_key(arg_struct.api_key)
        }
        parse::ExecutionMode::SetDefaultTargetLang => {
            interfaces::set_default_target_language(arg_struct.default_target_lang)
        }
        parse::ExecutionMode::ClearSettings => {
            interfaces::clear_settings()
        }
        parse::ExecutionMode::ListSourceLangs => {
            show_source_language_codes(&api_key)
        }
        parse::ExecutionMode::ListTargetLangs => {
            show_target_language_codes(&api_key)
        }
        _ => {
            source_lang = arg_struct.translate_from;
            target_lang = arg_struct.translate_to;
            text = arg_struct.source_text;
            multilines = arg_struct.multilines;

            if target_lang.len() == 0 {
                match interfaces::get_default_target_language_code() {
                    Ok(s) => target_lang = s,
                    Err(e) => Err(e).unwrap(),
                }
            }

            Ok(())
        }
    };
    if mode_switch.is_err() {
        println!("Error: {}", mode_switch.err().unwrap());
        return;
    }
    if mode != parse::ExecutionMode::TranslateInteractive && mode != parse::ExecutionMode::TranslateNormal {
        return;
    }

    // APIキーの確認
    if interfaces::get_api_key().unwrap_or_default().is_empty() {
        println!("Welcome to dptran!\nFirst, please set your DeepL API-key:\n  $ dptran -c api-key [YOUR_API_KEY]\nYou can get DeepL API-key for free here:\n  https://www.deepl.com/ja/pro-api?cta=header-pro-api/");
        return;
    }

    // 言語コードのチェック & 正しい言語コードに変換
    if source_lang.len() > 0 {
        match interfaces::correct_language_code(&source_lang.to_string()) {
            Ok(s) => source_lang = s,
            Err(e) => {
                println!("Error: {}", e);
                return;
            },
        }
    }
    if target_lang.len() > 0 {
        match interfaces::correct_language_code(&target_lang.to_string()) {
            Ok(t) => target_lang = t,
            Err(e) => {
                println!("Error: {}", e);
                return;
            },
        }
    }

    // (対話＆)翻訳
    let text_vec = vec![text.to_string()];
    match process(api_key, mode, source_lang, target_lang, multilines, text_vec) {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
