use crate::{Highlight, Kind};
use logos::Logos;

#[derive(Logos, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Sh {
    #[error]
    Error,

    #[regex(r"[^\s=$\(\)\{\}]*")]
    Word,

    #[token("$")]
    #[regex(r"\$[^{(\s][^\s]*")]
    #[regex(r"\$\{[^}]*\}")]
    Parameter,

    #[regex(r#""([^"\n]|\\["\n])*""#)]
    #[regex("'[^']*'")]
    Literal,

    #[token("|")]
    #[token("&")]
    #[token(";")]
    #[token("(")]
    #[token(")")]
    #[token("<")]
    #[token(">")]
    Glyph,

    #[token("\n")]
    Newline,

    #[token("}")]
    #[token("]]")]
    #[regex(r"esac|for|in|done|fi")]
    Keyword,

    #[token("{")]
    #[token("[[")]
    #[regex(r"!|do|then|elif|until|case|else|if|while|function|select")]
    KeywordCtx,

    #[regex("#[^\n]*")]
    Comment,
}

impl Highlight for Sh {
    const LANG: &'static str = "sh";

    fn kind(tokens: &[Self; 2]) -> Kind {
        use Sh::*;

        match tokens {
            [KeywordCtx, Word] | [Glyph, Word] | [Newline, Word] => Kind::StrongIdentifier,
            [_, Parameter] => Kind::Identifier,
            [_, Literal] => Kind::Literal,
            [_, Glyph] => Kind::Glyph,
            [_, Keyword] | [_, KeywordCtx] => Kind::Keyword,
            [_, Comment] => Kind::Comment,
            _ => Kind::None,
        }
    }
}
