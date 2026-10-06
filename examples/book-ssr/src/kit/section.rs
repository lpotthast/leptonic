use leptonic::components::prelude::AnchorLink;
use leptos::{context::Provider, prelude::*};

use super::page::TocRegistry;

/// The enclosing page or section: its heading level (the page title is level 1) and anchor id.
#[derive(Debug, Clone)]
pub(super) struct Parent {
    pub(super) level: u8,
    pub(super) id: Oco<'static, str>,
}

/// A titled part of a [`DocPage`](super::DocPage).
///
/// Renders a linkable heading one level below the enclosing section (`<h2>` directly in a page, `<h3>` in a section,
/// ...) and adds it to the page's table of contents. The anchor id is the slug of the title (`"Hooks Used"` becomes
/// `hooks-used`) unless `id` is given. If the slug is already taken on the page, the parent's id is prepended, so
/// repeated titles like "Input" under different hooks get distinct ids (`use-option-input`).
#[component]
pub fn Section(
    title: &'static str,
    #[prop(optional, into)] id: Option<Oco<'static, str>>,
    children: Children,
) -> impl IntoView {
    let parent = use_context::<Parent>();
    let level = parent.as_ref().map_or(2, |parent| parent.level + 1);
    let toc = use_context::<TocRegistry>();
    let id = id.unwrap_or_else(|| {
        let slug = slug(title);
        match (&parent, toc) {
            (Some(parent), Some(toc)) if toc.contains(&slug) => {
                format!("{}-{slug}", parent.id).into()
            }
            _ => slug.into(),
        }
    });
    if let Some(toc) = toc {
        toc.register(level, id.clone(), title);
    }

    view! {
        <section>
            {heading(level, id.clone(), title)}
            <Provider value=Parent { level, id }>{children()}</Provider>
        </section>
    }
}

/// A heading with a "direct link" anchor, as rendered by pages and sections.
pub(super) fn heading(level: u8, id: Oco<'static, str>, title: &'static str) -> AnyView {
    let link = view! {
        <AnchorLink href=format!("#{id}") description=format!("Direct link to section: {title}")/>
    };
    match level {
        1 => view! { <h1 id=id>{title}{link}</h1> }.into_any(),
        2 => view! { <h2 id=id>{title}{link}</h2> }.into_any(),
        3 => view! { <h3 id=id>{title}{link}</h3> }.into_any(),
        _ => view! { <h4 id=id>{title}{link}</h4> }.into_any(),
    }
}

/// Anchor id for a heading: lowercase ASCII alphanumerics, everything else collapsed into single dashes.
///
/// `"use_button"` becomes `use-button`, `"Hooks, Atoms & Components"` becomes `hooks-atoms-components`.
pub fn slug(title: &str) -> String {
    let mut slug = String::with_capacity(title.len());
    for c in title.chars() {
        if c.is_ascii_alphanumeric() {
            slug.push(c.to_ascii_lowercase());
        } else if !slug.is_empty() && !slug.ends_with('-') {
            slug.push('-');
        }
    }
    if slug.ends_with('-') {
        slug.pop();
    }
    slug
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::slug;

    #[test]
    fn slug_replaces_separators_with_single_dashes() {
        assert_that!(slug("use_button")).is_equal_to("use-button");
        assert_that!(slug("Hooks, Atoms & Components")).is_equal_to("hooks-atoms-components");
        assert_that!(slug("  When to Use?")).is_equal_to("when-to-use");
        assert_that!(slug("Choose Your Layer")).is_equal_to("choose-your-layer");
    }
}
