# Leptonic: lessons learned

Pitfalls of Leptos, the browsers and the tooling that cost time once (moved out of `PLAN.md`'s findings log). Rules
derived from them live in `leptos-and-dom.md` ("Effect Read Order", "Blur After Disposal", "No Nested Dispatch of
the Same Event Type", "Global State and SSR", ...) and `hooks-implementation.md` ("Props are Single-Use").

- Slot references during hydration (2026-10-09): a Description rendered before its date control was captured
  before the control's attributes hydrated. Tachys cached the resulting `Some(id)` but skipped writing it, assuming
  the server had rendered the same value; the server had rendered no reference. A later selected-date description
  happened to repair it, hiding the issue for nonempty pickers. `use_slot` now publishes its reference in an
  `Effect`, so initial hydration sees `None` regardless of element order, then follows mount/unmount. The picker
  slot tests cover descriptions before and after the controls, help text and errors.
- Inert referenced labels (2026-10-09): Chromium 155 ignores text under `inert` when computing an element's
  `aria-labelledby` name, even though the reference is explicit. A modal select's listbox lost its external label
  this way. A standalone WebDriver reproduction showed that adding `aria-hidden="true"` to the inert ancestor
  preserves the referenced name. `aria_hide_outside` now applies both attributes in inert mode and restores the
  author's previous `aria-hidden` value on cleanup; the content remains non-interactive. Browser coverage:
  `select_behavior::labelled_by_aria_labelledby` and `aria_hide_outside::inert_mode_hides_svg_with_aria_hidden`.
- Telling own scrolls from the user's (2026-10-08): matching a scroll event's position against the one a hook set
  breaks when several scrolls happen in a frame (one remembered position) or re-measured content makes the browser
  clamp the position. Chrome hid it by timing, Chrome Headless Shell didn't. Combine a frame's scroll requests and
  accept the clamped end; and keep "something scrolls" (react-aria's `isScrolling`, page scrolls included) apart from
  "the user scrolls this view".
- Chrome keeps renderer processes (2026-10-08): in headless Chrome for Testing 155 under chromedriver, closing a tab
  leaves its renderer process running, and leaving a page for another site keeps its process for the back/forward
  cache. A browser reused across tests grew by one renderer (100-250 MB) per test. Removing a BiDi user context does
  end its processes, so browser-test's default reset (`SessionReset::NewContext`) runs every test of a reused
  session in a user context of its own; its item-by-item reset (`SessionReset::Manual`, leptonic's suites since)
  navigates the tab to an empty page in a new renderer process. Count with
  `ps -eo args | grep -c 'chrome-linux64/chrome --type=renderer'` while tests run.
- `cargo fmt --all` formats path dependencies (2026-10-08): with leptonic's or the book's manifest, it also reformats
  the `../browser-test` checkout (a path dependency of both browser test suites), and `just fmt`'s nightly pass applies
  leptonic's import grouping to it. Format our own files only (`rustfmt --edition 2024 <files>`), or check
  `git -C ../browser-test status` afterwards, until browser-test is a crates.io dependency again.
- Leptos `erase_components` and spread attributes (2026-10-06): in erased builds (our `.cargo/config.toml`, and
  cargo-leptos' dev builds), attributes spread onto a component (`attr:id`, ...) are applied once, to the elements
  the component first renders (`AnyViewWithAttrs`); when the component's root element is replaced (a reactive
  branch switching `<a>`/`<span>`), they are lost. Non-erased builds re-apply them per render. Affects the `Link`
  atom when `is_disabled` changes (documented there); worth reporting upstream with a minimal reproduction (a
  component returning `move || if flag.get() { Either::Left(view!{<span/>}) } else { Either::Right(view!{<b/>}) }`
  with `attr:id`). Also: hooks called inside such a reactive branch are disposed while the old element's attribute
  effects still run once (panic "already been disposed"): create them outside the branch and clone their attrs
  (hooks-implementation.md, "Props are Single-Use").
- Breadcrumbs' current item (2026-10-06): "the last item to register" is not hydration-safe (`For` hydrates each
  row before the next registers, the server registers all first), so `Breadcrumb` takes `is_current`. Test
  harness: `goto_path` now reports the page's caught errors when hydration fails; `by_role_and_text` knows the
  implicit roles of `<a href>` and `<button>` (since replaced by `role(..)` locators, `tests/pages/locator.rs`, which
  match the role the browser computes).
- Long-press descriptions (2026-10-05): `use_press` rendered a long-press `aria-describedby` only on the client
  (static attribute; the SSR branch had none, and `use_description`'s ids come from a client counter), so hydrated
  pages never had it. Now `UsePressProps::aria_describedby` is a signal set in an effect after mount, as react-aria's
  `useDescription` (layout effect): the server renders none, the client adds it.
- Focus restore and automated browsers (2026-10-05): `FocusScope` restores focus in a `requestAnimationFrame`
  callback (as react-aria's `useRestoreFocus`). Chrome runs no rAF callbacks in windows it considers occluded or in
  the background, even with `visibilityState == "visible"`, so focus restore "fails" there (crudkit's select report;
  the Claude-in-Chrome window). Check with `await new Promise(r => { requestAnimationFrame(() => r(true));
  setTimeout(() => r(false), 1000) })` before debugging focus restore.
- Book (2026-10-05): dark-theme check findings explained: `Root` keeps the theme in local storage (`signal_ls`), so
  the server always renders the light theme and hydration switches to dark; every element with a color transition
  (`.leptonic-btn`: `transition: all 0.1s`, the demo buttons) then animates from light to dark, and the check ran as
  soon as `<body>` was dark. The check now waits for running `CSSTransition`s and parks the pointer at 0,0; all three
  book browser tests pass, dark-theme findings 0. A theme read only on the client flashes the light theme on every
  page load (and `data-theme` mismatches between server and client markup); the book now keeps it in a cookie, so
  the server renders the chosen theme (Themes guide, "Remembering the Theme on the Server").
- Builds (2026-10-05): the target dirs filled the disk. Cause: the staged `split-debuginfo = "unpacked"` in book-ssr's
  `[profile.dev]` plus ~16 GB of DWARF per plain-cargo book build (no `--cfg erase_components`): the `.dwo` files get
  new random names per rustc run and pile up (rust#161824), and the 17 GB rlib was rewritten on every build. Fixed
  (user decisions): book `debug = "line-tables-only"`, no split debuginfo; `--cfg=erase_components` for all builds in
  the root `.cargo/config.toml` (apps set `disable-erase-components`), no duplicated rustflags in the app configs, so
  every build shares fingerprints; one shared agent target dir per project (CLAUDE.md). Book: 3.8 GB, no `.dwo`.
  Verified: library clippy/unit/browser tests, book clippy/unit tests and dark-theme check pass; the book's phone-width
  and Markdown checks were cut off by an external SIGTERM (rerun). Release builds are type-erased now too.
- R3c (2026-10-05): behavior changes visible to users: field atoms (and the themed Checkbox/Radio/Switch) default to
  `ValidationBehavior::Native` like react-aria-components (errors on submission, focus to the first invalid field)
  instead of `Aria`; `NumberFormatOptions::default()` groups digits like `Intl.NumberFormat` ("1,024"). Leptos'
  `view!` needs generics on closing tags too (`<NumberField<u8>>..</NumberField<u8>>`); `#[component] fn Input`
  generates a struct `InputProps` (name clash with an enum of the same name). Contexts provided in a component's
  body (not with `<Provider>`) keep its element the root, so `attr:`/`on:` on the component reach it (they also
  reach its siblings; atoms now use `scoped_view`, which forwards those attributes: atoms-implementation.md,
  "Context Pattern").
- R3b (2026-10-05): `use_form_validation` only re-read native validity when the realtime validation changed;
  react-aria re-reads it after every render, before the queued commit. Checking a required checkbox or radio
  therefore never cleared the native error. Fixed with `NativeValidityReaders` run by the commit. Leptos' `view!`
  can't parse closure parameters with generic type annotations (`move |v: Option<Key>| ..`) as attribute values;
  brace them (`on_change={move |v: Option<Key>| ..}`). `#[prop(optional)]` `Option<T>` props can't receive an
  `Option` through `view!`; components forwarding them build the atom from its props struct.
- Audit 2026-10-05 (four read-only passes; its findings file was deleted 2026-10-07 once absorbed, see git
  history): 18 bug/crash risks, 13 library-wide API conventions to settle, ~78 untested hooks, 101 files on an old
  upstream commit; `PLAN.md` was reorganized into a roadmap. Leptos event delegation is off (tachys `delegation`
  feature unused): every `on:` handler is its own closure, which is what the nested-dispatch rule protects.
- An intermittent menu panic was `FocusScope`'s `FocusManager` reading its disposed `NodeRef` from a pending focus
  callback; the getters now treat a disposed scope as having no element. Found once page errors started to include
  Rust panic messages. `LocalStorage` stored values in hooks that render during SSR panic on the server (dropped on
  another thread); use thread-safe storage there.
- Focusing inside a `focusin` handler threw "closure invoked recursively" (Leptos listener closures can't be
  re-entered) in every collection and in `FocusScope` containment; focus moves now run in a microtask (see
  leptos-and-dom.md, "No Nested Dispatch of the Same Event Type"). Browser tests fail on uncaught page errors.
- FocusScope restore vs. a deferred auto focus (2026-10-06, also upstream): a native `<button>` activated by Enter
  fires a click with `detail == 0`, which counts as a virtual click, so a scope it mounts auto-focuses with
  `focus_safely` one frame later; an unmounting `restore_focus` scope's frame can run first and win (focus ends on
  its node to restore). Not hit by our atoms (menu items press via `use_press`, keyboard modality); the test-app's
  "Dialog from a menu" uses keydown handlers like upstream's test. Revisit if a user report shows it.
- Virtualized rows (2026-10-07): a text selection collapses when the element holding it is moved, and it follows
  the DOM order. tachys' keyed `For` diff moves rows that keep their relative order (e.g. `[4,5,6,7] → [5,6,7,8]`
  moves 7: additions at an index are counted before that index's move check), so `render_visible_items` mounts the
  rows itself (each built into a child owner, inserted before its visual successor, never moved). Reactive values a
  row needs (its `CapturedElement`) belong to the row's owner, not to the mounting effect's (disposed per run).
- Item registry vs. re-rendered items (2026-10-07): `ItemElements`' development-only duplicate-key check kept the
  previously registered item's `CapturedElement` in an effect; when two table cells swapped keys (column reorder),
  the other cell's old owner was disposed before the effect ran ("already disposed" panic). A deferred callback may
  only hold another owner's reactive values while that owner provably lives; the registry now keeps every live
  registration per key (removed by its own cleanup, which runs before the owner's values are disposed) and the
  check reads only those.
- `StoredValue::new_local` in code that runs during SSR panics on the server (a request moves between threads):
  keep it behind `cfg(not(feature = "ssr"))`, also in atoms. It isn't the only cross-thread problem: an `Effect`
  whose owner is dropped on another thread panics too (found with `use_toast_region`).
- Blurs right after a press are swallowed (2026-10-07): a press that keeps focus where it is (an option of a combo
  box popover, a button with `prevent_focus_on_press`) installs `prevent_focus`'s window capture listeners, which stop
  the focused element's `blur`/`focusout` until the next animation frame (react-aria's `preventFocus`). A test that
  blurs from JavaScript within that frame sees no blur, so nothing commits (`combobox_multiple_tests`, now the
  multiple-selection cases of `test_combobox_forms.rs`, failed 2 of 3 runs for this). Blur in the next frame in tests; users can't be that fast.
- Precompress the dev wasm too (2026-10-08, updated 2026-10-09, book): tower-http's `CompressionLayer` brotli-compressed the
  43 MB debug wasm on every request (3.4 s of server CPU each), so parallel browser tests queued behind it and one
  missed its 10 s hydration wait. The book now prepares `.gz` (level 6) and `.br` (quality 4) files at server startup
  in every profile (`src/assets.rs`), reusing fresh sidecars, including production's quality-11 Brotli files.
  `leptos_axum` negotiates those files; `application/wasm` stays excluded from per-request compression. cargo-leptos
  0.3.10 checks `release && precompress`, so `--precompress` alone cannot enable compression for `just serve`.
- Collection keys can be attacker-chosen (2026-10-08, the user): record ids and names, file names and log content
  come from users, and collections are built on the server during SSR. Keep std's randomly seeded hasher for
  key-indexed maps; a fixed-seed hasher (Fx) let crafted colliding keys make every build O(n²), on the server too.
  (Fx made a 20,000-row rebuild 30% faster; not worth that.)
- leptosfmt with the repository's `attr_value_brace_style = "WhenRequired"` drops the braces around an attribute value
  that needs them: `locale={"he".parse::<Locale>().expect(..)}` becomes `locale="he".parse::<Locale>()...`, which
  doesn't compile (found 2026-10-09). Check the build after `leptosfmt`, or bind such a value with `let` first.
- One writer of an element's `style` (2026-10-09): a reactive leptos-styles `Styles` rewrites the whole `style`
  attribute whenever one of its reactive parts changes, so a property a hook set on the element directly
  (`style.set_property`) is gone after the next change of any other part. `use_enter_animation`'s
  hide-until-placed styles were wiped as soon as a popover's `--trigger-width` arrived, and a popover whose
  positioning paused (pinch zoom) showed unplaced at (0,0). Hooks return such styles (`UseEnterAnimationReturn::
  styles`) for the atom to merge into its `Styles`; a hook may write the style directly only for values its returned
  `Styles` hold as well (`use_overlay_position` applies a position at once and returns the same values).
- Hydration-stable defaults (2026-10-06, 2026-10-07): server and client must derive the same initial state while
  rendering. `HashSet` iteration order differs between the server and the client process (std's randomly seeded
  hasher), so a single-expansion disclosure group picking "the first" default key rendered differently; pick
  deterministically (the first default, else the smallest key). A disabled first tab skipped only in a client
  `Effect` likewise gave different server HTML; compute such defaults while rendering. Platform checks
  (`is_ios()`, `is_android()`) are `false` on the server and hydration keeps the server's attributes: decide
  platform-dependent attributes reactively, after hydration, never once at hook creation.
- Effect timing differs from React (2026-10-06): Leptos `Effect`s run after a `MutationObserver`'s callback (React's
  layout effects run before it), so a modal's `aria_hide_outside` made overlays opened from inside it inert before
  they registered (`keep_visible` reveals them again); and render effects run before `Effect::new` (React commits
  attributes before `useEffect`), so a ported effect may observe a different DOM than upstream's. Check the order
  when porting layout effects.
- JavaScript APIs in native tests (2026-10-08): `js_sys::Date::now()` and other JS calls panic outside WebAssembly.
  Gate them on `target_arch = "wasm32"`, not `not(feature = "ssr")` (the virtualizer's and the toast's clocks
  panicked in native unit tests).
- RTL fixtures (2026-10-08): `I18nProvider`, like react-aria's, renders no `dir`; an RTL fixture must lay itself out
  with `dir="rtl"`, or position-based checks run against a left-to-right layout (the he-IL date picker "bug").
- Pointer paths in tests (2026-10-08): a test's moves must be as fine as a user's. The safe-triangle test failed
  because its second move already hovered the next item, which closed the submenu before the hook had two moves to
  judge the direction; move in small steps with real time between them.
- Smooth scrolling in the book's tests (2026-10-05): the book scrolls smoothly (`scroll-behavior: smooth`); a test
  pressing right after a navigation or below the fold pressed whatever was under the pointer mid-scroll. Wait for the
  scroll to end (or scroll instantly) before pressing.
- Labelling by a containing element (2026-10-09): an element labelled by an ancestor that contains it (a column
  resizer by its column header, a tree table's expand button by its row header) gets a doubled accessible name
  ("Resizer Name Resizer", "Collapse Collapse Games"), in upstream's structure too. Label it by a dedicated text
  element or an `aria-label` with the text, as a documented deviation (`atoms/table.rs` `ColumnResizer`,
  `use_table_row.rs`). Check names with the browser's computed name (`accessible_name()`).
- JavaScript numbers in ports (2026-10-09): `Math.sign(0)` is 0, `f64::signum(0.0)` is 1.0 (`table_utils.rs`
  `js_sign`: a fractional column clamped to its minimum overflowed the table).
