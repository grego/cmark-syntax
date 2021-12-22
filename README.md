# cmark syntax highlighting
[![Crates.io status](https://badgen.net/crates/v/cmark-syntax)](https://crates.io/crates/cmark-syntax)
[![Docs](https://docs.rs/cmark-syntax/badge.svg)](https://docs.rs/cmark-syntax)

This crate provides a preprocessor for [pulldown_cmark](https://docs.rs/pulldown_cmark)
events that implements syntax highlighting.
It is based on the work of [Maciej Hirsz](https://maciej.codes) for the Ramhorns templating
engine.

## Supported languages
* Rust
* JavaScript
* sh shell
* TOML

Files defining language syntax are located in `src/languages` directory.
The syntax is defined using regexes, which the [Logos](https://docs.rs/logos) procedural
macro turns into a lexer at the compile time.
PRs implementing new languages are very welcome!

## Features
With `latex2mathml` feature enabled, blocks denoted by `math` containing LaTeX formulas
are rendered into [MathML](https://developer.mozilla.org/docs/Web/MathML) in block mode
and analogously for inline code delimited by `$` at the start and the end in inline mode.

## Use
This preprocessor can be used as a callback for the [Ramhorns](https://docs.rs/ramhorns)
templating engine.
```rust
use ramhorns::encoding::Encoder;

pub fn encode<E: Encoder>(source: &str, encoder: &mut E) -> Result<(), E::Error> {
    let parser = pulldown_cmark::Parser::new(source);
    let processed = cmark_syntax::SyntaxPreprocessor::new(parser);
    encoder.write_html(processed)
}
```
