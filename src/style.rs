use crate::{to_camel_case, to_kebab_case, to_pascal_case, to_screaming_snake_case, to_snake_case};

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

    /// Return the converter function pointer for this case style.
    ///
    /// Useful when you need to pass a converter as a function argument without
    /// capturing `self`.
    #[must_use]
    pub fn as_converter_fn(&self) -> fn(&str) -> String {
        match self {
            Self::Pascal => to_pascal_case,
            Self::Snake => to_snake_case,
            Self::Camel => to_camel_case,
            Self::Kebab => to_kebab_case,
            Self::ScreamingSnake => to_screaming_snake_case,
        }
    }

    /// Returns all supported case styles.
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::Pascal,
            Self::Snake,
            Self::Camel,
            Self::Kebab,
            Self::ScreamingSnake,
        ]
    }

    /// Returns the canonical string label for this case style.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Pascal => "PascalCase",
            Self::Snake => "snake_case",
            Self::Camel => "camelCase",
            Self::Kebab => "kebab-case",
            Self::ScreamingSnake => "SCREAMING_SNAKE_CASE",
        }
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
        f.write_str(self.name())
    }
}

impl From<CaseStyle> for &'static str {
    fn from(style: CaseStyle) -> Self {
        style.name()
    }
}

impl PartialEq<str> for CaseStyle {
    fn eq(&self, other: &str) -> bool {
        other.parse::<Self>().is_ok_and(|parsed| parsed == *self)
    }
}

impl PartialEq<CaseStyle> for str {
    fn eq(&self, other: &CaseStyle) -> bool {
        other == self
    }
}

/// Error returned when parsing an unknown case style string.
///
/// See [`CaseStyle::from_str`](core::str::FromStr::from_str).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseCaseStyleError {
    pub(crate) unknown: String,
}

impl core::fmt::Display for ParseCaseStyleError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "unknown case style: {:?}", self.unknown)
    }
}

impl core::error::Error for ParseCaseStyleError {}

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

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn case_style_as_converter_fn() {
        for &style in CaseStyle::all() {
            let f = style.as_converter_fn();
            assert_eq!(f("hello-world"), style.convert("hello-world"));
        }
    }

    #[test]
    fn parse_case_style_error_is_error_trait() {
        let err: Box<dyn core::error::Error> = Box::new(ParseCaseStyleError {
            unknown: "nope".to_string(),
        });
        assert!(err.to_string().contains("nope"));
    }

    #[test]
    fn case_style_name_matches_display() {
        for &style in CaseStyle::all() {
            assert_eq!(style.name(), style.to_string());
        }
    }

    #[test]
    fn case_style_into_static_str() {
        let s: &'static str = CaseStyle::Pascal.into();
        assert_eq!(s, "PascalCase");
    }

    #[test]
    fn case_style_partial_eq_str() {
        assert!(CaseStyle::Snake == *"snake_case");
        assert!(CaseStyle::Snake == *"snake");
        assert!(CaseStyle::Pascal == *"PascalCase");
        assert!(CaseStyle::Snake != *"PascalCase");
        assert!(CaseStyle::Snake != *"unknown");
    }

    #[test]
    fn str_partial_eq_case_style() {
        assert!(*"snake_case" == CaseStyle::Snake);
        assert!(*"PascalCase" == CaseStyle::Pascal);
    }
}
