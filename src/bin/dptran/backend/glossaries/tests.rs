use super::*;

#[test]
fn impl_vec_string_to_word_pairs_even_test() {
    let vec = vec![
        "hello".to_string(),
        "こんにちは".to_string(),
        "world".to_string(),
        "世界".to_string(),
    ];
    let result = vec_string_to_word_pairs(&vec).unwrap();
    assert_eq!(result, vec![
        ("hello".to_string(), "こんにちは".to_string()),
        ("world".to_string(), "世界".to_string()),
    ]);
}

#[test]
fn impl_vec_string_to_word_pairs_odd_test() {
    let vec = vec![
        "hello".to_string(),
        "こんにちは".to_string(),
        "world".to_string(),
    ];
    let result = vec_string_to_word_pairs(&vec);
    assert_eq!(result.unwrap_err(), GlossariesError::WordPairMustBeEven);
}
