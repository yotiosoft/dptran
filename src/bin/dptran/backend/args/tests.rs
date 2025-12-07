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
fn conflict_args_of_glossary_test() {
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
        "--name",
        "test_name",
    ];
    let result = Args::try_parse_from(args);
    assert!(result.is_err());
}
