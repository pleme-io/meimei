//! Meimei (命名) — zero-dependency case convention converters for code generation.
//!
//! Provides pure string transformations between naming conventions commonly used
//! in code generation: `snake_case`, `PascalCase`, `camelCase`, `kebab-case`,
//! `SCREAMING_SNAKE_CASE`, and provider prefix stripping.
//!
//! Platform-specific modules ([`go`], [`rust`], [`python`], [`ruby`]) re-export
//! the appropriate converter under idiomatic names for each language.

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
    name.split(DELIMITERS)
        .filter(|s| !s.is_empty())
        .collect()
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

// ---------------------------------------------------------------------------
// NamingConvention trait
// ---------------------------------------------------------------------------

/// Trait for naming convention strategies.
///
/// Enables mockability in tests and swappable naming schemes.
/// The default implementation maps type names to `PascalCase`,
/// field/variable names to `snake_case`, and file names to `snake_case` —
/// the most common convention across Rust, Python, and Ruby.
pub trait NamingConvention: Send + Sync {
    /// Convert a name to the convention's type/class name form.
    ///
    /// Default: [`to_pascal_case`].
    #[must_use]
    fn to_type_name(&self, name: &str) -> String {
        to_pascal_case(name)
    }

    /// Convert a name to the convention's field/variable name form.
    ///
    /// Default: [`to_snake_case`].
    #[must_use]
    fn to_field_name(&self, name: &str) -> String {
        to_snake_case(name)
    }

    /// Convert a name to the convention's file name form.
    ///
    /// Default: [`to_snake_case`].
    #[must_use]
    fn to_file_name(&self, name: &str) -> String {
        to_snake_case(name)
    }
}

/// Rust naming convention: `PascalCase` types, `snake_case` fields and files.
#[derive(Debug, Clone, Copy, Default)]
pub struct RustConvention;
impl NamingConvention for RustConvention {}

/// Go naming convention: `PascalCase` types **and** fields, `snake_case` files.
#[derive(Debug, Clone, Copy, Default)]
pub struct GoConvention;

impl NamingConvention for GoConvention {
    fn to_field_name(&self, name: &str) -> String {
        to_pascal_case(name)
    }
}

/// Python naming convention: `PascalCase` types, `snake_case` fields and files.
#[derive(Debug, Clone, Copy, Default)]
pub struct PythonConvention;
impl NamingConvention for PythonConvention {}

/// Ruby naming convention: `PascalCase` types, `snake_case` fields and files.
#[derive(Debug, Clone, Copy, Default)]
pub struct RubyConvention;
impl NamingConvention for RubyConvention {}

// ---------------------------------------------------------------------------
// CaseStyle enum
// ---------------------------------------------------------------------------

/// Enumerates the supported identifier case styles.
///
/// Use [`CaseStyle::convert`] to apply a style to a name, or pattern-match
/// to dispatch on the style value.
///
/// # Examples
///
/// ```
/// use meimei::CaseStyle;
///
/// let style: CaseStyle = "snake_case".parse().unwrap();
/// assert_eq!(style, CaseStyle::Snake);
/// assert_eq!(style.convert("hello-world"), "hello_world");
/// assert_eq!(style.to_string(), "snake_case");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CaseStyle {
    /// `PascalCase`
    Pascal,
    /// `snake_case`
    Snake,
    /// `camelCase`
    Camel,
    /// `kebab-case`
    Kebab,
    /// `SCREAMING_SNAKE_CASE`
    ScreamingSnake,
}

impl CaseStyle {
    /// Apply this case style to the given identifier name.
    #[must_use]
    pub fn convert(&self, name: &str) -> String {
        match self {
            Self::Pascal => to_pascal_case(name),
            Self::Snake => to_snake_case(name),
            Self::Camel => to_camel_case(name),
            Self::Kebab => to_kebab_case(name),
            Self::ScreamingSnake => to_screaming_snake_case(name),
        }
    }

    /// Returns all supported case styles.
    #[must_use]
    pub fn all() -> &'static [Self] {
        &[
            Self::Pascal,
            Self::Snake,
            Self::Camel,
            Self::Kebab,
            Self::ScreamingSnake,
        ]
    }
}

impl Default for CaseStyle {
    /// Defaults to [`CaseStyle::Snake`], the most common convention in Rust.
    fn default() -> Self {
        Self::Snake
    }
}

