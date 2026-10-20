# Leptonic: lessons learned

Pitfalls of Leptos, the browsers and the tooling that cost time once, with their causes. Rules derived from them are
in `leptos-and-dom.md` and `hooks-implementation.md` (not repeated here).

## Leptos

- **`erase_components` and spread attributes**: in erased builds (every build here, `.cargo/config.toml`), attributes
  spread onto a component (`attr:id`, ...) are applied once, to the elements it first renders (`AnyViewWithAttrs`);
  when its root element is replaced (a reactive branch switching `<a>`/`<span>`), they are lost (the `Link` atom when
  `is_disabled` changes; issue draft `b-spread-attrs-lost`). Hooks called inside such a branch are disposed while the
  old element's attribute effects still run once ("already been disposed"): create them outside the branch and clone
  their attrs ("Props are Single-Use"). Spread rebuilds also keep the old handlers (`c-spread-listener-leak`).
- **Keyed `For` moves rows that keep their relative order** (`[4,5,6,7] → [5,6,7,8]` moves 7), and a moved element
  loses the text selection it holds: `render_visible_items` mounts virtualized rows itself (each in a child owner,
  inserted before its visual successor, never moved). A row's reactive values (its `CapturedElement`) belong to the
  row's owner, not to the mounting effect's (disposed per run). Issue draft `d-keyed-for-moves`.
- **`view!` parsing**: generics are needed on closing tags too (`<NumberField<u8>>..</NumberField<u8>>`); closure
  parameters with type annotations as attribute values need braces (`on_change={move |v: Option<Key>| ..}`);
  `#[prop(optional)] Option<T>` props can't receive an `Option` through `view!` (build the props struct instead);
  `#[component] fn Input` generates `InputProps` (clashes with a type of that name).
- **An arena item holding its own owner leaks** (2026-10-18): the collection's focus registry stored the `Owner` it
  was created under, in a `StoredValue` of that owner; collections in removed `<For>` rows were never freed and their
  focus effects kept running. Rule: `leptos-and-dom.md`, "Ownership and Cleanup".
- **Contexts provided in a component's body** reach the component's later siblings too (a component has no owner of
  its own): use `<Provider>` or `scoped_view` (`atoms-implementation.md`, "Context Pattern").
- **Client-only attributes are lost on hydrated pages**: an attribute only the client branch renders (a long-press
  `aria-describedby` whose ids come from a client counter) never appears after hydration. Set it from a signal in an
  effect after mount, as react-aria's `useDescription` uses a layout effect.

## Browsers

- **A frozen page blocks its whole WebDriver session**: ChromeDriver runs a session's commands in order, so all later
  ones wait, CDP commands through ChromeDriver and quitting included. The browser stays responsive:
  `Runtime.evaluate` over the page's own DevTools WebSocket (`goog:chromeOptions.debuggerAddress`) detects the freeze,
  and `/json/close/<id>` on the DevTools HTTP endpoint closes the page at once (`Page.crash` took 60 s and broke the
  session). browser-test does this when a test exceeds its time limit. The frozen page's stack can't be read
  (`Debugger.enable` waits for the main thread).
- **Chrome keeps renderer processes**: closing a tab leaves its renderer running, and leaving a page for another site
  keeps its process for the back/forward cache; a reused browser grew by 100-250 MB per test. Removing a BiDi user
  context ends its processes; leptonic's reset (`SessionReset::manual`) navigates to an empty page in a new renderer.
  Count with `ps -eo args | grep -c 'chrome-linux64/chrome --type=renderer'`.
