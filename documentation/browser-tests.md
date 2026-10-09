# Browser Test Style

How the browser tests in `leptonic/tests/` are written. Running them, fixtures and registration: CLAUDE.md, "Browser
Tests". Reference file: `ui_tests/test_checkbox.rs`. The helpers (`tests/pages/`): `PageActions` on a page,
`ElementActions` on an element, `Locator`s (`css`, `role`, `xpath`), `SyntheticEvent`s; the suite's timing (assertr's
`Patience`, settling) in `tests/timing/mod.rs`; `Case` in `tests/cases/mod.rs`.

Three rules behind everything below:

- **One way per thing.** Find an element with a `Locator`, then read, wait for, check or act on it with its one method.
  Every state has the same three methods: read it, `wait_for_<state>`, `<state>_stays`.
- **Strong, not brittle.** Wait for what an interaction changes, check that "nothing happens" over a time window, assert
  full known values, check focus by element identity. No fixed sleeps.
- **Plain test code.** Where a failure happened, what was expected and what the test did before comes from the helpers
  and browser-test's failure reports, not from the test code (see "Failure reports").

## Example

```rust
/// Pressing the label toggles the checkbox: `data-selected` on the label, the native `checked`
/// state, and the state the atom reports.
async fn selected_state(page: &Page<'_>) -> Result<(), Report> {
    // Lookups wait until the element exists.
    let label = page.element(css("label").text("Basic")).await?;
    let input = label.element("input").await?;
    let value = page.element("#test-cb-basic-value").await?;
    // Nothing happened yet: read once, assert with assertr.
    assert_that!(label.attr("data-selected").await?).is_none();
    assert_that!(input.is_selected().await?).is_false();

    // An interaction's effects arrive later: wait for them.
    label.click().await?;
    label.wait_for_attr("data-selected", Some("true")).await?;
    value.wait_for_inner_text("true").await?;
    // The wait proved the state settled: related values are assertions again.
    assert_that!(input.is_selected().await?).is_true();
    Ok(())
}

/// A disabled checkbox ignores presses.
async fn disabled_state(page: &Page<'_>) -> Result<(), Report> {
    page.element(css("label").text("Disabled")).await?.click().await?;
    // "Nothing happens" holds over a time window, not at one instant.
    let value = page.element("#test-cb-disabled-value").await?;
    value.inner_text_stays("false").await?;
    Ok(())
}
```

## Finding elements

A `Locator` says which elements; a plain `&str` or `String` is a CSS selector.

| Locator                          | Means                                                                          |
|----------------------------------|--------------------------------------------------------------------------------|
| `"#id"`, `"[role=grid] td"`      | elements matching the CSS selector                                             |
| `role("option")`                 | elements with the ARIA role (explicit; implicit for buttons and links)         |
| `css("label").text("Basic")`, `role("option").text("Apple")` | only those whose text content (whitespace collapsed) is the text, also while hidden |
| `xpath("ancestor-or-self::*[@inert]")` | only for relations CSS can't express (ancestors, following siblings)     |

Find elements as users do: by role and text, by label; ids for the fixture's own outputs and controls.

The same lookups exist on a page (in the document) and on an element (below it):

| Lookup                       | Does                                                                                 |
|------------------------------|--------------------------------------------------------------------------------------|
| `element(locator)`           | the first match, once it exists: the one way to wait for an element to appear        |
| `elements(locator)`          | the matches now (maybe none)                                                         |
| `count(locator)`             | the number of matches now                                                            |
| `inner_texts(locator)`       | the matches' inner texts now                                                         |
| `wait_for_count(locator, n)` | waits until `n` match (`0`: gone); before `elements` when a list is still appearing  |
| `count_stays(locator, n)`    | `n` match and keep doing so                                                          |

Never thirtyfour's `find`/`find_all`/`query`: the session's implicit wait is zero.

## Checking: four layers

