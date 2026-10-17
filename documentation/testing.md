# Testing

Tests are the definition of done (`conventions.md`): pure logic and `*_state` hooks get native unit tests, DOM
behavior browser tests against `testing/test-app`, both derived from react-aria's own tests (`porting.md`) and
asserting with `assertr`. Every bug fix starts with a failing test; browser tests are run, not only compiled. Test runs
go through the one agent that runs them for all (`AGENTS.md`).

Clippy runs with warnings denied in every feature configuration (`just verify` lists them); keep client-only code
behind its feature guards rather than allowing unused-code lints.

## Native Tests

`cargo test -p leptonic --features full --lib` (`just unit-test` also runs `full,ssr` and the theme). A hook needs a
reactive owner: run it in `crate::testing::with_owner` (`leptonic/src/testing.rs`, test builds only):

```rust
#[test]
fn focus_moves_to_the_row_that_took_the_removed_rows_place() {
    with_owner(|| {
        let rows = RwSignal::new(vec!["alice", "bob", "carol"]);
        let state = grid(rows, &[], GridFocusMode::Row); // calls `use_grid_state`
        flush_effects(); // the Effects' first runs
        state.list.selection.set_focused_key(Some(Key::from("bob")), None);
        rows.update(|rows| rows.retain(|row| *row != "bob"));
        flush_effects(); // the re-runs the change caused
        assert_that!(state.list.selection.focused_key()).is_equal_to(Some(Key::from("carol")));
    });
}
```

