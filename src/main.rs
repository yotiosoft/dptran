use std::{io, env};
use serde_json::Value;
use regex::Regex;

mod connection;
mod settings;

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

/// ヘルプの表示
fn show_help() {
    println!("To translate with optional languages, usage: deepl [options] [sentence]");
    println!("Options:");
    println!("  -f or --from\t\t\tSet source language");
    println!("  -t or --to\t\t\tSet target language");
    println!("If -f is not specified, the source language is automatically inferred by DeepL.");
    println!("If -t is not specified, the translation is done into the configured default target language.");
    println!("");
    println!("To setup setting options, usage: deepl -s [setting options]");
    println!("Setting options:");
    println!("  -s default-lang\t\tSetup default target language");
    println!("  -s api-key\t\t\tSetup your DeepL API key");
    println!("  -s clear\t\t\tClear all settings");
    println!("");
    println!("For other options, usage: deepl [options]");
    println!("Options:");
    println!("  -h or --help\t\t\tShow this help message");
    println!("  -lt\t\t\t\tShow all supported target language codes");
    println!("  -ls\t\t\t\tShow all supported source language codes");
    println!("  -v or --version\t\tShow version");
}

/// バージョン情報の表示  
/// CARGO_PKG_VERSIONから取得する
fn show_version() {
    println!("dptran version {}", env!("CARGO_PKG_VERSION"));
}

/// 翻訳可能な残り文字数の表示  
/// <https://api-free.deepl.com/v2/usage>より取得する  
/// 取得に失敗したらエラーを返す
fn get_remain() -> core::result::Result<(i32, i32), io::Error> {
    let url = "https://api-free.deepl.com/v2/uasage".to_string();
    let query = format!("auth_key={}", settings::get_settings().api_key);
    let res = connection::send_and_get(url, query)?;
    let v: Value = serde_json::from_str(&res)?;

    v.get("character_count").ok_or(io::Error::new(io::ErrorKind::Other, "failed to get character_count"))?;
    v.get("character_limit").ok_or(io::Error::new(io::ErrorKind::Other, "failed to get character_limit"))?;

    let character_count = v["character_count"].as_i64().expect("failed to get character_count") as i32;
    let character_limit = v["character_limit"].as_i64().expect("failed to get character_limit") as i32;
    Ok((character_count, character_limit))
}

