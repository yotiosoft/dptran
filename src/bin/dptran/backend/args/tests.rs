use super::*;

#[test]
fn parser_test() {
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
fn illegal_args_of_glossary_test() {
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
fn legal_args_of_glossary_test() {
    // --name with --source-lang and --target-lang
    let args = vec![
        "dptran",
        "glossary",
        "--name",
        "test_name",
        "--source-lang",
        "EN",
        "--target-lang",
        "JA"
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_ok());

    // --id with --source-lang and --target-lang
    let args = vec![
        "dptran",
        "glossary",
        "--id",
        "test_id",
        "--source-lang",
        "EN",
        "--target-lang",
        "JA"
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
        "--support-languages",
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
