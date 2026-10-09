//! Scripts in the page: the one implementation behind `LowLevel::eval`/`eval_async`, the
//! element helpers' scripts and the suite's settling.

use std::sync::Arc;

use browser_test::thirtyfour::session::handle::SessionHandle;
use rootcause::{Report, prelude::ResultExt};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// Run `script` in the page and return its result as a `T` (`()` for none). `args` are its
/// `arguments[n]`. A failure names the script.
pub(crate) async fn eval<T: DeserializeOwned>(
    session: &Arc<SessionHandle>,
    script: &str,
    args: Vec<Value>,
) -> Result<T, Report> {
    let failed = || failed(script);
    Ok(session
        .execute(script, args)
        .await
        .context_with(failed)?
        .convert()
        .context_with(failed)?)
}

/// Run the asynchronous `script` in the page: it finishes by calling its last argument (after
/// `args`) with its result, which is returned as a `T`. A failure names the script.
pub(crate) async fn eval_async<T: DeserializeOwned>(
    session: &Arc<SessionHandle>,
    script: &str,
    args: Vec<Value>,
) -> Result<T, Report> {
    let failed = || failed(script);
    Ok(session
        .execute_async(script, args)
        .await
        .context_with(failed)?
        .convert()
        .context_with(failed)?)
}

/// The error context of a failed script: its first line.
fn failed(script: &str) -> String {
    let first_line = script.trim().lines().next().unwrap_or_default();
    format!("the script `{first_line} …` failed")
}

/// Let two animation frames and a task pass. This is a scheduling barrier, not proof of quiescence.
pub(crate) async fn settle(session: &Arc<SessionHandle>) -> Result<(), Report> {
    eval_async(session, "const done = arguments[arguments.length - 1]; requestAnimationFrame(() => requestAnimationFrame(() => setTimeout(done)));", vec![]).await
}
