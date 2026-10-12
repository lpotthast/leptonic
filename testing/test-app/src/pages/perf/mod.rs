//! Performance fixtures (`/perf/{name}`): workloads measured in the page, on its clock, so that
//! `WebDriver` round trips don't count. Driven by the performance suite (`BROWSER_TEST_SUITE=performance`,
//! `just browser-perf`, see `documentation/testing.md`, "Performance measurements").
//!
//! A fixture renders a [`PerfScenarios`]: while the page is mounted, `window.leptonicPerf.run(name)`
//! runs the scenario `name` and resolves to its [`Measurements`] as JSON; the page also renders a
//! button per scenario that shows them, for runs by hand.
//!
//! A measurement ([`Measurements::measure`]) times an action (e.g. a signal update) twice:
//! - `<metric>.settled`: from the action until the page has settled (every effect the action caused
//!   has run: the next task, after all microtasks);
//! - `<metric>.render`: the next frame's work, from its animation frame callbacks to the first task
//!   after it (style, layout, resize observers and paint), without waiting for the frame.

pub mod virtual_list;

use std::{future::Future, pin::Pin, sync::Arc};

use leptos::{prelude::*, wasm_bindgen::JsCast, web_sys};
use serde_json::{Map, Value, json};
use wasm_bindgen::{JsValue, closure::Closure};
use wasm_bindgen_futures::JsFuture;

/// The results of a scenario run: its parameters and the samples of each metric, in
/// milliseconds.
#[derive(Debug, Clone, Default)]
pub struct Measurements {
    scenario: &'static str,
    parameters: Map<String, Value>,
    /// In the order of their first sample.
    metrics: Vec<(String, Vec<f64>)>,
}

impl Measurements {
    #[must_use]
    pub fn new(scenario: &'static str) -> Self {
        Self {
            scenario,
            ..Self::default()
        }
    }

    /// Records a parameter of the run (rows, batch size, rounds, ...).
    #[must_use]
    pub fn with_parameter(mut self, name: &str, value: impl Into<Value>) -> Self {
        self.parameters.insert(name.to_owned(), value.into());
        self
    }

    /// Adds a sample (milliseconds) to `metric`.
    pub fn sample(&mut self, metric: &str, milliseconds: f64) {
        match self.metrics.iter_mut().find(|(name, _)| name == metric) {
            Some((_, samples)) => samples.push(milliseconds),
            None => self.metrics.push((metric.to_owned(), vec![milliseconds])),
        }
    }

    /// Runs `action` and records how long until the page settled (`<metric>.settled`) and how long
    /// the next frame took to render (`<metric>.render`).
    pub async fn measure(&mut self, metric: &str, action: impl FnOnce()) {
        let start = now();
        action();
        settled().await;
        let settled_at = now();
        animation_frame().await;
        let frame_start = now();
        settled().await;
        let rendered_at = now();
        self.sample(&format!("{metric}.settled"), settled_at - start);
        self.sample(&format!("{metric}.render"), rendered_at - frame_start);
    }

    fn to_json(&self) -> String {
        let metrics: Vec<Value> = self
            .metrics
            .iter()
            .map(|(name, samples)| json!({ "name": name, "samples": samples }))
            .collect();
        json!({
            "scenario": self.scenario,
            "parameters": self.parameters,
            "metrics": metrics,
        })
        .to_string()
    }
}

/// The page's clock: milliseconds since the page started (`performance.now()`).
///
/// # Panics
///
/// Outside a browser.
#[must_use]
pub fn now() -> f64 {
    window()
        .performance()
        .expect("browsers have `performance`")
        .now()
}

