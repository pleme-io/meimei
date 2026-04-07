/// Delimiter characters used for word splitting in identifiers.
const DELIMITERS: [char; 2] = ['-', '_'];

/// Split a name into words at delimiter boundaries (hyphens and underscores).
/// This is the foundation all case converters build on.
///
/// # Examples
///
/// ```
/// assert_eq!(meimei::split_words("foo-bar_baz"), vec!["foo", "bar", "baz"]);
/// assert_eq!(meimei::split_words("single"), vec!["single"]);
/// assert_eq!(meimei::split_words("").is_empty(), true);
/// ```
#[must_use]
pub fn split_words(name: &str) -> Vec<&str> {
    split_words_iter(name).collect()
}

/// Lazily split a name into words at delimiter boundaries.
///
/// Unlike [`split_words`], this returns an iterator instead of collecting into
/// a `Vec`, which avoids allocation when the caller only needs to iterate.
///
/// # Examples
///
/// ```
/// let words: Vec<_> = meimei::split_words_iter("foo-bar_baz").collect();
/// assert_eq!(words, vec!["foo", "bar", "baz"]);
/// ```
pub fn split_words_iter(name: &str) -> impl Iterator<Item = &str> {
    name.split(DELIMITERS).filter(|s| !s.is_empty())
}

/// Apply a character transformation to the first character of a string,
/// leaving the rest unchanged.
fn transform_first<I: Iterator<Item = char>>(s: &str, f: impl FnOnce(char) -> I) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => {
            let transformed: String = f(c).collect();
            format!("{transformed}{}", chars.as_str())
        }
        None => String::new(),
    }
}

/// Capitalize the first character of a word, leaving the rest unchanged.
fn capitalize_first(word: &str) -> String {
    transform_first(word, char::to_uppercase)
}

/// Lowercase the first character of a string, leaving the rest unchanged.
fn lowercase_first(s: &str) -> String {
    transform_first(s, char::to_lowercase)
}

/// Convert a hyphenated or `snake_case` name to `PascalCase`.
///
/// Splits on `-` and `_`, capitalizes the first character of each segment,
/// and concatenates.
///
/// # Examples
///
/// ```
/// assert_eq!(meimei::to_pascal_case("bound-aws-account-id"), "BoundAwsAccountId");
/// assert_eq!(meimei::to_pascal_case("access_expires"), "AccessExpires");
/// assert_eq!(meimei::to_pascal_case("name"), "Name");
/// ```
#[must_use]
pub fn to_pascal_case(name: &str) -> String {
    split_words_iter(name).map(capitalize_first).collect()
}

/// Convert a name to `snake_case` (hyphens become underscores).
///
/// # Examples
///
/// ```
/// assert_eq!(meimei::to_snake_case("bound-aws-account-id"), "bound_aws_account_id");
/// ```
#[must_use]
pub fn to_snake_case(name: &str) -> String {
    name.replace('-', "_")
}

/// Convert a name to `camelCase`.
///
/// Produces `PascalCase` then lowercases the first character.
///
/// # Examples
///
/// ```
/// assert_eq!(meimei::to_camel_case("bound-aws-account-id"), "boundAwsAccountId");
/// ```
#[must_use]
pub fn to_camel_case(name: &str) -> String {
    lowercase_first(&to_pascal_case(name))
}

/// Convert a name to `kebab-case` (underscores become hyphens).
///
/// # Examples
///
/// ```
/// assert_eq!(meimei::to_kebab_case("bound_aws_account_id"), "bound-aws-account-id");
/// ```
#[must_use]
pub fn to_kebab_case(name: &str) -> String {
    name.replace('_', "-")
}

/// Convert a name to `SCREAMING_SNAKE_CASE`.
///
/// Replaces hyphens with underscores and uppercases the entire string.
///
/// # Examples
///
/// ```
/// assert_eq!(meimei::to_screaming_snake_case("bound-aws-account-id"), "BOUND_AWS_ACCOUNT_ID");
/// assert_eq!(meimei::to_screaming_snake_case("access_expires"), "ACCESS_EXPIRES");
/// ```
#[must_use]
pub fn to_screaming_snake_case(name: &str) -> String {
    name.replace('-', "_").to_uppercase()
}