| Layer          | Checks                                       | Fails with                                                    |
|----------------|----------------------------------------------|---------------------------------------------------------------|
| 1. Lookups     | the element exists (and returns it)          | `Report`: the locator                                         |
| 2. Waits       | a state is reached, within 10 s              | assertr panic: the expectation, the last value, the values seen and when |
| 3. Stays       | a state holds once the page settled ("nothing happens") | assertr panic: the expectation, the value it changed to, how long it held |
| 4. Assertions  | a value read now                             | assertr panic: the expression, the expected and actual value  |

Waits and stays are assertr's eventual assertions (`eventually`, `consistently`) on an observation of the page; the
state helpers below use them, too.

Which one, by what the check is about:

- **The effect of something the test just did** (a click, a key, a script): a wait. If the effect is that nothing
  changes: a stays check. Never an assertion: it reads before effects, animation frames and timers ran.

**Stays checks** wait until the page settled (`page.settle()`: two animation frames and a task, by when the event
handlers, effects, frame callbacks and zero-delay timers an interaction caused ran), then check the state. Settling is
counted in frames, as the page's reactions happen per frame; timers run on the clock, so behavior behind one (a
tooltip's delay, a long press, the press's 80 ms click fallback, a toast's timeout) is covered by observing past it:
`consistently_ok().for_at_least(delay + margin)`, with a comment naming the timer; a check that must end before a timer
fires states its window as well. React-aria's tests need no such checks: `act()`
flushes updates synchronously and fake timers advance on demand, so a single read after an action is final there.
`BROWSER_TEST_STAYS_MS=<ms>` makes every other stays check also observe that long (the suite's patience,
`tests/timing/mod.rs`): a check that then fails passes by default only because it doesn't observe long enough.
- **A state nothing is changing** (the initial render, values related to a state a wait just confirmed): an assertion.
- **That an element is there**: its lookup.

### The states

| State          | Read                                   | Wait until                              | Stays                                 |
|----------------|----------------------------------------|-----------------------------------------|---------------------------------------|
| Attribute      | `element.attr(name)`                   | `element.wait_for_attr(name, Some(v))` (`None`: absent) | `element.attr_stays(name, ..)` |
| DOM property   | `element.prop(name)`                   | `element.wait_for_prop(name, v)`        | `element.prop_stays(name, v)`         |
| Inner text     | `element.inner_text()`                 | `element.wait_for_inner_text(t)`        | `element.inner_text_stays(t)`         |
| Count          | `count(locator)`                       | `wait_for_count(locator, n)`            | `count_stays(locator, n)`             |
| Focus          | `page.focused_element()`               | `page.wait_for_focus(&element)`         | `page.focus_stays(&element)`          |
| Anything else  | any read                               | `assert_that!(read).eventually_ok().matches(..)` | `page.settle()`, then `assert_that!(read).consistently_ok().matches(..)` |

