use leptos::{html, prelude::*};
use leptos_classes::Classes;
use leptos_styles::Styles;

/// Sanitizes an HTML string using the given configurator or `ammonia`'s defaults.
///
/// Requires the `sanitize` feature.
pub fn sanitize(
    raw: &str,
    configure: Option<&dyn Fn(ammonia::Builder<'static>) -> ammonia::Builder<'static>>,
) -> String {
    let builder = match configure {
        Some(f) => f(ammonia::Builder::default()),
        None => ammonia::Builder::default(),
    };
    builder.clean(raw).to_string()
}

/// Renders sanitized HTML content.
///
/// All input is sanitized via [ammonia](https://docs.rs/ammonia) before rendering
/// to prevent XSS and other injection attacks.
///
/// Requires the `sanitize` feature.
#[component]
pub fn SanitizedHtml(
    /// The raw HTML string to sanitize and render.
    #[prop(into)]
    html: Signal<String>,

    /// Optional policy configurator. Receives ammonia's default `Builder` and
    /// returns a customized one. Called each time `html` changes.
    /// If not provided, ammonia's conservative defaults are used.
    #[prop(into, optional)]
    configure: Option<Callback<ammonia::Builder<'static>, ammonia::Builder<'static>>>,

    #[prop(into, optional)] classes: Classes,

    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let sanitized = Memo::new(move |_| {
        let raw = html.get();
        match configure {
            Some(cb) => {
                let f = move |b| cb.run(b);
                sanitize(&raw, Some(&f))
            }
            None => sanitize(&raw, None),
        }
    });

    let classes = classes.clone();
    let styles = styles.clone();
    move || {
        html::div()
            .class(classes.clone())
            .style(styles.clone())
            .inner_html(sanitized.get())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertr::prelude::*;

    #[test]
    fn strips_script_tags() {
        let result = sanitize("<b>Hello</b><script>alert('xss')</script>", None);
        assert_that(result).is_equal_to("<b>Hello</b>".to_string());
    }

    #[test]
    fn strips_event_handlers() {
        let result = sanitize(r#"<div onclick="alert('xss')">Click me</div>"#, None);
        assert_that(result.as_str()).does_not_contain("onclick");
        assert_that(result.as_str()).is_equal_to("Click me");
    }

    #[test]
    fn strips_javascript_urls() {
        let result = sanitize(r#"<a href="javascript:alert('xss')">Link</a>"#, None);
        assert_that(result).does_not_contain("javascript");
    }

    #[test]
    fn preserves_safe_html() {
        let result = sanitize(
            r#"<p>Hello <b>world</b> and <a href="https://example.com">link</a></p>"#,
            None,
        );
        // Ammonia adds rel="noopener noreferrer" to links by default.
        assert_that(result).is_equal_to(
            r#"<p>Hello <b>world</b> and <a href="https://example.com" rel="noopener noreferrer">link</a></p>"#.to_string()
        );
    }

    #[test]
    fn strips_iframe() {
        let result = sanitize(r#"<iframe src="https://evil.com"></iframe>"#, None);
        assert_that(result).does_not_contain("iframe");
    }

    #[test]
    fn custom_policy_allows_extra_tags() {
        let input = "<custom-tag>content</custom-tag>";
        let result_default = sanitize(input, None);
        assert_that(result_default).is_equal_to("content".to_string());

        let result_custom = sanitize(
            input,
            Some(&|mut builder| {
                builder.add_tags(&["custom-tag"]);
                builder
            }),
        );
        assert_that(result_custom).is_equal_to(input.to_string());
    }
}
