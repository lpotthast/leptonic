# Testing

Tests are the definition of done (`conventions.md`): pure logic and `*_state` hooks get native unit tests, DOM
behavior gets browser tests against `testing/test-app`, both derived from react-aria's own tests (`porting.md`,
"Tracking Upstream Changes") and asserting with `assertr`. Every bug fix starts with a failing test, and browser tests
are run, not only compiled, before work counts as done.

A successful build can still emit Rust warnings. Run library feature checks with `cargo clippy ... -- -D warnings`,
including default, no-default, atoms and SSR configurations, and inspect the actual `cargo leptos build` output for
metadata and dependency warnings. Keep client-only code behind its feature guards rather than suppressing unused-code warnings.

## Native Tests

Pure logic and `*_state` hooks get native unit tests (`cargo test -p leptonic --features full --lib`), with
`assertr`. A hook needs a reactive owner; run it through `crate::testing::with_owner` (`leptonic/src/testing.rs`,
test builds only), which also makes Effects run:

```rust
use crate::testing::{flush_effects, with_owner};

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

- `with_owner(f)` runs `f` in a fresh `Owner` (as a component body runs), without Leptos' "read outside a tracking
  context" warnings, and disposes the owner afterwards.
- `flush_effects()` runs every pending Effect of the test's thread until none can make progress: the first runs of
  new Effects, and the re-runs caused by signal changes. Effects never run on their own, so a test decides when (like
  React's `act`); call it after creating the state and after each change whose Effects matter.
- How: leptonic's dev-dependencies enable `reactive_graph`'s `effects` feature (Leptos enables it only for
  `csr`/`hydrate`), and `with_owner` installs an `any_spawner` executor that queues every task on the spawning
  thread, polled by `flush_effects`. Tests under a plain `Owner::new().with(..)` keep working; their Effects don't run
  (`any_spawner`'s `tracing` feature drops tasks spawned before an executor exists instead of panicking).
- Signals and memos work without `flush_effects`; only Effects need it. Code touching the DOM can't run natively:
  that is the browser tests' job.

**Reference**: `hooks/grid/use_grid_state.rs` (tests of the refocus Effect).

## Browser test setup

The browser tests live in `leptonic/tests/` (entry `tests/browser_test.rs`) and drive the test-app in
`testing/test-app/`. They use `leptos-browser-test` (starts `cargo leptos serve --release` on a random port: the
test-app's `wasm-release` profile, `opt-level = "z"` without LTO, and `server-release`, `opt-level = 1`: rebuilds as
fast as dev, pages load and render much faster) and `browser-test` (Chrome for Testing + chromedriver with Chrome
Headless Shell, 8 tests in parallel by default, `thirtyfour` re-exported as `browser_test::thirtyfour`).

- **Fixtures**: every test page is a module under `testing/test-app/src/pages/{atoms,hooks}/`, registered in
  `FIXTURES` (`testing/test-app/src/pages/mod.rs`) and served at `/{group}/{name}`. Large pages render `Section`s;
  `?only=<name>,...` (`page.goto_sections`) renders only the ones a case needs. The test-app sets `data-hydrated` on
  `<body>` once hydrated; `page.goto_path`/`goto_sections` wait for it, so no case acts before the handlers exist.
- **Cases**: every case is a `#[browser_test] pub async fn` in `tests/ui_tests/test_*.rs` that loads its page
  itself; cases never depend on each other or on shared server state. `#[browser_test]` (browser-test-macros)
  generates a unique PascalCase struct implementing `BrowserTest`, named after the module-qualified function
  (`name = "..."` overrides it), its doc comment the description. Registration is explicit, in
  `tests/ui_tests/mod.rs`: `TestGroup`s (logical groups may span modules) of `.with(fixture_test(test_x::Case {}))`;
  `ui_tests::all()` adds a sequential "after all" group for checks of the whole run. The harness (`tests/harness/`,
  `fixture_test`) supplies the page, checks its health and cleans up; helpers in `tests/pages/` (`Page`,
  `ElementActions` on thirtyfour's `WebElement`, `Locator`s, `SyntheticEvent`, `Stopwatch`), fixture navigation and
  the declared fixture defects in `tests/fixtures/`.
- **Sessions**: sessions return to browser-test's pool after a test and are reset for the next one, item by item,
  keeping the HTTP cache of the test-app's content-hashed wasm and JS (`SessionReset::manual([CachedData::Http])`);
  `BROWSER_TEST_SESSION_RESET=new-context` gives every test a new browser context instead (to cross-check),
  `BROWSER_TEST_SESSION_REUSE=0` a fresh browser.
- **Page health and failures**: after every test the page's health is checked (`tests/pages/health.rs`, policy in
  `tests/fixtures/expectations.rs`): Rust panics, uncaught errors, `console.error`/`console.warn`, duplicate ids, id
  references to missing elements and literal `attr:` attributes fail it; a case expecting a warning consumes it with
  an exact count (`take_warnings`). After a Rust panic, waits give up at once and the panic is the test's error. The
  runner (`FailurePolicy::RunAll`) reports every failing test with its failing test-code line and callers, its last
  steps, and the expected and last seen value; assertions use assertr, helpers return `Result<_, rootcause::Report>`.
