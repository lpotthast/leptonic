# Leptonic documentation

Leptonic is hooks (behavior and accessibility, ported from react-aria/react-stately) + atoms (unstyled
single-element components on the hooks, ported from react-aria-components) + an optional CSS theme for the atoms.
`AGENTS.md` has the working rules for agents; open work is in `PLAN.md`, finished work in `history.md`.

These documents hold valuable long-term knowledge only: decisions with their reasons, rules, pitfalls with their
causes. Each rule lives in one place; others point to it. No narratives, measurement logs or what the code says.

## Which document

- `conventions.md`: what did we decide, and which API conventions (C1–C16) apply?
- `hooks-implementation.md`: how is a hook shaped (types, props, events, propagation, state, ids, elements)?
- `atoms-implementation.md`: how is an atom built (single element, props, classes, data attributes, contexts)?
- `leptos-and-dom.md`: which Leptos/DOM rules does any code follow (SSR, hydration, effects, listeners, styles)?
- `performance.md`: what does work cost in Leptos/reactive_graph, and how do we optimize it?
- `porting.md`: how do we port from react-aria (upstream headers, deviation blocks)?
- `testing.md`: how are native, browser and performance tests set up, written and run?
- `design-collections.md`: why is the collections layer designed as it is?
- `atom-theme.md`: how do apps use the optional atom theme, and how is it ported?
- `lessons.md`: which Leptos/browser/tooling pitfalls cost us time once, and why?
- `build-performance.md`: compile times and binary sizes: how to measure, decisions, advice for users.
- `documentation-strategy.md`: how is the book (`examples/book-ssr`) structured and written? (Its look:
  `examples/book-ssr/STYLE_GUIDE.md`.)
- `history.md`: what was done when?

## Layer Examples

### Hooks

A hook adds behavior and accessibility to an element you render yourself:

```rust
use leptos::prelude::*;
use leptonic::hooks::{
    button::{UseButtonInput, UseButtonReturn, use_button},
    interactions::PressEvent,
};

#[component]
fn MyButton(#[prop(into)] on_press: Callback<PressEvent>, children: Children) -> impl IntoView {
    let UseButtonReturn { button, is_pressed, .. } = use_button(UseButtonInput {
        on_press: Some(on_press),
        ..UseButtonInput::default()
    });
    // `use_button` also sets inline styles: spread the attributes, apply the styles.
    let (attrs, styles) = button.attrs();

    view! {
        <button {..attrs} style=styles class:pressed=is_pressed>
            {children()}
        </button>
    }
}
```

### Atoms

An atom renders one element with the hooks wired in, a default class (`leptonic-Button`) and data attributes
(`data-pressed`, `data-hovered`, `data-focus-visible`, ...) to style it by:

```rust
use leptos::prelude::*;
use leptonic::{atoms::button::Button, hooks::interactions::PressEvent};

#[component]
fn MyButton(#[prop(into)] on_press: Callback<PressEvent>, children: Children) -> impl IntoView {
    view! {
        <Button on_press classes="my-button">
            {children()}
        </Button>
    }
}
```

## Where Things Live

| Concern                               | Place                                                                           |
|---------------------------------------|---------------------------------------------------------------------------------|
| Interaction, focus, ARIA, state logic | `leptonic/src/hooks/<family>/` (ported from react-aria/react-stately)           |
| Elements wiring hooks together        | `leptonic/src/atoms/` (ported from react-aria-components; feature `atoms`)      |
| Shared types and DOM utilities        | `leptonic/src/utils/` (ARIA types, i18n, focus, colors, dates, formatters, ...) |
| Feature flags                         | `leptonic/Cargo.toml` (documented there)                                        |
| Atom theme (optional)                 | `leptonic-theme/scss/atoms/` (`atom-theme.md`)                                  |
| Native test support                   | `leptonic/src/testing.rs` (test builds only; `testing.md`)                      |
| Browser and performance tests         | `leptonic/tests/` driving `testing/test-app/` (`testing.md`)                    |
| Performance metrics                   | `testing/perf-metrics/` (shared by the test-app and the tests)                  |
| The book                              | `examples/book-ssr/` (`documentation-strategy.md`)                              |
| Starter templates                     | `examples/leptonic-template-*` (git submodules; atoms + the atom theme)         |

Leptonic has no prelude: atoms and hooks are imported from their module (`leptonic::atoms::button::Button`,
`leptonic::hooks::button::use_button`), everything else (`Out`, `ValueBinding`, `Locale`, `I18nProvider`, ARIA
types, ...) from the crate root (`conventions.md`, "No prelude; one path per public item").

Apps using leptonic need `rustflags = ["--cfg=web_sys_unstable_apis"]` in their `.cargo/config.toml`: leptonic is
written against web-sys' unstable signatures (e.g. `Element::scroll_top()` returning `f64`).
