//! Measurements (`Page::measure`): what an action makes the app and the browser do, from the
//! action until the page is idle again ([`Metric`]: allocations, reactive work, DOM mutations,
//! layouts, ...).
//!
//! A measurement waits until the page is idle (a frame without DOM mutations, allocations and
//! reactive work), takes CDP's `Performance.getMetrics` and starts the app's counters
//! (`window.leptonicCounters`, `testing/test-app/src/pages/perf/counters.rs`) and the page's
//! `MutationObserver`, runs the action, waits until the page is idle again and reads them all
//! again. The scripts run no wasm while measuring but the counters' allocation-free reads, so
//! the app's counts are the action's.

use std::{collections::HashMap, sync::Arc};

use browser_test::thirtyfour::{cdp::Cdp, session::handle::SessionHandle};
use leptonic_perf_metrics::{Metric, Source};
use rootcause::{Report, bail, prelude::ResultExt};
use serde::Deserialize;
use serde_json::{Map, Value};

use super::script;

/// Frames a measurement waits at most for the page to become idle.
const MAX_FRAMES: u32 = 60;

/// Sets the page up for measurements (once per page: `window.leptonicMeasurement`, a
/// `MutationObserver` counting DOM mutations and a wait for idle) and waits until it is idle.
const PREPARE: &str = "const [maxFrames, done] = [arguments[0], arguments[arguments.length - 1]];
    if (!window.leptonicCounters) {
        done({error: 'the page has no `window.leptonicCounters` (not a hydrated test-app page)'});
        return;
    }
    if (!window.leptonicMeasurement) {
        const counts = {attributes: 0, text: 0, added_nodes: 0, removed_nodes: 0};
        const tally = records => {
            for (const record of records) {
                if (record.type === 'attributes') counts.attributes++;
                else if (record.type === 'characterData') counts.text++;
                else { counts.added_nodes += record.addedNodes.length; counts.removed_nodes += record.removedNodes.length; }
            }
        };
        const observer = new MutationObserver(tally);
        observer.observe(document.documentElement, {subtree: true, childList: true, attributes: true, characterData: true});
        const task = () => new Promise(resolve => {
            const channel = new MessageChannel();
            channel.port1.onmessage = () => resolve();
            channel.port2.postMessage(null);
        });
        const frame = () => new Promise(resolve => requestAnimationFrame(resolve)).then(task);
        window.leptonicMeasurement = {
            counts,
            task,
            flush: () => tally(observer.takeRecords()),
            reset() { this.flush(); for (const key in counts) counts[key] = 0; },
            activity() {
                this.flush();
                return counts.attributes + counts.text + counts.added_nodes + counts.removed_nodes
                    + window.leptonicCounters.activity();
            },
            async idle(maxFrames) {
                for (let frames = 1; frames <= maxFrames; frames++) {
                    const before = this.activity();
                    await frame();
                    if (this.activity() === before) return {frames, idle: true};
                }
                return {frames: maxFrames, idle: false};
            },
        };
    }
    window.leptonicMeasurement.idle(maxFrames).then(done);";

/// Starts the counting.
const START: &str = "window.leptonicMeasurement.reset(); window.leptonicCounters.start();";

/// Waits until the page is idle and returns the counts since [`START`].
const END: &str = "const [maxFrames, done] = [arguments[0], arguments[arguments.length - 1]];
    const measurement = window.leptonicMeasurement;
    if (!measurement) { done({error: 'the page changed during the measurement (a navigation?)'}); return; }
    (async () => {
        await measurement.task();
        const {frames, idle} = await measurement.idle(maxFrames);
        measurement.flush();
        const app = JSON.parse(window.leptonicCounters.since());
        done({app, dom: {...measurement.counts}, frames, idle, subjects: window.leptonicCounters.subjects()});
    })();";

/// What [`PREPARE`] answers.
#[derive(Deserialize)]
struct Prepared {
    idle: Option<bool>,
    error: Option<String>,
}

/// What [`END`] answers.
#[derive(Deserialize)]
struct Ended {
    app: Option<Map<String, Value>>,
    dom: Option<DomCounts>,
    frames: Option<i64>,
    idle: Option<bool>,
    subjects: Option<Vec<String>>,
    error: Option<String>,
}

