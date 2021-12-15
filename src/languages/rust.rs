use crate::{Highlight, Kind};
use logos::Logos;

#[derive(Logos, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Rust {
    #[error]
    Error,

    #[regex("[a-z_$][a-zA-Z0-9_]*!?")]
    #[regex("[A-Z_]*!?", priority = 3)]
    Identifier,

    #[regex("[A-Z][a-zA-Z0-9_]*!?")]
    StrongIdentifier,

    #[regex("\"([^\"\\n]|\\\\[\"\\n])*\"")]
    #[regex("'([^']|\\\\[^'\\n \\t]*)'")]
    #[regex("r\"[^\"]*\"")]
    #[regex("r#\"#?([^#]|[^\"]#)*\"#")]
    #[regex("b\"([^\"\\n]|\\\\[\"\\n])*\"")]
    #[regex("b'([^']|\\\\[^'\\n \\t]*)'")]
    #[regex("br\"[^\"]*\"")]
    #[regex("br#\"#?([^#]|[^\"]#)*\"#")]
    #[regex("[0-9][0-9_]*")]
    #[regex("0[xX][0-9a-fA-F_]+")]
    #[regex("0[oO][0-7_]+")]
    #[regex("0[bB][01_]+")]
    Literal,

    #[regex(r#"\?|!|\^|-|\+|\*|&|/|\\|<|>|=|=>|->|_|#\[[^\]]*\]|&"#)]
    Glyph,

    #[regex("\\.|:", priority = 3)]
    GlyphCtx,
    
    #[regex("\\{|\\}|\\[|\\]|\\(|\\)")]
    Bracket,

    #[regex("'[a-zA-Z_][a-zA-Z0-9_]*")]
    Lifetime,

    #[regex("as|break|const|continue|crate|dyn|else|extern")]
    #[regex("false|for|if|impl|in|let|loop|match|mod|move|mut")]
    #[regex("pub|ref|return|self|Self|static|super")]
    #[regex("true|unsafe|use|where|while")]
    #[regex("abstract|async|await|become|box|do|final|macro")]
    #[regex("override|priv|try|typeof|unsized|virtual|yield")]
    Keyword,

    #[regex("fn|enum|struct|type|trait")]
    KeywordCtx,

    #[regex("str|bool|[ui](8|16|32|64|size)|f32|f64")]
    Special,

    #[regex("//[^\n]*")]
    #[regex("/\\*([^/]|[^*]/)*\\*/")]
    Comment,
}

impl Highlight for Rust {
    const LANG: &'static str = "rust";

    fn kind(tokens: &[Self; 2]) -> Kind {
        use Rust::*;

        match tokens {
            [KeywordCtx, StrongIdentifier]
            | [KeywordCtx, Identifier]
            | [GlyphCtx, Identifier]
            | [_, Lifetime] => Kind::SpecialIdentifier,
            [_, Identifier] => Kind::Identifier,
            [_, StrongIdentifier] | [_, Special] => Kind::StrongIdentifier,
            [_, Literal] => Kind::Literal,
            [_, Glyph] | [_, GlyphCtx] => Kind::Glyph,
            [_, Keyword] | [_, KeywordCtx] => Kind::Keyword,
            [_, Comment] => Kind::Comment,
            _ => Kind::None,
        }
    }
}
