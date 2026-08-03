# meimei (命名)

Zero-dependency case-convention converters for code generation.

Pure string transformations between the naming conventions a generator has to
straddle: `snake_case`, `PascalCase`, `camelCase`, `kebab-case`,
`SCREAMING_SNAKE_CASE`, plus provider-prefix stripping.

Platform modules — `go`, `rust`, `python`, `ruby` — re-export the right
converter under each language's idiomatic name, so emitted code reads native
rather than translated.

## Usage

```toml
[dependencies]
meimei = "0.1"
```

Zero dependencies, by design: it sits at the bottom of the generation stack.

## License

MIT — see [LICENSE](./LICENSE).