- **Whole-run checks**: `test_hydration_ids.rs` compares server-rendered ids (and their elements' tag and role) with
  the hydrated DOM and checks the health of every fixture the test-app's index lists, incl. its query variants
  (`QUERY_VARIANTS` in `testing/test-app/src/app.rs`: add one when a case loads a new one), in 4 shards.
  `test_server_panics.rs` runs last (also in filtered runs, not with `BROWSER_TEST_KNOWN_ISSUES=1`) and fails the
  run if the server panicked, naming each panic's request, location and message.
- **Known issues**: cases of behavior known to be broken are registered only in the known-issues list
  (`known_issues_tests` in `ui_tests/mod.rs`, run with `BROWSER_TEST_KNOWN_ISSUES=1`); move them to the regular
  groups once fixed.
- **Running**: `just browser-test` (`cargo test -p leptonic --test browser_test`; a plain `cargo test` or `just test`
  skips the suite, `[[test]] test = false`). Variables (documented in `tests/browser_test.rs`):
  `BROWSER_TEST_VISIBLE=1` shows the browser (`just browser-test-visible`), `BROWSER_TEST_PAUSE=1` pauses before each
  test, `BROWSER_TEST_DRIVER_OUTPUT=1` forwards chromedriver output, `BROWSER_TEST_APP_OUTPUT=1` the test-app's
  (its build and server logs), `BROWSER_TEST_LOG=<filter>` chooses the logs (below),
  `BROWSER_TEST_PARALLELISM=<n>` (`1`: sequential; every parallel test is a browser, "Speed and memory"); selection:
  "Selecting tests". `TEST_APP_TARGET_DIR=<dir>` builds the test-app and its site (`<dir>/browser-test-site`)
  there instead of in the inherited `CARGO_TARGET_DIR` (agents: `CARGO_TARGET_DIR=<repo>/target/agents
  TEST_APP_TARGET_DIR=<repo>/testing/test-app/target/agents`). Two suites with the same target dir must never run at
  the same time; with different ones they can. Ctrl-C cancels a run cleanly. Sessions keep their Chrome profiles in
  `<target>/tmp/browser-test-profiles` (never in `/tmp`, a RAM disk here).
- **Logs**: failure reports and the run summary explain a failing test, so a run logs little: milestones, warnings and
  errors at `info` (the default of `BROWSER_TEST_LOG`, which the `just` recipes set explicitly, to edit there).
  Noisy events belong at `debug`: each test's start and timing, every step with its duration (no per-step slowness
  warnings; the summary lists the slowest steps). `BROWSER_TEST_LOG` takes a level (`debug`) or levels per target
  (`info,browser_test::step=debug`), as `tracing-subscriber`'s `Targets` parses them; a plain `debug` includes tokio's
  events (`debug,tokio=warn,runtime=warn`). One line per event.
- **Keyboard layout**: typing doesn't depend on the host's keyboard layout. chromedriver alone would type through the
  host's active input source (on macOS with a German layout, `/` arrives as Shift+7 with `key` `&` and `z` as `y`),
  so the helpers (`pages/keyboard.rs`) type every character a US keyboard has through CDP (`Input.dispatchKeyEvent`),
  as that keyboard produces it: `key`, `code`, `keyCode` and text of its key, with a Shift `keydown`/`keyup` around
  the characters that take Shift (`?` is Shift+Slash). Text fields get the text and their `input` events, type-ahead
  sees the right `key`. Named keys (`Key::Enter`, `Key::Tab`, arrows) and chords (whatever follows a modifier, up to
  `Key::Null`) stay on WebDriver, which sends them the same on every layout. So tests type only through
  `page.send_keys`/`page.type_text` and `element.type_keys`, never thirtyfour's `WebElement::send_keys`.
