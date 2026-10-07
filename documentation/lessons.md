# Leptonic: lessons learned

Pitfalls of Leptos, the browsers and the tooling that cost time once (moved out of `PLAN.md`'s findings log). Rules
derived from them live in `documentation/hooks-implementation.md` ("Effect Read Order", "Blur After Disposal", "No
Nested Dispatch of the Same Event Type", "Props are Single-Use", "Global State and SSR").

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
  implicit roles of `<a href>` and `<button>`.
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
  book browser tests pass, dark-theme findings 0. Library: users of the dark theme see the light theme flash on every
  page load (server renders light, client reads dark at hydration, so `data-theme` also mismatches between server
  and client markup); persist the theme in a cookie (or apply it before first paint) so SSR renders the chosen theme.
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
  body (not with `<Provider>`) keep its element the root, so `attr:`/`on:` on the component reach it.
- R3b (2026-10-05): `use_form_validation` only re-read native validity when the realtime validation changed;
  react-aria re-reads it after every render, before the queued commit. Checking a required checkbox or radio
  therefore never cleared the native error. Fixed with `NativeValidityReaders` run by the commit. Leptos' `view!`
  can't parse closure parameters with generic type annotations (`move |v: Option<Key>| ..`) as attribute values;
  brace them (`on_change={move |v: Option<Key>| ..}`). `#[prop(optional)]` `Option<T>` props can't receive an
  `Option` through `view!`; components forwarding them build the atom from its props struct.
- Audit 2026-10-05 (four read-only passes, `documentation/audit-2026-10-05.md`): 18 bug/crash risks, 13 library-wide
  API conventions to settle, ~78 untested hooks, 101 files on an old upstream commit; PLAN reorganized into the
  roadmap above. Leptos event delegation is off (tachys `delegation` feature unused): every `on:` handler is its own
  closure, which is what the nested-dispatch rule protects.
- An intermittent menu panic was `FocusScope`'s `FocusManager` reading its disposed `NodeRef` from a pending focus
  callback; the getters now treat a disposed scope as having no element. Found once page errors started to include
  Rust panic messages. `LocalStorage` stored values in hooks that render during SSR panic on the server (dropped on
  another thread); use thread-safe storage there.
- Focusing inside a `focusin` handler threw "closure invoked recursively" (Leptos listener closures can't be
  re-entered) in every collection and in `FocusScope` containment; focus moves now run in a microtask (see
  hooks-implementation.md, "No Nested Dispatch of the Same Event Type"). Browser tests fail on uncaught page errors.
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
- `StoredValue::new_local` in code that runs during SSR panics on the server (a request moves between threads):
  keep it behind `cfg(not(feature = "ssr"))`, also in atoms.

