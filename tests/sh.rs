use cmark_syntax::SyntaxPreprocessor;
use pulldown_cmark::{html, Parser};

// Test highlighting rust code
#[test]
fn toml_highlight() {
    static SCRIPT: &str = concat!("```sh\n", include_str!("script.sh"), "```");
    let parser = Parser::new(SCRIPT);
    let processed = SyntaxPreprocessor::new(parser);

    let mut rendered = String::new();
    html::push_html(&mut rendered, processed);
    // let _ = std::fs::write("sh.html", &rendered);
    assert_eq!(rendered, include_str!("sh.html"));
}