- **Toolchain**: the installed `wasm-bindgen` CLI must match the `wasm-bindgen` version in the test-app's
  `Cargo.lock`, otherwise `cargo leptos serve` fails: update the lockfile (`cargo update -p wasm-bindgen -p js-sys
  -p web-sys -p wasm-bindgen-futures`) or the CLI.

## Writing browser tests

How the browser tests in `leptonic/tests/` are written (setup, fixtures and registration: "Browser test setup"
above). Reference file: `ui_tests/test_checkbox.rs`. The helpers (`tests/pages/`) are a concrete `Page`, extensions on thirtyfour's
`WebElement`, locators and typed events. `tests/fixtures/` implements `Page::goto_path` and `Page::goto_sections`,
including hydration, fixture conventions and health policy (the declared fixture defects: `EXPECTED` in
`fixtures/expectations.rs`); `tests/harness/` (`fixture_test`) wraps every case: page context, health check, cleanup.
`tests/browser_test.rs` installs assertr's defaults; assertr owns polling, deadlines and failure history.

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
#[browser_test]
pub async fn selected_state(page: &Page<'_>) -> Result<(), Report> {
    // Every case loads its page itself.
    page.goto_path(PATH).await?;
    // Lookups wait until the element exists.
    let label = page.element(css("label").text("Basic")).await?;
    let input = label.element("input").await?;
    let value = page.element("#test-cb-basic-value").await?;
    // Nothing happened yet: read once, assert with assertr.
    assert_that!(label).attribute("data-selected").await.is_none();
    assert_that!(input).selected().await.is_false();

    // An interaction's effects arrive later: wait for them.
    label.click().await?;
    label.wait_for_attr("data-selected", Some("true")).await?;
    value.wait_for_inner_text("true").await?;
    // The wait proved the state settled: related values are assertions again.
    assert_that!(input).selected().await.is_true();
    Ok(())
}

/// A disabled checkbox ignores presses.
#[browser_test]
pub async fn disabled_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element(css("label").text("Disabled")).await?.click().await?;
    // "Nothing happens" holds over a time window, not at one instant.
    let value = page.element("#test-cb-disabled-value").await?;
    value.inner_text_stays("false", Duration::from_millis(100)).await?;
    Ok(())
}
```

## Finding elements

A `Locator` says which elements; a plain `&str` or `String` is a CSS selector.

| Locator                          | Means                                                                          |
|----------------------------------|--------------------------------------------------------------------------------|
| `"#id"`, `"[role=grid] td"`      | elements matching the CSS selector                                             |
| `role(AriaRole::Option)`         | elements with the ARIA role (leptonic's `AriaRole`): the browser-computed accessibility role, respecting hidden/inert content and explicit overrides |
| `css("label").text("Basic")`, `role(AriaRole::Option).text("Apple")` | only those whose text content (whitespace collapsed) equals the text; CSS can select hidden elements |
| `role(AriaRole::Row).has(role(AriaRole::Gridcell).text("Inbox"))` | only those containing an element the inner locator matches (relations: "the row with this cell") |

Lookups use typed WebDriver operations. `role(..)` asks the browser for the computed role, including native
semantics and explicit overrides. Text filters read `textContent` and collapse whitespace; use an accessible-name
assertion when testing labeling. CSS relations (`:has(...)`, `:scope`) remain CSS.

Find elements as users do: by role and text, by label; ids for the fixture's own outputs and controls.

The same lookups exist on a page (in the document) and on an element (below it):

| Lookup                       | Does                                                                                 |
|------------------------------|--------------------------------------------------------------------------------------|
| `element(locator)`           | exactly one match, waiting while there is none; several matches fail immediately |
| `first_element(locator)` | explicitly select the first match in document order (also waits for one) |
| `elements(locator)`          | the matches now (maybe none)                                                         |
| `count(locator)`             | the number of matches now                                                            |
| `inner_texts(locator)`       | the matches' inner texts now                                                         |
| `wait_for_count(locator, n)` | waits until `n` match (`0`: gone); before `elements` when a list is still appearing  |
| `count_stays(locator, n, duration)`    | `n` match and keep doing so                                                          |

Tests may use thirtyfour's `WebElement` directly. Shared locators add strict uniqueness, diagnostics and waits;
raw `find`/`find_all` are immediate because the session's implicit wait is zero. Node handles retain identity.
For a component that replaces its node, use `page.wait_for_attr(locator, name, expected)`, which re-resolves each read.

## Checking: four layers

| Layer          | Checks                                       | Fails with                                                    |
|----------------|----------------------------------------------|---------------------------------------------------------------|
| 1. Lookups     | the element exists (and returns it)          | `Report` containing an assertr failure naming the locator     |
| 2. Waits       | a state is reached, within 5 s               | `Report` containing an assertr failure with observed values and timing |
| 3. Stays       | a state holds over an explicit positive duration | `Report` containing an assertr failure with the rejected value and timing |
| 4. Assertions  | a value read now                             | assertr panic: the expression, the expected and actual value  |

Waits and stays are assertr's eventual assertions (`eventually`, `consistently`) on an observation of the page; the
state helpers below use them, too.

Which one, by what the check is about:

- **The effect of something the test just did** (a click, a key, a script): a wait. If the effect is that nothing
  changes: a stays check. Never an assertion: it reads before effects, animation frames and timers ran.
- **A state nothing is changing** (the initial render, values related to a state a wait just confirmed): an assertion.
- **That an element is there**: its lookup.

**Stays checks** sample immediately and throughout an explicit, positive duration. They do not settle first.
Use a duration extending past the timer under test, with a comment naming it. These are sampled observations;
a mutation recording is needed to detect transient changes between samples. `page.settle()` only lets two animation
frames and a task pass. It is a scheduling barrier, not proof that the page is quiescent. For a snapshot after this
barrier, call `settle()` and then assert on a single read. There is no environment switch that changes assertion strength.

### The states

| State          | Read                                   | Wait until                              | Stays                                 |
|----------------|----------------------------------------|-----------------------------------------|---------------------------------------|
| Attribute      | `element.attr(name)`                   | `element.wait_for_attr(name, Some(v))` (`None`: absent) | `element.attr_stays(name, expected, duration)` |
| DOM property   | `element.prop(name)`                   | `element.wait_for_prop(name, v)`        | `element.prop_stays(name, v, duration)`         |
| Inner text     | `element.inner_text()`                 | `element.wait_for_inner_text(t)`        | `element.inner_text_stays(t, duration)`         |
| Count          | `count(locator)`                       | `wait_for_count(locator, n)`            | `count_stays(locator, n, duration)`             |
| Focus          | `page.focused_element()`               | `page.wait_for_focus(&element)`         | `page.focus_stays(&element, duration)`          |
| Anything else  | any read                               | `assert_that!(read).eventually_ok().matches(..)` | `assert_that!(read).consistently_ok().for_at_least(duration).matches(..)` |

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
    // Hovered, it stays: twice its timeout of 500 ms.
    .for_at_least(Duration::from_millis(1000))
    .matches(eq(1))
    .await;
```

