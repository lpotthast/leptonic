// No upstream: leptonic-only (react-aria's ids come from React's `useId`; this checks leptonic's
// SSR/hydration id stability).
use std::{borrow::Cow, collections::BTreeSet};

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::WebDriver};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

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
        let fixtures = fixture_paths(&page).await?;
        assert_that!(fixtures.len())
            .with_detail_message("the index page links to the fixtures")
            .is_greater_than(0);
        for path in fixtures.iter().skip(self.shard).step_by(self.shards) {
            page.goto_path(path).await?;
            let server_ids = server_rendered_ids(&page).await?;
            let client_ids = string_set(
                &page,
                // `data-client-only` elements (e.g. `use_description`'s hidden descriptions) are
                // created after hydration by design.
                "return [...document.querySelectorAll('body [id]:not([data-client-only])')].map(e => e.id);",
            )
            .await?;
            let missing: BTreeSet<&String> = server_ids.difference(&client_ids).collect();
            assert_that!(missing)
                .with_detail_message(format!(
                    "server-rendered ids missing after hydration on {path}"
                ))
                .is_empty();

            let dangling = string_set(
                &page,
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
            )
            .await?;
            assert_that!(dangling)
                .with_detail_message(format!("dangling id references on {path}"))
                .is_empty();
        }
        Ok(())
    }
}

/// The paths of every fixture, in the order the index page lists them.
async fn fixture_paths(page: &Page<'_>) -> Result<Vec<String>, Report> {
    let result = page
        .driver
        .execute(
            r#"return [...document.querySelectorAll('a[href^="/atoms/"], a[href^="/hooks/"]')]
                .map(a => a.getAttribute('href'));"#,
            vec![],
        )
        .await?;
    Ok(result.convert()?)
}

/// The ids in the HTML the server sends for the current page, before any client code runs.
async fn server_rendered_ids(page: &Page<'_>) -> Result<BTreeSet<String>, Report> {
    let result = page
        .driver
        .execute_async(
            r"
            const done = arguments[arguments.length - 1];
            fetch(location.href)
                .then(response => response.text())
                .then(html => {
                    const doc = new DOMParser().parseFromString(html, 'text/html');
                    done([...doc.querySelectorAll('body [id]')].map(e => e.id));
                })
                .catch(error => done({ error: String(error) }));
            ",
            vec![],
        )
        .await?;
    Ok(result.convert()?)
}

async fn string_set(page: &Page<'_>, script: &str) -> Result<BTreeSet<String>, Report> {
    let result = page.driver.execute(script, vec![]).await?;
    Ok(result.convert()?)
}
