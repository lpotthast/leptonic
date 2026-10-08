// No upstream: the browser tests' polling.
//! Observing a value over time, for whatever the helpers of `crate::pages` don't wait for or
//! check themselves:
//!
//! ```ignore
//! wait_for("the last change")
//!     .observing(|| log.inner_text())
//!     .to_be_equal_to("change:200")
//!     .await?;
//! wait_for("the focused option")
//!     .observing(|| async { page.focused_element().await?.inner_text().await })
//!     .to_be("Option 4 or later", |text| option_number(text) >= 4)
//!     .await?;
//! expect("the press log")
//!     .observing(|| log.inner_text())
//!     .to_stay_equal_to("")
//!     .await?;
//! expect("the number of open toasts")
//!     .observing(|| page.count(TOAST))
//!     .for_at_least(Duration::from_millis(2500))
//!     .to_stay_equal_to(1)
//!     .await?;
//! ```
//!
//! The observation is a closure returning the read's future (`|| log.inner_text()`), or an
//! `async` block for several steps. A failure names the observed thing, the expectation and the
//! last value seen. Every observation runs as a `browser_test` step.
//!
//! Elements are waited for by thirtyfour: lookups and element waits poll with
//! [`element_query_wait`], which the runner installs for every session. The session's implicit
//! wait is zero, so a plain `find` never waits.

// The expectations read as English (`to_be_equal_to`, `to_stay_equal_to`) and consume the
// observation, as assertr's do.
#![allow(clippy::wrong_self_convention)]

use std::{fmt::Debug, future::Future, time::Duration};

use browser_test::{ElementQueryWait, StepExt};
use rootcause::{Report, bail};

/// How long a wait observes before it fails.
pub const TIMEOUT: Duration = Duration::from_secs(10);

/// The pause between two observations.
pub const INTERVAL: Duration = Duration::from_millis(50);

/// How long [`Expectation::to_stay_equal_to`] observes, unless [`Expectation::for_at_least`] says
/// otherwise: long enough for effects, animation frames and short timers.
pub const STAYS: Duration = Duration::from_millis(300);

/// The polling of thirtyfour's element queries and element waits.
pub fn element_query_wait() -> ElementQueryWait {
    ElementQueryWait::new(TIMEOUT, INTERVAL).expect("the poll interval is not zero")
}

/// Wait until `what` (e.g. "the change log") reaches a value: an interaction's effect.
pub fn wait_for(what: impl Into<String>) -> WaitFor<()> {
    WaitFor {
        what: what.into(),
        observe: (),
    }
}

/// Expect `what` (e.g. "the press log") to keep a value: "nothing happens".
pub fn expect(what: impl Into<String>) -> Expectation<()> {
    Expectation {
        what: what.into(),
        observe: (),
        window: STAYS,
    }
}

/// A wait for `what`, see [`wait_for`].
#[must_use = "a wait does nothing until it has an expectation and is awaited"]
pub struct WaitFor<F> {
    what: String,
    observe: F,
}

impl WaitFor<()> {
    /// Observe `what` with `observe`, called again for every observation.
    pub fn observing<F, Fut, T>(self, observe: F) -> WaitFor<F>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T, Report>>,
    {
        WaitFor {
            what: self.what,
            observe,
        }
    }
}

impl<F> WaitFor<F> {
    /// Wait until the observed value is `expected`.
    pub async fn to_be_equal_to<Fut, T, E>(self, expected: E) -> Result<(), Report>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T, Report>>,
        T: PartialEq<E> + Debug,
        E: Debug,
    {
        let detail = format!("{} = {expected:?}", self.what);
        self.until(&format!("become {expected:?}"), |actual| {
            *actual == expected
        })
        .step("wait_for")
        .detail(detail)
        .await
    }

    /// Wait until the observed value meets `condition`, described by `description` as what the
    /// value is to be (e.g. "Option 4 or later", "in view"): a failure says the value "did not
    /// become Option 4 or later" and shows the last value seen.
    pub async fn to_be<Fut, T>(
        self,
        description: &str,
        condition: impl Fn(&T) -> bool,
    ) -> Result<(), Report>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T, Report>>,
        T: Debug,
    {
        let detail = format!("{}: {description}", self.what);
        self.until(&format!("become {description}"), condition)
            .step("wait_for")
            .detail(detail)
            .await
    }

    async fn until<Fut, T>(
        self,
        expectation: &str,
        condition: impl Fn(&T) -> bool,
    ) -> Result<(), Report>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T, Report>>,
        T: Debug,
    {
        let deadline = std::time::Instant::now() + TIMEOUT;
        loop {
            let actual = (self.observe)().await?;
            if condition(&actual) {
                return Ok(());
            }
            if std::time::Instant::now() >= deadline {
                bail!(
                    "{} did not {expectation} within {TIMEOUT:?}; last seen {actual:?}",
                    self.what
                );
            }
            tokio::time::sleep(INTERVAL).await;
        }
    }
}

/// An expectation that `what` keeps a value, see [`expect`].
#[must_use = "an expectation does nothing until it has a value and is awaited"]
pub struct Expectation<F> {
    what: String,
    observe: F,
    window: Duration,
}

impl Expectation<()> {
    /// Observe `what` with `observe`, called again for every observation.
    pub fn observing<F, Fut, T>(self, observe: F) -> Expectation<F>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T, Report>>,
    {
        Expectation {
            what: self.what,
            observe,
            window: self.window,
        }
    }
}

impl<F> Expectation<F> {
    /// Observe for `window` instead of [`STAYS`], e.g. past a toast's timeout or a long-press
    /// delay.
    pub fn for_at_least(mut self, window: Duration) -> Self {
        self.window = window;
        self
    }

    /// Expect the observed value to be `expected` now and to stay so for the window: a value that
    /// changes and changes back fails, too.
    pub async fn to_stay_equal_to<Fut, T, E>(self, expected: E) -> Result<(), Report>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T, Report>>,
        T: PartialEq<E> + Debug,
        E: Debug,
    {
        let detail = format!("{} = {expected:?}", self.what);
        async {
            let deadline = std::time::Instant::now() + self.window;
            loop {
                let actual = (self.observe)().await?;
                if actual != expected {
                    bail!(
                        "{} changed to {actual:?}; expected it to stay {expected:?}",
                        self.what
                    );
                }
                if std::time::Instant::now() >= deadline {
                    return Ok(());
                }
                tokio::time::sleep(INTERVAL).await;
            }
        }
        .step("stays")
        .detail(detail)
        .await
    }
}
