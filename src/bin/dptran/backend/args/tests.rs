use super::*;

#[test]
fn arg_parser_test() {
    let args = vec![
        "dptran",
        "-f", "EN",
        "-t", "FR",
        "--multilines",
        "--remove-line-breaks",
        "-o", "output.txt",
        "Hello, world!"
    ];
    let arg_struct = Args::parse_from(args);
    assert_eq!(arg_struct.from.unwrap(), "EN".to_string());
    assert_eq!(arg_struct.to.unwrap(), "FR".to_string());
    assert_eq!(arg_struct.multilines, true);
    assert_eq!(arg_struct.remove_line_breaks, true);
    assert_eq!(arg_struct.output_file.unwrap(), "output.txt".to_string());
    assert_eq!(arg_struct.source_text.unwrap().join(" "), "Hello, world!".to_string());
}

#[test]
fn arg_illegal_args_of_main_struct_test() {
    // --input-file and --editor
    let args = vec![
        "dptran",
        "--input-file",
        "input.txt",
        "--editor"
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_err());

    // --output-file and --editor
    let args = vec![
        "dptran",
        "--output-file",
        "output.txt",
        "--editor"
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_err());

    // --usage and other main options
    let args = vec![
        "dptran",
        "--usage",
        "--remove-line-breaks"
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_err());
}

#[test]
fn arg_legal_args_of_main_struct_test() {
    // --input-file
    let args = vec![
        "dptran",
        "--input-file",
        "input.txt",
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_ok());

    // --output-file
    let args = vec![
        "dptran",
        "--output-file",
        "output.txt",
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_ok());

    // --editor
    let args = vec![
        "dptran",
        "--editor",
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_ok());

    // --usage
    let args = vec![
        "dptran",
        "--usage",
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_ok());

    // ---from and --to
    let args = vec![
        "dptran",
        "--from",
        "EN",
        "--to",
        "FR"
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_ok());

    // --multilines
    let args = vec![
        "dptran",
        "--multilines",
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_ok());

    // --remove-line-breaks
    let args = vec![
        "dptran",
        "--remove-line-breaks",
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_ok());

    // --no-cache
    let args = vec![
        "dptran",
        "--no-cache",
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_ok());

    // combination of main options except --usage
    let args = vec![
        "dptran",
        "--from",
        "EN",
        "--to",
        "FR",
        "--multilines",
        "--remove-line-breaks",
        "--no-cache",
        "--output-file",
        "output.txt"
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_ok());
}

#[test]
fn arg_config_illegal_multi_options_test() {
    // --set-api-key and --show-config
    let args = vec![
        "dptran",
        "config",
        "--set-api-key",
        "test_api_key",
        "--show-config"
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_err());
    
    // --set-default-target-lang and --show-config
    let args = vec![
        "dptran",
        "config",
        "--set-default-target-lang",
        "FR",
        "--show-config"
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_err());
}

#[test]
fn arg_api_illegal_multi_options_test() {
    // --usage and --list-translations
    let args = vec![
        "dptran",
        "api",
        "--api-key-free",
        "ABCDEF",
        "--api-key-pro",
        "123456",
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_err());
}

#[test]
fn arg_cache_illegal_multi_options_test() {
    // --max-entries and --clear
    let args = vec![
        "dptran",
        "cache",
        "--max-entries",
        "100",
        "--clear"
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_err());
}

