//! The lookups, shared by pages (below the document) and elements (below the element): one
//! implementation, so that both scopes behave the same.
//!
//! One script in the page finds the candidates of a lookup, already filtered by their text (one
//! round trip, whatever the page's size); roles come from the browser (WebDriver's computed
//! role, per remaining candidate), so the helper does not duplicate ARIA semantics. Descendant
//! filters compose in Rust.

use std::{sync::Arc, time::Duration};

use assertr::{matchers::eq, pattern, prelude::*};
use browser_test::{
    StepExt,
    thirtyfour::{
        error::{WebDriverError, WebDriverErrorInner},
        session::handle::SessionHandle,
    },
};
use rootcause::{Report, prelude::ResultExt};

use crate::pages::{ElementActions, Locator, WebElement, health, locator::Selector};

#[derive(Debug)]
struct CandidateChanged;
impl std::fmt::Display for CandidateChanged {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("a candidate was replaced during lookup; resolve the locator again")
    }
}
impl std::error::Error for CandidateChanged {}

fn candidate_error(error: Report) -> Report {
    let stale = error.iter_reports().any(|report| {
        report
            .downcast_current_context::<WebDriverError>()
            .is_some_and(|error| {
                matches!(
                    error.as_inner(),
                    WebDriverErrorInner::StaleElementReference(_)
                )
            })
    });
    if stale {
        error.context(CandidateChanged).into_dynamic()
    } else {
        error
    }
}

pub(crate) fn terminal_lookup_error(error: &Report) -> bool {
    !error.iter_reports().any(|report| {
        report
            .downcast_current_context::<CandidateChanged>()
            .is_some()
    })
}

/// What lookups search below: the document of a page's session, or an element.
#[derive(Clone, Copy)]
pub(crate) enum Root<'a> {
    Page(&'a Arc<SessionHandle>),
    WebElement(&'a WebElement),
}

impl Root<'_> {
    fn handle(&self) -> &Arc<SessionHandle> {
        match self {
            Root::Page(handle) => handle,
            Root::WebElement(element) => element.handle(),
        }
    }
}

/// Resolve exactly one node, retrying absence or candidates replaced mid-query.
/// Ambiguity, stale fixed roots and other protocol errors fail immediately.
#[track_caller]
pub(crate) fn element(
    root: Root<'_>,
    locator: Locator,
) -> impl Future<Output = Result<WebElement, Report>> {
    resolve(root, locator, false)
}

#[track_caller]
pub(crate) fn first(
    root: Root<'_>,
    locator: Locator,
) -> impl Future<Output = Result<WebElement, Report>> {
    resolve(root, locator, true)
}

#[track_caller]
fn resolve(
    root: Root<'_>,
    locator: Locator,
    first: bool,
) -> impl Future<Output = Result<WebElement, Report>> {
    let subject = format!("element matching {locator}");
    let check = assert_that_owned!(move || {
        let locator = locator.clone();
        async move {
            health::expect_no_panic(root.handle()).await?;
            let found = elements(root, locator.clone()).await?;
            if !first && found.len() > 1 {
                rootcause::bail!(
                    "{locator} matches {} elements; narrow the locator or use first_element explicitly",
                    found.len()
                );
            }
            Ok::<_, Report>(found.into_iter().next())
        }
    })
    .with_subject_name(subject)
    .eventually_ok()
    .giving_up_on(terminal_lookup_error)
    .try_matches(pattern!(Some(_)));
    async move {
        check
            .await?
            .ok_or_else(|| rootcause::report!("element disappeared"))
    }
    .step("find")
}

/// The elements below the root (`arguments[0]`, `null`: the document) matching the CSS selector
/// `arguments[1]`, in document order; with a text (`arguments[2]`), only those whose text content,
/// whitespace collapsed, is it.
const CANDIDATES: &str = "const [root, selector, text] = arguments;
    const candidates = [...(root ?? document).querySelectorAll(selector)];
    if (text === null) return candidates;
    return candidates.filter(element =>
        element.textContent.trim().split(/\\s+/).join(' ') === text);";

