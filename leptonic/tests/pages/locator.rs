//! [`Locator`]: the one way to say which elements a lookup means.

use std::fmt;

use browser_test::thirtyfour::{By, WebElement, extensions::query::ElementQuery, prelude::*};

/// Which elements a lookup means: a CSS selector (a plain `&str` or `String`), elements with an
/// ARIA [`role`], or an [`xpath`] for relations CSS can't express; optionally only those showing a
/// [`text`](Self::text).
///
/// ```ignore
/// page.element("#test-cb-before")
/// page.element(role("option").text("Apple"))
/// label.element("input")
/// row.elements("td")
/// cell.count(xpath("ancestor-or-self::*[@inert]"))
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Locator {
    kind: Kind,
    text: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Kind {
    Css(String),
    Role(String),
    XPath(String),
}

/// The elements matching the CSS `selector`. A plain `&str` or `String` is one, too.
pub fn css(selector: impl Into<String>) -> Locator {
    Locator {
        kind: Kind::Css(selector.into()),
        text: None,
    }
}

/// The elements with the ARIA `role`: explicit (`role="option"`), and implicit for buttons
/// (`<button>`) and links (`<a href>`).
pub fn role(role: impl Into<String>) -> Locator {
    Locator {
        kind: Kind::Role(role.into()),
        text: None,
    }
}

/// The elements an XPath `expression` selects: only for relations CSS can't express (ancestors,
/// following siblings), relative to the element it is used on.
pub fn xpath(expression: impl Into<String>) -> Locator {
    Locator {
        kind: Kind::XPath(expression.into()),
        text: None,
    }
}

impl Locator {
    /// Only the elements whose text is `text`: their text content with whitespace collapsed, also
    /// while hidden (a collapsed panel's button). What an element shows is
    /// [`inner_text`](crate::pages::ElementActions::inner_text).
    #[must_use]
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// A thirtyfour query for these elements, below `root` (a page's session or an element).
    pub(crate) fn query(&self, root: &dyn ElementQueryable) -> ElementQuery {
        let by = match &self.kind {
            Kind::Css(selector) => By::Css(selector.clone()),
            Kind::Role(role) => By::Css(match role.as_str() {
                "button" => "button, [role=button]".to_owned(),
                "link" => "a[href], [role=link]".to_owned(),
                role => format!("[role={role}]"),
            }),
            Kind::XPath(expression) => By::XPath(expression.clone()),
        };
        let query = root.query(by).desc(&self.to_string());
        match &self.text {
            Some(text) => {
                let text = text.clone();
                query.with_filter(move |element: WebElement| {
                    let text = text.clone();
                    async move {
                        // A candidate the page replaced while it was read (a re-rendered list)
                        // doesn't match; the next poll queries again.
                        let Ok(content) = element.prop("textContent").await else {
                            return Ok(false);
                        };
                        let content = content.unwrap_or_default();
                        Ok(content.split_whitespace().collect::<Vec<_>>().join(" ") == text)
                    }
                })
            }
            None => query,
        }
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
        match &self.kind {
            Kind::Css(selector) => write!(f, "{selector:?}")?,
            Kind::Role(role) => write!(f, "role={role}")?,
            Kind::XPath(expression) => write!(f, "xpath {expression:?}")?,
        }
        if let Some(text) = &self.text {
            write!(f, " with text {text:?}")?;
        }
        Ok(())
    }
}
