use std::{io, env};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::exit;
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

fn get_args(args: Vec<String>, settings: &Settings) -> (ExecutionMode, String, String, String) {
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
                println!("Usage: dptran [options]");
                println!("Options:");
                println!("  -i, --interactive\t\tInteractive mode");
                println!("  -h, --help\t\tShow this help message");
                println!("  -v, --version\t\tShow version");
                exit(0);
            }
            // バージョン情報
            "-v" | "--version" => {
                println!("dptran {}", env!("CARGO_PKG_VERSION"));
                exit(0);
            }
            // 残り翻訳可能文字数
            "-r" | "--remain" => {
                println!("remain");
                exit(0);
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
                        println!("translate from: {}", arg);
                        source_lang = arg.to_string();
                        arg_mode = ArgMode::Sentence;
                    }
                    // 翻訳元言語指定
                    ArgMode::TargetLanguage => {
                        println!("translate to: {}", arg);
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

    (mode, source_lang, target_lang, text)
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
    let (mode, source_lang, target_lang, text) = get_args(args, &settings);

    println!("sentence: {}", text);

    // 翻訳
    // 対話モードならループする; 通常モードでは1回で抜ける
    loop {
        // 対話モードなら標準入力から取得
        // 通常モードでは引数から取得
        let input = match mode {
            ExecutionMode::Interactive => {
                let mut input = String::new();
                let bytes = io::stdin().read_line(&mut input).expect("Failed to read line.");
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

        if mode == ExecutionMode::Normal {
            break;
        }
    }
}
