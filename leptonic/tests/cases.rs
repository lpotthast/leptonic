// No upstream: the browser tests' case runner.
//! Runs the cases of a browser test as named steps.

/// Runs the cases of a test in order: each `$case` is a call returning
/// `impl Future<Output = Result<(), Report>>`, e.g. `cases!(selection(&page), hover(&page))`.
///
/// Every case runs as a `case` step, so a failure report's "Last steps" show which case was
/// running (browser-test's failure reports add the test code's frames and the error itself).
macro_rules! cases {
    ($($case:expr),+ $(,)?) => {{
        $(
            {
                use ::browser_test::StepExt as _;
                let case = $crate::cases::name(stringify!($case));
                ::tracing::info!("Case {case}");
                $case.step("case").detail(case).await?;
            }
        )+
    }};
}

/// The name of a case for messages: `selection(&page)` is `selection`, a call with further
/// arguments keeps them (`provides_slots(&page, "input")`).
pub fn name(call: &str) -> String {
    let call = call.split_whitespace().collect::<Vec<_>>().join(" ");
    for suffix in ["(&page)", "(& page)", "(page)"] {
        if let Some(name) = call.strip_suffix(suffix) {
            return name.to_owned();
        }
    }
    call
}
