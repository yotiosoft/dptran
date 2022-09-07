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

    // 引数を解析
    let mut arg_mode: ArgMode = ArgMode::Sentence;
    for arg in &args[1..] {
        match arg.as_str() {
            "-h" | "--help" => {
                println!("Usage: dptran [options]");
                println!("Options:");
                println!("  -h, --help\t\tShow this help message");
                println!("  -v, --version\t\tShow version");
                return;
            }
            "-v" | "--version" => {
                println!("dptran {}", env!("CARGO_PKG_VERSION"));
                return;
            }
            "-r" | "--remain" => {
                println!("remain");
                return;
            }
            "-s" | "--set" => {
                arg_mode = ArgMode::Settings;
            }
            _ => {
                match arg_mode {
                    ArgMode::Sentence => {
                        println!("sentence: {}", arg);
                    }
                    ArgMode::TranslateTo => {
                        println!("translate to: {}", arg);
                    }
                    ArgMode::TranslateFrom => {
                        println!("translate from: {}", arg);
                    }
                    ArgMode::Settings => {
                        match arg.as_str() {
                            "key" => {
                                arg_mode = ArgMode::SettingAPIKey;
                            }
                            "default-lang" => {
                                arg_mode = ArgMode::SettingDefaultTranslateLanguage;
                            }
                            "clear" => {
                                break;
                            }
                            _ => {
                                println!("Unknown settings: {}", arg);
                                break;
                            }
                        }
                    }
                    ArgMode::SettingAPIKey => {
                        println!("api key: {}", arg);
                        break;
                    }
                    ArgMode::SettingDefaultTranslateLanguage => {
                        println!("default language to: {}", arg);
                        break;
                    }
                    _ => {
                        panic!("Unknown arg: {}", arg);
                    }
                }
            }
        }
    }
}
