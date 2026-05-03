use crate::utils::data_attributes::ToStaticStrRepr;
use leptos::prelude::*;

use crate::{
    Out,
    components::prelude::{Button, ButtonVariant, Icon},
    utils::{classes::Classes, styles::Styles},
};

/// Programming language for syntax highlighting in the [`Code`] component.
///
/// Named variants cover syntect's built-in languages plus common extras.
/// Languages without syntect support gracefully fall back to plain text.
/// Use [`Other`](Language::Other) for unlisted languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    // — Syntect defaults (highlighted) —
    ActionScript,
    AppleScript,
    Asp,
    BatchFile,
    C,
    Clojure,
    Cpp,
    CSharp,
    Css,
    D,
    Diff,
    Erlang,
    Go,
    Graphviz,
    Groovy,
    Haskell,
    Html,
    Java,
    JavaScript,
    Json,
    LaTeX,
    Lisp,
    Lua,
    Makefile,
    Markdown,
    Matlab,
    OCaml,
    ObjectiveC,
    Pascal,
    Perl,
    Php,
    Python,
    R,
    RestructuredText,
    Ruby,
    Rust,
    Scala,
    Shell,
    Sql,
    Tcl,
    Textile,
    Toml,
    Xml,
    Yaml,

    // — Common extras (no built-in highlighting, fall back to plain text) —
    Dart,
    Elixir,
    Kotlin,
    Nix,
    Scss,
    Swift,
    TypeScript,
    Zig,

    /// Any language not covered by a named variant.
    Other(&'static str),
}

impl Language {
    /// Returns the token used for syntect lookup and the `data-language` attribute.
    pub fn token(self) -> &'static str {
        match self {
            Self::ActionScript => "actionscript",
            Self::AppleScript => "applescript",
            Self::Asp => "asp",
            Self::BatchFile => "bat",
            Self::C => "c",
            Self::Clojure => "clojure",
            Self::Cpp => "cpp",
            Self::CSharp => "cs",
            Self::Css => "css",
            Self::D => "d",
            Self::Diff => "diff",
            Self::Erlang => "erlang",
            Self::Go => "go",
            Self::Graphviz => "dot",
            Self::Groovy => "groovy",
            Self::Haskell => "hs",
            Self::Html => "html",
            Self::Java => "java",
            Self::JavaScript => "js",
            Self::Json => "json",
            Self::LaTeX => "tex",
            Self::Lisp => "lisp",
            Self::Lua => "lua",
            Self::Makefile => "makefile",
            Self::Markdown => "md",
            Self::Matlab => "matlab",
            Self::OCaml => "ml",
            Self::ObjectiveC => "objective-c",
            Self::Pascal => "pascal",
            Self::Perl => "pl",
            Self::Php => "php",
            Self::Python => "py",
            Self::R => "r",
            Self::RestructuredText => "rst",
            Self::Ruby => "rb",
            Self::Rust => "rust",
            Self::Scala => "scala",
            Self::Shell => "sh",
            Self::Sql => "sql",
            Self::Tcl => "tcl",
            Self::Textile => "textile",
            Self::Toml => "toml",
            Self::Xml => "xml",
            Self::Yaml => "yaml",
            Self::Dart => "dart",
            Self::Elixir => "elixir",
            Self::Kotlin => "kotlin",
            Self::Nix => "nix",
            Self::Scss => "scss",
            Self::Swift => "swift",
            Self::TypeScript => "typescript",
            Self::Zig => "zig",
            Self::Other(s) => s,
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.token())
    }
}

#[derive(Clone)]
#[slot]
pub struct Li {
    children: ChildrenFn,
}

#[component]
pub fn Ul(
    #[prop(default=vec![])] li: Vec<Li>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    view! {
        <ul class=classes style=styles>
            <For
                each=move || { li.clone().into_iter().enumerate() }
                key=move |(index, _e)| *index
                children=move |(_index, e)| view! { <li>{(e.children)()}</li> }
            />
        </ul>
    }
}

