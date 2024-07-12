#[cfg(feature = "latex2mathml")]
use cmark_syntax::SyntaxPreprocessor;
#[cfg(feature = "latex2mathml")]
use pulldown_cmark::{html, Parser, Options};

// Test highlighting rust code
#[cfg(feature = "latex2mathml")]
#[test]
fn math_highlight() {
    static RIEMANN: &str = "casual $\\sum_{n=0}^\\infty \\frac{1}{n!}$ text\n$$R(X, Y)Z = \\nabla_X\\nabla_Y Z - \\nabla_Y \\nabla_X Z - \\nabla_{[X, Y]} Z$$";
    let parser = Parser::new_ext(RIEMANN, Options::ENABLE_MATH);
    let processed = SyntaxPreprocessor::new(parser);

    let mut rendered = String::new();
    html::push_html(&mut rendered, processed);
    // let _ = std::fs::write("math.html", &rendered);
    assert_eq!(rendered, include_str!("math.html"));
}
