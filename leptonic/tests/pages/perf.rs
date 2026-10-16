//! Performance scenarios of the test-app's `/perf` fixtures (`testing/test-app/src/pages/perf`):
//! timed in the page, on its clock ([`run`]), or counted action by action
//! (`Page::count_perf_scenario`, [`COUNT`]).

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

/// Where a counting run of a scenario stopped: at a measurement, at its end, or at an error.
#[derive(Deserialize)]
pub(crate) struct Stop {
    pub(crate) metric: Option<String>,
    results: Option<String>,
    error: Option<String>,
}

/// Starts the scenario for counting and resolves at its first stop.
pub(crate) const COUNT: &str =
    "const [name, done] = [arguments[0], arguments[arguments.length - 1]];
    if (!window.leptonicPerf) { done({error: 'the page has no performance scenarios'}); return; }
    window.leptonicPerf.count(name);
    window.leptonicPerf.next().then(done);";

/// Runs the action the scenario stopped at.
pub(crate) const GO: &str = "window.leptonicPerf.go();";

/// Lets the scenario go on and resolves at its next stop.
pub(crate) const NEXT: &str = "const done = arguments[arguments.length - 1];
    window.leptonicPerf.done();
    window.leptonicPerf.next().then(done);";

impl Stop {
    /// The parameters of the scenario `name`, if it ended here (`None`: it stopped at a
    /// measurement).
    pub(crate) fn parameters(self, name: &str) -> Result<Option<Map<String, Value>>, Report> {
        match self {
            Self {
                metric: Some(_), ..
            } => Ok(None),
            Self {
                results: Some(json),
                ..
            } => Ok(Some(serde_json::from_str::<PerfResults>(&json)?.parameters)),
            Self {
                error: Some(error), ..
            } => bail!("counting the scenario {name:?} failed: {error}"),
            Self { .. } => bail!("counting the scenario {name:?} stopped without a result"),
        }
    }
}