- Observe the value the expectation is about, not a `bool` computed from it: a failure shows the last value and the
  values seen before it. Several values: a tuple or a struct.
- The failure shows the observation's expression (`|| log.inner_text()`), so it names itself. `with_subject_name` only
  for a runtime value the expression can't show, in helpers: `format!("attr {name}")`.
- `eventually_ok`/`consistently_ok` for observations returning a `Result` (every helper read): an error is an
  observation that failed, retried until the timeout by `eventually_ok`. `eventually`/`consistently` for plain values.
- The expectation: `matches(eq(v))`, another matcher (`ge`, `all_of`, ...), or `satisfies(|it| { .. })` with any
  assertr assertion. `predicate(..).described_as(..)` only when no typed assertion says it.
- The timing is the suite's (`Patience` in `tests/browser_test.rs`: 5 s, every 20 ms); `within(d)` and
  `for_at_least(d)` override it for one check. Every consistency assertion states its observation duration.
- Direct case assertions use `.matches(..).await` and panic on failure. Fallible shared helpers use
  `.try_matches(..).await?`, which returns the observed value or a structured assertion failure. Both paths use
  assertr's same execution engine. Fixed-node helpers stop immediately on read errors. Locator waits also retry candidates replaced during a query; stale fixed roots and other protocol errors remain terminal.

The inner text is `innerText`, trimmed: what the element shows (never thirtyfour's `text()`, which skips
`display: contents` children). Waits pass the moment the state is reached, so waiting for a state that may already hold
costs nothing. An element handle waits on that element; if the page replaces it (re-rendering), the wait gives up at
once with a stale element error, which the failure shows: look the element up after the change. Waits also give up as
soon as the page panicked (Rust): the panic is then the test's error, not a timeout (see "Page health").

### Other reads

