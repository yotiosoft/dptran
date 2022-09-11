use std::{io, env};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
mod translate;

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
    Translate,
    Interactive
}

#[derive(Serialize, Deserialize)]
struct Settings {
    api_key: String,
    default_target_language: String
}

fn get_api() -> String {
    let mut api_key = String::new();
    let mut f = File::open("api_key.txt").expect("api key has not been set");
    f.read_to_string(&mut api_key).expect("api_key.txt is empty");
    api_key
}

const SETTING_FILEPATH: &str = "settings.json";
fn get_settings() -> Settings {
    // ファイルが存在しない場合は新規作成＆初期化
    if Path::new(SETTING_FILEPATH).exists() == false {
        let mut f = File::create(SETTING_FILEPATH).expect("failed to create settings.json");
        let settings = Settings {
            api_key: String::new(),
            default_target_language: String::new()
        };
        let json_str = serde_json::to_string(&settings).expect("failed to serialize settings");
        f.write_all(json_str.as_bytes()).expect("failed to write settings.json");
    }

    // 設定ファイル読み込み
    let mut api_key = String::new();
    let mut default_target_language = String::new();
    let mut f = File::open(SETTING_FILEPATH).expect("settings.json has not been set");
    let mut s = String::new();
    f.read_to_string(&mut s).expect(format!("{} is empty", SETTING_FILEPATH).as_str());
    let v: Value = serde_json::from_str(&s).unwrap();

    // 各設定項目の取得
    api_key = v["api_key"].to_string();
    default_target_language = v["default_target_language"].to_string();

    Settings {
        api_key: api_key,
        default_target_language: default_target_language
    }
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

    // 原文
    let mut text = String::new();

    // 引数を解析
    let mut arg_mode: ArgMode = ArgMode::Sentence;
    for arg in &args[1..] {
        match arg.as_str() {
            // オプションの抽出
            // ヘルプ
            "-h" | "--help" => {
                println!("Usage: dptran [options]");
                println!("Options:");
                println!("  -h, --help\t\tShow this help message");
                println!("  -v, --version\t\tShow version");
                return;
            }
            // バージョン情報
            "-v" | "--version" => {
                println!("dptran {}", env!("CARGO_PKG_VERSION"));
                return;
            }
            // 残り翻訳可能文字数
            "-r" | "--remain" => {
                println!("remain");
                return;
            }
            // 設定（次の引数を参照）
            "-s" | "--set" => {
                arg_mode = ArgMode::Settings;
            }
            // 翻訳先言語指定
            "-t" | "--to" => {
                arg_mode = ArgMode::SourceLanguage;
            }
            // 翻訳元言語指定
            "-f" | "--from" => {
                arg_mode = ArgMode::TargetLanguage;
            }
            // それ以外
            _ => {
                match arg_mode {
                    // 入力原文
                    ArgMode::Sentence => {
                        if text.len() > 0 {
                            text.push(' ');
                        }
                        text += arg;
                    }
                    // 翻訳先言語指定
                    ArgMode::SourceLanguage => {
                        println!("translate to: {}", arg);
                        arg_mode = ArgMode::Sentence;
                    }
                    // 翻訳元言語指定
                    ArgMode::TargetLanguage => {
                        println!("translate from: {}", arg);
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
                                println!("Unknown settings: {}", arg);
                                break;
                            }
                        }
                    }
                    // APIキーの設定：APIキー値を取得
                    ArgMode::SettingAPIKey => {
                        println!("api key: {}", arg);
                        break;
                    }
                    // 既定の翻訳先言語の設定：言語コードを取得
                    ArgMode::SettingDefaultTagetLanguage => {
                        println!("default language to: {}", arg);
                        break;
                    }
                }
            }
        }
    }

    println!("sentence: {}", text);

    // 原文が0文字なら対話モードへ
    let mode = if text.len() == 0 {
        ExecutionMode::Interactive
    } else {
        ExecutionMode::Translate
    };

    loop {
        let input = match mode {
            ExecutionMode::Interactive => {
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line.");
                input
            }
            ExecutionMode::Translate => {
                text.clone()
            }
        };

        if mode == ExecutionMode::Interactive && input.clone().trim_end() == "exit" {
            break;
        }

        let target_lang = "JA".to_string();
        let source_lang = "EN".to_string();
        let translated_sentence = translate::translate(&settings.api_key, input, &target_lang, &source_lang);
        match translated_sentence {
            Ok(s) => {
                println!("translated: {}", s);
                let j_translate: Result<Value> = serde_json::from_str(&s);
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
                println!("send text error: {}", e);
            }
        }

        if mode == ExecutionMode::Translate {
            break;
        }
    }
}
