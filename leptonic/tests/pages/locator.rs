//! [`Locator`]: the one way to say which elements a lookup means.

use std::fmt;

use leptonic::AriaRole;

/// Which elements a lookup means: a CSS selector (a plain `&str` or `String`) or the elements
/// with an ARIA [`role`]; optionally only those showing a [`text`](Self::text) or containing an
/// element of another locator ([`has`](Self::has)).
///
/// Lookups use typed WebDriver commands. Role matching uses the browser-computed role.
///
/// ```ignore
/// page.element("#test-cb-before")
/// page.element(role(AriaRole::Option).text("Apple"))
/// page.element(role(AriaRole::Row).has(role(AriaRole::Gridcell).text("Inbox")))
/// label.element("input")
/// row.elements("td")
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Locator {
    pub(super) selector: Selector,
    pub(super) text: Option<String>,
    pub(super) has: Vec<Locator>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Selector {
    Css(String),
    Role(AriaRole),
}

/// The elements matching the CSS `selector`. A plain `&str` or `String` is one, too.
pub fn css(selector: impl Into<String>) -> Locator {
    Locator {
        selector: Selector::Css(selector.into()),
        text: None,
        has: Vec::new(),
    }
}

/// Match the role computed by the browser, including native semantics and explicit overrides.
pub fn role(role: AriaRole) -> Locator {
    Locator {
        selector: Selector::Role(role),
        text: None,
        has: Vec::new(),
    }
}

impl Locator {
    /// Only the elements whose text is `text`: their text content with whitespace collapsed, also
    /// while hidden when using a CSS locator. A role locator still follows browser accessibility
    /// visibility. What an element shows is
    /// [`inner_text`](crate::pages::ElementActions::inner_text).
    #[must_use]
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Only the elements containing (below them) an element `inner` matches: the row whose cell
    /// shows a text, `role(AriaRole::Row).has(role(AriaRole::Gridcell).text("Inbox"))`.
    #[must_use]
    pub fn has(mut self, inner: impl Into<Locator>) -> Self {
        self.has.push(inner.into());
        self
    }
}

impl From<&str> for Locator {
    fn from(selector: &str) -> Self {
        css(selector)
    }
}

impl From<String> for Locator {
    fn from(selector: String) -> Self {
        css(selector)
    }
}

impl From<&String> for Locator {
    fn from(selector: &String) -> Self {
        css(selector.as_str())
    }
}

impl fmt::Display for Locator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.selector {
            Selector::Css(selector) => write!(f, "{selector:?}")?,
            Selector::Role(role) => write!(f, "role={}", role.into_str())?,
        }
        if let Some(text) = &self.text {
            write!(f, " with text {text:?}")?;
        }
        for inner in &self.has {
            write!(f, " having ({inner})")?;
        }
        Ok(())
    }
}
