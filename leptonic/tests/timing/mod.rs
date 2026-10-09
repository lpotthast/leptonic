// No upstream: the browser tests' timing.
//! How long and how often the browser tests observe the page.
//!
//! The suite has one [`Patience`], installed by [`install`] before the first test: assertr's
//! eventual assertions use it, and thirtyfour's element lookups poll with the same timeout and
//! interval ([`element_query_wait`]).
//!
//! ```ignore
//! // Waits until the observed value meets the expectation.
//! assert_that!(|| log.inner_text())
//!     .eventually_ok()
//!     .matches(eq("change:200"))
//!     .await;
//! // "Nothing happens": settle, then the value meets the expectation and keeps doing so.
//! page.settle().await?;
//! assert_that!(|| page.count(TOAST))
//!     .consistently_ok()
//!     .for_at_least(Duration::from_millis(2500))
//!     .matches(eq(1))
//!     .await;
//! ```
//!
//! "Nothing happens" checks settle the page first ([`settle`]: two animation frames and a task,
//! by when the event handlers, effects, frame callbacks and zero-delay timers an interaction
//! caused ran), then observe it. Settling is counted in frames, as the page's reactions happen
//! per frame. Timers run on the clock, so behavior behind one (a tooltip's delay, a long press, a
//! toast's timeout) is covered by observing past it: `consistently_ok().for_at_least(..)`.
//! react-aria's tests need neither: `act()` flushes every update synchronously, and fake timers
//! advance on demand.

use std::{sync::Arc, time::Duration};

use assertr::prelude::Patience;
use browser_test::{ElementQueryWait, thirtyfour::session::handle::SessionHandle};
use rootcause::{Report, prelude::ResultExt};

/// Resolves after two animation frames and a task: by then, the page ran the event handlers,
/// effects (microtasks), frame callbacks and zero-delay timers an interaction caused.
const SETTLE: &str = "const done = arguments[arguments.length - 1];
    requestAnimationFrame(() => requestAnimationFrame(() => setTimeout(done)));";

/// The suite's patience: waits fail after 10 s, observations are 50 ms apart, and
/// `consistently` observes the settled page once, unless `for_at_least` says otherwise.
///
/// `BROWSER_TEST_STAYS_MS` makes `consistently` observe for that long: raise it to look for
/// checks that pass only because they don't observe long enough.
fn patience() -> Patience {
    let consistency = std::env::var("BROWSER_TEST_STAYS_MS").map_or(Duration::ZERO, |ms| {
        Duration::from_millis(
            ms.parse()
                .expect("BROWSER_TEST_STAYS_MS is a number of milliseconds"),
        )
    });
    Patience::DEFAULT
        .within(Duration::from_secs(10))
        .polling_every(Duration::from_millis(50))
        .consistently_for(consistency)
}

/// Makes [the suite's patience](patience) the global one.
pub fn install() {
    patience().set_global();
}

/// The polling of thirtyfour's element lookups: the global patience's timeout and interval.
pub fn element_query_wait() -> ElementQueryWait {
    let patience = Patience::global();
    ElementQueryWait::new(patience.timeout(), patience.interval())
        .expect("the poll interval is not zero")
}

/// Wait until the page of `session` settled (see [`SETTLE`]).
pub async fn settle(session: &Arc<SessionHandle>) -> Result<(), Report> {
    session
        .execute_async(SETTLE, Vec::new())
        .await
        .context("the page did not settle (two animation frames and a task)")?;
    Ok(())
}
