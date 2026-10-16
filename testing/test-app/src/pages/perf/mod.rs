//! Performance fixtures: the pages of the performance tests of hooks and atoms
//! (`leptonic/tests/perf_tests/`, see `documentation/testing.md`, "Performance measurements"),
//! one per subject: [`atoms`] (`/perf/atoms/<atom>`, `atoms/<atom>.rs`) and [`hooks`]
//! (`/perf/hooks/<hook>`, `hooks/<hook>.rs`). A fixture renders its subject alone, with
//! [`PerfControls`]: what the test does to it besides user input (mounting it, changing its
//! props), run from a script so that the page allocates nothing for the call.
//!
//! Workloads of many actions, e.g. a long list growing, are scenarios ([`PerfScenarios`]).
//!
//! A fixture renders a [`PerfScenarios`]: while the page is mounted, `window.leptonicPerf` runs
//! its scenarios in one of two ways:
//! - `run(name)` times the scenario `name` on the page's clock, so that `WebDriver` round trips
//!   don't count (the performance suite, `just browser-perf`), and resolves to its
//!   [`Measurements`] as JSON; the page also renders a button per scenario that shows them, for
//!   runs by hand.
//! - `count(name)` runs it for the regular suite's measurements (`Page::measure` in the tests),
//!   stopping before each action (see [`Measurements::measure`]).
//!
//! A timed measurement ([`Measurements::measure`]) times an action (e.g. a signal update) twice:
//! - `<metric>.settled`: from the action until the page has settled (every effect the action caused
//!   has run: the next task, after all microtasks);
//! - `<metric>.render`: the next frame's work, from its animation frame callbacks to the first task
//!   after it (style, layout, resize observers and paint), without waiting for the frame.
//!
//! When counting, the scenario waits until the page is idle before each action and stops: `next()`
//! resolves to `{metric}`; the test starts its measurement (`window.leptonicCounters`, see
//! [`counters::register`]) and calls `go()`, the page runs the action, the test ends the
//! measurement once the page is idle, and `done()` lets the scenario go on. The page's own code
//! allocates nothing between `go()` and the action, so the counts are the action's. When the
//! scenario is done, `next()` resolves to `{results}`.

pub mod atoms;
pub mod counters;
pub mod hooks;

use std::{any::Any, cell::RefCell, future::Future, pin::Pin, sync::Arc};