For anything else, assert on an observation (a closure returning the read's future) with assertr's eventual
assertions, ending with any matcher or assertion callback:

```rust
assert_that!(|| log.inner_text())
    .eventually_ok()
    .matches(eq("change:200"))
    .await;
assert_that!(|| number(&red))
    .eventually_ok()
    .satisfies(|red| {
        red.is_close_to(64.0, 1.0);
    })
    .await;
page.settle().await?;
assert_that!(|| page.count(TOAST))
    .consistently_ok()
    // Past the toast's timeout (2 s).
    .for_at_least(Duration::from_millis(2500))
    .matches(eq(1))
    .await;
```

- Observe the value the expectation is about, not a `bool` computed from it: a failure shows the last value and the
  values seen before it. Several values: a tuple or a struct.
- The failure shows the observation's expression (`|| log.inner_text()`), so it names itself. `with_subject_name` only
  for a runtime value the expression can't show, in helpers: `format!("attribute {name}")`.
- `eventually_ok`/`consistently_ok` for observations returning a `Result` (every helper read): an error is an
  observation that failed, retried until the timeout by `eventually_ok`. `eventually`/`consistently` for plain values.
- The expectation: `matches(eq(v))`, another matcher (`ge`, `all_of`, ...), or `satisfies(|it| { .. })` with any
  assertr assertion. `predicate(..).described_as(..)` only when no typed assertion says it.
- The timing is the suite's (`tests/timing/mod.rs`: 10 s, every 50 ms); `within(d)` and `for_at_least(d)` override it
  for one check, `for_at_least` only for real timers the check must outlast (a toast's timeout, a long-press delay).
- Every check is awaited, and ends the statement: `.await;`. A failure panics, which the runner reports as the test's
  failure.

The inner text is `innerText`, trimmed: what the element shows (never thirtyfour's `text()`, which skips
`display: contents` children). Waits pass the moment the state is reached, so waiting for a state that may already hold
costs nothing. An element handle waits on that element; if the page replaces it (re-rendering), every observation fails
with a stale element error until the timeout, which the failure shows: look the element up after the change.

### Other reads

| Value                                          | Read with                                                                |
|------------------------------------------------|--------------------------------------------------------------------------|
| Checked or selected, enabled, displayed        | `element.is_selected()`, `element.is_enabled()`, `element.is_displayed()` |
| What an id list refers to (name, description)  | `element.referenced_text("aria-describedby")`                            |
| Native validity                                | `element.is_valid()` (fires nothing)                                     |
| Position and size                              | `element.client_rect()` (unrounded; thirtyfour's `rect()` rounds)        |
| CSS property                                   | `element.css_value(name)`; custom properties (`--x`) through `page.eval` |
| What the page reported                         | `page.diagnostics()`: `panics`, `uncaught_errors`, `console_errors`, `console_warnings`; `page.clear_diagnostics()` after checking an expected one |

### Assertions

assertr, with the assertion that states the expectation, so a failure shows the actual value:

```rust
assert_that!(label.attr("data-selected").await?).is_none();
assert_that!(input.attr("type").await?).get_some().is_equal_to("checkbox");
assert_that!(group.referenced_text("aria-describedby").await?).is_equal_to("Pick your pets.");
assert_that!(page.inner_texts(role("option")).await?).contains_exactly(["Apple", "Banana"]);
assert_that!(page.count("[role=row]").await?).is_equal_to(3);
assert_that!(input.is_selected().await?).is_true();
assert_that!(rect.width).is_close_to(200.0, 0.5);
```

- Assert full known values (`is_equal_to`, `contains_exactly`), not parts (`contains`, `is_some`), where the value is
  known and stable.
- Never wrap a comparison into a `bool` (`assert_that!(a == b).is_true()`): use `is_equal_to`, `is_empty`,
  `has_length`, `is_greater_than`, `is_close_to`, ... `is_true()`/`is_false()` only for values that are booleans.
- An `Option<String>` attribute: `.is_none()`, `.get_some().is_equal_to("v")`; against an `Option` variable:
  `assert_that!(a.as_deref()).is_equal_to(b.as_deref())`.
- No `.await` inside an assertion's arguments (the chain is not `Send` across an await): read into a local first.
  Eventual assertions are the exception: they await their observation themselves.

## Acting

- **Pointer**: `element.click()`, `element.hover()`; `page.driver.action_chain()` for drags, context and modifier
  clicks. The action chain has no pause step: a pause inside a gesture is a commented sleep between two `perform()`s.
- **Keyboard**, to the focused element: `page.send_keys(keys)` (`Key::Shift + Key::Tab` holds Shift),
  `page.type_text(text)` (one key at a time, for inputs that move focus while typing), `page.hold_key(key, n)`
  (repeated `keydown`s). Focus: `element.focus()`, `page.blur_focused()`.
- **As assistive technology does** (from script): `element.virtual_click()`, `element.virtual_input(value)`.
- **Forms**: `form.check_validity()` (fires `invalid`), `form.form_values(name)`, `form.submit()`, `form.reset()`.
- **Events WebDriver can't produce** (touch and pen pointers, wheel, drag, `beforematch`, custom events):
  `element.dispatch(SyntheticEvent::pointer("pointerdown").with("pointerType", "touch"))`; the result says whether a
  handler prevented the default.
- **Scrolling**: `element.scroll_into_view()`, `element.scroll_to_top(px)`.
- **Anything else in the page** (recording listeners, app state on `window`, `DataTransfer`s): `page.eval::<T>(script,
  vec![..])`, values passed as `arguments[n]`, never formatted into the script. Never `driver.execute`.
- **Sleeps** only for real timers the test must let pass (a type-ahead reset, a long-press delay), with a comment.

## File layout

```rust
// Upstream: react-aria-components/test/Checkbox.test.js @ 99e6102368
//! What the cases cover. Spec: react-aria-components `Checkbox.test.js`.
use assertr::prelude::*;
use rootcause::Report;

use crate::pages::{ElementActions, Page, PageActions, css};

const PATH: &str = "/atoms/checkbox";

// File-local lookups, then one documented `pub async fn` per case.

/// Pressing the label toggles the checkbox.
pub async fn selected_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    // ...
    Ok(())
}
```

- Every case is a test of its own, registered in `ui_tests::all()` (`.case(test_checkbox::selected_state)`) and named
  after its function (`checkbox::selected_state`). Its first line loads its page (`page.goto_path(PATH)`, or a local
  `async fn open(page)` when the page needs more, e.g. a permission or a window size), so what it runs on is visible
  in the case itself. A case taking parameters gets a named case per variant (`provides_slots_input`).
- **Cases are independent.** Each one runs in a tab of its own browser context (browser-test's session reuse: a fresh
  user context per test, so no cookies, storage, cache, permissions or held keys of other tests) and never relies on what another case did: it sets up the state it
  needs itself, or expects the fixture's initial state. `BROWSER_TEST_FILTER=checkbox::hover` runs a single case. A
  flow whose steps belong together is one case.
- Every case is `pub async fn case(page: &Page<'_>) -> Result<(), Report>`, ends with `Ok(())`, and has a doc comment naming
  the behavior and the upstream test it mirrors (`"should ..."`) where there is one.
- Name what a case uses more than once with `let` (`let value = page.element("#..-value").await?;`).
- File-local helpers only where they add a lookup, a fixture convention or a setup several cases share (`label(page,
  text)`, a `reset(page)` that clears a fixture's log), never to rename a shared method. A helper two files need goes
  into `tests/pages/`.
- Fixture-specific actions (`tests/pages/<name>.rs`) only for a fixture with an id convention several cases share:
  a trait on `Page` (`FocusManagerActions`, `DndActions`).
- A check that isn't a case of one fixture (`test_hydration_ids.rs`, `test_server_panics.rs`) implements `BrowserTest`
  itself.

## Speed and memory

Measured 2026-10-08 (822 tests, 32 threads, 64 GB), Chrome Headless Shell and settle-based stays checks: 49s and
3.3 GB of browser memory (PSS, avg; 66 processes) at `BROWSER_TEST_PARALLELISM=8`, the default. Before, with Chrome
and 300 ms stays checks:

| `BROWSER_TEST_PARALLELISM` | wall time | Chrome memory (PSS, avg) |
|----------------------------|-----------|--------------------------|
| 8                          | 1m 10s    | 4.9 GB                   |
| 12                         | 1m 01s    | 6.1 GB                   |
| 16                         | 58s       | 7.2 GB                   |

- Every parallel test is a browser (~0.4 GB with the shell, ~0.5 GB with Chrome: renderers, network service, browser
  and GPU processes), plus one spare browser per eight. Above 8 the CPU is saturated: page loads slow down and the
  run barely gets faster.
- Chrome Headless Shell (`ChromeBinary::ChromeHeadlessShell`): a third less memory than Chrome, session resets in 25ms
  instead of 100ms. Visible runs use Chrome.
- The test-app is built with `--release` (`wasm-release`: `opt-level = "z"`, no LTO, incremental, debug assertions),
  by the suite and by `just serve-test-app` alike; it has no dev profiles of its own.
  Dev builds (33 MB of wasm instead of 13 MB) made every page load 2.5 times slower (450ms) and the suite 40s slower;
  LTO and one codegen unit saved 1 MB but made a rebuild after a library change take 84s instead of 20s.
- Session resets (browser-test's `SessionReset`), measured at parallelism 8 with the content-hashed, cacheable wasm and
  JS (`hash-files`, `cache-control: immutable` for `/pkg/`):

  | `BROWSER_TEST_SESSION_RESET` | wall time | browser memory (PSS, avg / max) | page load avg |
  |------------------------------|-----------|---------------------------------|---------------|
  | `manual` (default)           | 49s       | 1.3 GB / 2.3 GB                 | 155ms         |
  | `new-context`                | 50s       | 3.3 GB / 4.8 GB                 | 165ms         |

  `manual` resets the tab item by item and keeps the HTTP cache (the wasm and JS, and the code V8 compiled for them);
  every test still gets a new renderer process (the reset's `data:` page). Keeping the renderer too (`about:blank`)
  ran in 43-46s, but its memory grew with every test (4.2 GB on average, 5.7 GB at the end) and it kept caches beyond
  the HTTP cache (decoded resources, compiled code in the process). `new-context` keeps nothing (a cold load per test)
  and is the cross-check: both must pass alike. Clearing storage of type `all` took 1-2s per reset on these pages
  (shader cache and more); the manual reset clears the types pages use, and file systems only when used.
- Page loads (navigation and hydration) are the biggest part of the test bodies' time: 107-135 ms on average (45-49 s
  runs). The test-app's server renders at `opt-level = 1` in release builds (`server-release`: 1.8 ms per fixture page
  instead of 11 ms, the calendar page 87 ms instead of 550 ms, no slower rebuilds), and `goto_path` waits for
  hydration in the page (a `MutationObserver` on `data-hydrated`) instead of polling every 50 ms. Stays checks
  watched for 300 ms each before (~20% of the bodies' time); settling takes ~30 ms.
- Tried without gain: V8 without wasm tier-up, a minimal HTTP cache, longest-tests-first ordering (the run is CPU-bound,
  not waiting for one long test), mold (links the test server in 0.75s instead of 1.2s with Rust's default lld; the user decided against it,
  2026-10-08).
- Not compressed: the wasm (13.9 MB) downloads in ~35 ms over loopback during a run, part of it hidden by streaming
  compilation; decompressing it takes ~30 ms of CPU per page load, and cargo-leptos' `--precompress` (brotli 11) 20 s
  per build. Compression pays off over real networks (the book precompresses its release build), not here.

## Failure reports

browser-test's failure reports (browser-test README, "Failure Reports") show for every failure, without anything in the
test code:

- **where**: the test code's frames, from the helper that failed up to the case and line that called it (for a panic,
  e.g. a failed assertion: its location);
- **when**: the test's last steps (page loads, lookups) with their timing;
- **what**: the error, or for a failed check assertr's report, e.g. for a wait:

  ```text
  Assertion failed at leptonic/tests/pages/element.rs:274:14

  Expression: `|| self.inner_text()`

  Expected: "checked"

    Actual: "true"

  Details:
    - Waited: 10.01s (193 observations)
  ```

  followed by the test code's frames (`tests/ui_tests/test_checkbox.rs:46  test_checkbox::selected_state`) and the
  last steps. A value that changed while observed adds `Observed values`, each with its time.

The helpers keep this working: a check names runtime values its expression can't show (`with_subject_name`), focus
checks name the focused element (`ElementActions::describe`). A new helper does the same; test code propagates errors with `?`.
