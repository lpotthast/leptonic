use std::sync::LazyLock;

use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;

static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);

/// Highlights source code, returning HTML with `<span class="syn-...">` spans.
/// Returns `None` if the language is not recognized.
pub(crate) fn highlight_to_classed_html(code: &str, language: &str) -> Option<String> {
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