/// Waits for the next task: every microtask queued until now (Leptos' effects, and the effects
/// they cause) has run.
///
/// # Panics
///
/// Outside a browser.
pub async fn settled() {
    let promise = js_sys::Promise::new(&mut |resolve, _reject| {
        let channel = web_sys::MessageChannel::new().expect("browsers have `MessageChannel`");
        let port = channel.port1();
        let on_message = Closure::once_into_js(move || {
            let _ = resolve.call0(&JsValue::NULL);
        });
        port.set_onmessage(Some(on_message.unchecked_ref()));
        channel
            .port2()
            .post_message(&JsValue::NULL)
            .expect("posting to a new channel succeeds");
    });
    let _ = JsFuture::from(promise).await;
}

/// Waits until the next frame was rendered: its animation frame callbacks, then a task after
/// its style, layout and paint.
pub async fn next_frame() {
    animation_frame().await;
    settled().await;
}

/// Waits for the next animation frame callback.
async fn animation_frame() {
    let promise = js_sys::Promise::new(&mut |resolve, _reject| {
        let on_frame = Closure::once_into_js(move |_: f64| {
            let _ = resolve.call0(&JsValue::NULL);
        });
        let _ = window().request_animation_frame(on_frame.unchecked_ref());
    });
    let _ = JsFuture::from(promise).await;
}

/// The future of a scenario run.
pub type ScenarioRun = Pin<Box<dyn Future<Output = Measurements>>>;

/// A workload of a performance fixture, run by `window.leptonicPerf.run(name)`.
#[derive(Clone)]
pub struct Scenario {
    pub name: &'static str,
    pub run: Arc<dyn Fn() -> ScenarioRun + Send + Sync>,
}

impl Scenario {
    pub fn new<F, Fut>(name: &'static str, run: F) -> Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Measurements> + 'static,
    {
        Self {
            name,
            run: Arc::new(move || Box::pin(run())),
        }
    }
}

/// Registers `scenarios` as `window.leptonicPerf` while mounted, and renders a button per scenario
/// (`#perf-run-<name>`) showing its last results (`#perf-results`).
#[component]
pub fn PerfScenarios(scenarios: Vec<Scenario>) -> impl IntoView {
    let results = RwSignal::new(String::new());
    let scenarios = StoredValue::new(scenarios);
    Effect::new(move || {
        let run = Closure::<dyn Fn(String) -> js_sys::Promise>::new(move |name: String| {
            let scenario = scenarios.with_value(|scenarios| {
                scenarios
                    .iter()
                    .find(|scenario| scenario.name == name)
                    .map(|scenario| Arc::clone(&scenario.run))
            });
            wasm_bindgen_futures::future_to_promise(async move {
                let run =
                    scenario.ok_or_else(|| JsValue::from_str(&format!("no scenario {name:?}")))?;
                Ok(JsValue::from_str(&run().await.to_json()))
            })
        });
        let api = js_sys::Object::new();
        let _ = js_sys::Reflect::set(&api, &JsValue::from_str("run"), run.as_ref());
        let _ = js_sys::Reflect::set(&window(), &JsValue::from_str("leptonicPerf"), &api);
        // The closure lives as long as the page registers it.
        let run = send_wrapper::SendWrapper::new(run);
        on_cleanup(move || {
            let _ = js_sys::Reflect::delete_property(&window(), &JsValue::from_str("leptonicPerf"));
            drop(run);
        });
    });
    let names =
        scenarios.with_value(|scenarios| scenarios.iter().map(|s| s.name).collect::<Vec<_>>());
    view! {
        <div id="perf-scenarios">
            {names
                .into_iter()
                .map(|name| {
                    let run = move |_| {
                        let scenario = scenarios
                            .with_value(|scenarios| {
                                scenarios
                                    .iter()
                                    .find(|scenario| scenario.name == name)
                                    .map(|scenario| Arc::clone(&scenario.run))
                            });
                        if let Some(run) = scenario {
                            leptos::task::spawn_local(async move {
                                results.set(run().await.to_json());
                            });
                        }
                    };
                    view! {
                        <button id=format!("perf-run-{name}") on:click=run>
                            {format!("Run {name}")}
                        </button>
                    }
                })
                .collect_view()} <pre id="perf-results">{results}</pre>
        </div>
    }
}