#[test]
fn arg_illegal_args_of_glossary_test() {
    // --name and --id
    let args = vec![
        "dptran",
        "glossary",
        "--name",
        "test_name",
        "--id",
        "test_id"
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_err());

    // only --name
    let args = vec![
        "dptran",
        "glossary",
        "--name",
        "test_name",
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_err());

    // only --id
    let args = vec![
        "dptran",
        "glossary",
        "--id",
        "test_id",
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_err());

    // only --source-lang
    let args = vec![
        "dptran",
        "glossary",
        "--source-lang",
        "EN",
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_err());

    // only --target-lang
    let args = vec![
        "dptran",
        "glossary",
        "--target-lang",
        "JA",
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_err());

    // --create without --name
    let args = vec![
        "dptran",
        "glossary",
        "--create",
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_err());

    // --remove without --name or --id
    let args = vec![
        "dptran",
        "glossary",
        "--remove",
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_err());

    // --add-word-pairs without --name or --id
    let args = vec![
        "dptran",
        "glossary",
        "--add-word-pairs",
        "TEST",
        "テスト",
        "--source-lang",
        "EN",
        "--target-lang",
        "JA"
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_err());

    // --add-word-pairs without --source-lang and --target-lang
    let args = vec![
        "dptran",
        "glossary",
        "--add-word-pairs",
        "TEST",
        "テスト",
        "--name",
        "test-name"
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_err());

    // --set-default-glossary without "--name" or "--id"
    let args = vec![
        "dptran",
        "glossary",
        "--set-default-glossary"
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_err());

    // --support-languages with --name
    let args = vec![
        "dptran",
        "glossary",
        "--support-languages",
        "--name",
        "test_name"
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_err());

    // --support-languages with --id
    let args = vec![
        "dptran",
        "glossary",
        "--support-languages",
        "--id",
        "test_id"
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_err());

    // --clear-default-glossary with --name
    let args = vec![
        "dptran",
        "glossary",
        "--clear-default-glossary",
        "--name",
        "test_name"
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_err());

    // --clear-default-glossary with --id
    let args = vec![
        "dptran",
        "glossary",
        "--clear-default-glossary",
        "--id",
        "test_id"
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_err());

    // --set-default-glossary with --source-lang
    let args = vec![
        "dptran",
        "glossary",
        "--set-default-glossary",
        "--source-lang",
        "EN"
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_err());

    // --set-default-glossary with --target-lang
    let args = vec![
        "dptran",
        "glossary",
        "--set-default-glossary",
        "--target-lang",
        "JA"
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_err());

    // --clear-default-glossary with --source-lang
    let args = vec![
        "dptran",
        "glossary",
        "--clear-default-glossary",
        "--source-lang",
        "EN"
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_err());

    // --clear-default-glossary with --target-lang
    let args = vec![
        "dptran",
        "glossary",
        "--clear-default-glossary",
        "--target-lang",
        "JA"
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_err());
}

#[test]
fn arg_legal_args_of_glossary_test() {
    // --name with --source-lang, --target-lang and --add-word-pairs
    let args = vec![
        "dptran",
        "glossary",
        "--name",
        "test_name",
        "--source-lang",
        "EN",
        "--target-lang",
        "JA",
        "--add-word-pairs",
        "TEST",
        "テスト"
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_ok());

    // --id with --source-lang, --target-lang and --add-word-pairs
    let args = vec![
        "dptran",
        "glossary",
        "--id",
        "test_id",
        "--source-lang",
        "EN",
        "--target-lang",
        "JA",
        "--add-word-pairs",
        "TEST",
        "テスト"
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_ok());

    // --create with --name
    let args = vec![
        "dptran",
        "glossary",
        "--create",
        "--name",
        "test_name"
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_ok());

    // --remove with --name
    let args = vec![
        "dptran",
        "glossary",
        "--remove",
        "--name",
        "test_name"
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_ok());

    // --remove with --id
    let args = vec![
        "dptran",
        "glossary",
        "--remove",
        "--id",
        "test_id"
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_ok());

    // --add-word-pairs with --name
    let args = vec![
        "dptran",
        "glossary",
        "--add-word-pairs",
        "TEST",
        "テスト",
        "--name",
        "test-name",
        "--source-lang",
        "EN",
        "--target-lang",
        "JA"
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_ok());

    // --add-word-pairs with --id
    let args = vec![
        "dptran",
        "glossary",
        "--add-word-pairs",
        "TEST",
        "テスト",
        "--id",
        "test-id",
        "--source-lang",
        "EN",
        "--target-lang",
        "JA"
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_ok());

    // --set-default-glossary with --name
    let args = vec![
        "dptran",
        "glossary",
        "--set-default-glossary",
        "--name",
        "test_name"
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_ok());

    // --set-default-glossary with --id
    let args = vec![
        "dptran",
        "glossary",
        "--set-default-glossary",
        "--id",
        "test_id"
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_ok());

    // --support-languages
    let args = vec![
        "dptran",
        "glossary",
        "--supported-languages",
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_ok());

    // --clear-default-glossary
    let args = vec![
        "dptran",
        "glossary",
        "--clear-default-glossary",
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_ok());
}
