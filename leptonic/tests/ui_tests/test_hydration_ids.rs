use std::{borrow::Cow, collections::BTreeSet};

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::WebDriver};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// Pages whose hooks generate element ids. Every page with id-generating hooks should be listed.
const PAGES: &[&str] = &[
    "/hooks/button",
    "/hooks/menu-trigger",
    "/hooks/number-field",
    "/atoms/listbox",
    "/atoms/select",
];

/// Element ids must be identical in the server-rendered HTML and in the hydrated page, and every
/// id reference (`aria-labelledby`, `aria-controls`, ...) must point at an existing element.
/// Random ids break both: attributes the client updates after hydration would reference ids
/// that only exist on the server's side, or vice versa.
pub struct HydrationIdTests {}

#[async_trait]
impl BrowserTest<str> for HydrationIdTests {
    fn name(&self) -> Cow<'_, str> {
        "hydration_id_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        for path in PAGES {
            page.goto_path(path).await?;
            let server_ids = server_rendered_ids(&page).await?;
            let client_ids = string_set(
                &page,
                "return [...document.querySelectorAll('body [id]')].map(e => e.id);",
            )
            .await?;
            assert_that!(client_ids)
                .with_detail_message(format!("ids on {path}"))
                .is_equal_to(server_ids);

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
