use crate::{to_pascal_case, to_snake_case};

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
