//! The lookups, shared by pages (below the document) and elements (below the element): one
//! implementation, so that both scopes behave the same.

use std::sync::Arc;

use assertr::{matchers::eq, prelude::*};
use browser_test::{
    StepExt,
    thirtyfour::{WebElement, prelude::*, session::handle::SessionHandle},
};
use rootcause::{Report, prelude::ResultExt};

use crate::{
    pages::{ElementActions, Locator},
    timing,
};

/// What lookups search below: a page's session or an element.
pub(crate) type Root<'a> = &'a (dyn ElementQueryable + Send + Sync);

pub(crate) async fn element(root: Root<'_>, locator: Locator) -> Result<WebElement, Report> {
    let detail = locator.to_string();
    async {
        Ok(locator
            .query(root)
            .first()
            .await
            .context_with(|| format!("no element matches {locator}"))?)
    }
    .step("find")
    .detail(detail)
    .await
}

pub(crate) async fn elements(root: Root<'_>, locator: Locator) -> Result<Vec<WebElement>, Report> {
    Ok(locator
        .query(root)
        .nowait()
        .any()
        .await
        .context_with(|| format!("failed to look up {locator}"))?)
}

pub(crate) async fn count(root: Root<'_>, locator: Locator) -> Result<usize, Report> {
    Ok(elements(root, locator).await?.len())
}

pub(crate) async fn inner_texts(root: Root<'_>, locator: Locator) -> Result<Vec<String>, Report> {
    let mut texts = Vec::new();
    for element in elements(root, locator).await? {
        texts.push(element.inner_text().await?);
    }
    Ok(texts)
}

/// Builds the assertion when called (`#[track_caller]`): a failure names the test's line.
#[track_caller]
pub(crate) fn wait_for_count(
    root: Root<'_>,
    locator: Locator,
    expected: usize,
) -> impl Future<Output = Result<(), Report>> {
    let subject = format!("the number of elements matching {locator}");
    let check = assert_that_owned!(move || count(root, locator.clone()))
        .with_subject_name(subject)
        .eventually_ok()
        .matches(eq(expected));
    async move {
        check.await;
        Ok(())
    }
}

/// Builds the assertion when called (`#[track_caller]`): a failure names the test's line.
#[track_caller]
pub(crate) fn count_stays<'a>(
    session: &'a Arc<SessionHandle>,
    root: Root<'a>,
    locator: Locator,
    expected: usize,
) -> impl Future<Output = Result<(), Report>> + 'a {
    let subject = format!("the number of elements matching {locator}");
    let check = assert_that_owned!(move || count(root, locator.clone()))
        .with_subject_name(subject)
        .consistently_ok()
        .matches(eq(expected));
    async move {
        timing::settle(session).await?;
        check.await;
        Ok(())
    }
}
