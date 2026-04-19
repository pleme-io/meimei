//! Meimei (命名) — zero-dependency case convention converters for code generation.
//!
//! Provides pure string transformations between naming conventions commonly used
//! in code generation: `snake_case`, `PascalCase`, `camelCase`, `kebab-case`,
//! `SCREAMING_SNAKE_CASE`, and provider prefix stripping.
//!
//! Platform-specific modules ([`go`], [`rust`], [`python`], [`ruby`]) re-export
//! the appropriate converter under idiomatic names for each language.

mod convert;
mod convention;
mod style;

pub use convert::{
    pascal_to_kebab_case, pascal_to_snake_case, split_words, split_words_iter,
    strip_provider_prefix, to_camel_case, to_kebab_case, to_pascal_case, to_screaming_snake_case,
    to_snake_case,
};
pub use convention::{
    GoConvention, NamingConvention, PythonConvention, RubyConvention, RustConvention,
};
pub use style::{CaseStyle, ParseCaseStyleError};

// ---------------------------------------------------------------------------
// Platform-specific naming modules
// ---------------------------------------------------------------------------

/// Go naming conventions.
pub mod go {
    /// Convert to a Go exported (public) identifier — `PascalCase`.
    #[must_use]
    pub fn to_public(name: &str) -> String {
        crate::to_pascal_case(name)
    }

    /// Produce a Go struct field JSON tag value — `snake_case`.
    #[must_use]
    pub fn to_field_tag(name: &str) -> String {
        crate::to_snake_case(name)
    }
}

/// Rust naming conventions.
pub mod rust {
    /// Convert to a Rust type name — `PascalCase`.
    #[must_use]
    pub fn to_type(name: &str) -> String {
        crate::to_pascal_case(name)
    }

    /// Convert to a Rust field / variable name — `snake_case`.
    #[must_use]
    pub fn to_field(name: &str) -> String {
        crate::to_snake_case(name)
    }
}

/// Python naming conventions.
pub mod python {
    /// Convert to a Python class name — `PascalCase`.
    #[must_use]
    pub fn to_class(name: &str) -> String {
        crate::to_pascal_case(name)
    }

    /// Convert to a Python variable / function name — `snake_case`.
    #[must_use]
    pub fn to_var(name: &str) -> String {
        crate::to_snake_case(name)
    }
}

/// Ruby naming conventions.
pub mod ruby {
    /// Convert to a Ruby class / module name — `PascalCase`.
    #[must_use]
    pub fn to_class(name: &str) -> String {
        crate::to_pascal_case(name)
    }

    /// Convert to a Ruby method / local variable name — `snake_case`.
    #[must_use]
    pub fn to_method(name: &str) -> String {
        crate::to_snake_case(name)
    }
}

