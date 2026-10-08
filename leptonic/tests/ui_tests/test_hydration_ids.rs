// No upstream: leptonic-only (react-aria's ids come from React's `useId`; this checks leptonic's
// SSR/hydration id stability).
use std::{borrow::Cow, collections::BTreeSet};

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::{Report, prelude::ResultExt};

use crate::pages::{Page, PageActions};

/// Every element id of the server-rendered HTML must survive hydration, and every id reference
/// (`aria-labelledby`, `aria-controls`, ...) must point at an existing element. Random ids break
/// both: attributes the client updates after hydration would reference ids that only exist on the
/// server's side, or vice versa. The client may add ids the server didn't render (a trigger gets
/// one once a panel needs to reference it).
///
/// Checks every fixture the test app's index page links to, so new fixtures are covered without
/// registering them here. The fixtures are split into `shards` tests (this one checks every
/// `shards`-th, starting at `shard`), which run in parallel.
pub struct HydrationIdTests {
    pub shard: usize,
    pub shards: usize,
}

#[async_trait]
impl BrowserTest<str> for HydrationIdTests {
    fn name(&self) -> Cow<'_, str> {
        format!("hydration_id_tests_{}_of_{}", self.shard + 1, self.shards).into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/").await?;

        cases!(fixtures_keep_their_ids(&page, self.shard, self.shards));

        Ok(())
    }
}

/// Every `shards`-th fixture the index page links to, starting at `shard`, keeps its ids.
async fn fixtures_keep_their_ids(
    page: &Page<'_>,
    shard: usize,
    shards: usize,
) -> Result<(), Report> {
    let fixtures = fixture_paths(page).await?;
    assert_that!(&fixtures)
        .with_detail_message("the index page links to the fixtures")
        .is_not_empty();
    for path in fixtures.iter().skip(shard).step_by(shards) {
        fixture_keeps_its_ids(page, path)
            .await
            .context_with(|| format!("on {path}"))?;
    }
    Ok(())
}

/// The fixture at `path` keeps the server-rendered ids through hydration and references no id
/// that doesn't exist.
async fn fixture_keeps_its_ids(page: &Page<'_>, path: &str) -> Result<(), Report> {
    page.goto_path(path).await?;
    let server_ids = server_rendered_ids(page).await?;
    // `data-client-only` elements (e.g. `use_description`'s hidden descriptions) are created
    // after hydration by design.
    let client_ids: BTreeSet<String> = page
        .eval(
            "return [...document.querySelectorAll('body [id]:not([data-client-only])')].map(e => e.id);",
            vec![],
        )
        .await?;
    let missing: BTreeSet<&String> = server_ids.difference(&client_ids).collect();
    assert_that!(missing)
        .with_detail_message(format!(
            "server-rendered ids missing after hydration on {path}"
        ))
        .is_empty();

    let dangling: Vec<String> = page
        .eval(
            r"
            const attrs = ['aria-labelledby', 'aria-describedby', 'aria-controls', 'for'];
            const dangling = [];
            for (const el of document.querySelectorAll('body *')) {
                for (const attr of attrs) {
                    for (const id of (el.getAttribute(attr) || '').split(/\s+/).filter(Boolean)) {
                        if (!document.getElementById(id)) dangling.push(`${attr}=${id}`);
                    }
                }
            }
            return dangling;
            ",
            vec![],
        )
        .await?;
    assert_that!(dangling)
        .with_detail_message(format!("dangling id references on {path}"))
        .is_empty();
    Ok(())
}

/// The paths of every fixture, in the order the index page lists them.
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
async fn server_rendered_ids(page: &Page<'_>) -> Result<BTreeSet<String>, Report> {
    page.eval(
        r"
        return fetch(location.href)
            .then(response => response.text())
            .then(html => {
                const doc = new DOMParser().parseFromString(html, 'text/html');
                return [...doc.querySelectorAll('body [id]')].map(e => e.id);
            });
        ",
        vec![],
    )
    .await
}
