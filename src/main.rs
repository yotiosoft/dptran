enum ArgMode {
    Sentence,
    TranslateTo,
    TranslateFrom,
    Settings,
    SettingAPIKey,
    SettingDefaultTranslateLanguage
}

fn main() {
    // 文字列を受け取る
    let args: Vec<String> = std::env::args().collect();

    // 原文
    let mut sentence = String::new();

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
                arg_mode = ArgMode::TranslateTo;
            }
            // 翻訳元言語指定
            "-f" | "--from" => {
                arg_mode = ArgMode::TranslateFrom;
            }
            // それ以外
            _ => {
                match arg_mode {
                    // 入力原文
                    ArgMode::Sentence => {
                        if sentence.len() > 0 {
                            sentence.push(' ');
                        }
                        sentence += arg;
                    }
                    // 翻訳先言語指定
                    ArgMode::TranslateTo => {
                        println!("translate to: {}", arg);
                        arg_mode = ArgMode::Sentence;
                    }
                    // 翻訳元言語指定
                    ArgMode::TranslateFrom => {
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
                                arg_mode = ArgMode::SettingDefaultTranslateLanguage;
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
                    ArgMode::SettingDefaultTranslateLanguage => {
                        println!("default language to: {}", arg);
                        break;
                    }
                }
            }
        }
    }

    println!("sentence: {}", sentence);
}