/// 引数から各値を抽出  
/// 引数リスト、設定値を渡し、翻訳の実行が必要か否かを示すboolean、実行モード、翻訳元言語、翻訳先言語、原文を抽出してタプルとして返す
fn get_args(args: Vec<String>, settings: &settings::Settings) -> core::result::Result<(bool, ExecutionMode, String, String, String), io::Error> {
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
                show_help();
                return Ok((false, ExecutionMode::Normal, String::new(), String::new(), String::new()));
            }
            // 言語コード一覧の表示
            "-ls" => {
                show_source_language_codes()?;
                return Ok((false, ExecutionMode::Normal, String::new(), String::new(), String::new()));
            }
            "-lt" => {
                show_target_language_codes()?;
                return Ok((false, ExecutionMode::Normal, String::new(), String::new(), String::new()));
            }
            // バージョン情報
            "-v" | "--version" => {
                show_version();
                return Ok((false, ExecutionMode::Normal, String::new(), String::new(), String::new()));
            }
            // 残り翻訳可能文字数
            "-r" | "--remain" => {
                let (character_count, character_limit) = get_remain()?;
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
                        text += arg;
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
                        settings::set_apikey(arg.to_string())?;
                        return Ok((false, ExecutionMode::Normal, source_lang, target_lang, text));
                    }
                    // 既定の翻訳先言語の設定：言語コードを取得
                    ArgMode::SettingDefaultTagetLanguage => {
                        settings::set_default_target_language(arg.to_string())?;
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

/// 翻訳  
/// 失敗したらエラーを返す
fn translate(auth_key: &String, text: String, target_lang: &String, source_lang: &String) -> core::result::Result<String, io::Error> {
    let url = "https://api-free.deepl.com/v2/translate".to_string();
    let query = if source_lang.trim_matches('"').is_empty() {
        format!("auth_key={}&text={}&target_lang={}", auth_key, text, target_lang)
    } else {
        format!("auth_key={}&text={}&target_lang={}&source_lang={}", auth_key, text, target_lang, source_lang)
    };
    
    connection::send_and_get(url, query)
}

/// 翻訳結果の表示  
/// json形式の翻訳結果を受け取り、翻訳結果を表示する  
/// jsonのパースに失敗したらエラーを返す
fn show_translated_text(json_str: &String) -> core::result::Result<(), io::Error> {
    let json: serde_json::Value = serde_json::from_str(json_str)?;
    json.get("translations").ok_or(io::Error::new(io::ErrorKind::Other, "Invalid response"))?;
    let translations = &json["translations"];
    for translation in translations.as_array().expect("failed to get array") {
        let len = translation["text"].to_string().len();
        let translation_trimmed= translation["text"].to_string()[1..len-1].to_string();
        println!("{}", translation_trimmed);
    }

    Ok(())
}

/// 対話と翻訳  
/// 対話モードであれば繰り返し入力を行う  
/// 通常モードであれば一回で終了する
fn process(mode: ExecutionMode, source_lang: String, target_lang: String, text: String, settings: &settings::Settings) -> core::result::Result<(), io::Error> {
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
        let translated_sentence = translate(&settings.api_key, input, &target_lang, &source_lang);
        match translated_sentence {
            Ok(s) => {
                // 翻訳結果が成功なら取得したJSONデータをパース
                show_translated_text(&s)?;
            }
            // 翻訳結果が失敗ならエラー表示
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

type LangCode = (String, String);
/// 言語コード一覧の取得  
/// <https://api-free.deepl.com/v2/languages>から取得する
fn get_language_codes(type_name: String) -> core::result::Result<Vec<LangCode>, io::Error> {
    let url = "https://api-free.deepl.com/v2/languages".to_string();
    let query = format!("type={}&auth_key={}", type_name, settings::get_settings().api_key);
    let res = connection::send_and_get(url, query)?;
    let v: Value = serde_json::from_str(&res)?;

    let mut lang_codes: Vec<LangCode> = Vec::new();
    for value in v.as_array().expect("Invalid response at get_language_codes") {
        value.get("language").ok_or(io::Error::new(io::ErrorKind::Other, "Invalid response"))?;
        let lang_code = (value["language"].to_string(), value["name"].to_string());
        lang_codes.push(lang_code);
    }

    Ok(lang_codes)
}
/// 翻訳元言語コード一覧の表示  
/// <https://api-free.deepl.com/v2/languages>から取得する
fn show_source_language_codes() -> core::result::Result<(), io::Error> {
    // 翻訳元言語コード一覧
    let source_lang_codes = get_language_codes("source".to_string())?;
    println!("Source language codes:");
    for lang_code in source_lang_codes {
        println!("{}: {}", lang_code.0.trim_matches('"'), lang_code.1.trim_matches('"'));
    }

    Ok(())
}
/// 翻訳先言語コード一覧の表示
fn show_target_language_codes() -> core::result::Result<(), io::Error> {
    // 翻訳先言語コード一覧
    let mut target_lang_codes = get_language_codes("target".to_string())?;

    // 特例コード変換
    target_lang_codes.push(("EN".to_string(), "English".to_string()));
    target_lang_codes.push(("PT".to_string(), "Portuguese".to_string()));

    println!("Target languages:");
    for lang_code in target_lang_codes {
        println!("{}: {}", lang_code.0.trim_matches('"'), lang_code.1.trim_matches('"'));
    }

    Ok(())
}

/// メイン関数
/// 引数の取得と翻訳処理の呼び出し
fn main() {
    // 設定の取得
    let settings = settings::get_settings();

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