impl core::fmt::Display for CaseStyle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let label = match self {
            Self::Pascal => "PascalCase",
            Self::Snake => "snake_case",
            Self::Camel => "camelCase",
            Self::Kebab => "kebab-case",
            Self::ScreamingSnake => "SCREAMING_SNAKE_CASE",
        };
        f.write_str(label)
    }
}

/// Error returned when parsing an unknown case style string.
///
/// See [`CaseStyle::from_str`](core::str::FromStr::from_str).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseCaseStyleError {
    unknown: String,
}

impl core::fmt::Display for ParseCaseStyleError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "unknown case style: {:?}", self.unknown)
    }
}

impl core::str::FromStr for CaseStyle {
    type Err = ParseCaseStyleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PascalCase" | "pascal" | "Pascal" => Ok(Self::Pascal),
            "snake_case" | "snake" | "Snake" => Ok(Self::Snake),
            "camelCase" | "camel" | "Camel" => Ok(Self::Camel),
            "kebab-case" | "kebab" | "Kebab" => Ok(Self::Kebab),
            "SCREAMING_SNAKE_CASE" | "screaming_snake" | "ScreamingSnake" => {
                Ok(Self::ScreamingSnake)
            }
            _ => Err(ParseCaseStyleError {
                unknown: s.to_owned(),
            }),
        }
    }
}

