//! Fixture-specific setup and adapters. Browser primitives do not depend on fixture conventions.
pub mod clipboard;
pub mod dnd;
pub mod focus_manager;
pub mod virtual_list;
use assertr::assertions::Patience;
use browser_test::StepExt;
use rootcause::{Report, bail, prelude::ResultExt};

use crate::pages::{Page, health};
mod expectations;
use expectations::{check, check_initial};
pub use expectations::{check as check_health, take_warnings};
/// Resolves with `true` as soon as `<body>` has `data-hydrated` (the test-app sets it once
/// hydration finished), or with `false` after `arguments[0]` milliseconds: a wait in the page,
/// without polling.
const WAIT_FOR_HYDRATION: &str = "const [timeout, done] = arguments;
    const hydrated = () => document.body?.hasAttribute('data-hydrated') ?? false;
    if (hydrated()) return done(true);
    let timer;
    const finish = value => { observer.disconnect(); clearTimeout(timer); done(value); };
    const observer = new MutationObserver(() => {
        if (hydrated()) finish(true);
    });
    observer.observe(document.documentElement, {
        attributes: true, attributeFilter: ['data-hydrated'], subtree: true,
    });
    timer = setTimeout(() => finish(hydrated()), timeout);";

// Navigation and the page's own reports.

impl Page<'_> {
    /// Navigate to `path` and wait until the test-app finished hydrating, so that event handlers
    /// are attached before the test starts interacting with the page.
    ///
    /// Runs as a `page_load` step (navigation and hydration), so the run summary shows what
    /// loading pages costs.
    pub async fn goto_path(&self, path: &str) -> Result<(), Report> {
        // The page we leave must not have reported errors.
        check(self).await?;
        let url = format!("{}{path}", self.base_url());
        async {
            self.driver()
                .goto(&url)
                .step("navigate")
                .detail(path)
                .await
                .context_with(|| format!("failed to go to {url}"))?;
            let hydration_timeout = Patience::global().timeout();
            let hydrated = async {
                let timeout = u64::try_from(hydration_timeout.as_millis()).unwrap_or(u64::MAX);
                self.driver()
                    .execute_async(WAIT_FOR_HYDRATION, vec![timeout.into()])
                    .await?
                    .convert::<bool>()
            }
            .step("wait_for_hydration")
            .detail(path)
            .await
            .context_with(|| format!("failed to wait for {url} to hydrate"))?;
            if !hydrated {
                // Usually a panic while hydrating: report what the page caught.
                let diagnostics = health::diagnostics(self.driver()).await;
                bail!(
                    "{url} did not finish hydrating within {:?}; the page reported {diagnostics:#?}",
                    hydration_timeout
                );
            }
            check_initial(self).await?;
            Ok(())
        }
        .step("page_load")
        .detail(path)
        .await
    }

    /// Navigate to the fixture at `path` showing only its sections `sections` (the test-app's
    /// `Section`s; `?only=<name>,...`), like [`Self::goto_path`]: a test of some parts
    /// of a fixture with many renders and hydrates only those.
    pub async fn goto_sections(&self, path: &str, sections: &[&str]) -> Result<(), Report> {
        self.goto_path(&format!("{path}?only={}", sections.join(",")))
            .await
    }
}
