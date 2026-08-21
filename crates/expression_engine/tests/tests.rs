//! Tests for expression engine.

// Integration tests favor clarity and brevity over the strictness we require of library
// code: panicking helpers (`unwrap`/`expect`/indexing/`panic!`) and approximate float
// comparisons are idiomatic and expected in tests.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::float_cmp,
    clippy::as_conversions,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::unreadable_literal,
    clippy::unnecessary_wraps,
    clippy::similar_names,
    clippy::arithmetic_side_effects,
    clippy::wildcard_enum_match_arm
)]

mod evaluation;
mod input;

#[test]
fn error_message_translations_support_the_agreed_languages() {
    let translations =
        expression_engine::evaluation::expression::translations::get_error_message_translations();
    let key = "expression_engine_lexer_invalid_character";

    for (language, expected) in [
        ("en", "Invalid character in expression: '%{character}'"),
        ("zh", "表达式中存在无效字符：'%{character}'"),
        ("de", "Ungültiges Zeichen im Ausdruck: '%{character}'"),
        ("es", "Carácter no válido en la expresión: '%{character}'"),
        (
            "fr",
            "Caractère non valide dans l’expression : '%{character}'",
        ),
        ("ja", "式に無効な文字があります: '%{character}'"),
        ("ko", "식에 잘못된 문자가 있습니다: '%{character}'"),
    ] {
        let message = translations
            .get_translation(key, language, None)
            .expect("the lexer invalid-character message should be translated");

        assert_eq!(
            message.as_str(),
            expected,
            "unexpected translation for {language}"
        );
    }

    let fallback = translations
        .get_translation(key, "it", None)
        .expect("the fallback translation should exist");
    assert_eq!(
        fallback.as_str(),
        "Invalid character in expression: '%{character}'"
    );

    for key in [
        "expression_engine_lexer_invalid_character",
        "expression_engine_lexer_invalid_number",
        "expression_engine_lexer_invalid_operator",
        "expression_engine_lexer_invalid_string",
        "expression_engine_lexer_unterminated_string_literal",
    ] {
        let english = translations
            .get_translation(key, "en", None)
            .expect("the English translation should exist");

        for language in ["zh", "de", "es", "fr", "ja", "ko"] {
            let message = translations
                .get_translation(key, language, None)
                .expect("each supported language should have a translation");

            assert_ne!(
                message.as_str(),
                english.as_str(),
                "{key} should not fall back to English for {language}"
            );
        }
    }
}
