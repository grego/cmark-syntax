#[cfg(feature = "latex2mathml")]
use cmark_syntax::SyntaxPreprocessor;
#[cfg(feature = "latex2mathml")]
use pulldown_cmark::{html, Parser};

// Test highlighting rust code
#[cfg(feature = "latex2mathml")]
#[test]
fn toml_highlight() {
    static RIEMANN: &str = "```math\nR(X, Y)Z = \\nabla_X\\nabla_Y Z - \\nabla_Y \\nabla_X Z - \\nabla_{[X, Y]} Z\n```";
    let parser = Parser::new(RIEMANN);
    let processed = SyntaxPreprocessor::new(parser);

    let mut rendered = String::new();
    html::push_html(&mut rendered, processed);
    //let _ = std::fs::write("math.html", &rendered);
    assert_eq!(rendered, include_str!("math.html"));
}
