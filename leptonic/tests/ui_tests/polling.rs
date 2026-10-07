// No upstream: polling macros for the collection tests.
//! Polling as macros: `BaseActions::wait_for_value`/`assert_stays` take closures returning
//! futures, which `#[async_trait]` test bodies can't hold across awaits (the `Send` check of the
//! boxed future fails with "one type is more general than the other"). These expand to plain
//! loops instead.

/// Waits until `$observe` (an expression, may `.await` and use `?`) equals `$expected`; fails
/// with `$what` and the last value seen after 10 s.
macro_rules! wait_for {
    ($what:expr, $expected:expr, $observe:expr $(,)?) => {{
        let expected = $expected;
        let deadline = ::std::time::Instant::now() + ::std::time::Duration::from_secs(10);
        loop {
            let actual = $observe;
            if actual == expected {
                break;
            }
            if ::std::time::Instant::now() >= deadline {
                ::leptos_browser_test::bail!(
                    "{} did not become {:?} within 10s; last seen {:?}",
                    $what,
                    expected,
                    actual
                );
            }
            ::tokio::time::sleep(::std::time::Duration::from_millis(50)).await;
        }
    }};
}

/// Negative check: `$observe` equals `$expected` now and keeps doing so for 300 ms (long enough
/// for effects, animation frames and short timers after an interaction).
macro_rules! stays {
    ($what:expr, $expected:expr, $observe:expr $(,)?) => {{
        let expected = $expected;
        let deadline = ::std::time::Instant::now() + ::std::time::Duration::from_millis(300);
        loop {
            let actual = $observe;
            if actual != expected {
                ::leptos_browser_test::bail!(
                    "{} changed to {:?}; expected it to stay {:?}",
                    $what,
                    actual,
                    expected
                );
            }
            if ::std::time::Instant::now() >= deadline {
                break;
            }
            ::tokio::time::sleep(::std::time::Duration::from_millis(50)).await;
        }
    }};
}