/// Strip a common provider prefix from a resource name.
///
/// If `name` starts with `"{provider}_"`, returns the remainder.
/// Otherwise returns `name` unchanged.
///
/// # Examples
///
/// ```
/// assert_eq!(meimei::strip_provider_prefix("akeyless_static_secret", "akeyless"), "static_secret");
/// assert_eq!(meimei::strip_provider_prefix("other_secret", "akeyless"), "other_secret");
/// ```
#[must_use]
pub fn strip_provider_prefix<'a>(name: &'a str, provider: &str) -> &'a str {
    let prefix = format!("{provider}_");
    name.strip_prefix(&prefix).unwrap_or(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── PascalCase ────────────────────────────────────────────────────

    #[test]
    fn pascal_case_from_kebab() {
        assert_eq!(to_pascal_case("bound-aws-account-id"), "BoundAwsAccountId");
    }

    #[test]
    fn pascal_case_from_snake() {
        assert_eq!(to_pascal_case("access_expires"), "AccessExpires");
    }

    #[test]
    fn pascal_case_single_word() {
        assert_eq!(to_pascal_case("name"), "Name");
    }

    #[test]
    fn pascal_case_already_pascal() {
        assert_eq!(to_pascal_case("AlreadyPascal"), "AlreadyPascal");
    }

    #[test]
    fn pascal_case_empty() {
        assert_eq!(to_pascal_case(""), "");
    }

    #[test]
    fn pascal_case_single_char() {
        assert_eq!(to_pascal_case("a"), "A");
    }

    #[test]
    fn pascal_case_consecutive_delimiters() {
        assert_eq!(to_pascal_case("a--b__c"), "ABC");
    }

    #[test]
    fn pascal_case_leading_delimiter() {
        assert_eq!(to_pascal_case("_leading"), "Leading");
    }

    #[test]
    fn pascal_case_trailing_delimiter() {
        assert_eq!(to_pascal_case("trailing_"), "Trailing");
    }

    #[test]
    fn pascal_case_mixed_delimiters() {
        assert_eq!(to_pascal_case("foo-bar_baz"), "FooBarBaz");
    }

    // ── snake_case ────────────────────────────────────────────────────

    #[test]
    fn snake_case_from_kebab() {
        assert_eq!(to_snake_case("bound-aws-account-id"), "bound_aws_account_id");
    }

    #[test]
    fn snake_case_already_snake() {
        assert_eq!(to_snake_case("already_snake"), "already_snake");
    }

    #[test]
    fn snake_case_empty() {
        assert_eq!(to_snake_case(""), "");
    }

    #[test]
    fn snake_case_single_char() {
        assert_eq!(to_snake_case("x"), "x");
    }

    // ── camelCase ─────────────────────────────────────────────────────

    #[test]
    fn camel_case_from_kebab() {
        assert_eq!(to_camel_case("bound-aws-account-id"), "boundAwsAccountId");
    }

    #[test]
    fn camel_case_single_word() {
        assert_eq!(to_camel_case("name"), "name");
    }

    #[test]
    fn camel_case_empty() {
        assert_eq!(to_camel_case(""), "");
    }

    #[test]
    fn camel_case_single_char() {
        assert_eq!(to_camel_case("A"), "a");
    }

    #[test]
    fn camel_case_from_snake() {
        assert_eq!(to_camel_case("access_expires"), "accessExpires");
    }

    // ── kebab-case ────────────────────────────────────────────────────

    #[test]
    fn kebab_case_from_snake() {
        assert_eq!(to_kebab_case("bound_aws_account_id"), "bound-aws-account-id");
    }

    #[test]
    fn kebab_case_already_kebab() {
        assert_eq!(to_kebab_case("already-kebab"), "already-kebab");
    }

    #[test]
    fn kebab_case_empty() {
        assert_eq!(to_kebab_case(""), "");
    }

    // ── SCREAMING_SNAKE_CASE ──────────────────────────────────────────

    #[test]
    fn screaming_snake_from_kebab() {
        assert_eq!(
            to_screaming_snake_case("bound-aws-account-id"),
            "BOUND_AWS_ACCOUNT_ID"
        );
    }

    #[test]
    fn screaming_snake_from_snake() {
        assert_eq!(to_screaming_snake_case("access_expires"), "ACCESS_EXPIRES");
    }

    #[test]
    fn screaming_snake_already_screaming() {
        assert_eq!(to_screaming_snake_case("ALREADY_UPPER"), "ALREADY_UPPER");
    }

    #[test]
    fn screaming_snake_empty() {
        assert_eq!(to_screaming_snake_case(""), "");
    }

    #[test]
    fn screaming_snake_single_char() {
        assert_eq!(to_screaming_snake_case("a"), "A");
    }

    #[test]
    fn screaming_snake_mixed() {
        assert_eq!(to_screaming_snake_case("foo-bar_baz"), "FOO_BAR_BAZ");
    }

    // ── strip_provider_prefix ─────────────────────────────────────────

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
    fn strip_prefix_empty_name() {
        assert_eq!(strip_provider_prefix("", "akeyless"), "");
    }

    #[test]
    fn strip_prefix_exact_prefix() {
        assert_eq!(strip_provider_prefix("akeyless_", "akeyless"), "");
    }

    #[test]
    fn strip_prefix_partial_match() {
        assert_eq!(
            strip_provider_prefix("akeylessfoo", "akeyless"),
            "akeylessfoo"
        );
    }

    // ── Word splitting ──────────────────────────────────────────

    #[test]
    fn split_words_basic() {
        assert_eq!(split_words("foo-bar_baz"), vec!["foo", "bar", "baz"]);
    }

    #[test]
    fn split_words_empty() {
        assert!(split_words("").is_empty());
    }

    #[test]
    fn split_words_no_delimiters() {
        assert_eq!(split_words("foobar"), vec!["foobar"]);
    }

    #[test]
    fn split_words_consecutive_delimiters() {
        assert_eq!(split_words("foo--bar__baz"), vec!["foo", "bar", "baz"]);
    }

    // ── Stress tests ────────────────────────────────────────────

    #[test]
    fn pascal_case_long_name() {
        let result = to_pascal_case("this-is-a-very-long-name-with-many-segments");
        assert_eq!(result, "ThisIsAVeryLongNameWithManySegments");
    }

    #[test]
    fn snake_case_preserves_numbers() {
        assert_eq!(to_snake_case("v2-api-endpoint"), "v2_api_endpoint");
    }

    #[test]
    fn camel_case_with_numbers() {
        assert_eq!(to_camel_case("get-v2-items"), "getV2Items");
    }

    #[test]
    fn screaming_snake_numbers() {
        assert_eq!(to_screaming_snake_case("api-v2-key"), "API_V2_KEY");
    }

    #[test]
    fn strip_prefix_unicode() {
        assert_eq!(strip_provider_prefix("café_latte", "café"), "latte");
    }

    #[test]
    fn all_conventions_roundtrip() {
        let name = "my-api-resource";
        let pascal = to_pascal_case(name);
        let screaming = to_screaming_snake_case(name);
        assert_eq!(pascal, "MyApiResource");
        assert_eq!(screaming, "MY_API_RESOURCE");
    }

    // ── Additional split_words edge cases ─────────────────────────

    #[test]
    fn split_words_leading_delimiter() {
        assert_eq!(split_words("_leading"), vec!["leading"]);
    }

    #[test]
    fn split_words_trailing_delimiter() {
        assert_eq!(split_words("trailing-"), vec!["trailing"]);
    }

    #[test]
    fn split_words_only_delimiters() {
        assert!(split_words("_-_-").is_empty());
    }

    #[test]
    fn split_words_single_char() {
        assert_eq!(split_words("x"), vec!["x"]);
    }

    #[test]
    fn split_words_mixed_delimiters() {
        assert_eq!(split_words("a-b_c-d"), vec!["a", "b", "c", "d"]);
    }

    // ── Additional snake_case edge cases ──────────────────────────

    #[test]
    fn snake_case_consecutive_hyphens() {
        assert_eq!(to_snake_case("a--b"), "a__b");
    }

    #[test]
    fn snake_case_mixed_delimiters() {
        assert_eq!(to_snake_case("foo-bar_baz"), "foo_bar_baz");
    }

    #[test]
    fn snake_case_leading_hyphen() {
        assert_eq!(to_snake_case("-leading"), "_leading");
    }

    #[test]
    fn snake_case_trailing_hyphen() {
        assert_eq!(to_snake_case("trailing-"), "trailing_");
    }

    // ── Additional camelCase edge cases ───────────────────────────

    #[test]
    fn camel_case_consecutive_delimiters() {
        assert_eq!(to_camel_case("a--b__c"), "aBC");
    }

    #[test]
    fn camel_case_leading_delimiter() {
        assert_eq!(to_camel_case("_leading"), "leading");
    }

    #[test]
    fn camel_case_trailing_delimiter() {
        assert_eq!(to_camel_case("trailing_"), "trailing");
    }

    #[test]
    fn camel_case_mixed_delimiters() {
        assert_eq!(to_camel_case("foo-bar_baz"), "fooBarBaz");
    }

    #[test]
    fn camel_case_already_camel() {
        assert_eq!(to_camel_case("alreadyCamel"), "alreadyCamel");
    }

    // ── Additional kebab-case edge cases ──────────────────────────

    #[test]
    fn kebab_case_single_char() {
        assert_eq!(to_kebab_case("x"), "x");
    }

    #[test]
    fn kebab_case_consecutive_underscores() {
        assert_eq!(to_kebab_case("a__b"), "a--b");
    }

    #[test]
    fn kebab_case_leading_underscore() {
        assert_eq!(to_kebab_case("_leading"), "-leading");
    }

    #[test]
    fn kebab_case_trailing_underscore() {
        assert_eq!(to_kebab_case("trailing_"), "trailing-");
    }

    #[test]
    fn kebab_case_mixed_delimiters() {
        assert_eq!(to_kebab_case("foo_bar-baz"), "foo-bar-baz");
    }

    #[test]
    fn kebab_case_with_numbers() {
        assert_eq!(to_kebab_case("v2_api_endpoint"), "v2-api-endpoint");
    }

    // ── Additional SCREAMING_SNAKE edge cases ─────────────────────

    #[test]
    fn screaming_snake_consecutive_hyphens() {
        assert_eq!(to_screaming_snake_case("a--b"), "A__B");
    }

    #[test]
    fn screaming_snake_leading_delimiter() {
        assert_eq!(to_screaming_snake_case("-leading"), "_LEADING");
    }

    #[test]
    fn screaming_snake_trailing_delimiter() {
        assert_eq!(to_screaming_snake_case("trailing-"), "TRAILING_");
    }

    // ── Additional strip_provider_prefix edge cases ───────────────

    #[test]
    fn strip_prefix_empty_provider() {
        assert_eq!(strip_provider_prefix("_foo", ""), "foo");
    }

    #[test]
    fn strip_prefix_name_equals_provider() {
        assert_eq!(strip_provider_prefix("aws", "aws"), "aws");
    }

    #[test]
    fn strip_prefix_multiple_underscores() {
        assert_eq!(
            strip_provider_prefix("aws_s3_bucket", "aws"),
            "s3_bucket"
        );
    }

    // ── Table-driven cross-converter tests ────────────────────────

    #[test]
    fn table_driven_case_conversions() {
        let cases: &[(&str, &str, &str, &str, &str, &str)] = &[
            //  input,               pascal,              snake,               camel,               kebab,               screaming
            ("hello-world",         "HelloWorld",        "hello_world",       "helloWorld",        "hello-world",       "HELLO_WORLD"),
            ("hello_world",         "HelloWorld",        "hello_world",       "helloWorld",        "hello-world",       "HELLO_WORLD"),
            ("single",             "Single",            "single",            "single",            "single",            "SINGLE"),
            ("",                   "",                  "",                  "",                  "",                  ""),
            ("a-b-c",             "ABC",               "a_b_c",             "aBC",               "a-b-c",             "A_B_C"),
            ("api-v2-endpoint",   "ApiV2Endpoint",     "api_v2_endpoint",   "apiV2Endpoint",     "api-v2-endpoint",   "API_V2_ENDPOINT"),
            ("x",                 "X",                 "x",                 "x",                 "x",                 "X"),
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

    // ── Table-driven strip_provider_prefix tests ──────────────────

    #[test]
    fn table_driven_strip_provider_prefix() {
        let cases: &[(&str, &str, &str)] = &[
            ("aws_s3_bucket", "aws", "s3_bucket"),
            ("aws_lambda", "aws", "lambda"),
            ("gcp_compute", "gcp", "compute"),
            ("gcp_compute", "aws", "gcp_compute"),
            ("standalone", "aws", "standalone"),
            ("", "aws", ""),
            ("aws_", "aws", ""),
        ];

        for &(name, provider, expected) in cases {
            assert_eq!(
                strip_provider_prefix(name, provider),
                expected,
                "strip({name}, {provider})"
            );
        }
    }

    // ── Unicode edge cases ────────────────────────────────────────

    #[test]
    fn pascal_case_unicode() {
        assert_eq!(to_pascal_case("über-straße"), "ÜberStraße");
    }

    #[test]
    fn camel_case_unicode() {
        assert_eq!(to_camel_case("über-straße"), "überStraße");
    }

    #[test]
    fn screaming_snake_unicode() {
        assert_eq!(to_screaming_snake_case("über-straße"), "ÜBER_STRASSE");
    }

    #[test]
    fn split_words_unicode() {
        assert_eq!(split_words("über-straße"), vec!["über", "straße"]);
    }

    #[test]
    fn kebab_case_unicode() {
        assert_eq!(to_kebab_case("über_straße"), "über-straße");
    }

    // ── split_words_iter ──────────────────────────────────────────

    #[test]
    fn split_words_iter_matches_split_words() {
        let inputs = ["", "foo-bar_baz", "single", "_-_-", "a--b__c"];
        for input in inputs {
            let iter_result: Vec<_> = split_words_iter(input).collect();
            assert_eq!(iter_result, split_words(input), "mismatch for {input:?}");
        }
    }
}
