//! Performance of leptonic in the browser tests (`documentation/testing.md`, "Performance
//! measurements"):
//! - [`budgets`]: what the measurements (`Page::measure`) of every regular test counted, checked
//!   against the budgets in `tests/budgets/`, and the coverage of hooks and atoms ([`inventory`]).
//! - Timings ([`report`], the performance suite): each scenario's statistics are logged, written
//!   to this machine's `tests/timings/<machine>/scenarios/<fixture>/<scenario>.json` (samples
//!   included, `crate::timing`) and compared with the previous run's file, so that a change can be
//!   measured before and after. Timings depend on the machine and its load: compare runs of one
//!   machine, not with fixed budgets.

pub mod budgets;
pub mod inventory;

use std::{fmt::Write as _, path::PathBuf};

use rootcause::{Report, prelude::ResultExt};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::pages::PerfResults;

/// The statistics of a metric's samples, in milliseconds.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    pub count: usize,
    pub min: f64,
    pub median: f64,
    pub p90: f64,
    pub max: f64,
    pub mean: f64,
}

impl Stats {
    /// The statistics of `samples` (`None` without any).
    pub fn of(samples: &[f64]) -> Option<Self> {
        if samples.is_empty() {
            return None;
        }
        let mut sorted = samples.to_vec();
        sorted.sort_by(f64::total_cmp);
        // The nearest rank.
        let percentile = |p: f64| {
            #[allow(
                clippy::cast_possible_truncation,
                clippy::cast_sign_loss,
                clippy::cast_precision_loss
            )]
            let rank = ((p * sorted.len() as f64).ceil() as usize).clamp(1, sorted.len());
            sorted[rank - 1]
        };
        #[allow(clippy::cast_precision_loss)]
        let mean = sorted.iter().sum::<f64>() / sorted.len() as f64;
        Some(Self {
            count: sorted.len(),
            min: sorted[0],
            median: percentile(0.5),
            p90: percentile(0.9),
            max: sorted[sorted.len() - 1],
            mean,
        })
    }
}

/// A metric as recorded in a report file.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct RecordedMetric {
    name: String,
    stats: Stats,
    samples: Vec<f64>,
}

/// A scenario's report file.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Recorded {
    scenario: String,
    parameters: Map<String, Value>,
    metrics: Vec<RecordedMetric>,
}

/// Where the reports of `fixture` go.
fn reports_dir(fixture: &str) -> Result<PathBuf, Report> {
    Ok(crate::timing::machine_dir()?
        .join("scenarios")
        .join(fixture))
}

/// Logs `results` of the scenario of `fixture` (`/perf/<fixture>`) with the change of each median
/// since the previous run of the scenario, and records them, replacing the previous report.
pub fn report(fixture: &str, results: &PerfResults) -> Result<(), Report> {
    let dir = reports_dir(fixture)?;
    let path = dir.join(format!("{}.json", results.scenario));
    let previous: Option<Recorded> = std::fs::read_to_string(&path)
        .ok()
        .and_then(|json| serde_json::from_str(&json).ok())
        // Another workload isn't comparable.
        .filter(|previous: &Recorded| previous.parameters == results.parameters);
    let metrics: Vec<RecordedMetric> = results
        .metrics
        .iter()
        .filter_map(|metric| {
            Some(RecordedMetric {
                name: metric.name.clone(),
                stats: Stats::of(&metric.samples)?,
                samples: metric.samples.clone(),
            })
        })
        .collect();

    let mut table = format!(
        "{} ({}), milliseconds:\n  {:<24} {:>5} {:>8} {:>8} {:>8} {:>8}  {}",
        results.scenario,
        parameters_text(&results.parameters),
        "metric",
        "n",
        "median",
        "p90",
        "max",
        "mean",
        if previous.is_some() {
            "median vs. previous run"
        } else {
            ""
        },
    );
    for metric in &metrics {
        let Stats {
            count,
            median,
            p90,
            max,
            mean,
            ..
        } = metric.stats;
        let change = previous
            .as_ref()
            .and_then(|previous| previous.metrics.iter().find(|m| m.name == metric.name))
            .map(|before| {
                let before = before.stats.median;
                let change = median - before;
                // Below a millisecond, the clock's 0.1 ms steps make percentages noise.
                if before >= 1.0 {
                    format!(
                        "{before:.2} -> {median:.2} ({change:+.2} ms, {:+.0}%)",
                        (median / before - 1.0) * 100.0
                    )
                } else {
                    format!("{before:.2} -> {median:.2} ({change:+.2} ms)")
                }
            })
            .unwrap_or_default();
        let _ = write!(
            table,
            "\n  {:<24} {count:>5} {median:>8.2} {p90:>8.2} {max:>8.2} {mean:>8.2}  {change}",
            metric.name
        );
    }
    tracing::info!("{table}");

    let recorded = Recorded {
        scenario: results.scenario.clone(),
        parameters: results.parameters.clone(),
        metrics,
    };
    std::fs::create_dir_all(&dir).context("failed to create the performance reports dir")?;
    std::fs::write(&path, serde_json::to_string_pretty(&recorded)? + "\n")
        .context_with(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

fn parameters_text(parameters: &Map<String, Value>) -> String {
    parameters
        .iter()
        .map(|(name, value)| format!("{name} {value}"))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn stats_use_the_nearest_rank() {
        let stats =
            Stats::of(&[5.0, 1.0, 3.0, 2.0, 4.0, 10.0, 6.0, 7.0, 8.0, 9.0]).expect("samples");
        assert_that!(stats.count).is_equal_to(10);
        assert_that!(stats.min).is_equal_to(1.0);
        assert_that!(stats.median).is_equal_to(5.0);
        assert_that!(stats.p90).is_equal_to(9.0);
        assert_that!(stats.max).is_equal_to(10.0);
        assert_that!(stats.mean).is_equal_to(5.5);
        assert_that!(Stats::of(&[])).is_none();
    }
}
