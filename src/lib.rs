//! Meimei (命名) — zero-dependency case convention converters for code generation.
//!
//! Provides pure string transformations between naming conventions commonly used
//! in code generation: `snake_case`, `PascalCase`, `camelCase`, `kebab-case`,
//! `SCREAMING_SNAKE_CASE`, and provider prefix stripping.
//!
//! Platform-specific modules ([`go`], [`rust`], [`python`], [`ruby`]) re-export
//! the appropriate converter under idiomatic names for each language.

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
}
