//! Syntax highlighting of source code (feature `syntax-highlight`, built on `syntect`): pure
//! Rust, so it works during server-side rendering too.
use std::sync::LazyLock;

use syntect::{
    html::{ClassStyle, ClassedHTMLGenerator},
    parsing::SyntaxSet,
};

static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);

/// Highlights `code`, returning HTML with `<span class="syn-...">` spans (one class per scope
/// part, e.g. `syn-keyword syn-control`) for a stylesheet to color.
///
/// `language` names one of syntect's default syntaxes by name or file extension (`"rust"`, `"rs"`,
/// `"toml"`, `"html"`, ...); the set is syntect's data, not a closed list. Returns `None` if no
/// syntax matches. Insert the result as inner HTML (it escapes `code`).
pub fn highlight_to_classed_html(code: &str, language: &str) -> Option<String> {
    let syntax = SYNTAX_SET.find_syntax_by_token(language)?;
    let mut html_gen = ClassedHTMLGenerator::new_with_class_style(
        syntax,
        &SYNTAX_SET,
        ClassStyle::SpacedPrefixed { prefix: "syn-" },
    );
    for line in code.split_inclusive('\n') {
        html_gen
            .parse_html_for_line_which_includes_newline(line)
            .ok()?;
    }
    Some(html_gen.finalize())
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn highlights_known_languages_and_escapes() {
        let html = highlight_to_classed_html("fn main() { 1 < 2; }", "rust").unwrap();
        assert_that!(html.as_str()).contains("syn-");
        assert_that!(html.as_str()).contains("&lt;");
        assert_that!(highlight_to_classed_html("x", "no-such-language")).is_none();
    }
}