// ---------------------------------------------------------------------------
// Case converters
// ---------------------------------------------------------------------------

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
    split_words(name).into_iter().map(capitalize_first).collect()
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
// Tests
// ---------------------------------------------------------------------------

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
        // Name is exactly "provider_" with nothing after — returns empty string.
        assert_eq!(strip_provider_prefix("akeyless_", "akeyless"), "");
    }

    #[test]
    fn strip_prefix_partial_match() {
        // "akeyless" without trailing underscore should NOT be stripped.
        assert_eq!(
            strip_provider_prefix("akeylessfoo", "akeyless"),
            "akeylessfoo"
        );
    }

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

    // ── NamingConvention trait ──────────────────────────────────

    #[test]
    fn rust_convention() {
        let c = RustConvention;
        assert_eq!(c.to_type_name("my-type"), "MyType");
        assert_eq!(c.to_field_name("my-field"), "my_field");
        assert_eq!(c.to_file_name("my-module"), "my_module");
    }

    #[test]
    fn go_convention() {
        let c = GoConvention;
        assert_eq!(c.to_type_name("my-type"), "MyType");
        assert_eq!(c.to_field_name("my-field"), "MyField");
    }

    #[test]
    fn python_convention() {
        let c = PythonConvention;
        assert_eq!(c.to_type_name("my-type"), "MyType");
        assert_eq!(c.to_field_name("my-field"), "my_field");
    }

    #[test]
    fn custom_naming_convention() {
        // Demonstrates that NamingConvention is mockable
        struct MockConvention;
        impl NamingConvention for MockConvention {
            fn to_type_name(&self, _: &str) -> String {
                "MockType".to_string()
            }
            fn to_field_name(&self, _: &str) -> String {
                "mock_field".to_string()
            }
            fn to_file_name(&self, _: &str) -> String {
                "mock_file".to_string()
            }
        }
        let c = MockConvention;
        assert_eq!(c.to_type_name("anything"), "MockType");
        assert_eq!(c.to_field_name("anything"), "mock_field");
        assert_eq!(c.to_file_name("anything"), "mock_file");
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

    // ── RubyConvention trait ──────────────────────────────────────

    #[test]
    fn ruby_convention_type_name() {
        let c = RubyConvention;
        assert_eq!(c.to_type_name("my-type"), "MyType");
    }

    #[test]
    fn ruby_convention_field_name() {
        let c = RubyConvention;
        assert_eq!(c.to_field_name("my-field"), "my_field");
    }

    #[test]
    fn ruby_convention_file_name() {
        let c = RubyConvention;
        assert_eq!(c.to_file_name("my-module"), "my_module");
    }

    // ── Convention trait completeness ─────────────────────────────

    #[test]
    fn go_convention_file_name() {
        let c = GoConvention;
        assert_eq!(c.to_file_name("my-module"), "my_module");
    }

    #[test]
    fn python_convention_file_name() {
        let c = PythonConvention;
        assert_eq!(c.to_file_name("my-module"), "my_module");
    }

    #[test]
    fn convention_as_trait_object() {
        let conventions: Vec<Box<dyn NamingConvention>> = vec![
            Box::new(RustConvention),
            Box::new(GoConvention),
            Box::new(PythonConvention),
            Box::new(RubyConvention),
        ];
        for c in &conventions {
            assert!(!c.to_type_name("test-name").is_empty());
            assert!(!c.to_field_name("test-name").is_empty());
            assert!(!c.to_file_name("test-name").is_empty());
        }
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

    // ── Convention consistency ─────────────────────────────────────

    #[test]
    fn all_conventions_produce_nonempty_for_nonempty_input() {
        let input = "test-name";
        let conventions: Vec<(&str, Box<dyn NamingConvention>)> = vec![
            ("Rust", Box::new(RustConvention)),
            ("Go", Box::new(GoConvention)),
            ("Python", Box::new(PythonConvention)),
            ("Ruby", Box::new(RubyConvention)),
        ];
        for (label, c) in &conventions {
            assert!(
                !c.to_type_name(input).is_empty(),
                "{label}::to_type_name produced empty"
            );
            assert!(
                !c.to_field_name(input).is_empty(),
                "{label}::to_field_name produced empty"
            );
            assert!(
                !c.to_file_name(input).is_empty(),
                "{label}::to_file_name produced empty"
            );
        }
    }

    #[test]
    fn all_conventions_produce_empty_for_empty_input() {
        let input = "";
        let conventions: Vec<(&str, Box<dyn NamingConvention>)> = vec![
            ("Rust", Box::new(RustConvention)),
            ("Go", Box::new(GoConvention)),
            ("Python", Box::new(PythonConvention)),
            ("Ruby", Box::new(RubyConvention)),
        ];
        for (label, c) in &conventions {
            assert!(
                c.to_type_name(input).is_empty(),
                "{label}::to_type_name not empty for empty input"
            );
            assert!(
                c.to_field_name(input).is_empty(),
                "{label}::to_field_name not empty for empty input"
            );
            assert!(
                c.to_file_name(input).is_empty(),
                "{label}::to_file_name not empty for empty input"
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

    // ── CaseStyle ─────────────────────────────────────────────────

    #[test]
    fn case_style_convert_all_variants() {
        let name = "hello-world";
        assert_eq!(CaseStyle::Pascal.convert(name), "HelloWorld");
        assert_eq!(CaseStyle::Snake.convert(name), "hello_world");
        assert_eq!(CaseStyle::Camel.convert(name), "helloWorld");
        assert_eq!(CaseStyle::Kebab.convert(name), "hello-world");
        assert_eq!(CaseStyle::ScreamingSnake.convert(name), "HELLO_WORLD");
    }

    #[test]
    fn case_style_display_roundtrip() {
        for &style in CaseStyle::all() {
            let s = style.to_string();
            let parsed: CaseStyle = s.parse().expect("roundtrip");
            assert_eq!(parsed, style, "roundtrip failed for {s}");
        }
    }

    #[test]
    fn case_style_from_str_aliases() {
        assert_eq!("pascal".parse::<CaseStyle>().unwrap(), CaseStyle::Pascal);
        assert_eq!("Pascal".parse::<CaseStyle>().unwrap(), CaseStyle::Pascal);
        assert_eq!("PascalCase".parse::<CaseStyle>().unwrap(), CaseStyle::Pascal);
        assert_eq!("snake".parse::<CaseStyle>().unwrap(), CaseStyle::Snake);
        assert_eq!("camel".parse::<CaseStyle>().unwrap(), CaseStyle::Camel);
        assert_eq!("kebab".parse::<CaseStyle>().unwrap(), CaseStyle::Kebab);
        assert_eq!(
            "screaming_snake".parse::<CaseStyle>().unwrap(),
            CaseStyle::ScreamingSnake
        );
    }

    #[test]
    fn case_style_from_str_unknown() {
        let err = "unknown".parse::<CaseStyle>().unwrap_err();
        assert_eq!(
            err,
            ParseCaseStyleError {
                unknown: "unknown".to_string()
            }
        );
        assert!(err.to_string().contains("unknown"));
    }

    #[test]
    fn case_style_default() {
        assert_eq!(CaseStyle::default(), CaseStyle::Snake);
    }

    #[test]
    fn case_style_all_length() {
        assert_eq!(CaseStyle::all().len(), 5);
    }

    #[test]
    fn case_style_debug() {
        assert_eq!(format!("{:?}", CaseStyle::Pascal), "Pascal");
    }
}
