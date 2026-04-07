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
    split_words, strip_provider_prefix, to_camel_case, to_kebab_case, to_pascal_case,
    to_screaming_snake_case, to_snake_case,
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
}
