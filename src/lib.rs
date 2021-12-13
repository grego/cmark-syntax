// This file is part of cmark-syntax. This program comes with ABSOLUTELY NO WARRANTY;
// This is free software, and you are welcome to redistribute it under the
// conditions of the GNU General Public License version 3.0.
//
// You should have received a copy of the GNU General Public License
// along with cmark-syntax.  If not, see <http://www.gnu.org/licenses/>
#![doc = include_str!("../README.md")]
use logos::Logos;
use pulldown_cmark::{CodeBlockKind, Event, Tag};

/// Definition of syntaxes of various languages.
pub mod languages;

/// A type of token that can be highlighted.
pub trait Highlight: Sized {
    /// Name of the language of this highlighter.
    const LANG: &'static str;

    /// Determine the kind of a token from the current and the previous token.
    fn kind(tokens: &[Self; 2]) -> Kind;
}

/// Possible kind of a token in the highlighted syntax.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// Not contained in any tags.
    None,
    /// Rendered among `u` tags.
    Glyph,
    /// Rendered among `span` tags.
    Literal,
    /// Rendered among `var` tags.
    Identifier,
    /// Rendered among `em` tags.
    SpecialIdentifier,
    /// Rendered among `b` tags.
    Keyword,
    /// Rendered among `i` tags.
    Comment,
}

static HIGHLIGHT_TAG: [Option<&'static str>; 7] = {
    let mut tags = [None; 7];

    tags[Kind::Glyph as usize] = Some("u");
    tags[Kind::Literal as usize] = Some("span");
    tags[Kind::Identifier as usize] = Some("var");
    tags[Kind::SpecialIdentifier as usize] = Some("em");
    tags[Kind::Keyword as usize] = Some("b");
    tags[Kind::Comment as usize] = Some("i");

    tags
};

/// A preprocessor that highlights syntax in `pulldown_cmark` events.
#[derive(Debug, Default)]
pub struct SyntaxPreprocessor<'a, I: Iterator<Item = Event<'a>>> {
    parent: I,
}

impl<'a, I: Iterator<Item = Event<'a>>> SyntaxPreprocessor<'a, I> {
    /// Create a new syntax preprocessor from `parent`.
    pub fn new(parent: I) -> Self {
        Self { parent }
    }
}

impl<'a, I: Iterator<Item = Event<'a>>> Iterator for SyntaxPreprocessor<'a, I> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let lang = match self.parent.next()? {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => lang,
            other => return Some(other),
        };

        let mut code = String::new();

        for event in &mut self.parent {
            match event {
                Event::Text(text) => code.push_str(&text),
                Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(ref l))) if *l == lang => break,
                other => println!("Unexpected event {:#?}", other),
            }
        }

        let mut html = String::with_capacity(code.len() + code.len() / 4 + 60);

        html.push_str("<pre><code class=\"language-");
        html.push_str(lang.as_ref());
        html.push_str("\">");

        match lang.as_ref() {
            "rust" => highlight::<languages::Rust>(&code, &mut html),
            "js" | "javascript" => highlight::<languages::JavaScript>(&code, &mut html),
            "toml" => highlight::<languages::Toml>(&code, &mut html),
            _ => write_escaped(&mut html, &code),
        }

        html.push_str("</code></pre>");

        Some(Event::Html(html.into()))
    }
}

/// Write with escaping special HTML characters
#[inline]
fn write_escaped(s: &mut String, part: &str) {
    let mut start = 0;

    for (idx, byte) in part.bytes().enumerate() {
        let replace = match byte {
            b'<' => "&lt;",
            b'>' => "&gt;",
            b'&' => "&amp;",
            b'"' => "&quot;",
            _ => continue,
        };
        s.push_str(&part[start..idx]);
        s.push_str(replace);

        start = idx + 1;
    }

    s.push_str(&part[start..]);
}

/// Highlight the code in `source`, placing the output into `buf`.
pub fn highlight<'a, Token>(source: &'a str, buf: &mut String)
where
    Token: Highlight + Logos<'a, Source=str> + Eq + Copy + std::fmt::Debug,
    Token::Extras: Default,
{
    let mut lex = Token::lexer(source);
    let mut open = Kind::None;
    let mut last = 0usize;
    let mut tokens = [Token::ERROR; 2];

    while let Some(token) = lex.next() {
        tokens[0] = tokens[1];
        tokens[1] = dbg!(token);
        dbg!(lex.slice());

        let kind = Token::kind(&tokens);

        if open != kind {
            // Close previous tag
            if let Some(tag) = HIGHLIGHT_TAG[open as usize] {
                buf.push_str("</");
                buf.push_str(tag);
                buf.push('>');
            }

            // Include trivia
            write_escaped(buf, &source[last..lex.span().start]);

            // Open new tag
            if let Some(tag) = HIGHLIGHT_TAG[kind as usize] {
                buf.push('<');
                buf.push_str(tag);
                buf.push('>');
            }

            open = kind;

            write_escaped(buf, lex.slice());
        } else {
            // Include trivia
            write_escaped(buf, &source[last..lex.span().end]);
        }

        last = lex.span().end;
    }

    // Close tail tag
    if let Some(tag) = HIGHLIGHT_TAG[open as usize] {
        buf.push_str("</");
        buf.push_str(tag);
        buf.push('>');
    }
}
