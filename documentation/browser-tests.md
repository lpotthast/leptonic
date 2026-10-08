# Browser Test Style

How the browser tests in `leptonic/tests/` are written. Running them, fixtures and registration: CLAUDE.md, "Browser
Tests". Reference file: `ui_tests/test_checkbox.rs`. The helpers (`tests/pages/`): `PageActions` on a page,
`ElementActions` on an element, `Locator`s (`css`, `role`, `xpath`), `SyntheticEvent`s; `wait_for`/`expect` in
`tests/polling/mod.rs`; `Case` in `tests/cases/mod.rs`.

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
| 2. Waits       | a state is reached, within 10 s              | `Report`: the element, the expected and the last seen value   |
| 3. Stays       | a state holds for 300 ms ("nothing happens") | `Report`: the element, the value it changed to                |
| 4. Assertions  | a value read now                             | assertr panic: the expression, the expected and actual value  |

Which one, by what the check is about:

- **The effect of something the test just did** (a click, a key, a script): a wait. If the effect is that nothing
  changes: a stays check. Never an assertion: it reads before effects, animation frames and timers ran.
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
| Anything else  | any read                               | `wait_for(what).observing(read).to_be_equal_to(v)`, `.to_be("description", condition)` | `expect(what).observing(read).to_stay_equal_to(v)` |

For anything else, `crate::polling` observes a value until it meets the expectation, in a sentence:

```rust
wait_for("the last change")
    .observing(|| log.inner_text())
    .to_be_equal_to("change:200")
    .await?;
wait_for("the red value")
    .observing(|| number(&red))
    .to_be("64 (±1)", |red| (red - 64.0).abs() <= 1.0)
    .await?;
expect("the number of open toasts")
    .observing(|| page.count(TOAST))
    .for_at_least(Duration::from_millis(2500))
    .to_stay_equal_to(1)
    .await?;
```

- Observe the value the expectation is about, not a `bool` computed from it: a failure shows the last value seen
  ("the red value did not become 64 (±1) within 10s; last seen 70"). Several values: a tuple or a struct.
- `what` names the observed thing ("the last change"); the `to_be` description completes "to be …".
- The observation is a closure returning the read's future: `|| log.inner_text()` for one of the helpers above,
  `|| async { Ok(element.attr("x").await?) }` for a thirtyfour read or several steps.
- `for_at_least` only for real timers the check must outlast (a toast's timeout, a long-press delay).

The inner text is `innerText`, trimmed: what the element shows (never thirtyfour's `text()`, which skips
`display: contents` children). Waits pass the moment the state is reached, so waiting for a state that may already hold
costs nothing. An element handle waits on that element; if the page replaces it (re-rendering), the wait fails at once
with "the element was removed from the page": look it up after the change.

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

## Failure reports

browser-test's failure reports (browser-test README, "Failure Reports") show for every failure, without anything in the
test code:

- **where**: the test code's frames, from the helper that failed up to the case and line that called it (for a panic,
  e.g. a failed assertion: its location);
- **when**: the test's last steps (page loads, lookups, waits) with their timing;
- **what**: the error, e.g. "the inner text of `<span id="test-cb-basic-value">` did not become "checked" within 10s;
  it is "false"".

The helpers keep this working: every wait and stays check names the element (`ElementActions::describe`), the expected
and the last seen value, and runs as a step. A new helper does the same; test code propagates errors with `?`.