- **No `requestAnimationFrame` in occluded windows**: Chrome runs no rAF callbacks in windows it considers occluded or
  in the background, even with `visibilityState == "visible"` (seen in the Claude-in-Chrome window), so focus restore
  (in a rAF, as react-aria's) "fails" there. Check with `await new Promise(r => { requestAnimationFrame(() =>
  r(true)); setTimeout(() => r(false), 1000) })` before debugging focus restore.
- **Inert hides referenced labels**: Chromium ignores text under `inert` when computing an `aria-labelledby` name, even
  though the reference is explicit; adding `aria-hidden="true"` to the inert ancestor keeps it (`aria_hide_outside`
  sets both in inert mode and restores the author's `aria-hidden`).
- **Labelling by a containing element doubles the name** ("Resizer Name Resizer", "Collapse Collapse Games"), in
  upstream's structure too: label by a dedicated text element or an `aria-label` (a documented deviation in
  `atoms/table.rs`, `use_table_row.rs`). Check names with the computed `accessible_name()`.
- **Telling own scrolls from the user's**: matching a scroll event's position against the one a hook set breaks when
  several scrolls happen in a frame or re-measured content makes the browser clamp the position. Combine a frame's
  scroll requests, accept the clamped end, and keep "something scrolls" (react-aria's `isScrolling`, page scrolls
  included) apart from "the user scrolls this view".
- **A theme read only on the client flashes** the default theme on every load (and `data-theme` mismatches between
  server and client): keep the choice in a cookie so the server renders it (the book's Themes guide).
- **A virtual click restores focus late**: a native `<button>` activated by Enter fires a click with `detail == 0`,
  counted as virtual, so a scope it mounts auto-focuses one frame later, and an unmounting `restore_focus` scope's
  frame can win (also upstream). Our atoms aren't affected (menu items press via `use_press`).
- **Chrome blurs a focused element that becomes disabled late**: only when it next updates the rendering, after
  `MutationObserver` callbacks, so the native `blur`/`focusout` follows react-aria's synthetic ones (react-aria's
  comment claims the opposite; Firefox fires none). `use_focus` drops that one native blur (`SyntheticBlurObserver`).

## Tests

- **Blurs right after a press are swallowed**: a press that keeps focus where it is (a combo box option,
  `prevent_focus_on_press`) installs `prevent_focus`'s capture listeners, which stop the focused element's
  `blur`/`focusout` until the next frame. A test blurring from script within that frame sees no blur: blur in the
  next frame.
- **Pointer paths must be as fine as a user's**: a coarse move hovers the next item before a hook has two moves to
  judge the direction (the submenu safe triangle): move in small steps with real time between them.
- **RTL fixtures need `dir="rtl"`**: `I18nProvider` renders no `dir` (as react-aria's), so position checks otherwise
  run against a left-to-right layout.
- **Smooth scrolling** (the book's `scroll-behavior: smooth`): a press right after a navigation or below the fold
  hits whatever passes under the pointer: wait for the scroll to end, or scroll instantly.
- **Color transitions after hydration** animate from the server's theme: visual checks wait for running
  `CSSTransition`s and park the pointer at 0,0.
- **Load-dependent counts**: reproduce with `Emulation.setCPUThrottlingRate` (CDP, in the test), per-window counts
  and a JS-only event log; never instrument shared measurement code (anything allocating in `count()` skews every
  scenario). A wall-clock timer in the library (`use_scroll_view`'s 300 ms scroll end) makes counts timing-dependent:
  find what starts it, often a bug.
- **WebDriver pointer moves** take 250 ms in a varying number of steps: path-sensitive or throttled hooks need
  `action_chain_with_delay(None, Some(Duration::ZERO))`.
- **Finding nondeterminism in counts**: trace every allocation of a page from load in two runs of one build and
  compare (`wasm-opt -g` via the test-app's `wasm-opt-features` keeps names, so `Error().stack` in the allocator
  gives Rust call sites; remove it afterwards). Byte diffs at equal counts between builds came from address-hashed
  subscriber sets, not from the measured action.

## Porting

- **JavaScript numbers**: `Math.sign(0)` is 0, `f64::signum(0.0)` is 1.0 (`table_utils.rs` `js_sign`).
- **React re-reads after every render**: `use_form_validation` re-read native validity only when its realtime
  validation changed, so checking a required checkbox never cleared the native error; react-aria re-reads it after
  every render, before the queued commit (now the readers of
  `FormValidationState::register_native_validity_reader`, run by the commit). Look for such implicit per-render work
  when porting.
- **Virtual focus moves no DOM focus**: it dispatches `blur`/`focusout` on the input and `focus`/`focusin` on an
  option while the input keeps focus, and the input's real blur comes later. A guard reporting a blur only once per
  focus swallowed that real blur (an open combo box stayed open after a click outside), and a tracker taking a
  `focusin`'s target as the focused element made the combo box popover's restoring focus scope active, which then
  refocused the input when it closed. React-aria reports every blur, and its restoring scopes check the active
  element.
- **Collection keys can be attacker-chosen** (record ids, file names, log content; built on the server too): keep
  std's seeded hasher for key-indexed maps. A fixed-seed hasher (Fx, 30% faster rebuilds) lets crafted colliding keys
  make every build O(n²).

## Tooling and builds

- **leptosfmt drops needed braces**: with `attr_value_brace_style = "WhenRequired"`,
  `locale={"he".parse::<Locale>().expect(..)}` becomes uncompilable: check the build after `leptosfmt`, or bind such a
  value with `let` first. It also breaks generic components (`<DateField<DateTime> ..>`) into uncompilable layouts:
  format such files with rustfmt only. An attribute value directly followed by a spread needs its braces:
  `style=styles {..signal_attr(..)}` parses as the struct literal `styles { ..signal_attr(..) }` ("cannot find
  struct `styles`"); leptosfmt removes them again (`atoms/table.rs`, also reformatted elsewhere: rustfmt only).
- **Debug info filled the disk**: `split-debuginfo = "unpacked"` plus ~16 GB of DWARF per book build without
  `erase_components` piled up `.dwo` files with new random names per rustc run (rust#161824). Since: the book uses
  `debug = "line-tables-only"` without split debuginfo, and `--cfg=erase_components` is set for every build in the
  root `.cargo/config.toml` (apps set `disable-erase-components`), so all builds share fingerprints.
- **Compressing the dev wasm per request** (tower-http's `CompressionLayer`, 3.4 s of CPU per 43 MB request) queued
  parallel browser tests until one missed its hydration wait: the book precompresses at server startup in every
  profile (`src/assets.rs`). cargo-leptos 0.3.10 checks `release && precompress`, so `--precompress` alone does
  nothing for `just serve`.
