use cmark_syntax::SyntaxPreprocessor;
use pulldown_cmark::{html, Parser};

// Test highlighting rust code
#[test]
fn rust_highlight() {
    static RUST_CODE: &str = concat!("```rust\n", include_str!("rust.rs"), "```");
    let parser = Parser::new(RUST_CODE);
    let processed = SyntaxPreprocessor::new(parser);

    let mut rendered = String::new();
    html::push_html(&mut rendered, processed);
    // let _ = std::fs::write("rust.html", &rendered);
    assert_eq!(rendered, include_str!("rust.html"));
}
