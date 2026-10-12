//! Performance scenarios of the test-app's `/perf` fixtures (`testing/test-app/src/pages/perf`):
//! run in the page, measured on its clock.

use std::sync::Arc;

use browser_test::thirtyfour::session::handle::SessionHandle;
use rootcause::{Report, bail};
use serde::Deserialize;
use serde_json::{Map, Value};

use super::script;

/// The results of a scenario run.
#[derive(Debug, Clone, Deserialize)]
pub struct PerfResults {
    pub scenario: String,
    /// What the fixture ran (rows, batch size, rounds, ...).
    pub parameters: Map<String, Value>,
    /// In the order the scenario first measured them.
    pub metrics: Vec<PerfMetric>,
}

/// The samples of one metric, in milliseconds.
#[derive(Debug, Clone, Deserialize)]
pub struct PerfMetric {
    pub name: String,
    pub samples: Vec<f64>,
}

impl PerfResults {
    /// The samples of `metric`.
    pub fn samples(&self, metric: &str) -> Result<&[f64], Report> {
        match self.metrics.iter().find(|m| m.name == metric) {
            Some(metric) => Ok(&metric.samples),
            None => bail!("{} measured no {metric:?}", self.scenario),
        }
    }

    /// The integer parameter `name`.
    pub fn parameter(&self, name: &str) -> Result<u64, Report> {
        match self.parameters.get(name).and_then(Value::as_u64) {
            Some(value) => Ok(value),
            None => bail!("{} has no integer parameter {name:?}", self.scenario),
        }
    }
}

/// What the page answers: the results as JSON, or why it couldn't run the scenario.
#[derive(Deserialize)]
struct Outcome {
    json: Option<String>,
    error: Option<String>,
}

const RUN: &str = "const [name, done] = [arguments[0], arguments[arguments.length - 1]];
    if (!window.leptonicPerf) { done({error: 'the page has no performance scenarios'}); return; }
    window.leptonicPerf.run(name).then(json => done({json}), error => done({error: String(error)}));";

/// Runs the scenario `name` of the current page.
pub(crate) async fn run(session: &Arc<SessionHandle>, name: &str) -> Result<PerfResults, Report> {
    let outcome: Outcome = script::eval_async(session, RUN, vec![name.into()]).await?;
    match (outcome.json, outcome.error) {
        (Some(json), _) => Ok(serde_json::from_str(&json)?),
        (None, Some(error)) => bail!("the scenario {name:?} failed: {error}"),
        (None, None) => bail!("the scenario {name:?} returned nothing"),
    }
}
