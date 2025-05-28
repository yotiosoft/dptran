mod func_tests {
    use super::*;

    fn retry_or_panic(e: &RuntimeError, times: u8) -> bool {
        if e == &RuntimeError::DeeplApiError(DpTranError::DeeplApiError(dptran::DeeplAPIError::ConnectionError(dptran::ConnectionError::TooManyRequests))) && times < 3 {
            // Because the DeepL API has a limit on the number of requests per second, retry after 2 seconds if the error is TooManyRequests.
            std::thread::sleep(std::time::Duration::from_secs(2));
            return true;
        }
        else {
            panic!("Error: {}", e.to_string());
        }
    }

    fn impl_app_show_source_language_codes_test(times: u8) {
        let result = show_source_language_codes();
        if let Err(e) = &result {
            if retry_or_panic(e, 1) {
                return impl_app_show_source_language_codes_test(times + 1);
            }
        }
        assert!(result.is_ok());
    }

    fn impl_app_show_target_language_codes_test(times: u8) {
        let result = show_target_language_codes();
        if let Err(e) = &result {
            if retry_or_panic(e, 1) {
                return impl_app_show_target_language_codes_test(times + 1);
            }
        }
        assert!(result.is_ok());
    }

    fn impl_app_show_usage_test(times: u8) {
        let result = show_usage();
        if let Err(e) = &result {
            if retry_or_panic(e, 1) {
                return impl_app_show_usage_test(times + 1);
            }
        }
        assert!(result.is_ok());
    }

    fn impl_app_process_test(times: u8) {
        let api_key = backend::get_api_key().unwrap().unwrap();
        let dptran = dptran::DpTran::with(&api_key.api_key, api_key.api_key_type);
        let mode = ExecutionMode::TranslateNormal;
        let multilines = false;
        let rm_line_breaks = false;
        let text = Some("Hello, world!".to_string());
        let source_lang = Some("en".to_string());
        let target_lang = "fr".to_string();
        let ofile = None;

        let result = process(&dptran, mode, source_lang, target_lang, multilines, rm_line_breaks, text, ofile);
        if let Err(e) = &result {
            if retry_or_panic(e, 1) {
                return impl_app_process_test(times + 1);
            }
        }
        assert!(result.is_ok());
    }

    #[test]
    fn app_get_langcodes_maxlen_test() {
        let lang_codes = vec![
            ("en".to_string(), "English".to_string()),
            ("fr".to_string(), "French".to_string()),
            ("de".to_string(), "German".to_string()),
        ];
        let (len, max_code_len, max_str_len) = get_langcodes_maxlen(&lang_codes);
        assert_eq!(len, 3);
        assert_eq!(max_code_len, 2);
        assert_eq!(max_str_len, 7);
    }

    #[test]
    fn app_display_settings_test() {
        let result = display_settings();
        assert!(result.is_ok());
    }

    #[test]
    fn app_show_source_language_codes_test() {
        impl_app_show_source_language_codes_test(0);
    }

    #[test]
    fn app_show_target_language_codes_test() {
        impl_app_show_target_language_codes_test(0);
    }

    #[test]
    fn app_show_usage_test() {
        impl_app_show_usage_test(0);
    }

    #[test]
    fn app_get_input_test() {
        let mode = ExecutionMode::TranslateNormal;
        let multilines = false;
        let rm_line_breaks = false;
        let text = "Hello, world!".to_string();

        let result = get_input(&mode, multilines, rm_line_breaks, &Some(text));
        assert!(result.is_some());
    }

    #[test]
    fn app_process_test() {
        impl_app_process_test(0);
    }
}

mod runtime_tests {
    use std::{io::Write, process::Command, process::Stdio};

    #[test]
    fn runtime_test() {
        let mut cmd = Command::new("cargo");
        std::thread::sleep(std::time::Duration::from_secs(2));
        let text = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("Hello, world!")
            .arg("-t")
            .arg("ja")
            .output();

        assert!(text.is_ok());
        let text = text.unwrap();
        if text.status.success() != true {
            panic!("Error: {}", String::from_utf8_lossy(&text.stderr));
        }
        assert!(text.stdout != b"Hello\n");

        let mut cmd = Command::new("cargo");
        std::thread::sleep(std::time::Duration::from_secs(2));
        let text = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("Hello")
            .arg("-t")
            .arg("en")
            .output();

        assert!(text.is_ok());
        let text = text.unwrap();
        if text.status.success() != true {
            panic!("Error: {}", String::from_utf8_lossy(&text.stderr));
        }
        assert!(text.stdout == b"Hello\n");
    }