- `with_owner(f)` runs `f` in a fresh `Owner` without Leptos' untracked-read warnings and disposes it afterwards.
- `flush_effects()` runs every pending Effect of the thread until none can progress. Effects never run on their own
  (like React's `act`): call it after creating the state and after each change whose Effects matter. Signals and
  memos need no flush. (How: the dev-dependencies enable `reactive_graph`'s `effects` feature, and `with_owner`
  installs an `any_spawner` executor queueing tasks on the thread.)
- DOM code can't run natively (JavaScript calls panic outside wasm: `leptos-and-dom.md`): that is the browser tests'
  job.

## Browser Test Setup

The browser tests live in `leptonic/tests/` (entry `tests/browser_test.rs`) and drive the test-app. They use
`leptos-browser-test` (starts `cargo leptos serve --release` on a random port) and `browser-test` (Chrome for Testing
+ chromedriver with Chrome Headless Shell, 8 tests in parallel by default, `thirtyfour` re-exported).

- **Fixtures**: every test page is a module under `testing/test-app/src/pages/{atoms,hooks}/`, registered in
  `FIXTURES` (`pages/mod.rs`), served at `/{group}/{name}`. Large pages render `Section`s; `?only=<name>,...`
  (`page.goto_sections`) renders only what a case needs. `page.goto_path`/`goto_sections` wait for `data-hydrated` on
  `<body>`, so no case acts before the handlers exist.
- **Cases**: `#[browser_test] pub async fn` in `tests/ui_tests/test_*.rs`, each loading its own page; cases never
  depend on each other or on shared server state. The macro generates a `BrowserTest` struct named after the
  module-qualified function (`name = "..."` overrides it; the doc comment is its description). Registration is
  explicit in `tests/ui_tests/mod.rs`: `TestGroup`s (logical groups may span modules) of
  `.with(fixture_test(test_x::Case {}))`. The harness (`tests/harness/`) supplies the page, checks its health and
  cleans up; helpers are in `tests/pages/` (`Page`, `ElementActions`, `Locator`s, `SyntheticEvent`, `Stopwatch`),
  fixture navigation, adapters and declared fixture defects in `tests/fixtures/`.
- **Sessions** return to browser-test's pool and are reset item by item for the next test, keeping only the HTTP cache
  of the content-hashed wasm and JS (`SessionReset::manual([CachedData::Http])`; no cookies, storage, permissions or
  held keys survive). `BROWSER_TEST_SESSION_RESET=new-context` (a cold browser context per test) is the cross-check:
  both must pass alike. `BROWSER_TEST_SESSION_REUSE=0`: a fresh browser per test.
- **Page health** after every test (`tests/pages/health.rs`, policy in `tests/fixtures/expectations.rs`): Rust
  panics, uncaught errors, `console.error`/`console.warn`, duplicate ids, references to missing ids and literal
  `attr:` attributes fail it. Declared fixture defects match exact messages and counts; a case expecting a warning
  consumes it with `fixtures::take_warnings(page, containing, count)` (which can't erase panics, errors or other
  warnings). After a Rust panic, waits give up at once and the panic is the test's error.
- **Whole-run checks**: `test_hydration_ids.rs` compares server-rendered ids (with their elements' tag and role) with
  the hydrated DOM and checks the health of every fixture of the test-app's index, incl. `QUERY_VARIANTS`
  (`testing/test-app/src/app.rs`: add one when a case loads a new variant). `test_server_panics.rs` runs last (also in
  filtered runs) and fails the run if the server panicked.
- **Known issues**: cases of behavior known to be broken are registered only in `known_issues_tests`
  (`BROWSER_TEST_SUITE=known-issues`); move them to the regular groups once fixed.
- **Running**: `just browser-test` (a plain `cargo test` skips the suite). Variables (documented in
  `tests/browser_test.rs`): `BROWSER_TEST_VISIBLE=1` (`just browser-test-visible`), `BROWSER_TEST_PAUSE=1`,
  `BROWSER_TEST_DRIVER_OUTPUT=1`, `BROWSER_TEST_APP_OUTPUT=1` (the test-app's build and server logs),
  `BROWSER_TEST_LOG=<filter>` (a level or `tracing-subscriber` targets, e.g. `info,browser_test::step=debug`; run
  logs stay at `info`, per-test and per-step events at `debug`), `BROWSER_TEST_PARALLELISM=<n>`.
  `TEST_APP_TARGET_DIR=<dir>` builds the test-app and its site there instead of in `CARGO_TARGET_DIR`. Two suites with
  the same target dir must never run at once. Chrome profiles live in `<target>/tmp/browser-test-profiles` (never
  `/tmp`, a RAM disk here). Ctrl-C cancels cleanly.
- **Keyboard layout**: chromedriver alone types through the host's input source (on a German layout `/` arrives as
  Shift+7 with `key` `&`, `z` as `y`), so the helpers (`pages/keyboard.rs`) type every US-keyboard character through
  CDP (`Input.dispatchKeyEvent`) as that keyboard produces it, with Shift around shifted characters. Named keys and
  chords stay on WebDriver. Tests type only through `page.send_keys`/`page.type_text`/`element.type_keys`, never
  thirtyfour's `WebElement::send_keys`.
- **Toolchain**: the installed `wasm-bindgen` CLI must match the `wasm-bindgen` version in the test-app's
  `Cargo.lock` (else `cargo leptos serve` fails): `cargo update -p wasm-bindgen -p js-sys -p web-sys -p
  wasm-bindgen-futures`, or update the CLI.

## Selecting Tests

```sh
BROWSER_TEST_FILTER=checkbox::hover just browser-test        # substrings of test names, comma-separated
BROWSER_TEST_GROUP=button,focus BROWSER_TEST_FILTER=keyboard just browser-test
```

`BROWSER_TEST_GROUP` matches exact group names; both apply when both are set; unmatched selections run no cases (the
server panic check still runs). `BROWSER_TEST_SUITE=known-issues` and `=performance` select those suites, filters
apply there too. While working on a hook or atom, run only its tests (its behavior tests' module and
`perf_tests::<kind>::<subject>::`); the full suite is the final verification step.

## Writing Browser Tests

Reference file: `ui_tests/test_checkbox.rs`. Three rules behind everything below:

- **One way per thing.** Find an element with a `Locator`, then read, wait for, check or act on it with its one
  method. Every state has the same three: read it, `wait_for_<state>`, `<state>_stays`.
- **Strong, not brittle.** Wait for what an interaction changes, check "nothing happens" over a time window, assert
  full known values, check focus by element identity. No fixed sleeps.
- **Plain test code.** Where a failure happened, what was expected and what the test did before come from the
  helpers and browser-test's failure reports, not from the test code.

```rust
// Upstream: react-aria-components/test/Checkbox.test.js @ 99e6102368
//! What the cases cover.
const PATH: &str = "/atoms/checkbox";

/// Pressing the label toggles the checkbox: `data-selected` on the label and the native `checked` state.
#[browser_test]
pub async fn selected_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;                              // every case loads its page itself
    let label = page.element(css("label").text("Basic")).await?; // lookups wait until the element exists
    let input = label.element("input").await?;
    assert_that!(label).attribute("data-selected").await.is_none(); // nothing happened yet: assert
    label.click().await?;
    label.wait_for_attr("data-selected", Some("true")).await?;     // an interaction's effects: wait
    assert_that!(input).selected().await.is_true();                // the wait proved the state settled
    Ok(())
}

/// A disabled checkbox ignores presses.
#[browser_test]
pub async fn disabled_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element(css("label").text("Disabled")).await?.click().await?;
    let value = page.element("#test-cb-disabled-value").await?;
    value.inner_text_stays("false", Duration::from_millis(100)).await?; // "nothing happens" over a window
    Ok(())
}
```

- Every case's doc comment says in one or two sentences what it tests, as the expected behavior, plus the upstream
  test it mirrors (`"should ..."`) where there is one (the user). A case taking parameters becomes a named case per
  variant; a flow whose steps belong together is one case. Name what a case uses twice with `let`.
- A case loads its page first (`page.goto_path(PATH)`, `goto_sections(PATH, &["basic"])`, or a local
  `async fn open(page)` when it needs more, e.g. a permission or a window size), so what it runs on is visible in it.
- **Fixtures** of many independent parts wrap each in a `Section` named like the part. Fixtures configure short real
  timers where the case's purpose allows (toast 500 ms, tooltip close delay 400 ms), and keep upstream's default where
  the default is the behavior (the 500 ms long-press threshold).
- File-local helpers only where they add a lookup, a fixture convention or a setup several cases share, never to
  rename or wrap a shared method (no `expect_focus(page, text)`); a helper two files need goes into `tests/pages/`.
  Fixture-specific actions (`tests/fixtures/<name>.rs`, a struct borrowing the `Page`: `DndActions`) only for a fixture
  with an id convention several cases share, with the standard names (`wait_for_log`, `log_stays`).
- **Time limits**: a case guarding against a freeze sets a limit above the 5 s a failed wait takes
  (`#[browser_test(time_limit = Duration::from_secs(15))]`); a page that never yields blocks its WebDriver session, so
  browser-test closes the case's frozen pages and names them. Default: 10 minutes.
- A check that isn't a case of one fixture (`test_hydration_ids.rs`) implements `BrowserTest<Page>` itself.

### Finding Elements

A `Locator` says which elements; a plain `&str` is a CSS selector. `role(AriaRole::Option)` matches the
browser-computed role (respecting hidden/inert content and overrides); `.text("Apple")` keeps those whose text content
(whitespace collapsed) equals the text; `.has(inner)` keeps those containing a match ("the row with this cell"). Find
elements as users do: by role, text and label; ids for the fixture's own outputs and controls.

| Lookup (on a page or an element)    | Does                                                                  |
|-------------------------------------|-----------------------------------------------------------------------|
| `element(locator)`                  | exactly one match, waiting while there is none; several fail at once  |
| `first_element(locator)`            | the first match in document order (waits for one)                     |
| `elements(locator)`, `count`, `inner_texts` | the matches now (maybe none)                                  |
| `wait_for_count(locator, n)`        | until `n` match (`0`: gone)                                           |
| `count_stays(locator, n, duration)` | `n` match and keep doing so                                           |

Raw thirtyfour `find`/`find_all` are immediate (the implicit wait is zero). An element handle keeps its node: for a
component that replaces its node, `page.wait_for_attr(locator, name, expected)` re-resolves each read; a wait on a
replaced handle gives up at once with a stale element error.

### Checking: Lookups, Waits, Stays, Assertions

- **The effect of something the test just did** (a click, a key, a script): a wait (5 s, every 20 ms: `Patience` in
  `tests/browser_test.rs`). If the effect is that nothing changes: a stays check over an explicit duration extending
  past the timer under test, with a comment naming it. Never an assertion: it reads before effects, frames and timers
  ran.
- **A state nothing is changing** (the initial render, values related to a state a wait just confirmed): an
  assertion.
- **That an element is there**: its lookup.

| State          | Read                     | Wait until                                       | Stays                                   |
|----------------|--------------------------|--------------------------------------------------|-----------------------------------------|
| Attribute      | `element.attr(name)`     | `element.wait_for_attr(name, Some(v))` (`None`: absent) | `element.attr_stays(name, v, d)` |
| DOM property   | `element.prop(name)`     | `element.wait_for_prop(name, v)`                 | `element.prop_stays(name, v, d)`        |
| Inner text     | `element.inner_text()`   | `element.wait_for_inner_text(t)`                 | `element.inner_text_stays(t, d)`        |
| Count          | `count(locator)`         | `wait_for_count(locator, n)`                     | `count_stays(locator, n, d)`            |
| Focus          | `page.focused_element()` | `page.wait_for_focus(&element)`                  | `page.focus_stays(&element, d)`         |
| Anything else  | any read                 | `assert_that!(read).eventually_ok().matches(..)` | `.consistently_ok().for_at_least(d)`    |

```rust
assert_that!(|| log.inner_text()).eventually_ok().matches(eq("change:200")).await;
page.settle().await?;
assert_that!(|| page.count(TOAST))
    .consistently_ok()
    .for_at_least(Duration::from_millis(1000)) // hovered, it stays: twice its timeout of 500 ms
    .matches(eq(1))
    .await;
```

- Observe the value the expectation is about, not a `bool` computed from it (a failure shows the values seen);
  several values as a tuple or struct. The observation's expression names it; `with_subject_name` only for runtime
  values in helpers.
- `eventually_ok`/`consistently_ok` for observations returning a `Result` (every helper read: an error is a failed
  observation, retried), `eventually`/`consistently` for plain values. Shared helpers use `.try_matches(..).await?`,
  cases `.matches(..).await`.
- Stays checks sample from the start, they don't settle first; they can miss changes between samples (a recording
  catches those). `page.settle()` lets two frames and a task pass: a scheduling barrier, not proof of quiescence.
- Inner text is `innerText`, trimmed (never thirtyfour's `text()`, which skips `display: contents` children).

Other reads: `is_selected`/`is_enabled`/`is_displayed`, `accessible_name()`/`accessible_description()`/
`accessible_role()` (the browser's computed values: assert the full name), `is_within("[inert]")`, `number_value()`,
`is_valid()` (fires nothing), `client_rect()` (viewport coordinates), `scroll_extent()`, `css_value(name)`,
`style_property("--x")`, `health::diagnostics(..)`.

### Assertions

assertr, with the assertion that states the expectation (leptonic enables its `thirtyfour-cdp` integration):

```rust
assert_that!(label).attribute("data-selected").await.is_none();          // optional attribute
assert_that!(input).has_attribute("type").await.is_equal_to("checkbox"); // required attribute
assert_that!(row).accessible_name().await.is_equal_to("Banana Yellow");
assert_that!(page.inner_texts(role(AriaRole::Option)).await?).contains_exactly(["Apple", "Banana"]);
assert_that!(rect.width).is_close_to(200.0, 0.5);
```

- Assert full known values (`is_equal_to`, `contains_exactly`), not parts (`contains`, `is_some`).
- Never wrap a comparison into a `bool` (`assert_that!(a == b).is_true()`); `is_true()`/`is_false()` only for values
  that are booleans. Assert on `Result`s directly (`.is_err()`, `.get_ok()`), not on unwrapped values.
- Convenience getters become projections (`.value()` → `.property("value").await`); parse inside the chain
  (`map_owned`, `derive_owned`) so failures keep the observation's context. Browser reads are snapshots, and protocol
  errors never become absence.
- No `.await` inside an assertion's arguments (the chain isn't `Send` across it): read into a local first, and finish
  a chain before the next suspension.

### Acting

Prefer typed thirtyfour and CDP operations to JavaScript; scripts only for what WebDriver can't (synthetic events,
mutation observers, atomic measurements), behind typed helpers in `tests/pages/` with values passed as
`arguments[n]`, never formatted into the script.

- **Pointer**: `click()`, `double_click()`, `hover()`, `page.click_at(x, y)` (an outside click); a held press:
  `let held = element.press_and_hold().await?`, `held.move_by(..)`/`move_to(..)`, `held.release()` (`#[must_use]`). A
  gesture whose pace matters is one action chain with timed moves (`ActionChain::new_with_delay`).
- **Keyboard**: `page.send_keys(keys)` (`Key::Shift + Key::Tab` holds Shift), `page.type_text(text)` (one key at a
  time), `page.hold_key(key, n)` (repeated `keydown`s), `element.type_keys(keys)` (focuses first, caret at the end);
  `element.focus()`, `page.blur_focused()`.
- **As assistive technology does**: `element.virtual_click()`, `element.virtual_input(value)`.
- **Forms**: `form.check_validity()` (fires `invalid`), `form.form_values(name)`, `form.submit()`, `form.reset()`.
- **Events WebDriver can't produce** (touch and pen pointers, wheel, drag, `beforematch`, custom), typed:
  `element.dispatch(SyntheticEvent::pointer(PointerKind::Down).pointer_type(PointerType::Touch))`,
  `SyntheticEvent::keyboard(..)`, `::wheel()`, `::mouse(..)`, `::plain(..)`; `dispatch_both(first, second)` fires two
  in one task; `page.dispatch_to(GlobalTarget::Window, event)`; `page.dispatch_all(..)` for sequences faster than
  WebDriver (a touch drag ending before a 200 ms timer); `count_synthetic_events(name)`.
- **Scrolling**: `scroll_into_view()`, `scroll_to_top(px)`, `scroll_by_steps(&steps, interval)` (paced by the page).
- **States only held during a gesture**: `element.record_attr("style")` (or `page.record_attr_of(selector, name)` for
  elements inserted later), act, `recording.finish()`.
- **Other platforms**: `page.emulate_platform(Platform::Mac)` (`Linux`, `IPhone`, `Android`) before `goto_path`
  (user agent and `navigator.userAgentData`; Chrome ignores the override's `navigator.platform`). A case about
  Mac-specific behavior emulates it; a case using modifiers takes the page's: `page.primary_modifier()` (Meta on a
  Mac, else Control), `page.non_contiguous_selection_modifier()` (Alt on Apple, else Control),
  `click_with_primary_modifier`. Never hard-code `Key::Control`: the suite runs on macOS and Linux.
- **A timer that must not fire early** (a tooltip delay): `page.start_stopwatch(&target, PointerKind::Enter,
  StopwatchEnd::Appears(sel))`, act, `stopwatch.finish()`: the duration on the page's clock.
- **Sleeps** only for real timers the test must let pass (a type-ahead reset), with a comment.

### Failure Reports

browser-test reports for every failure, with nothing in the test code: **where** (the test code's frames, from the
failing helper up to the case's line; helpers are `#[track_caller]`), **when** (the last steps with their timing) and
**what** (the error, or assertr's report: subject, expression, expected and actual value, how long it waited, and the
values observed when they changed). A new helper keeps this working: it names runtime values its expression can't show
(`with_subject_name`), focus checks name the focused element (`ElementActions::describe`), errors propagate with `?`.

## Performance Measurements

Every hook and atom has a **performance test** measuring what everything it offers costs, and the regular suite
checks the counts against budgets. Behavior tests don't measure (their steps change for behavior's sake, and their
fixtures render many variants). The metrics are defined and described once, in `testing/perf-metrics/src/lib.rs`:
heap `allocations`, `reallocations`, `allocated_bytes`, `largest_allocation`, `retained_bytes`; memo, effect, render
effect and immediate effect runs, signal notifications; DOM attribute, text, added and removed node mutations;
`layouts`, `style_recalcs`; `frames` until idle. Priorities: wasm size, then allocation count, then allocated bytes
(`conventions.md`).

Measured at the test-app's `wasm-release` profile (`opt-level = "z"`, debug assertions on), and that suffices
(compared 2026-10-16 on all 5,009 measurements): the counts are identical at `opt-level = 3` (the opt-level doesn't
change the work done), and debug assertions add only 0.05% of allocations and 0.6% of effect runs (274 measurements
differ, by at most 10%). So counts need no re-measuring per opt-level; wasm sizes: `build-performance.md`.

- **A performance test** is `leptonic/tests/perf_tests/<kind>/<subject>.rs` (`atoms/button.rs`,
  `hooks/use_press.rs`); its path is the subject's id (`perf/inventory.rs`) and its tests' names say it
  (`perf_tests::atoms::button::mount_and_unmount`). Each module registers its own tests in its `register` function.
  Reference: `perf_tests/atoms/button.rs`.
- **Its fixture** (`testing/test-app/src/pages/perf/<kind>/<subject>.rs`, a `PagePerf` component at
  `/perf/<kind>/<subject>`) renders the subject alone: an atom; a hook on a plain element, or with what it needs (a
  state hook with controls for its methods, an item hook in its collection); plain elements around it to tab from and
  to. **One fixture per subject** (the user, 2026-10-16): fixtures are never shared within a family (no shared
  fixture component or helper, no sibling's `PagePerf` re-exported; a parent the subject needs is written in its own
  fixture), and test modules share no helpers either. `PerfControls` are what the test does besides user input
  (`mount`, `unmount`, prop changes, reads), called with `page.control(name)` from a script, so the call allocates
  nothing in the page.
- **What it measures: everything the subject offers**, one test per offer, each from a fresh page: mounting and
  unmounting; every interaction (pointer: click, hold and release, hover and leave, drag; each key it handles; focus
  and blur by keyboard and pointer; typing; scrolling); every reactive prop and controlled state, each change and
  back; what it ignores where it decides that (a click while disabled); for subjects whose cost grows with their
  content, sizes that show how (10, 1,000, 10,000 items).
- **Measuring**: `page.measure("click", button.click())` wraps a step and returns its result; measurements of one
  name in a test add up. Check behavior afterwards as anywhere (the page is idle then). A measurement waits until the
  page is idle (a frame without mutations, allocations and reactive work; at most 60 frames), reads CDP's
  `Performance.getMetrics`, resets a `MutationObserver` and the app's counters (`window.leptonicCounters`,
  `pages/perf/counters.rs`: a counting global allocator and `reactive_graph::metrics`), runs the action, waits for
  idle again and reads everything (`tests/pages/measure.rs`).
