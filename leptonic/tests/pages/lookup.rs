//! The lookups, shared by pages (below the document) and elements (below the element): one
//! implementation, so that both scopes behave the same.
//!
//! Native WebDriver queries find candidates. Text and descendant filters compose in Rust;
//! roles come from the browser, so the helper does not duplicate ARIA semantics.

use std::{sync::Arc, time::Duration};

use assertr::{matchers::eq, pattern, prelude::*};
use browser_test::{
    StepExt,
    thirtyfour::{
        By,
        error::{WebDriverError, WebDriverErrorInner},
        session::handle::SessionHandle,
    },
};
use rootcause::Report;

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

pub(crate) async fn elements(root: Root<'_>, locator: Locator) -> Result<Vec<WebElement>, Report> {
    let selector = match &locator.selector {
        Selector::Css(css) => css.as_str(),
        Selector::Role(_) => "*",
    };
    let candidates = match root {
        Root::Page(session) => session.find_all(By::Css(selector)).await?,
        Root::WebElement(element) => element.find_all(By::Css(selector)).await?,
    };
    let mut found = Vec::new();
    for element in candidates {
        let matches = async {
            if let Selector::Role(role) = &locator.selector
                && element.accessible_role().await? != role.into_str()
            {
                return Ok(false);
            }
            if let Some(text) = &locator.text {
                let actual = element.prop("textContent").await?.unwrap_or_default();
                if actual.split_whitespace().collect::<Vec<_>>().join(" ") != *text {
                    return Ok(false);
                }
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
