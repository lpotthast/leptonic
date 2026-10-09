// No upstream: leptonic-only (react-aria's ids come from React's `useId`; this checks leptonic's
// SSR/hydration id stability).
use std::{borrow::Cow, collections::BTreeMap, fmt};

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::{Report, bail, prelude::ResultExt};
use serde::Deserialize;

use crate::pages::Page;

/// Every element id of the server-rendered HTML must survive hydration on the same kind of element
/// (tag and role), and every fixture's initial render must be healthy (no duplicate ids, no id
/// reference to a missing element, no warnings; `::expect_no_page_errors`). Random ids
/// break both: attributes the client updates after hydration would reference ids that only exist
/// on the server's side, or vice versa. The client may add ids the server didn't render (a trigger
/// gets one once a panel needs to reference it).
///
/// Checks every fixture the test app's index page links to (and the query variants it lists), so
/// new fixtures are covered without registering them here. The fixtures are split into `shards`
/// tests (this one checks every `shards`-th, starting at `shard`), which run in parallel. A shard
/// checks all its fixtures and reports every failing one.
pub struct HydrationIdTests {
    pub shard: usize,
    pub shards: usize,
}

#[async_trait]
impl<'page> BrowserTest<Page<'page>> for HydrationIdTests {
    fn name(&self) -> Cow<'_, str> {
        format!("hydration_id_tests_{}_of_{}", self.shard + 1, self.shards).into()
    }

    async fn run(&self, _driver: &WebDriver, page: &Page<'page>) -> Result<(), Report> {
        page.goto_path("/").await?;
        fixtures_keep_their_ids(page, self.shard, self.shards).await
    }
}

/// Every `shards`-th fixture the index page links to, starting at `shard`, keeps its ids and is
/// healthy. Fails listing every fixture that isn't.
async fn fixtures_keep_their_ids(
    page: &Page<'_>,
    shard: usize,
    shards: usize,
) -> Result<(), Report> {
    let fixtures = fixture_paths(page).await?;
    assert_that!(&fixtures)
        .with_detail_message("the index page links to the fixtures")
        .is_not_empty();
    let mut failures = Vec::new();
    let mut checked = 0;
    for path in fixtures.iter().skip(shard).step_by(shards) {
        checked += 1;
        if let Err(failure) = fixture_keeps_its_ids(page, path).await {
            failures.push(format!("{path}: {failure}"));
            leave_unchecked(page).await?;
        }
    }
    if !failures.is_empty() {
        bail!(
            "{} of {checked} fixtures failed:\n\n{}",
            failures.len(),
            failures.join("\n\n")
        );
    }
    Ok(())
}

/// Leave a page that failed its checks without checking it again (`goto_path` checks the page it
/// leaves), so that the next fixture's check reports only its own problems.
async fn leave_unchecked(page: &Page<'_>) -> Result<(), Report> {
    page.low_level()
        .driver()
        .goto("about:blank")
        .await
        .context("failed to leave the page")?;
    Ok(())
}

/// The fixture at `path` keeps the server-rendered ids through hydration, on elements of the same
/// tag and role, and is healthy after hydration.
async fn fixture_keeps_its_ids(page: &Page<'_>, path: &str) -> Result<(), Report> {
    page.goto_path(path).await?;
    let server = server_rendered_ids(page).await?;
    // `data-client-only` elements (e.g. `use_description`'s hidden descriptions) are created
    // after hydration by design.
    let client: BTreeMap<String, Element> = page
        .low_level()
        .eval(
            &format!("{IDS} return ids(document, 'body [id]:not([data-client-only])');"),
            vec![],
        )
        .await?;
    let mut problems = Vec::new();
    for (id, on_server) in &server {
        match client.get(id) {
            None => problems.push(format!("#{id} ({on_server}) is missing after hydration")),
            Some(on_client) if on_client != on_server => problems.push(format!(
                "#{id} moved from {on_server} (server) to {on_client} (client)"
            )),
            Some(_) => {}
        }
    }
    if !problems.is_empty() {
        bail!("{}", problems.join("\n"));
    }
    crate::fixtures::check_health(page).await
}

/// The kind of element an id is on: its tag and explicit role.
#[derive(Debug, PartialEq, Eq, Deserialize)]
struct Element {
    tag: String,
    role: Option<String>,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.role {
            Some(role) => write!(f, "<{} role={role:?}>", self.tag),
            None => write!(f, "<{}>", self.tag),
        }
    }
}

/// Defines `ids(root, selector)`: the [`Element`] of every id among the elements matching
/// `selector` below `root` (the last one per id; duplicate ids are the page check's).
const IDS: &str = "const ids = (root, selector) => Object.fromEntries(
    [...root.querySelectorAll(selector)].map(element =>
        [element.id, { tag: element.tagName.toLowerCase(), role: element.getAttribute('role') }]));";

/// The paths of every fixture and fixture variant, in the order the index page lists them.
async fn fixture_paths(page: &Page<'_>) -> Result<Vec<String>, Report> {
    let mut paths = Vec::new();
    for link in page
        .elements(r#"a[href^="/atoms/"], a[href^="/hooks/"]"#)
        .await?
    {
        paths.extend(link.attr("href").await?);
    }
    Ok(paths)
}

/// The ids in the HTML the server sends for the current page, before any client code runs. The
/// script's promise is awaited by WebDriver.
async fn server_rendered_ids(page: &Page<'_>) -> Result<BTreeMap<String, Element>, Report> {
    page.low_level().eval(
        &format!(
            "{IDS}
            return fetch(location.href)
                .then(response => response.text())
                .then(html => ids(new DOMParser().parseFromString(html, 'text/html'), 'body [id]'));"
        ),
        vec![],
    )
    .await
}
