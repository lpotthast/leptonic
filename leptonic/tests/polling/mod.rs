// No upstream: the browser tests' polling.
//! Polling, for page objects and tests alike. Macros rather than functions taking closures:
//! `#[async_trait]` test bodies can't hold a closure returning a future across awaits (the `Send`
//! check of the boxed future fails with "one type is more general than the other"). `$observe`
//! and `$condition` are expressions, re-evaluated on every poll; they may `.await` and use `?`.
//! Every poll runs as a `browser_test` step (`wait_for`, `wait_until`, `stays`), which the run
//! summary times.

/// Waits until `$observe` equals `$expected`; fails with `$what` (anything `Display`) and the last
/// value seen after 10 s.
macro_rules! wait_for {
    ($what:expr, $expected:expr, $observe:expr $(,)?) => {{
        use ::browser_test::StepExt as _;
        let what = &$what;
        let expected = $expected;
        let detail = format!("{} = {:?}", what, expected);
        async {
            let deadline = ::std::time::Instant::now() + ::std::time::Duration::from_secs(10);
            loop {
                let actual = $observe;
                if actual == expected {
                    return Ok::<(), ::leptos_browser_test::Report>(());
                }
                if ::std::time::Instant::now() >= deadline {
                    ::leptos_browser_test::bail!(
                        "{} did not become {:?} within 10s; last seen {:?}",
                        what,
                        expected,
                        actual
                    );
                }
                ::tokio::time::sleep(::std::time::Duration::from_millis(50)).await;
            }
        }
        .step("wait_for")
        .detail(detail)
        .await?;
    }};
}

/// Waits until `$condition` (a `bool`) holds, e.g. a text contains a message. Prefer
/// [`wait_for!`] where there's an expected value: its error shows what was seen.
macro_rules! wait_until {
    ($what:expr, $condition:expr $(,)?) => {{
        use ::browser_test::StepExt as _;
        let what = &$what;
        let detail = what.to_string();
        async {
            let deadline = ::std::time::Instant::now() + ::std::time::Duration::from_secs(10);
            loop {
                if $condition {
                    return Ok::<(), ::leptos_browser_test::Report>(());
                }
                if ::std::time::Instant::now() >= deadline {
                    ::leptos_browser_test::bail!("{} didn't happen within 10s", what);
                }
                ::tokio::time::sleep(::std::time::Duration::from_millis(50)).await;
            }
        }
        .step("wait_until")
        .detail(detail)
        .await?;
    }};
}

/// Negative check ("nothing happens"): `$observe` equals `$expected` now and keeps doing so for
/// 300 ms. A single read right after an interaction passes before the effects it would trigger
/// ran; this lets effects, animation frames and short timers run (and catches a value that
/// changes and changes back).
macro_rules! stays {
    ($what:expr, $expected:expr, $observe:expr $(,)?) => {
        stays_for!(
            $what,
            ::std::time::Duration::from_millis(300),
            $expected,
            $observe
        )
    };
}

/// [`stays!`] over `$window` (e.g. past a toast's timeout or a long-press delay).
macro_rules! stays_for {
    ($what:expr, $window:expr, $expected:expr, $observe:expr $(,)?) => {{
        use ::browser_test::StepExt as _;
        let what = &$what;
        let expected = $expected;
        let window: ::std::time::Duration = $window;
        let detail = format!("{} = {:?}", what, expected);
        async {
            let deadline = ::std::time::Instant::now() + window;
            loop {
                let actual = $observe;
                if actual != expected {
                    ::leptos_browser_test::bail!(
                        "{} changed to {:?}; expected it to stay {:?}",
                        what,
                        actual,
                        expected
                    );
                }
                if ::std::time::Instant::now() >= deadline {
                    return Ok::<(), ::leptos_browser_test::Report>(());
                }
                ::tokio::time::sleep(::std::time::Duration::from_millis(50)).await;
            }
        }
        .step("stays")
        .detail(detail)
        .await?;
    }};
}
