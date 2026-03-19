//! Meimei (命名) — zero-dependency case convention converters for code generation.
//!
//! Provides pure string transformations between naming conventions commonly used
//! in code generation: `snake_case`, `PascalCase`, `camelCase`, `kebab-case`,
//! `SCREAMING_SNAKE_CASE`, and provider prefix stripping.
//!
//! Platform-specific modules ([`go`], [`rust`], [`python`], [`ruby`]) re-export
//! the appropriate converter under idiomatic names for each language.

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
    name.split(|c: char| c == '-' || c == '_')
        .filter(|s| !s.is_empty())
        .collect()
}

// ---------------------------------------------------------------------------
// NamingConvention trait
// ---------------------------------------------------------------------------

/// Trait for naming convention strategies.
/// Enables mockability in tests and swappable naming schemes.
pub trait NamingConvention: Send + Sync {
    /// Convert a name to the convention's type/class name form.
    fn to_type_name(&self, name: &str) -> String;
    /// Convert a name to the convention's field/variable name form.
    fn to_field_name(&self, name: &str) -> String;
    /// Convert a name to the convention's file name form.
    fn to_file_name(&self, name: &str) -> String;
}

/// Rust naming convention.
pub struct RustConvention;

impl NamingConvention for RustConvention {
    fn to_type_name(&self, name: &str) -> String {
        to_pascal_case(name)
    }
    fn to_field_name(&self, name: &str) -> String {
        to_snake_case(name)
    }
    fn to_file_name(&self, name: &str) -> String {
        to_snake_case(name)
    }
}

/// Go naming convention.
pub struct GoConvention;

impl NamingConvention for GoConvention {
    fn to_type_name(&self, name: &str) -> String {
        to_pascal_case(name)
    }
    fn to_field_name(&self, name: &str) -> String {
        to_pascal_case(name)
    }
    fn to_file_name(&self, name: &str) -> String {
        to_snake_case(name)
    }
}

/// Python naming convention.
pub struct PythonConvention;

impl NamingConvention for PythonConvention {
    fn to_type_name(&self, name: &str) -> String {
        to_pascal_case(name)
    }
    fn to_field_name(&self, name: &str) -> String {
        to_snake_case(name)
    }
    fn to_file_name(&self, name: &str) -> String {
        to_snake_case(name)
    }
}

// ---------------------------------------------------------------------------
// Case converters
// ---------------------------------------------------------------------------

/// Convert a hyphenated or snake_case name to `PascalCase`.
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
    name.split(|c: char| c == '-' || c == '_')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut chars = s.chars();
            match chars.next() {
                Some(c) => {
                    let upper: String = c.to_uppercase().collect();
                    format!("{upper}{}", chars.as_str())
                }
                None => String::new(),
            }
        })
        .collect()
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
    let pascal = to_pascal_case(name);
    let mut chars = pascal.chars();
    match chars.next() {
        Some(c) => {
            let lower: String = c.to_lowercase().collect();
            format!("{lower}{}", chars.as_str())
        }
        None => String::new(),
    }
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
}