    #[test]
    fn runtime_with_file_test() {
        let mut cmd = Command::new("cargo");
        std::thread::sleep(std::time::Duration::from_secs(2));
        let text = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("-t")
            .arg("en")
            .arg("Hello")
            .arg("-o")
            .arg("test.txt")
            .output();

        assert!(text.is_ok());
        let text = text.unwrap();
        if text.status.success() != true {
            panic!("Error: {}", String::from_utf8_lossy(&text.stderr));
        }

        // Check if the file exists
        let file_path = std::path::Path::new("test.txt");
        assert!(file_path.exists(), "File test.txt does not exist.");
        // Check if the file is not empty
        let metadata = std::fs::metadata(file_path).unwrap();
        assert!(metadata.len() > 0, "File test.txt is empty.");
        let file_content = std::fs::read_to_string(file_path).unwrap();
        assert!(file_content.contains("Hello"), "File test.txt does not contain the expected content.");
        // Clean up the file
        std::fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn runtime_with_cache_test() {
        // 1st run..
        let mut cmd = Command::new("cargo");
        std::thread::sleep(std::time::Duration::from_secs(2));
        let text = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("Hello, world!")
            .arg("-t")
            .arg("ja")
            .output();
        let text = text.unwrap();
        if text.status.success() != true {
            panic!("Error: {}", String::from_utf8_lossy(&text.stderr));
        }

        // Get usage..
        let mut cmd = Command::new("cargo");
        std::thread::sleep(std::time::Duration::from_secs(2));
        let usage = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("-u")
            .output();
        let usage = usage.unwrap();
        if usage.status.success() != true {
            panic!("Error: {}", String::from_utf8_lossy(&usage.stderr));
        }
        // to u32
        let usage_before = usage.stdout
            .split(|&b| b == b' ')
            .filter_map(|s| std::str::from_utf8(s).ok())
            .filter_map(|s| s.parse::<u32>().ok())
            .next()
            .unwrap();

        // 2nd run.
        std::thread::sleep(std::time::Duration::from_secs(2));
        let mut cmd = Command::new("cargo");
        let text = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("Hello, world!")
            .arg("-t")
            .arg("ja")
            .output();
        let text = text.unwrap();
        if text.status.success() != true {
            panic!("Error: {}", String::from_utf8_lossy(&text.stderr));
        }

        // Get usage once again.
        std::thread::sleep(std::time::Duration::from_secs(2));
        let mut cmd = Command::new("cargo");
        let usage = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("-u")
            .output();
        let usage = usage.unwrap();
        if usage.status.success() != true {
            panic!("Error: {}", String::from_utf8_lossy(&usage.stderr));
        }

        // to u32
        let usage_after = usage.stdout
            .split(|&b| b == b' ')
            .filter_map(|s| std::str::from_utf8(s).ok())
            .filter_map(|s| s.parse::<u32>().ok())
            .next()
            .unwrap();

        // Check if the usage has not changed.
        assert!(usage_after == usage_before);
    }

    /// Test for the interactive mode.
    #[test]
    fn runtime_interactive_mode_test() {
        let mut cmd = Command::new("cargo");
        std::thread::sleep(std::time::Duration::from_secs(2));
        let text = cmd.arg("run")
            .arg("--release")
            .arg("--")
            .arg("-t")
            .arg("en")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn();

        let mut child = text.unwrap();
        std::thread::sleep(std::time::Duration::from_secs(2));
        let input = "Hello, world!\nquit\n";
        let output = child.stdin.as_mut().unwrap().write_all(input.as_bytes());
        if let Err(e) = output {
            panic!("Error: {}", e);
        }
        let output = child.wait_with_output();
        if let Err(e) = output {
            panic!("Error: {}", e);
        }
        let output = output.unwrap();
        if output.status.success() != true {
            panic!("Error: {}", String::from_utf8_lossy(&output.stderr));
        }
        // Check if the output contains "Hello, world!"
        let output_str = String::from_utf8_lossy(&output.stdout);
        assert!(output_str.contains("Hello, world!"), "Output does not contain the expected text.");
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn runtime_from_pipe_test() {
        let mut echo_cmd = Command::new("echo")
            .arg("Hello, world!")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start echo command");

        let dptran_cmd = Command::new("cargo")
            .arg("run")
            .arg("--release")
            .arg("--")
            .arg("-t")
            .arg("en")
            .stdin(Stdio::from(echo_cmd.stdout.take().unwrap()))
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start dptran command");
        
        let output = dptran_cmd.wait_with_output().expect("Failed to read dptran output");
        if output.status.success() != true {
            panic!("Error: {}", String::from_utf8_lossy(&output.stderr));
        }
        // Check if the output contains "Hello, world!"
        let output_str = String::from_utf8_lossy(&output.stdout);
        assert!(output_str.contains("Hello, world!"), "Output does not contain the expected text.");
    }
}
