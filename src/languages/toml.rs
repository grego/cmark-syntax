use crate::{Highlight, Kind};
use logos::Logos;

#[derive(Logos, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Toml {
    #[regex("[a-zA-Z0-9_-]*")]
    Identifier,

    #[regex(r#""([^"\n]|\\"])*""#)]
    #[regex(r#""""([^"]|\\"]|"[^"]|""[^"])*""""#)]
    #[regex(r#"'[^'\n]*'"#)]
    #[regex(r#"'''([^']|'[^']|''[^'])*'''"#)]
    #[regex("[+-]?[0-9_]+(.[0-9]+)?")]
    #[regex("[+-]?0[xX][0-9a-fA-F_]+", priority = 7)]
    #[regex("[+-]?0[oO][0-7_]+")]
    #[regex("[+-]?0[bB][01_]+")]
    #[regex("[+-]?[0-9](.[0-9]+)?[eE][+-]?[0-9]+")]
    #[regex("[+-]?(inf|nan)")]
    #[regex("(true|false)")]
    #[regex("[0-9]+-[0-9]+-[0-9]+")]
    #[regex("[0-9]+:[0-9]+:[0-9]+(.[0-9]+)?([Zz]|([+-][0-9]+:[0-9]+))?")]
    #[regex("[0-9]+-[0-9]+-[0-9]+[Tt ]?[0-9]+:[0-9]+:[0-9]+(.[0-9]+)?([Zz]|([+-][0-9]+:[0-9]+))?")]
    Literal,

    #[regex("\\{|\\}|\\]")]
    Glyph,

    #[regex("\\[")]
    GlyphCtx,

    #[regex("#[^\n]*")]
    Comment,

    None,
}

impl Highlight for Toml {
    const LANG: &'static str = "toml";
    const START: Self = Self::None;

    fn kind(tokens: &[Self; 2]) -> Kind {
        use Toml::*;

        match tokens {
            [GlyphCtx, Identifier] => Kind::StrongIdentifier,
            [Identifier, Identifier] => Kind::SpecialIdentifier,
            [_, Identifier] => Kind::Identifier,
            [_, Literal] => Kind::Literal,
            [_, GlyphCtx] => Kind::Glyph,
            [_, Glyph] => Kind::Glyph,
            [_, Comment] => Kind::Comment,
            _ => Kind::None,
        }
    }
}
