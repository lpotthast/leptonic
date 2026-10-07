use std::time::Duration;

use leptonic::{
    atoms::prelude::Button,
    utils::{
        clipboard::write_text, live_announcer::announce_polite,
        syntax_highlight::highlight_to_classed_html,
    },
};
use leptos::prelude::*;
use leptos_classes::Classes;

use super::Icon;

/// The language of a code block, naming the syntax it is highlighted with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Css,
    Html,
    Rust,
    Shell,
    /// Not highlighted: leptonic's highlighter has no TOML syntax.
    Toml,
}

impl Language {
    /// The highlighter's name of the language, also the block's `data-language` (the info string of its fence in the
    /// Markdown export).
    const fn token(self) -> &'static str {
        match self {
            Self::Css => "css",
            Self::Html => "html",
            Self::Rust => "rust",
            Self::Shell => "sh",
            Self::Toml => "toml",
        }
    }
}

/// Code in a sentence (`inline=true`) or a code block (styled by `.doc-code` in `_code.scss`).
///
/// A block with a `language` is highlighted (leptonic's `syntax_highlight`), and has a button copying the code.
///
/// The server sends blocks highlighted. The client keeps the server's highlighting when hydrating, and highlights the
/// blocks of pages it renders itself (client-side navigation) each in its own task after rendering them as plain
/// text, so that a page with many blocks shows without waiting for them.
#[component]
pub fn Code(
    /// Code in a sentence: no highlighting, no copy button.
    #[prop(optional)]
    inline: bool,
    #[prop(optional)] language: Option<Language>,
    #[prop(into, optional)] classes: Classes,
    children: TypedChildren<impl Into<Oco<'static, str>>>,
) -> impl IntoView {
    let code: Oco<'static, str> = children.into_inner()().into_inner().into();
    if inline {
        return view! { <code class=classes.add("doc-code") data-inline="true">{code}</code> }
            .into_any();
    }

    let code = StoredValue::new(code.into_owned());
    let html = RwSignal::new(code.with_value(|code| initial_html(code, language)));
    #[cfg(not(feature = "ssr"))]
    if let Some(language) = language
        && !hydrating()
    {
        set_timeout(
            move || {
                if let Some(highlighted) = code
                    .try_with_value(|code| highlight_to_classed_html(code, language.token()))
                    .flatten()
                {
                    // The block may be gone by now (the reader navigated on).
                    let _ = html.try_set(highlighted);
                }
            },
            Duration::ZERO,
        );
    }

    view! {
        <code class=classes.add("doc-code") data-language=language.map(Language::token)>
            // `inner_html` keeps the server's content when hydrating (see `initial_html`).
            <span class="doc-code-text" inner_html=move || html.get()></span>
            <CopyCodeButton code/>
        </code>
    }
    .into_any()
}

/// The block's HTML when it is created: highlighted on the server, plain on the client, which highlights it later or,
/// when hydrating, keeps the server's (`inner_html` doesn't touch the server's content when hydrating).
fn initial_html(code: &str, language: Option<Language>) -> String {
    if cfg!(feature = "ssr")
        && let Some(highlighted) =
            language.and_then(|language| highlight_to_classed_html(code, language.token()))
    {
        return highlighted;
    }
    escape_html(code)
}

/// Whether the client is hydrating the server's HTML.
#[cfg(not(feature = "ssr"))]
fn hydrating() -> bool {
    Owner::current_shared_context().is_some_and(|context| context.during_hydration())
}

/// `text` as HTML text.
fn escape_html(text: &str) -> String {
    let mut html = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => html.push_str("&amp;"),
            '<' => html.push_str("&lt;"),
            '>' => html.push_str("&gt;"),
            c => html.push(c),
        }
    }
    html
}

/// Copies a code block, confirming it (or the failure) with its icon and to screen readers.
#[component]
fn CopyCodeButton(code: StoredValue<String>) -> impl IntoView {
    let copied = RwSignal::new(false);
    let copy = move |_| {
        let text = code.get_value();
        leptos::task::spawn_local(async move {
            let ok = write_text(&text).await.is_ok();
            copied.set(ok);
            announce_polite(if ok {
                "Copied the code"
            } else {
                "Couldn\u{2019}t copy the code"
            });
            set_timeout(move || copied.set(false), Duration::from_secs(2));
        });
    };
    view! {
        <Button on_press=copy aria_label="Copy" classes="doc-code-copy">
            <Icon icon=move || {
                if copied.get() { icondata::VsCheck } else { icondata::VsCopy }
            }/>
        </Button>
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn escapes_html() {
        assert_that!(escape_html("a < b && c > d"))
            .is_equal_to("a &lt; b &amp;&amp; c &gt; d".to_owned());
    }

    #[test]
    fn highlights_the_languages_it_names() {
        for language in [
            Language::Css,
            Language::Html,
            Language::Rust,
            Language::Shell,
        ] {
            assert_that!(highlight_to_classed_html("x", language.token()))
                .with_detail_message(format!("{language:?}"))
                .is_some();
        }
        // Once leptonic's highlighter knows TOML, the comment on `Language::Toml` and the TOML blocks change.
        assert_that!(highlight_to_classed_html("x", Language::Toml.token())).is_none();
    }
}
