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

fn main() {
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

    let auth_key = "1c664a9f-4696-d92d-1caa-b4a3634ec562:fx".to_string();
    let translated_sentence = translate::translate(auth_key, text, "JA".to_string(), "EN".to_string());
    println!("translated sentence: {}", translated_sentence);

    let j_translate: Value = serde_json::from_str(&translated_sentence).unwrap();
    println!("translated sentence: {}", j_translate["translations"][0]["text"]);
}