- **What ran is reported, not listed**: `metrics::subject!()` records the hooks and atoms that ran per measurement
  (`ran` in the budget), and every measurement must have run its test's subject (else: does the fixture render it,
  does it start with `subject!()`?). A new internal dependency shows in the budget's diff.
- **Budgets** (`leptonic/tests/budgets/<kind>/<subject>.json`, checked in): per test and measurement what ran, how
  often it was measured and every budgeted metric's total. Counts are exact: above the budget is a regression, below
  an improvement to lock in, and both fail until recorded again: `BROWSER_TEST_PERF_BUDGETS=update
  BROWSER_TEST_FILTER=<test>` (`BROWSER_TEST_PERF_BUDGETS_UPDATE=<filters>` records the matching tests and checks the
  others in the same run). Review the budget diff like code. Re-record after changing Leptos or other dependencies,
  the wasm profile or Chrome (layouts and style recalculations may change with it).
- **Unbudgeted**: `retained_bytes` (freed whenever the browser collects garbage) and `frames` (a late frame takes in
  work that otherwise spans two) change under load; `allocated_bytes` and `largest_allocation` depend on the heap's
  layout: Leptos' subscriber sets (`IndexSet<AnySubscriber>`, hashed by address) allocate entries for their hash
  table's capacity when cloned (every signal notification) or grown, and tombstones from removals, placed by the
  addresses, lower it. Any change to what the page allocated earlier, or a free landing later, shifts them by multiples
  of 16 bytes with equal counts (traced per allocation, 2026-10-16); one build repeated them exactly in two runs.
  Every other count repeated exactly in every run, also with all CPU threads busy; debug assertions barely change
  them (5 of 337,154 allocations).