| Value                                          | Read with                                                                |
|------------------------------------------------|--------------------------------------------------------------------------|
| Checked or selected, enabled, displayed        | `element.is_selected()`, `element.is_enabled()`, `element.is_displayed()` |
| Accessible name                                | `element.accessible_name()` (the browser's computed name: assert the full name) |
| Accessible description                         | `element.accessible_description()` (the browser accessibility tree's computed description) |
| Computed role                                  | `element.accessible_role()`                                              |
| Inside an element matching a selector          | `element.is_within("[inert]")` (itself or an ancestor)                   |
| An input's value as a number                   | `element.number_value()`                                                 |
| Native validity                                | `element.is_valid()` (fires nothing)                                     |
| Position and size                              | `element.client_rect()` (viewport coordinates; `rect()` uses document coordinates)        |
| Scroll position and extent                     | `element.scroll_extent()` (`scrollTop`, `clientHeight`, `scrollHeight`, read at once) |
| CSS property                                   | `element.css_value(name)`; a custom property (`--x`) set in the element's `style`: `element.style_property(name)` |
| What the page reported                         | `health::diagnostics(page.low_level().driver())`: panics, errors and warnings |

### Page health

`pages::health` collects runtime diagnostics and DOM problems. `fixtures::check_health` applies policy after each
case and before fixture navigation. `Page::goto_path` waits for hydration, validates instrumentation, and records
the declared initial fixture defects. Warning messages and counts are explicit, including per-section counts for
partial fixtures. Reference exceptions match an exact suffix and count. Missing or extra occurrences fail initial
validation; later checks accept only the recorded messages and multiplicities.

A case expecting warnings uses `fixtures::take_warnings(page, containing, count)` after its assertions. This consumes
only that warning set when its count matches and retires its initial allowance; it cannot erase panics, console errors or other warnings. Missing
instrumentation is an error. The harness cleans held input, recorders and stopwatches on normal return, errors and
assertion panics. The runner owns session reset and cancellation cleanup.

### Assertions

assertr, with the assertion that states the expectation, so a failure shows the actual value:

```rust
assert_that!(label).attribute("data-selected").await.is_none();
assert_that!(input).has_attribute("type").await.is_equal_to("checkbox");
assert_that!(group).accessible_description().await.is_equal_to("Pick your pets.");
assert_that!(row).accessible_name().await.is_equal_to("Banana Yellow");
assert_that!(page.inner_texts(role(AriaRole::Option)).await?).contains_exactly(["Apple", "Banana"]);
assert_that!(page.count("[role=row]").await?).is_equal_to(3);
assert_that!(input).selected().await.is_true();
assert_that!(rect.width).is_close_to(200.0, 0.5);
```

Leptonic enables assertr's `thirtyfour-cdp` integration. `has_attribute(name).await` asserts presence
and exposes the string to ordinary assertions such as `starts_with`, `contains`, and `is_equal_to`.
`attribute(name).await` preserves the optional value, so absence uses `.is_none()`. Browser reads
are snapshots, and protocol errors never become absence. For changes over time, use a fresh read
closure with the existing waits or eventual/consistency builders. The shared typed accessibility
and inner-text reads live in `assertr::assertions::thirtyfour::read`; local adapters preserve the
fixture helpers' error type.

Convenience getters follow the same rule: `.value()` becomes `.property("value").await`,
and `.id()` or `.class_name()` becomes an attribute projection. Keep parsing and other transformations
inside the chain with `map_owned` or `derive_owned`, so failures retain the browser observation context.
When a validated snapshot is needed after an interaction, finish the chain and copy its `.actual()` value
before the next suspension. Raw reads remain useful for polling, expected values, locator construction,
and observations outside the shared integration's surface, such as geometry and CSS values.

- Assert full known values (`is_equal_to`, `contains_exactly`), not parts (`contains`, `is_some`), where the value is
  known and stable.
- Never wrap a comparison into a `bool` (`assert_that!(a == b).is_true()`): use `is_equal_to`, `is_empty`,
  `has_length`, `is_greater_than`, `is_close_to`, ... `is_true()`/`is_false()` only for values that are booleans.
- Assert on `Result` values directly in helper contracts: `.is_err()`, `.is_ok()`, `.get_err()` or
  `.get_ok()`. Avoid asserting on `.is_err()` booleans or unwrapping before creating the chain; that
  discards the error or successful value that assertr can explain. Decompose predicates with `satisfies`
  and `derive` when ordinary assertions can describe each condition.
- For required attributes, use `.has_attribute(name).await` and ordinary string assertions. Optional
  attributes use `.attribute(name).await.is_none()` or compare the optional value directly.
- No `.await` inside an assertion's arguments (the chain is not `Send` across an await): read into a local first.
  Eventual assertions and assertr's browser projections await their own observations safely. Finish the
  returned ordinary assertion chain before another suspension.

## Acting

Prefer typed thirtyfour and CDP operations to raw JavaScript. Tests can work directly with `WebElement` and use
`ElementActions` for reusable operations. Keep fixture adapters in `tests/fixtures/`, not on every element or page.
Share operations when their semantics recur; a one-off protocol operation can use `page.low_level().driver()`.
Use scripts for capabilities WebDriver does not provide, such as synthetic events, mutation observers, and atomic
DOM measurements. Encapsulate reusable scripts behind typed arguments and return values.

- **Pointer**: `element.click()`, `element.double_click()`, `element.hover()`, `page.click_at(x, y)` (on whatever is
  at that viewport point: an outside click); a press held while the test checks the pressed state or moves:
  `let held = element.press_and_hold().await?` (or `press_and_hold_at(x, y)` from the center), `held.move_by(x, y)`,
  `held.move_to(&other)`, `held.release()` (`#[must_use]`: a forgotten release leaves the button down). A gesture
  whose pace matters (moves a throttle or debounce must see apart) is one action chain with timed moves
  (`ActionChain::new_with_delay`), so WebDriver latency can't stretch it.
- **Keyboard**, to the focused element: `page.send_keys(keys)` (`Key::Shift + Key::Tab` holds Shift),
  `page.type_text(text)` (one key at a time, for inputs that move focus while typing), `page.hold_key(key, n)`
  (repeated `keydown`s); to an element: `element.type_keys(keys)` (focuses it first, a text field with the caret at
  the end, as WebDriver's send keys does). Characters arrive as a US keyboard types them, whatever the host's layout
  ("Browser test setup", "Keyboard layout"). Focus: `element.focus()`, `page.blur_focused()`. The focused-element
  lookup descends into open shadow roots before sending native element keys, preserving browser shortcuts such as
  Shift+F10.
- **As assistive technology does** (from script): `element.virtual_click()`, `element.virtual_input(value)`.
- **Forms**: `form.check_validity()` (fires `invalid`), `form.form_values(name)`, `form.submit()`, `form.reset()`.
- **Events WebDriver can't produce** (touch and pen pointers, wheel, drag, `beforematch`, custom events), built
  typed: `element.dispatch(SyntheticEvent::pointer(PointerKind::Down).pointer_type(PointerType::Touch))`,
  `SyntheticEvent::keyboard(KeyKind::Down, "a").repeat(true)`, `SyntheticEvent::wheel().delta_y(10.0)
  .modifiers(&[Modifier::Control])`, `SyntheticEvent::mouse(MouseKind::ContextMenu).at(x, y)
  .button(MouseButton::Secondary)`, `SyntheticEvent::plain(EventKind::BeforeMatch).bubbles(false)`. Each interface
  has only its own setters; pointer events come from the primary pointer (a mouse unless `pointer_type` says
  otherwise). The result says whether a handler prevented the default.
  `element.dispatch_both(first, second)` fires two in one task, as the browser fires an event and the one it causes
  (no timer runs in between); `page.dispatch_to(GlobalTarget::Window, event)` (or `Document`) for a `resize` or a
  document `scroll`. `element.count_synthetic_events(name)` counts the script-dispatched (`isTrusted` false) events of
  a type the element gets, `finish()` returns the count.
- **Scrolling**: `element.scroll_into_view()`, `element.scroll_to_top(px)`; a user scroll of several steps whose pace
  matters (a scroll-end debounce): `element.scroll_by_steps(&steps, interval)`, paced by the page's timers.
- **States that only hold during a gesture** (a style while the pointer moves): `let recording =
  element.record_attr("style").await?`, perform the gesture, then `recording.finish()`: every value the attribute
  took, recorded in the page. `page.record_attr_of(selector, name)` also records elements inserted later (an overlay
  opening).
- **Other platforms**: `page.emulate_platform(Platform::Mac)` (or `Linux`, `IPhone`, `Android`) before `goto_path`:
  the user agent and `navigator.userAgentData` leptonic's platform checks read (Chrome 155 ignores the override's
  `navigator.platform`); session resets end it. Without emulation the browser reports the host. A case about
  behavior that differs on a Mac (the `ShortcutKeys` labels, Ctrl+Enter, Alt in drag and drop) emulates the
  platform it describes; a case using modifiers takes the page's: `page.primary_modifier()` (select all,
  Ctrl+Home/End, `Mod` shortcuts: Meta on a Mac, else Control), `page.non_contiguous_selection_modifier()` (moving
  through a collection without selecting: Alt on Apple devices, else Control), `page.click_with_primary_modifier`.
  Never hard-code `Key::Control` for these: the suite runs on macOS and Linux hosts.
- **Sequences faster than WebDriver** (a touch drag that must end before a 200 ms timer): `page.dispatch_all(vec![(
  &element, event), ..])` dispatches them in one script.
- **Behavior behind a timer that must not come early** (a tooltip's delay, a touch drag delay):
  `page.start_stopwatch(&target, PointerKind::Enter, StopwatchEnd::Appears(sel))` (or a plain `EventKind` start;
  `Disappears(sel)`, `AttributeChanges { element, name }` ends), act, then `stopwatch.finish()`: the `Duration` from
  the event to the end, measured on the page's clock, without WebDriver round trips.
- **Anything else in the page** (recording listeners, app state on `window`, `DataTransfer`s): a typed helper in
  `tests/pages/` (or a fixture that exposes the state as DOM the helpers can read); the helper may use
  `page.low_level().eval::<T>(script, vec![..])` (or `eval_async`) with values passed as `arguments[n]`, never
  formatted into the script. One-off fixture setup may use the same escape hatch directly.
- **Sleeps** only for real timers the test must let pass (a type-ahead reset, a long-press delay), with a comment.

## File layout

```rust
// Upstream: react-aria-components/test/Checkbox.test.js @ 99e6102368
//! What the cases cover. Spec: react-aria-components `Checkbox.test.js`.
use assertr::prelude::*;
use browser_test::browser_test;
use rootcause::Report;

use crate::pages::{ElementActions, Page, css};

const PATH: &str = "/atoms/checkbox";

// File-local lookups, then one documented `pub async fn` per case.

/// Pressing the label toggles the checkbox.
#[browser_test]
pub async fn selected_state(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    // ...
    Ok(())
}
```

- Every case is a test of its own, explicitly registered in `ui_tests/mod.rs` with
  `.with(fixture_test(test_checkbox::SelectedState {}))`. Each function is annotated with `#[browser_test]`,
  which generates a unique PascalCase struct implementing `BrowserTest`. Its default name is the module-qualified
  function name
  (`browser_test::ui_tests::test_checkbox::selected_state`); no name string is repeated in the registry.
  `#[browser_test(name = "checkbox::selected")]` can override the filterable name. Doc comments remain on the
  struct and are available through `BrowserTest::description()`, separate from the name.
  Existing substring filters such as `checkbox::selected_state` still match. `fixture_test` wraps the test in
  `FixtureTest`, which supplies the `Page` context, applies fixture health policy and cleans up. `TestGroup` membership
  is explicit here, independent of module placement. Groups can combine functions from several modules (for example,
  `table` includes navigation, selection, resizing and tree tests).
  All group members share the suite's parallelism limit. This central registry is intentionally large.
  Each case loads its page first (`page.goto_path(PATH)`, or a local
  `async fn open(page)` when the page needs more, e.g. a permission or a window size), so what it runs on is visible
  in the case itself. A case taking parameters gets a named case per variant (`provides_slots_input`).
- **Sections.** A fixture made of many independent parts wraps each in the test-app's `Section` (`pages/mod.rs`),
  named like the part. A case loads only the sections it uses: `page.goto_sections(PATH, &["basic"])` loads
  `PATH?only=basic` (the calendar page renders 1 of 85 sections). Without `only` every section renders: the
  hydration test and manual inspection see the whole page. 14 fixtures use them (calendar, date field, date picker,
  select behavior, tree cases, ...).
- **Real timers.** Cases wait for real time (timeouts, delays, thresholds), so fixtures configure short ones where
  the case's purpose allows (toast 500 ms, tooltip close delay 400 ms), and keep upstream's default where the default
  itself is the behavior (the 500 ms long-press threshold).
- **Cases are independent.** Each one runs in a reset browser (browser-test's session reuse with
  `SessionReset::manual`: the tab is reset item by item, keeping only the HTTP cache of the test-app's content-hashed
  wasm and JS, so no cookies, storage, permissions or held keys of other tests) and never relies on what another case did: it sets up the state it
  needs itself, or expects the fixture's initial state. `BROWSER_TEST_FILTER=checkbox::hover` runs a single case. A
  flow whose steps belong together is one case.
- Every case has `#[browser_test]` on `pub async fn case(page: &Page<'_>) -> Result<(), Report>`, ends with `Ok(())`, and has a doc comment:
  one or two human-readable sentences saying what the case tests, as the expected behavior ("Pressing the label
  toggles the checkbox."), plus the upstream test it mirrors (`"should ..."`) where there is one (the user's rule,
  2026-10-09; exceptions can exist, but are rare).
- Name what a case uses more than once with `let` (`let value = page.element("#..-value").await?;`).
- File-local helpers only where they add a lookup, a fixture convention or a setup several cases share (`label(page,
  text)`, a `reset(page)` that clears a fixture's log), never to rename or wrap a shared method (no `expect_focus(page,
  text)`: `page.wait_for_focus(&item(page, text).await?)`). A helper two files need goes into `tests/pages/`.
- Fixture-specific actions (`tests/fixtures/<name>.rs`) only for a fixture with an id convention several cases share:
  a struct borrowing the `Page` (`FocusManagerActions::new(page).item(name)`, `DndActions`), with the standard names
  (`wait_for_log`, `log_stays`), not `expect_*`.
- A check that isn't a case of one fixture (`test_hydration_ids.rs`, `test_server_panics.rs`) implements
  `BrowserTest<Page>` itself (still registered through `fixture_test`).

## Selecting tests

Selection uses browser-test's `TestFilter::from_env()`, applied to the registered suite before its whole-run checks:

```sh
BROWSER_TEST_GROUP=table cargo test -p leptonic --test browser_test
BROWSER_TEST_GROUP=button,focus BROWSER_TEST_FILTER=keyboard cargo test -p leptonic --test browser_test
```

`BROWSER_TEST_GROUP` matches exact logical group names; `BROWSER_TEST_FILTER` matches substrings of test names.
Each accepts comma-separated alternatives, trims whitespace, and ignores empty entries. Both restrictions apply
when both are set. Unset or empty lists impose no restriction; unmatched selections run no UI cases. The server
panic check still runs afterwards. `BROWSER_TEST_KNOWN_ISSUES=1` selects the separate known-issues registrations
(without the server panic check); name and group filters apply to those as well. Registration and grouping live only
in `ui_tests/mod.rs`, never in per-file macros or module-derived registration functions.

## Speed and memory

The measurements below predate the helper refactor: they describe the former script-based lookups and settled
snapshot checks, not the current typed lookups and explicit observation windows. Re-measure before comparing.

Waits and stays checks are steps (`wait_for_attr`, `inner_text_stays`, `wait_for_focus`, ...), so the run summary
shows what they cost. Measured 2026-10-09 (871 tests, parallelism 8): the waits and stays checks together take ~27 s of
4m 21s test-body time, nearly all of it the page reaching the state (exit transitions, timers), not polling latency:
most waits pass at their first read (2-8 ms on average). A 10 ms poll interval instead of 50 ms ran no faster (35.5 s
vs 34.8 s wall time; the run is CPU-bound), so the interval stayed at 50 ms then (now 20 ms, `tests/browser_test.rs`)
and waits stay polls (no in-page waits). Lookups filtered in the page then (one script per lookup instead of one round
trip per candidate per poll): `find` went from 27.6 s to 10.8 s in total. Today's lookups are typed WebDriver queries
filtered in Rust (round trips per candidate and filter).

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
  by the suite and by `just serve-test-app` alike; it is never built without `--release`.
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
- ICU4X data of the fixtures' locales only (`testing/test-app/icu4x-data`, `just test-app-icu-data`; as the book,
  `documentation/build-performance.md`): wasm 13.9 MB → 10.0 MB.
- Not compressed: the wasm (10.0 MB) downloads in ~35 ms over loopback during a run, part of it hidden by streaming
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
  Assertion failed at leptonic/tests/ui_tests/test_checkbox.rs:53:11

  Subject: inner text
  Expression: `move || async move { health::expect_no_panic(self.handle()).await?; self.inner_text().await }`

  Expected: "checked"

    Actual: "true"

  Details:
    - Waited: 5.01s (193 observations)
  ```

  (the helpers are `#[track_caller]`, so the location is the test line that called the wait), followed by the test
  code's frames (`tests/ui_tests/test_checkbox.rs:53  test_checkbox::selected_state`) and the last steps. A value
  that changed while observed adds `Observed values`, each with its time.

The helpers keep this working: a check names runtime values its expression can't show (`with_subject_name`), focus
checks name the focused element (`ElementActions::describe`). A new helper does the same; test code propagates errors with `?`.