use counters::Counters;
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
    /// the next frame took to render (`<metric>.render`). When counting (`leptonicPerf.count`),
    /// runs it in a counting window instead (see the module documentation).
    pub async fn measure(&mut self, metric: &str, action: impl FnOnce()) {
        if COUNTING.with_borrow(Option::is_some) {
            count(metric, action).await;
            return;
        }
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
/// its style, layout and paint. When counting (`leptonicPerf.count`), waits until the page is
/// idle instead (see [`idle`]): frames come late on a busy machine, and a scenario that went on
/// while the page still worked (a virtualizer measuring what it rendered) would reach another
/// state, and count otherwise.
pub async fn next_frame() {
    if COUNTING.with_borrow(Option::is_some) {
        idle().await;
    } else {
        animation_frame().await;
        settled().await;
    }
}

/// Frames [`idle`] waits at most.
const MAX_IDLE_FRAMES: usize = 60;

/// Waits until a frame passed without reactive work (memo computations, effect runs, signal
/// notifications), at most [`MAX_IDLE_FRAMES`]. Allocations don't count: waiting allocates.
async fn idle() {
    for _ in 0..MAX_IDLE_FRAMES {
        let before = Counters::now().reactive_activity();
        animation_frame().await;
        settled().await;
        if Counters::now().reactive_activity() == before {
            return;
        }
    }
    leptos::logging::warn!("the page wasn't idle after {MAX_IDLE_FRAMES} frames");
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

/// A scenario run for counting (`leptonicPerf.count`), stopping at each measurement for the test
/// (see the module documentation).
#[derive(Default)]
struct Counting {
    /// The test's `next()`, waiting for the scenario's next stop.
    waiting: Option<js_sys::Function>,
    /// A stop the test hasn't asked for yet.
    stop: Option<JsValue>,
    /// Whether that stop is the last (the results).
    last: bool,
    /// Runs the action of the measurement the scenario stopped at (`go()`).
    go: Option<js_sys::Function>,
    /// Lets the scenario go on after the window (`done()`).
    done: Option<js_sys::Function>,
}

thread_local! {
    /// The scenario running for counting, if any.
    static COUNTING: RefCell<Option<Counting>> = const { RefCell::new(None) };
}

/// A new promise and the function resolving it.
fn promise() -> (js_sys::Promise, js_sys::Function) {
    let mut resolve = None;
    let promise = js_sys::Promise::new(&mut |resolve_promise, _reject| {
        resolve = Some(resolve_promise);
    });
    let resolve = resolve.expect("`Promise::new` runs its executor at once");
    (promise, resolve)
}

/// `{<key>: value}`.
fn object(key: &str, value: &JsValue) -> JsValue {
    let object = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&object, &JsValue::from_str(key), value);
    object.into()
}

/// Hands `stop` to the test: to its waiting `next()`, or to the next one. The counting run ends
/// with the `last` stop.
fn hand_over(stop: JsValue, last: bool) {
    let waiting = COUNTING.with_borrow_mut(|counting| {
        let waiting = counting.as_mut()?.waiting.take();
        if waiting.is_some() && last {
            *counting = None;
        }
        waiting
    });
    match waiting {
        Some(waiting) => {
            let _ = waiting.call1(&JsValue::NULL, &stop);
        }
        None => COUNTING.with_borrow_mut(|counting| {
            if let Some(run) = counting.as_mut() {
                run.stop = Some(stop);
                run.last = last;
            }
        }),
    }
}

/// A counted measurement: waits until the page is idle, stops for the test, runs `action` when it
/// calls `go()` and goes on when it calls `done()`. Nothing allocates in between but `action`
/// (and what it causes).
async fn count(metric: &str, action: impl FnOnce()) {
    let (go, resolve_go) = promise();
    let (done, resolve_done) = promise();
    let go = JsFuture::from(go);
    let done = JsFuture::from(done);
    idle().await;
    COUNTING.with_borrow_mut(|counting| {
        if let Some(run) = counting.as_mut() {
            run.go = Some(resolve_go);
            run.done = Some(resolve_done);
        }
    });
    hand_over(object("metric", &JsValue::from_str(metric)), false);
    let _ = go.await;
    action();
    let _ = done.await;
}

/// Sets `api.<name>` to `closure`, which lives as long as `closures`.
fn register<T: ?Sized + 'static>(
    api: &js_sys::Object,
    closures: &mut Vec<Box<dyn Any>>,
    name: &str,
    closure: Closure<T>,
) {
    let _ = js_sys::Reflect::set(api, &JsValue::from_str(name), closure.as_ref());
    closures.push(Box::new(closure));
}

/// Starts the scenario `run` for counting (`leptonicPerf.count`).
fn start_counting(name: &str, run: Option<Arc<dyn Fn() -> ScenarioRun + Send + Sync>>) {
    COUNTING.set(Some(Counting::default()));
    let name = name.to_owned();
    leptos::task::spawn_local(async move {
        let stop = match run {
            Some(run) => object("results", &JsValue::from_str(&run().await.to_json())),
            None => object(
                "error",
                &JsValue::from_str(&format!("no scenario {name:?}")),
            ),
        };
        hand_over(stop, true);
    });
}

/// `leptonicPerf.next()`: resolves at the counting run's next stop.
fn next_stop() -> js_sys::Promise {
    let (promise, resolve) = promise();
    let stop = COUNTING.with_borrow_mut(|counting| {
        let Some(run) = counting.as_mut() else {
            return Some(object(
                "error",
                &JsValue::from_str("no scenario is counting"),
            ));
        };
        if let Some(stop) = run.stop.take() {
            if run.last {
                *counting = None;
            }
            Some(stop)
        } else {
            run.waiting = Some(resolve.clone());
            None
        }
    });
    if let Some(stop) = stop {
        let _ = resolve.call1(&JsValue::NULL, &stop);
    }
    promise
}

