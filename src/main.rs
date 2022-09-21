use std::io;
use regex::Regex;

mod interfaces;

enum ArgMode {
    Sentence,
    SourceLanguage,
    TargetLanguage,
    Settings,
    SettingAPIKey,
    SettingDefaultTagetLanguage
}

#[derive(PartialEq)]
enum ExecutionMode {
    Normal,
    Interactive
}

/// 引数から各値を抽出  
/// 引数リスト、設定値を渡し、翻訳の実行が必要か否かを示すboolean、実行モード、翻訳元言語、翻訳先言語、原文を抽出してタプルとして返す
fn get_args(args: Vec<String>, settings: &interfaces::settings::Settings) -> core::result::Result<(bool, ExecutionMode, String, String, String), io::Error> {
    // 引数を解析
    let mut arg_mode: ArgMode = ArgMode::Sentence;
    let mut source_lang = String::new();
    let mut target_lang = String::new();
    let mut text = String::new();
    for arg in &args[1..] {
        match arg.as_str() {
            // オプションの抽出
            // ヘルプ
            "-h" | "--help" => {
                interfaces::show_help();
                return Ok((false, ExecutionMode::Normal, String::new(), String::new(), String::new()));
            }
            // 言語コード一覧の表示
            "-ls" => {
                interfaces::show_source_language_codes()?;
                return Ok((false, ExecutionMode::Normal, String::new(), String::new(), String::new()));
            }
            "-lt" => {
                interfaces::show_target_language_codes()?;
                return Ok((false, ExecutionMode::Normal, String::new(), String::new(), String::new()));
            }
            // バージョン情報
            "-v" | "--version" => {
                interfaces::show_version();
                return Ok((false, ExecutionMode::Normal, String::new(), String::new(), String::new()));
            }
            // 残り翻訳可能文字数
            "-r" | "--remain" => {
                let (character_count, character_limit) = interfaces::get_remain()?;
                println!("remain: {} / {}", character_count, character_limit);
                return Ok((false, ExecutionMode::Normal, String::new(), String::new(), String::new()));
            }
            // 設定（次の引数を参照）
            "-s" | "--set" => {
                arg_mode = ArgMode::Settings;
            }
            // 翻訳先言語指定
            "-t" | "--to" => {
                arg_mode = ArgMode::TargetLanguage;
            }
            // 翻訳元言語指定
            "-f" | "--from" => {
                arg_mode = ArgMode::SourceLanguage;
            }
            // それ以外
            _ => {
                // 無効なオプション
                let re = Regex::new(r"^-.+").expect("failed to compile regex");
                if re.is_match(arg.as_str()) {
                    return Err(io::Error::new(io::ErrorKind::Other, "Invalid option"))
                }

                // それ以外
                match arg_mode {
                    // 入力原文
                    ArgMode::Sentence => {
                        // 入力引数文字列間に半角空白文字を挿入
                        if text.len() > 0 {
                            text.push(' ');
                        }
                        text.push_str(arg.as_str());
                    }
                    // 翻訳先言語指定
                    ArgMode::SourceLanguage => {
                        source_lang = arg.to_string();
                        arg_mode = ArgMode::Sentence;
                    }
                    // 翻訳元言語指定
                    ArgMode::TargetLanguage => {
                        target_lang = arg.to_string();
                        arg_mode = ArgMode::Sentence;
                    }
                    // 各設定項目のオプション
                    ArgMode::Settings => {
                        match arg.as_str() {
                            // APIキー
                            "api-key" => {
                                arg_mode = ArgMode::SettingAPIKey;
                            }
                            // 既定の翻訳先言語
                            "default-lang" => {
                                arg_mode = ArgMode::SettingDefaultTagetLanguage;
                            }
                            // 設定のクリア
                            "clear" => {
                                break;
                            }
                            // その他：無効な設定オプション
                            _ => {
                                Err(io::Error::new(io::ErrorKind::Other, "Unknown settings"))?;
                            }
                        }
                    }
                    // APIキーの設定：APIキー値を取得
                    ArgMode::SettingAPIKey => {
                        interfaces::settings::set_apikey(arg.to_string())?;
                        return Ok((false, ExecutionMode::Normal, source_lang, target_lang, text));
                    }
                    // 既定の翻訳先言語の設定：言語コードを取得
                    ArgMode::SettingDefaultTagetLanguage => {
                        interfaces::settings::set_default_target_language(arg.to_string())?;
                        return Ok((false, ExecutionMode::Normal, source_lang, target_lang, text));
                    }
                }
            }
        }
    }

    // 引数に原文がなければ対話モードへ
    let mode = if text.is_empty() {
        ExecutionMode::Interactive
    } else {
        ExecutionMode::Normal
    };

    // 翻訳先言語が未指定なら既定値へ
    if target_lang.is_empty() {
        target_lang = settings.default_target_language.clone();
    }

    return Ok((true, mode, source_lang, target_lang, text));
}

/// 対話と翻訳  
/// 対話モードであれば繰り返し入力を行う  
/// 通常モードであれば一回で終了する
fn process(mode: ExecutionMode, source_lang: String, target_lang: String, text: String, settings: &interfaces::settings::Settings) -> core::result::Result<(), io::Error> {
    // 翻訳
    // 対話モードならループする; 通常モードでは1回で抜ける
    loop {
        // 対話モードなら標準入力から取得
        // 通常モードでは引数から取得
        let input = match mode {
            ExecutionMode::Interactive => {
                let mut input = String::new();
                let bytes = io::stdin().read_line(&mut input).expect("Failed to read line.");
                // 入力が空なら終了
                if bytes == 0 {
                    break;
                }
                input
            }
            ExecutionMode::Normal => {
                text.clone()
            }
        };

        // 対話モード："exit"で終了
        if mode == ExecutionMode::Interactive && input.clone().trim_end() == "exit" {
            break;
        }
        // 通常モード：空文字列なら終了
        if mode == ExecutionMode::Normal && input.clone().trim_end().is_empty() {
            break;
        }

        // 翻訳
        let translated_texts = interfaces::translate(&settings.api_key, input, &target_lang, &source_lang);
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
        if mode == ExecutionMode::Normal {
            break;
        }
    }

    Ok(())
}

/// メイン関数
/// 引数の取得と翻訳処理の呼び出し
fn main() {
    // 設定の取得
    let settings = interfaces::settings::get_settings();

    // APIキーの確認
    if settings.api_key.is_empty() {
        println!("API key is not set. Please set it with the -s option:\n\t$ dptran -s api-key [YOUR_API_KEY]");
        return;
    }

    // 引数を受け取る
    let args: Vec<String> = std::env::args().collect();

    // 引数を解析
    let (to_translate, mode, source_lang, target_lang, text) = match get_args(args, &settings) {
        Ok(v) => v,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    if to_translate == false {
        return;
    }

    // (対話＆)翻訳
    match process(mode, source_lang, target_lang, text, &settings) {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