// ---------------------------------------------------------------------------
// Platform module tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ── Go module ─────────────────────────────────────────────────────

    #[test]
    fn go_to_public() {
        assert_eq!(go::to_public("bound-aws-account-id"), "BoundAwsAccountId");
        assert_eq!(go::to_public("access_expires"), "AccessExpires");
        assert_eq!(go::to_public("name"), "Name");
    }

    #[test]
    fn go_to_field_tag() {
        assert_eq!(
            go::to_field_tag("bound-aws-account-id"),
            "bound_aws_account_id"
        );
        assert_eq!(go::to_field_tag("access_expires"), "access_expires");
    }

    // ── Rust module ───────────────────────────────────────────────────

    #[test]
    fn rust_to_type() {
        assert_eq!(rust::to_type("bound-aws-account-id"), "BoundAwsAccountId");
        assert_eq!(rust::to_type("static_secret"), "StaticSecret");
    }

    #[test]
    fn rust_to_field() {
        assert_eq!(
            rust::to_field("bound-aws-account-id"),
            "bound_aws_account_id"
        );
        assert_eq!(rust::to_field("access_expires"), "access_expires");
    }

    // ── Python module ─────────────────────────────────────────────────

    #[test]
    fn python_to_class() {
        assert_eq!(
            python::to_class("bound-aws-account-id"),
            "BoundAwsAccountId"
        );
        assert_eq!(python::to_class("static_secret"), "StaticSecret");
    }

    #[test]
    fn python_to_var() {
        assert_eq!(
            python::to_var("bound-aws-account-id"),
            "bound_aws_account_id"
        );
        assert_eq!(python::to_var("access_expires"), "access_expires");
    }

    // ── Ruby module ───────────────────────────────────────────────────

    #[test]
    fn ruby_to_class() {
        assert_eq!(
            ruby::to_class("bound-aws-account-id"),
            "BoundAwsAccountId"
        );
        assert_eq!(ruby::to_class("static_secret"), "StaticSecret");
    }

    #[test]
    fn ruby_to_method() {
        assert_eq!(
            ruby::to_method("bound-aws-account-id"),
            "bound_aws_account_id"
        );
        assert_eq!(ruby::to_method("access_expires"), "access_expires");
    }

    // ── Platform module stress ──────────────────────────────────

    #[test]
    fn go_to_public_numbers() {
        assert_eq!(go::to_public("get-v2-items"), "GetV2Items");
    }

    #[test]
    fn go_field_tag_special_chars() {
        assert_eq!(
            go::to_field_tag("bound-aws-account-id"),
            "bound_aws_account_id"
        );
    }

    #[test]
    fn rust_to_type_edge() {
        assert_eq!(rust::to_type(""), "");
        assert_eq!(rust::to_type("x"), "X");
    }

    #[test]
    fn ruby_to_class_complex() {
        assert_eq!(ruby::to_class("static-secret-value"), "StaticSecretValue");
    }

    // ── Additional platform module edge cases ─────────────────────

    #[test]
    fn go_to_public_empty() {
        assert_eq!(go::to_public(""), "");
    }

    #[test]
    fn go_to_field_tag_empty() {
        assert_eq!(go::to_field_tag(""), "");
    }

    #[test]
    fn rust_to_field_edge() {
        assert_eq!(rust::to_field(""), "");
        assert_eq!(rust::to_field("x"), "x");
    }

    #[test]
    fn python_to_class_edge() {
        assert_eq!(python::to_class(""), "");
        assert_eq!(python::to_class("x"), "X");
    }

    #[test]
    fn python_to_var_edge() {
        assert_eq!(python::to_var(""), "");
        assert_eq!(python::to_var("x"), "x");
    }

    #[test]
    fn ruby_to_class_edge() {
        assert_eq!(ruby::to_class(""), "");
        assert_eq!(ruby::to_class("x"), "X");
    }

    #[test]
    fn ruby_to_method_edge() {
        assert_eq!(ruby::to_method(""), "");
        assert_eq!(ruby::to_method("x"), "x");
    }

    // ══════════════════════════════════════════════════════════════════
    // Core converter tests (public API)
    // ══════════════════════════════════════════════════════════════════

    // ── snake_case ────────────────────────────────────────────────────

    #[test]
    fn snake_from_kebab() {
        assert_eq!(to_snake_case("kebab-case"), "kebab_case");
    }

    #[test]
    fn snake_already_snake() {
        assert_eq!(to_snake_case("already_snake"), "already_snake");
    }

    #[test]
    fn snake_empty() {
        assert_eq!(to_snake_case(""), "");
    }

    #[test]
    fn snake_single_char() {
        assert_eq!(to_snake_case("x"), "x");
    }

    #[test]
    fn snake_no_delimiters() {
        // snake_case only replaces hyphens; no uppercase splitting
        assert_eq!(to_snake_case("camelCase"), "camelCase");
    }

    #[test]
    fn snake_preserves_uppercase() {
        // This is by design: snake_case is a simple hyphen→underscore replacement
        assert_eq!(to_snake_case("PascalCase"), "PascalCase");
    }

    #[test]
    fn snake_numbers() {
        assert_eq!(to_snake_case("v2-api-endpoint"), "v2_api_endpoint");
    }

    #[test]
    fn snake_consecutive_hyphens() {
        assert_eq!(to_snake_case("a--b"), "a__b");
    }

    #[test]
    fn snake_leading_hyphen() {
        assert_eq!(to_snake_case("-leading"), "_leading");
    }

    #[test]
    fn snake_trailing_hyphen() {
        assert_eq!(to_snake_case("trailing-"), "trailing_");
    }

    // ── PascalCase ───────────────────────────────────────────────────

    #[test]
    fn pascal_from_snake() {
        assert_eq!(to_pascal_case("snake_case"), "SnakeCase");
    }

    #[test]
    fn pascal_from_kebab() {
        assert_eq!(to_pascal_case("kebab-case"), "KebabCase");
    }

    #[test]
    fn pascal_from_mixed_delimiters() {
        assert_eq!(to_pascal_case("foo-bar_baz"), "FooBarBaz");
    }

    #[test]
    fn pascal_single_word() {
        assert_eq!(to_pascal_case("hello"), "Hello");
    }

    #[test]
    fn pascal_empty() {
        assert_eq!(to_pascal_case(""), "");
    }

    #[test]
    fn pascal_single_char() {
        assert_eq!(to_pascal_case("a"), "A");
    }

    #[test]
    fn pascal_already_pascal() {
        // No delimiters, so treated as single word; first char capitalized
        assert_eq!(to_pascal_case("AlreadyPascal"), "AlreadyPascal");
    }

    #[test]
    fn pascal_with_numbers() {
        assert_eq!(to_pascal_case("get-v2-items"), "GetV2Items");
    }

    #[test]
    fn pascal_consecutive_delimiters() {
        assert_eq!(to_pascal_case("a--b__c"), "ABC");
    }

    #[test]
    fn pascal_leading_delimiter() {
        assert_eq!(to_pascal_case("_leading"), "Leading");
    }

    #[test]
    fn pascal_trailing_delimiter() {
        assert_eq!(to_pascal_case("trailing_"), "Trailing");
    }

    // ── camelCase ────────────────────────────────────────────────────

    #[test]
    fn camel_from_snake() {
        assert_eq!(to_camel_case("snake_case"), "snakeCase");
    }

    #[test]
    fn camel_from_kebab() {
        assert_eq!(to_camel_case("hello-world"), "helloWorld");
    }

    #[test]
    fn camel_single_word() {
        assert_eq!(to_camel_case("name"), "name");
    }

    #[test]
    fn camel_empty() {
        assert_eq!(to_camel_case(""), "");
    }

    #[test]
    fn camel_single_char_upper() {
        assert_eq!(to_camel_case("A"), "a");
    }

    #[test]
    fn camel_single_char_lower() {
        assert_eq!(to_camel_case("x"), "x");
    }

    #[test]
    fn camel_already_camel() {
        // No delimiters, first char lowercased
        assert_eq!(to_camel_case("alreadyCamel"), "alreadyCamel");
    }

    #[test]
    fn camel_with_numbers() {
        assert_eq!(to_camel_case("get-v2-items"), "getV2Items");
    }

    #[test]
    fn camel_consecutive_delimiters() {
        assert_eq!(to_camel_case("a--b__c"), "aBC");
    }

    #[test]
    fn camel_leading_delimiter() {
        assert_eq!(to_camel_case("_leading"), "leading");
    }

    // ── kebab-case ───────────────────────────────────────────────────

    #[test]
    fn kebab_from_snake() {
        assert_eq!(to_kebab_case("snake_case"), "snake-case");
    }

    #[test]
    fn kebab_already_kebab() {
        assert_eq!(to_kebab_case("already-kebab"), "already-kebab");
    }

    #[test]
    fn kebab_empty() {
        assert_eq!(to_kebab_case(""), "");
    }

    #[test]
    fn kebab_single_char() {
        assert_eq!(to_kebab_case("x"), "x");
    }

    #[test]
    fn kebab_preserves_uppercase() {
        // kebab-case is a simple underscore→hyphen replacement
        assert_eq!(to_kebab_case("PascalCase"), "PascalCase");
    }

    #[test]
    fn kebab_with_numbers() {
        assert_eq!(to_kebab_case("v2_api_endpoint"), "v2-api-endpoint");
    }

    #[test]
    fn kebab_consecutive_underscores() {
        assert_eq!(to_kebab_case("a__b"), "a--b");
    }

    #[test]
    fn kebab_leading_underscore() {
        assert_eq!(to_kebab_case("_leading"), "-leading");
    }

    // ── SCREAMING_SNAKE_CASE ─────────────────────────────────────────

    #[test]
    fn screaming_from_snake() {
        assert_eq!(to_screaming_snake_case("snake_case"), "SNAKE_CASE");
    }

    #[test]
    fn screaming_from_kebab() {
        assert_eq!(
            to_screaming_snake_case("bound-aws-account-id"),
            "BOUND_AWS_ACCOUNT_ID"
        );
    }

    #[test]
    fn screaming_empty() {
        assert_eq!(to_screaming_snake_case(""), "");
    }

    #[test]
    fn screaming_single_char() {
        assert_eq!(to_screaming_snake_case("a"), "A");
    }

    #[test]
    fn screaming_already_screaming() {
        assert_eq!(to_screaming_snake_case("ALREADY_UPPER"), "ALREADY_UPPER");
    }

    #[test]
    fn screaming_mixed_delimiters() {
        assert_eq!(to_screaming_snake_case("foo-bar_baz"), "FOO_BAR_BAZ");
    }

    #[test]
    fn screaming_with_numbers() {
        assert_eq!(to_screaming_snake_case("api-v2-key"), "API_V2_KEY");
    }

    // ── strip_provider_prefix ────────────────────────────────────────

    #[test]
    fn strip_prefix_match() {
        assert_eq!(
            strip_provider_prefix("akeyless_static_secret", "akeyless"),
            "static_secret"
        );
    }

    #[test]
    fn strip_prefix_no_match() {
        assert_eq!(
            strip_provider_prefix("other_secret", "akeyless"),
            "other_secret"
        );
    }

    #[test]
    fn strip_prefix_empty() {
        assert_eq!(strip_provider_prefix("", "akeyless"), "");
    }

    #[test]
    fn strip_prefix_exact_provider_with_underscore() {
        assert_eq!(strip_provider_prefix("akeyless_", "akeyless"), "");
    }

    #[test]
    fn strip_prefix_partial_no_underscore() {
        // "akeylessfoo" does NOT match "akeyless_" prefix pattern
        assert_eq!(
            strip_provider_prefix("akeylessfoo", "akeyless"),
            "akeylessfoo"
        );
    }

    #[test]
    fn strip_prefix_name_equals_provider() {
        // "aws" does not have trailing "_" so no stripping
        assert_eq!(strip_provider_prefix("aws", "aws"), "aws");
    }

    #[test]
    fn strip_prefix_nested_underscores() {
        assert_eq!(
            strip_provider_prefix("aws_s3_bucket", "aws"),
            "s3_bucket"
        );
    }

    #[test]
    fn strip_prefix_unicode() {
        assert_eq!(strip_provider_prefix("café_latte", "café"), "latte");
    }

    // ── split_words ──────────────────────────────────────────────────

    #[test]
    fn split_basic() {
        assert_eq!(split_words("foo-bar_baz"), vec!["foo", "bar", "baz"]);
    }

    #[test]
    fn split_empty() {
        assert!(split_words("").is_empty());
    }

    #[test]
    fn split_single_word() {
        assert_eq!(split_words("single"), vec!["single"]);
    }

    #[test]
    fn split_only_delimiters() {
        assert!(split_words("_-_-").is_empty());
    }

    #[test]
    fn split_leading_delimiter() {
        assert_eq!(split_words("_leading"), vec!["leading"]);
    }

    #[test]
    fn split_trailing_delimiter() {
        assert_eq!(split_words("trailing-"), vec!["trailing"]);
    }

    #[test]
    fn split_consecutive_delimiters() {
        assert_eq!(split_words("foo--bar__baz"), vec!["foo", "bar", "baz"]);
    }

    #[test]
    fn split_words_iter_equivalence() {
        let inputs = ["", "foo-bar_baz", "single", "_-_-", "a--b__c"];
        for input in inputs {
            let iter_result: Vec<_> = split_words_iter(input).collect();
            assert_eq!(
                iter_result,
                split_words(input),
                "iter != vec for {input:?}"
            );
        }
    }

    // ── CaseStyle enum ───────────────────────────────────────────────

    #[test]
    fn case_style_convert_all() {
        let name = "my-api-resource";
        assert_eq!(CaseStyle::Pascal.convert(name), "MyApiResource");
        assert_eq!(CaseStyle::Snake.convert(name), "my_api_resource");
        assert_eq!(CaseStyle::Camel.convert(name), "myApiResource");
        assert_eq!(CaseStyle::Kebab.convert(name), "my-api-resource");
        assert_eq!(CaseStyle::ScreamingSnake.convert(name), "MY_API_RESOURCE");
    }

    #[test]
    fn case_style_parse_roundtrip() {
        for &style in CaseStyle::all() {
            let s = style.to_string();
            let parsed: CaseStyle = s.parse().expect("should roundtrip");
            assert_eq!(parsed, style);
        }
    }

    #[test]
    fn case_style_all_len() {
        assert_eq!(CaseStyle::all().len(), 5);
    }

    #[test]
    fn case_style_default_is_snake() {
        assert_eq!(CaseStyle::default(), CaseStyle::Snake);
    }

    #[test]
    fn case_style_parse_aliases() {
        assert_eq!("pascal".parse::<CaseStyle>().unwrap(), CaseStyle::Pascal);
        assert_eq!("Pascal".parse::<CaseStyle>().unwrap(), CaseStyle::Pascal);
        assert_eq!(
            "PascalCase".parse::<CaseStyle>().unwrap(),
            CaseStyle::Pascal
        );
        assert_eq!("snake".parse::<CaseStyle>().unwrap(), CaseStyle::Snake);
        assert_eq!(
            "snake_case".parse::<CaseStyle>().unwrap(),
            CaseStyle::Snake
        );
        assert_eq!("camel".parse::<CaseStyle>().unwrap(), CaseStyle::Camel);
        assert_eq!(
            "camelCase".parse::<CaseStyle>().unwrap(),
            CaseStyle::Camel
        );
        assert_eq!("kebab".parse::<CaseStyle>().unwrap(), CaseStyle::Kebab);
        assert_eq!(
            "kebab-case".parse::<CaseStyle>().unwrap(),
            CaseStyle::Kebab
        );
        assert_eq!(
            "screaming_snake".parse::<CaseStyle>().unwrap(),
            CaseStyle::ScreamingSnake
        );
        assert_eq!(
            "ScreamingSnake".parse::<CaseStyle>().unwrap(),
            CaseStyle::ScreamingSnake
        );
        assert_eq!(
            "SCREAMING_SNAKE_CASE".parse::<CaseStyle>().unwrap(),
            CaseStyle::ScreamingSnake
        );
    }

    #[test]
    fn case_style_parse_unknown_error() {
        let err = "UNKNOWN".parse::<CaseStyle>().unwrap_err();
        assert!(err.to_string().contains("UNKNOWN"));
    }

    #[test]
    fn case_style_as_converter_fn_matches_convert() {
        for &style in CaseStyle::all() {
            let f = style.as_converter_fn();
            assert_eq!(f("test-input"), style.convert("test-input"));
        }
    }

    #[test]
    fn case_style_partial_eq_str() {
        assert!(CaseStyle::Snake == *"snake_case");
        assert!(CaseStyle::Snake == *"snake");
        assert!(CaseStyle::Pascal == *"PascalCase");
        assert!(CaseStyle::Snake != *"PascalCase");
        assert!(CaseStyle::Snake != *"bogus");
    }

    #[test]
    fn case_style_into_static_str() {
        let s: &'static str = CaseStyle::Kebab.into();
        assert_eq!(s, "kebab-case");
    }

    #[test]
    fn case_style_name() {
        assert_eq!(CaseStyle::Pascal.name(), "PascalCase");
        assert_eq!(CaseStyle::Snake.name(), "snake_case");
        assert_eq!(CaseStyle::Camel.name(), "camelCase");
        assert_eq!(CaseStyle::Kebab.name(), "kebab-case");
        assert_eq!(CaseStyle::ScreamingSnake.name(), "SCREAMING_SNAKE_CASE");
    }

    // ── NamingConvention trait ────────────────────────────────────────

    #[test]
    fn rust_convention_via_trait() {
        let c = RustConvention;
        assert_eq!(c.to_type_name("my-type"), "MyType");
        assert_eq!(c.to_field_name("my-field"), "my_field");
        assert_eq!(c.to_file_name("my-module"), "my_module");
    }

    #[test]
    fn go_convention_via_trait() {
        let c = GoConvention;
        assert_eq!(c.to_type_name("my-type"), "MyType");
        // Go uses PascalCase for exported fields
        assert_eq!(c.to_field_name("my-field"), "MyField");
        assert_eq!(c.to_file_name("my-module"), "my_module");
    }

    #[test]
    fn python_convention_via_trait() {
        let c = PythonConvention;
        assert_eq!(c.to_type_name("my-type"), "MyType");
        assert_eq!(c.to_field_name("my-field"), "my_field");
        assert_eq!(c.to_file_name("my-module"), "my_module");
    }

    #[test]
    fn ruby_convention_via_trait() {
        let c = RubyConvention;
        assert_eq!(c.to_type_name("my-type"), "MyType");
        assert_eq!(c.to_field_name("my-field"), "my_field");
        assert_eq!(c.to_file_name("my-module"), "my_module");
    }

    #[test]
    fn convention_trait_object_dispatch() {
        let conventions: Vec<Box<dyn NamingConvention>> = vec![
            Box::new(RustConvention),
            Box::new(GoConvention),
            Box::new(PythonConvention),
            Box::new(RubyConvention),
        ];
        for c in &conventions {
            assert!(!c.to_type_name("test").is_empty());
            assert!(!c.to_field_name("test").is_empty());
            assert!(!c.to_file_name("test").is_empty());
        }
    }

    #[test]
    fn convention_empty_input() {
        let conventions: Vec<Box<dyn NamingConvention>> = vec![
            Box::new(RustConvention),
            Box::new(GoConvention),
            Box::new(PythonConvention),
            Box::new(RubyConvention),
        ];
        for c in &conventions {
            assert!(c.to_type_name("").is_empty());
            assert!(c.to_field_name("").is_empty());
            assert!(c.to_file_name("").is_empty());
        }
    }

    // ── Table-driven cross-converter (public API) ────────────────────

    #[test]
    fn table_driven_all_converters() {
        let cases: &[(&str, &str, &str, &str, &str, &str)] = &[
            // (input, pascal, snake, camel, kebab, screaming)
            (
                "hello-world",
                "HelloWorld",
                "hello_world",
                "helloWorld",
                "hello-world",
                "HELLO_WORLD",
            ),
            (
                "hello_world",
                "HelloWorld",
                "hello_world",
                "helloWorld",
                "hello-world",
                "HELLO_WORLD",
            ),
            ("single", "Single", "single", "single", "single", "SINGLE"),
            ("", "", "", "", "", ""),
            ("a-b-c", "ABC", "a_b_c", "aBC", "a-b-c", "A_B_C"),
            (
                "api-v2-endpoint",
                "ApiV2Endpoint",
                "api_v2_endpoint",
                "apiV2Endpoint",
                "api-v2-endpoint",
                "API_V2_ENDPOINT",
            ),
            ("x", "X", "x", "x", "x", "X"),
        ];

        for &(input, pascal, snake, camel, kebab, screaming) in cases {
            assert_eq!(to_pascal_case(input), pascal, "pascal({input})");
            assert_eq!(to_snake_case(input), snake, "snake({input})");
            assert_eq!(to_camel_case(input), camel, "camel({input})");
            assert_eq!(to_kebab_case(input), kebab, "kebab({input})");
            assert_eq!(
                to_screaming_snake_case(input),
                screaming,
                "screaming({input})"
            );
        }
    }

    // ── Unicode edge cases ───────────────────────────────────────────

    #[test]
    fn pascal_unicode() {
        assert_eq!(to_pascal_case("über-straße"), "ÜberStraße");
    }

    #[test]
    fn camel_unicode() {
        assert_eq!(to_camel_case("über-straße"), "überStraße");
    }

    #[test]
    fn screaming_unicode() {
        assert_eq!(to_screaming_snake_case("über-straße"), "ÜBER_STRASSE");
    }

    #[test]
    fn kebab_unicode() {
        assert_eq!(to_kebab_case("über_straße"), "über-straße");
    }

    #[test]
    fn snake_unicode() {
        assert_eq!(to_snake_case("über-straße"), "über_straße");
    }

    // ── Idempotency ──────────────────────────────────────────────────

    #[test]
    fn snake_idempotent() {
        let input = "already_snake_case";
        assert_eq!(to_snake_case(&to_snake_case(input)), to_snake_case(input));
    }

    #[test]
    fn kebab_idempotent() {
        let input = "already-kebab-case";
        assert_eq!(to_kebab_case(&to_kebab_case(input)), to_kebab_case(input));
    }

    #[test]
    fn screaming_idempotent() {
        let input = "ALREADY_SCREAMING";
        assert_eq!(
            to_screaming_snake_case(&to_screaming_snake_case(input)),
            to_screaming_snake_case(input)
        );
    }

    #[test]
    fn pascal_idempotent_single_word() {
        // Single word without delimiters stays the same
        let input = "Hello";
        assert_eq!(
            to_pascal_case(&to_pascal_case(input)),
            to_pascal_case(input)
        );
    }

    // ── Conversion chains ────────────────────────────────────────────

    #[test]
    fn snake_then_kebab_roundtrip() {
        let input = "foo-bar-baz";
        let snake = to_snake_case(input);
        let kebab = to_kebab_case(&snake);
        assert_eq!(kebab, input);
    }

    #[test]
    fn kebab_then_snake_roundtrip() {
        let input = "foo_bar_baz";
        let kebab = to_kebab_case(input);
        let snake = to_snake_case(&kebab);
        assert_eq!(snake, input);
    }

    #[test]
    fn snake_then_screaming() {
        assert_eq!(
            to_screaming_snake_case(&to_snake_case("api-v2-key")),
            "API_V2_KEY"
        );
    }

    #[test]
    fn pascal_then_camel_same_as_camel() {
        // camelCase is defined as lowercase_first(to_pascal_case)
        let input = "some-api-name";
        let pascal = to_pascal_case(input);
        let camel_via_pascal = {
            let mut c = pascal.chars();
            match c.next() {
                None => String::new(),
                Some(first) => {
                    let lower: String = first.to_lowercase().collect();
                    format!("{lower}{}", c.as_str())
                }
            }
        };
        assert_eq!(to_camel_case(input), camel_via_pascal);
    }

    // ── Long/stress inputs ───────────────────────────────────────────

    #[test]
    fn pascal_long_name() {
        assert_eq!(
            to_pascal_case("this-is-a-very-long-name-with-many-segments"),
            "ThisIsAVeryLongNameWithManySegments"
        );
    }

    #[test]
    fn screaming_long_name() {
        assert_eq!(
            to_screaming_snake_case("this-is-a-very-long-name"),
            "THIS_IS_A_VERY_LONG_NAME"
        );
    }
}
