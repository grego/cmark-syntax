use crate::{Highlight, Kind};
use logos::Logos;

#[derive(Logos, PartialEq, Eq, Clone, Copy, Debug)]
pub enum C {
    #[regex("[a-zA-Z_$][a-zA-Z0-9_]*")]
    Identifier,

    #[regex("\"([^\"\\\\]|\\\\[.\n])*\"")]
    #[regex("'([^']|\\\\')'")]
    #[regex("[0-9][0-9]*")]
    #[regex("0[xX][0-9a-fA-F]+")]
    #[regex("0[bB][01]+")]
    Literal,

    #[regex(r#"\?|:|!|\^|-|\+|\*|&|/|<|>|="#, priority = 3)]
    Glyph,

    #[regex(r"\.|->")]
    GlyphCtx,

    #[regex("\\{|\\}|\\[|\\]|\\(|\\)")]
    Bracket,

    #[regex("asm|break|case|continue|default|defined|do|else|for|goto|if")]
    #[regex("return|sizeof|static_assert|switch|typeof|typeof_unqual|while")]
    #[regex("_Generic|_Noreturn")]
    Keyword,

    #[regex("#(define|elif|elifdef|elifndef|else|embed|endif|error|if|ifdef)")]
    #[regex("#(ifndef|include|line|pragma|undef|warning)")]
    Macro,

    #[regex("enum|struct|typedef|union")]
    KeywordCtx,

    #[regex("auto|const|extern|inline|register|static|volatile")]
    Qualifier,

    #[regex("bool|char|complex|double|float|imaginary|long")]
    #[regex("short|signed|unsigned|void")]
    Type,

    #[regex("false|NULL|nullptr|true")]
    Constant,

    #[regex("//[^\n]*")]
    #[regex("/\\*([^/]|[^*]/)*\\*/")]
    Comment,

    None,
}

impl Highlight for C {
    const LANG: &'static str = "c";
    const START: Self = Self::None;

    fn kind(tokens: &[Self; 2]) -> Kind {
        use C::*;

        match tokens {
            [KeywordCtx, Identifier] | [GlyphCtx, Identifier] | [_, Type] => {
                Kind::SpecialIdentifier
            }
            [_, Identifier] => Kind::Identifier,
            [_, Literal] => Kind::Literal,
            [_, Glyph] | [_, GlyphCtx] => Kind::Glyph,
            [_, Keyword] | [_, KeywordCtx] | [_, Constant] | [_, Macro] | [_, Qualifier] => {
                Kind::Keyword
            }
            [_, Comment] => Kind::Comment,
            _ => Kind::None,
        }
    }
}