/// Renders a code block with optional syntax highlighting.
///
/// ## Syntax highlighting (`syntax-highlight` feature)
///
/// Highlighting is deferred to avoid blocking page rendering. During SSR, no highlighting runs —
/// plain text is sent immediately. On the client, each `Code` block schedules its highlighting in
/// a separate browser macrotask via `set_timeout(Duration::ZERO)`. This lets the browser paint and
/// handle events between highlights, keeping navigation fast even on pages with many code blocks.
/// The view reactively swaps from plain text to highlighted HTML once highlighting completes.
#[component]
pub fn Code(
    #[prop(optional)] inline: Option<bool>,
    #[prop(optional)] language: Option<Language>,
    #[prop(optional)] show_copy_button: Option<bool>,
    #[prop(into, optional)] on_copy: Option<Out<Result<(), ()>>>,
    /// When `false` and the `sanitize` feature is enabled, the highlighted HTML
    /// is sanitized via ammonia before rendering. Defaults to `true`.
    #[prop(optional)]
    trusted: Option<bool>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: TypedChildren<impl Into<Oco<'static, str>>>,
) -> impl IntoView {
    let code = children.into_inner()().into_inner().into();

    let code_text = StoredValue::new(code);

    let is_inline = inline.unwrap_or(false);

    let highlighted_html = RwSignal::new(None::<String>);

    #[cfg(all(feature = "syntax-highlight", not(feature = "ssr")))]
    {
        if !is_inline {
            if let Some(lang) = language {
                let token = lang.token();
                // Defer highlighting to a separate browser macrotask.
                // Each Code block highlights in its own macrotask, allowing the
                // browser to paint and handle events between them.
                set_timeout(
                    move || {
                        highlighted_html.set(
                            crate::utils::syntax_highlight::highlight_to_classed_html(
                                code_text.get_value().as_str(),
                                token,
                            ),
                        );
                    },
                    std::time::Duration::ZERO,
                );
            }
        }
    }

    let show_copy_button = show_copy_button.unwrap_or(!is_inline);
    let on_success = Callback::new(move |()| {
        if let Some(on_copy) = on_copy {
            on_copy.set(Ok(()));
        }
    });
    let on_err = Callback::new(move |()| {
        if let Some(on_copy) = on_copy {
            on_copy.set(Err(()));
        } else {
            tracing::warn!("copy to clipboard failed");
        }
    });

    let copy_btn = show_copy_button.then(|| {
        view! {
            <Button
                classes="leptonic-code-copy-button"
                variant=ButtonVariant::Flat
                on_press=move |_| {
                    let text = code_text.get_value();
                    copy_to_clipboard(text.as_str(), on_success, on_err);
                }
            >
                <Icon icon=icondata::VsCopy />
            </Button>
        }
    });

    #[cfg(not(feature = "sanitize"))]
    let _ = trusted;

    let code_text_view = move || {
        if let Some(html) = highlighted_html.get() {
            #[cfg(feature = "sanitize")]
            let html = if trusted.unwrap_or(true) {
                html
            } else {
                ammonia::clean(&html)
            };

            leptos::html::span()
                .class("leptonic-code-text")
                .attr("data-inline", inline.map(|it| it.to_static_str_repr()))
                // SAFETY (when sanitize feature disabled): `html` is generated by
                // syntect's ClassedHTMLGenerator. Trusted library output, not user input.
                .inner_html(html)
                .into_any()
        } else {
            view! {
                <span class="leptonic-code-text" data-inline=inline.map(|it| it.to_static_str_repr())>
                    {code_text.get_value()}
                </span>
            }
            .into_any()
        }
    };

    view! {
        <code class=classes.add("leptonic-code") style=styles data-inline=inline.map(|it| it.to_static_str_repr()) data-language=language.map(Language::token)>
            {code_text_view}
            {copy_btn}
        </code>
    }
}

#[cfg(feature = "clipboard")]
fn copy_to_clipboard(text: &str, on_success: Callback<(), ()>, on_err: Callback<(), ()>) {
    match leptos_use::use_window().navigator() {
        Some(navigator) => {
            let promise = navigator.clipboard().write_text(text);
            let future = wasm_bindgen_futures::JsFuture::from(promise);
            wasm_bindgen_futures::spawn_local(async move {
                match future.await {
                    Ok(_result) => {
                        on_success.run(());
                    }
                    Err(_err) => {
                        on_err.run(());
                    }
                }
            });
        }
        None => {
            on_err.run(());
        }
    }
}

#[cfg(not(feature = "clipboard"))]
fn copy_to_clipboard(_text: &str, _on_success: Callback<()>, _on_err: Callback<()>) {
    tracing::warn!(
        "Clipboard related functionality requires leptonic's 'Clipboard' feature as well as '--cfg=web_sys_unstable_apis'."
    );
}