pub(crate) async fn elements(root: Root<'_>, locator: Locator) -> Result<Vec<WebElement>, Report> {
    let selector = match &locator.selector {
        Selector::Css(css) => css.as_str(),
        Selector::Role(_) => "*",
    };
    let root_argument = match root {
        Root::Page(_) => serde_json::Value::Null,
        Root::WebElement(element) => element.to_json()?,
    };
    // One round trip: a page of thousands of elements would cost thousands of WebDriver
    // commands, one per candidate.
    let candidates = root
        .handle()
        .execute(
            CANDIDATES,
            vec![root_argument, selector.into(), locator.text.clone().into()],
        )
        .await
        .context_with(|| format!("failed to find the candidates of {locator}"))?
        .elements()?;
    let mut found = Vec::new();
    for element in candidates {
        let matches = async {
            if let Selector::Role(role) = &locator.selector
                && element.accessible_role().await? != role.into_str()
            {
                return Ok(false);
            }
            let mut matches = true;
            for inner in &locator.has {
                if Box::pin(elements(Root::WebElement(&element), inner.clone()))
                    .await?
                    .is_empty()
                {
                    matches = false;
                    break;
                }
            }
            Ok::<_, Report>(matches)
        }
        .await
        .map_err(candidate_error)?;
        if matches {
            found.push(element);
        }
    }
    Ok(found)
}

pub(crate) async fn count(root: Root<'_>, locator: Locator) -> Result<usize, Report> {
    Ok(elements(root, locator).await?.len())
}

pub(crate) async fn inner_texts(root: Root<'_>, locator: Locator) -> Result<Vec<String>, Report> {
    let mut texts = Vec::new();
    for element in elements(root, locator).await? {
        texts.push(element.inner_text().await.map_err(candidate_error)?);
    }
    Ok(texts)
}

#[track_caller]
pub(crate) fn wait_for_count(
    root: Root<'_>,
    locator: Locator,
    expected: usize,
) -> impl Future<Output = Result<(), Report>> {
    let subject = format!("count of {locator}");
    let check = assert_that_owned!(move || {
        let locator = locator.clone();
        async move {
            health::expect_no_panic(root.handle()).await?;
            count(root, locator.clone()).await
        }
    })
    .with_subject_name(subject)
    .eventually_ok()
    .giving_up_on(terminal_lookup_error)
    .try_matches(eq(expected));
    async move {
        check.await?;
        Ok(())
    }
    .step("wait_for_count")
}

#[track_caller]
pub(crate) fn count_stays(
    root: Root<'_>,
    locator: Locator,
    expected: usize,
    duration: Duration,
) -> impl Future<Output = Result<(), Report>> {
    let subject = format!("count of {locator}");
    let check = assert_that_owned!(move || {
        let locator = locator.clone();
        async move {
            health::expect_no_panic(root.handle()).await?;
            count(root, locator.clone()).await
        }
    })
    .with_subject_name(subject)
    .consistently_ok()
    .for_at_least(duration)
    .try_matches(eq(expected));
    async move {
        if duration.is_zero() {
            rootcause::bail!("stability checks require a positive duration");
        }
        check.await?;
        Ok(())
    }
    .step("count_stays")
}

/// Re-resolve the locator on every read, allowing a component to replace its DOM node.
#[track_caller]
pub(crate) fn wait_for_attr<'a>(
    root: Root<'a>,
    locator: Locator,
    name: &'a str,
    expected: Option<&'a str>,
) -> impl Future<Output = Result<(), Report>> + 'a {
    let subject = format!("{locator} attribute {name}");
    let check = assert_that_owned!(move || {
        let locator = locator.clone();
        async move {
            health::expect_no_panic(root.handle()).await?;
            let elements = elements(root, locator.clone()).await?;
            match elements.as_slice() {
                [] => Ok::<_, Report>(None),
                [element] => Ok(Some(
                    element
                        .attr(name)
                        .await
                        .map_err(|error| candidate_error(error.into()))?,
                )),
                _ => rootcause::bail!("{locator} is ambiguous: {} elements", elements.len()),
            }
        }
    })
    .with_subject_name(subject)
    .eventually_ok()
    .giving_up_on(terminal_lookup_error)
    .try_matches(eq(Some(expected.map(str::to_owned))));
    async move {
        check.await?;
        Ok(())
    }
    .step("wait_for_locator_attr")
}
