use std::{io, env};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use regex::Regex;

mod connection;

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

#[derive(Serialize, Deserialize)]
struct Settings {
    api_key: String,
    default_target_language: String
}

const SETTING_FILEPATH: &str = "settings.json";
fn get_settings() -> Settings {
    // ファイルが存在しない場合は新規作成＆初期化
    if Path::new(SETTING_FILEPATH).exists() == false {
        let mut f = File::create(SETTING_FILEPATH).expect("failed to create settings.json");
        let settings = Settings {
            api_key: String::new(),
            default_target_language: "JA".to_string()
        };
        let json_str = serde_json::to_string(&settings).expect("failed to serialize settings");
        f.write_all(json_str.as_bytes()).expect("failed to write settings.json");
    }

    // 設定ファイル読み込み
    let mut f = File::open(SETTING_FILEPATH).expect("settings.json has not been set");
    let mut s = String::new();
    f.read_to_string(&mut s).expect(format!("{} is empty", SETTING_FILEPATH).as_str());
    let v: Value = serde_json::from_str(&s).unwrap();

    // 各設定項目の取得
    Settings {
        api_key: v["api_key"].to_string().trim_matches('"').to_string(),
        default_target_language: v["default_target_language"].to_string().trim_matches('"').to_string()
    }
}

fn show_help() {
    println!("To translate with optional languages, usage: deepl [options] [sentence]");
    println!("Options:");
    println!("  -f, --from\t\t\tSet source language");
    println!("  -t, --to\t\t\tSet target language");
    println!("");
    println!("To setup setting options, usage: deepl -s [setting options]");
    println!("Setting options:");
    println!("  -s default-lang\t\tSetup default target language");
    println!("  -s api-key\t\t\tSetup your DeepL API key");
    println!("  -s clear\t\t\tClear all settings");
    println!("");
    println!("For other options, usage: deepl [options]");
    println!("Options:");
    println!("  -h, --help\t\tShow this help message");
    println!("  -v, --version\t\tShow version");
}

fn show_version() {
    println!("dptran {}", env!("CARGO_PKG_VERSION"));
}

fn get_remain() -> core::result::Result<(i32, i32), io::Error> {
    let url = "https://api-free.deepl.com/v2/usage".to_string();
    let d = format!("auth_key={}", get_settings().api_key);
    let res = connection::send_and_get(url, d)?;
    let v: Value = serde_json::from_str(&res)?;

    v.get("character_count").ok_or(io::Error::new(io::ErrorKind::Other, "failed to get character_count"))?;
    v.get("character_limit").ok_or(io::Error::new(io::ErrorKind::Other, "failed to get character_limit"))?;

    let character_count = v["character_count"].as_i64().expect("failed to get character_count") as i32;
    let character_limit = v["character_limit"].as_i64().expect("failed to get character_limit") as i32;
    Ok((character_count, character_limit))
}

fn set_apikey(api_key: String) -> core::result::Result<(), io::Error> {
    let mut settings = get_settings();
    settings.api_key = api_key;
    let json_str = serde_json::to_string(&settings)?;
    let mut f = File::create(SETTING_FILEPATH)?;
    f.write_all(json_str.as_bytes())?;

    Ok(())
}

fn set_default_target_language(default_target_language: String) -> core::result::Result<(), io::Error> {
    let mut settings = get_settings();
    settings.default_target_language = default_target_language;
    let json_str = serde_json::to_string(&settings)?;
    let mut f = File::create(SETTING_FILEPATH)?;
    f.write_all(json_str.as_bytes())?;

    Ok(())
}

fn get_args(args: Vec<String>, settings: &Settings) -> core::result::Result<(bool, ExecutionMode, String, String, String), io::Error> {
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
                let re = Regex::new(r"^-.+").unwrap();
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
                            "key" => {
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
                        set_apikey(arg.to_string())?;
                        return Ok((false, ExecutionMode::Normal, source_lang, target_lang, text));
                    }
                    // 既定の翻訳先言語の設定：言語コードを取得
                    ArgMode::SettingDefaultTagetLanguage => {
                        set_default_target_language(arg.to_string())?;
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
pub fn translate(auth_key: &String, text: String, target_lang: &String, source_lang: &String) -> core::result::Result<String, io::Error> {
    let url = "https://api-free.deepl.com/v2/translate".to_string();
    let d = if source_lang.trim_matches('"').is_empty() {
        format!("auth_key={}&text={}&target_lang={}", auth_key, text, target_lang)
    } else {
        format!("auth_key={}&text={}&target_lang={}&source_lang={}", auth_key, text, target_lang, source_lang)
    };
    
    connection::send_and_get(url, d)
}

fn show_translated_text(json_str: &String) -> core::result::Result<(), io::Error> {
    let json: serde_json::Value = serde_json::from_str(json_str)?;
    json.get("translations").ok_or(io::Error::new(io::ErrorKind::Other, "Invalid response"))?;
    let translations = &json["translations"];
    for translation in translations.as_array().unwrap() {
        let len = translation["text"].to_string().len();
        let translation_trimmed= translation["text"].to_string()[1..len-1].to_string();
        println!("{}", translation_trimmed);
    }

    Ok(())
}

fn process(mode: ExecutionMode, source_lang: String, target_lang: String, text: String, settings: &Settings) -> core::result::Result<(), io::Error> {
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

fn main() {
    // 設定の取得
    let settings = get_settings();

    // APIキーの確認
    if settings.api_key.is_empty() {
        println!("API key is not set. Please set it with the -s option:\n\t$ dptran -s key [YOUR_API_KEY]");
        return;
    }

    // 文字列を受け取る
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