/// What a measurement counted.
pub(crate) struct Counted {
    /// Every [`Metric`]'s value, in [`Metric::ALL`]'s order.
    pub(crate) values: Vec<(Metric, i64)>,
    /// The hooks and atoms that ran on the page so far, as leptonic reports them (sorted).
    pub(crate) ran: Vec<String>,
}

/// The DOM mutations: mutation records of attributes and text, and the nodes the child list
/// records added and removed.
#[derive(Deserialize)]
struct DomCounts {
    attributes: i64,
    text: i64,
    added_nodes: i64,
    removed_nodes: i64,
}

/// The browser's counters (CDP's `Performance.getMetrics`): layouts and style recalculations.
async fn browser_counters(cdp: &Cdp) -> Result<(i64, i64), Report> {
    let metrics: HashMap<String, f64> = cdp
        .performance()
        .get_metrics()
        .await
        .context("failed to get the page's performance metrics")?
        .into_iter()
        .map(|metric| (metric.name, metric.value))
        .collect();
    let count = |name: &str| match metrics.get(name) {
        #[allow(clippy::cast_possible_truncation)]
        Some(value) => Ok(*value as i64),
        None => Err(rootcause::report!("Chrome reported no {name}")),
    };
    Ok((count("LayoutCount")?, count("RecalcStyleCount")?))
}

/// Runs `action` and measures what it makes the app and the browser do (see the module
/// documentation).
pub(crate) async fn measure<T>(
    session: &Arc<SessionHandle>,
    action: impl Future<Output = Result<T, Report>> + Send,
) -> Result<(T, Counted), Report> {
    let cdp = Cdp::new(Arc::clone(session));
    cdp.performance()
        .enable()
        .await
        .context("failed to enable CDP's performance metrics")?;
    let prepared: Prepared = script::eval_async(session, PREPARE, vec![MAX_FRAMES.into()]).await?;
    match prepared {
        Prepared {
            error: Some(error), ..
        } => bail!("can't measure: {error}"),
        Prepared {
            idle: Some(false), ..
        } => bail!("can't measure: the page wasn't idle within {MAX_FRAMES} frames"),
        Prepared { .. } => {}
    }
    let (layouts_before, styles_before) = browser_counters(&cdp).await?;
    script::eval::<()>(session, START, vec![]).await?;
    let output = action.await?;
    let ended: Ended = script::eval_async(session, END, vec![MAX_FRAMES.into()]).await?;
    let (layouts_after, styles_after) = browser_counters(&cdp).await?;
    let (Some(app), Some(dom), Some(frames), Some(mut ran)) =
        (ended.app, ended.dom, ended.frames, ended.subjects)
    else {
        bail!(
            "the measurement failed: {}",
            ended
                .error
                .as_deref()
                .unwrap_or("the page returned no counts")
        );
    };
    if ended.idle == Some(false) {
        bail!("the page wasn't idle within {MAX_FRAMES} frames after the measured action");
    }
    let app_value = |metric: Metric| match app.get(metric.name()).and_then(Value::as_i64) {
        Some(value) => Ok(value),
        None => Err(rootcause::report!(
            "the app's counters have no {}",
            metric.name()
        )),
    };
    if let Some(unknown) = app
        .keys()
        .find(|name| Metric::from_name(name).is_none_or(|metric| metric.source() != Source::App))
    {
        bail!("the app counted an unknown metric {unknown:?}");
    }
    let mut values = Vec::with_capacity(Metric::ALL.len());
    for metric in Metric::ALL {
        let value = match metric {
            Metric::Allocations
            | Metric::Reallocations
            | Metric::AllocatedBytes
            | Metric::LargestAllocation
            | Metric::RetainedBytes
            | Metric::MemoRuns
            | Metric::MemoRunsUnchanged
            | Metric::EffectRuns
            | Metric::RenderEffectRuns
            | Metric::ImmediateEffectRuns
            | Metric::SignalNotifications => app_value(metric)?,
            Metric::DomAttributes => dom.attributes,
            Metric::DomText => dom.text,
            Metric::DomAddedNodes => dom.added_nodes,
            Metric::DomRemovedNodes => dom.removed_nodes,
            Metric::Layouts => layouts_after - layouts_before,
            Metric::StyleRecalcs => styles_after - styles_before,
            Metric::Frames => frames,
        };
        values.push((metric, value));
    }
    ran.sort();
    Ok((output, Counted { values, ran }))
}