- **Coverage** (`perf::budgets::PerformanceCoverage`, after every run; problems fail unfiltered runs only): every hook
  (`pub fn use_*` in `src/hooks/`) and atom (`#[component] pub fn` in `src/atoms/`) has a performance test with
  budgets, or is listed in `leptonic/tests/budgets/unmeasured.txt`, which only shrinks: a new hook or atom comes with
  its performance test. Update runs remove budgets of tests that no longer exist.
- **Scenarios**: workloads of many actions (a long list growing), as a fixture's `PerfScenarios` calling
  `Measurements::measure(metric, action)` per action; record what the run did (`with_parameter`) and warm up before
  measuring. The regular suite counts each action (`page.count_perf_scenario(name)`), the performance suite times
  them (`perf_tests/atoms/virtual_list.rs`).
- **Timings** (`just browser-perf`): the scenarios' timings on the page's clock, one case after the other, in a build
  without debug assertions and overflow checks (`<target>/performance`). Each action is timed `.settled` (until every
  effect it caused ran) and `.render` (the next frame's own work); statistics go to
  `<target>/tmp/performance/<fixture>/<scenario>.json`, each median compared with the previous run's. Timings depend
  on the machine and its load (other builds moved medians by up to 30%, full load by 80-200%), so they have no
  budgets: compare runs on one quiet machine, before and after, twice. The clock's resolution is 0.1 ms.
- **`reactive_graph::metrics`**: Leptos counts no reactive work, so the test-app patches `reactive_graph`
  (`[patch.crates-io]`) with the user's fork at `~/dev/leptos`, branch `leptonic-metrics` (v0.8.22 plus a `metrics`
  feature), with the crates it takes from that workspace (`any_spawner`, `hydration_context`, `or_poisoned`,
  `throw_error`). The test-app builds only while that checkout is on the branch. When leptonic updates Leptos, rebase
  the branch onto the new release, or drop the patch once upstream has the counters.