/// Calls the counting run's `go` or `done` function.
fn resume(take: impl FnOnce(&mut Counting) -> Option<js_sys::Function>) {
    let resume = COUNTING.with_borrow_mut(|counting| counting.as_mut().and_then(take));
    if let Some(resume) = resume {
        let _ = resume.call0(&JsValue::NULL);
    }
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

/// Registers `scenarios` as `window.leptonicPerf` while mounted (see the module documentation),
/// and renders a button per scenario (`#perf-run-<name>`) that times it and shows its results
/// (`#perf-results`).
#[component]
pub fn PerfScenarios(scenarios: Vec<Scenario>) -> impl IntoView {
    let results = RwSignal::new(String::new());
    let scenarios = StoredValue::new(scenarios);
    let find = move |name: &str| {
        scenarios.with_value(|scenarios| {
            scenarios
                .iter()
                .find(|scenario| scenario.name == name)
                .map(|scenario| Arc::clone(&scenario.run))
        })
    };
    Effect::new(move || {
        let api = js_sys::Object::new();
        // The closures live as long as the page registers them.
        let mut closures: Vec<Box<dyn Any>> = Vec::new();
        let run = Closure::<dyn Fn(String) -> js_sys::Promise>::new(move |name: String| {
            let scenario = find(&name);
            wasm_bindgen_futures::future_to_promise(async move {
                let run =
                    scenario.ok_or_else(|| JsValue::from_str(&format!("no scenario {name:?}")))?;
                Ok(JsValue::from_str(&run().await.to_json()))
            })
        });
        register(&api, &mut closures, "run", run);
        let count = Closure::<dyn Fn(String)>::new(move |name: String| {
            start_counting(&name, find(&name));
        });
        register(&api, &mut closures, "count", count);
        let next = Closure::<dyn Fn() -> js_sys::Promise>::new(next_stop);
        register(&api, &mut closures, "next", next);
        let go = Closure::<dyn Fn()>::new(|| resume(|run| run.go.take()));
        register(&api, &mut closures, "go", go);
        let done = Closure::<dyn Fn()>::new(|| resume(|run| run.done.take()));
        register(&api, &mut closures, "done", done);
        let _ = js_sys::Reflect::set(&window(), &JsValue::from_str("leptonicPerf"), &api);
        let closures = send_wrapper::SendWrapper::new(closures);
        on_cleanup(move || {
            let _ = js_sys::Reflect::delete_property(&window(), &JsValue::from_str("leptonicPerf"));
            drop(closures);
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
                        if let Some(run) = find(name) {
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

/// Something a performance test does to a fixture's subject besides user input
/// (`window.leptonicControls.<name>()`, `Page::control` in the tests): an action (mount it,
/// disable it) or a read (how often it was pressed).
#[derive(Clone)]
pub struct Control {
    pub name: &'static str,
    pub run: Arc<dyn Fn() -> f64 + Send + Sync>,
}

impl Control {
    /// An action, e.g. `mounted.set(true)`.
    pub fn action(name: &'static str, action: impl Fn() + Send + Sync + 'static) -> Self {
        Self {
            name,
            run: Arc::new(move || {
                action();
                0.0
            }),
        }
    }

    /// A read, e.g. a press counter.
    pub fn read(name: &'static str, read: impl Fn() -> f64 + Send + Sync + 'static) -> Self {
        Self {
            name,
            run: Arc::new(read),
        }
    }
}

/// Registers `controls` as `window.leptonicControls` while mounted (calling one allocates
/// nothing but what it does). Renders nothing.
#[component]
pub fn PerfControls(controls: Vec<Control>) -> impl IntoView {
    let controls = StoredValue::new(controls);
    Effect::new(move || {
        let api = js_sys::Object::new();
        let mut closures: Vec<Box<dyn Any>> = Vec::new();
        for control in controls.get_value() {
            let run = Arc::clone(&control.run);
            let closure = Closure::<dyn Fn() -> f64>::new(move || run());
            register(&api, &mut closures, control.name, closure);
        }
        let _ = js_sys::Reflect::set(&window(), &JsValue::from_str("leptonicControls"), &api);
        let closures = send_wrapper::SendWrapper::new(closures);
        on_cleanup(move || {
            let _ =
                js_sys::Reflect::delete_property(&window(), &JsValue::from_str("leptonicControls"));
            drop(closures);
        });
    });
}
