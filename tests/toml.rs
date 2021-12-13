use pulldown_cmark::{html, Parser};
use cmark_syntax::SyntaxPreprocessor;

// Test highlighting rust code
#[test]
fn toml_highlight() {
    static TOML: &str = concat!("```toml\n", include_str!("test.toml"), "```");
    let parser = Parser::new(TOML);
    let processed = SyntaxPreprocessor::new(parser);
    
    let mut rendered = String::new();
    html::push_html(&mut rendered, processed);
    // let _ = std::fs::write("toml.html", &rendered);
    assert_eq!(rendered, include_str!("toml.html"));
}