### Keeping Measurements Deterministic

A measured step must start from the same state and contain no timer whose firing depends on timing:

- The live announcer's first announcement waits on a 100 ms timer: fixtures whose subject announces (selection,
  sorting, spin buttons, drag and drop) announce `""` at mount and wait for it; sort tests also wait out the
  announcement's 500 ms removal.
- `use_dialog` starts a 500 ms blur-and-refocus timer per opened dialog (react-aria's VoiceOver workaround): wait
  ~600 ms before measuring. Tooltips have a 500 ms global cooldown: one opening per test. Number field steppers
  auto-repeat after 400 ms: click, don't hold. Hold tests use fixtures without row actions (no long-press timer).
- Scrolling keeps `is_scrolling` until 300 ms after the last event: scroll measurements include the end, others wait
  for it unmeasured. Overscan direction depends on the previous scroll's timing: measure downward scrolls only.
- WebDriver's default pointer moves take 250 ms with a varying number of `pointermove`s: drag with
  `action_chain_with_delay(None, Some(Duration::ZERO))`. Keys sent together may land in different frames: one key per
  measured step.
- Animations started inside a step vary style recalculations: start them at mount (paused), finish them in the step
  with `getAnimations().finish()`.
- The first key press on a page allocates ~40 times once: keep the step that presses it the same.
- Fixture controls read hook state with `untrack` (else Leptos warns, failing page health); a part unmounted by a test
  whose element a parent references is mounted again before the test ends (dangling references fail page health).
- Lookups by text in lists of 1,000 rows can exceed 5 s: select rows by a `data-*` attribute.
- WebDriver reads boolean attributes (`disabled`, `hidden="until-found"`) as `"true"`: read the property where the
  value matters. `checkValidity()` fires `invalid` (committing a native-mode field): read `validationMessage`.

## Speed and Memory

What the suite's speed rests on (measured 2026-10-08/09; re-measure before relying on a number):

- Every parallel test is a browser (~0.4 GB with Chrome Headless Shell, which also resets sessions in 25 ms instead of
  Chrome's 100 ms); above parallelism 8 the CPU saturates and the run barely gets faster.
- The test-app is always built `--release` (`wasm-release`: `opt-level = "z"`, no LTO, incremental, debug
  assertions), by the suite and `just serve-test-app` alike: dev builds made page loads 2.5 times slower; LTO with one
  codegen unit saved 1 MB but made a rebuild after a library change take 84 s instead of 20 s. Its server renders at
  `opt-level = 1` (`server-release`: 6x faster rendering, no slower rebuilds).
- Page loads (navigation and hydration, ~100 ms) dominate the test bodies' time: the wasm and JS are content-hashed
  and cached across session resets, `goto_path` waits for hydration in the page (a `MutationObserver`), big fixtures
  render only the needed `Section`s, and the ICU4X data covers only the fixtures' locales (`just test-app-icu-data`;
  a fixture in another locale silently falls back to its parent's data).
- Most waits pass at their first read; a shorter poll interval made the CPU-bound run no faster.
- The wasm isn't compressed: over loopback it downloads in ~35 ms, while decompressing costs ~30 ms of CPU per load.
- Tried without gain: V8 without wasm tier-up, a minimal HTTP cache, longest-tests-first ordering, mold (the user
  decided against it).
