//! The lookups, shared by pages (below the document) and elements (below the element): one
//! implementation, so that both scopes behave the same.

use browser_test::{
    StepExt,
    thirtyfour::{WebElement, prelude::*},
};
use rootcause::{Report, prelude::ResultExt};

use crate::{
    pages::{ElementActions, Locator},
    polling::{expect, wait_for},
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

pub(crate) async fn wait_for_count(
    root: Root<'_>,
    locator: Locator,
    expected: usize,
) -> Result<(), Report> {
    wait_for(format!("the number of elements matching {locator}"))
        .observing(|| count(root, locator.clone()))
        .to_be_equal_to(expected)
        .await
}

pub(crate) async fn count_stays(
    root: Root<'_>,
    locator: Locator,
    expected: usize,
) -> Result<(), Report> {
    expect(format!("the number of elements matching {locator}"))
        .observing(|| count(root, locator.clone()))
        .to_stay_equal_to(expected)
        .await
}
