# Leptonic documentation

Leptonic is hooks (behavior and accessibility, ported from react-aria/react-stately) + atoms (unstyled
single-element components on the hooks, ported from react-aria-components) + an optional CSS theme for the atoms.
`CLAUDE.md` has the working rules for agents; open work is in `PLAN.md`, finished work in `history.md`.

## Which document

- `conventions.md`: what did we decide, and which API conventions (C1–C16) apply?
- `hooks-implementation.md`: how is a hook shaped (types, props, events, state, ids, elements)?
- `atoms-implementation.md`: how is an atom built (single element, props, classes, contexts)?
- `leptos-and-dom.md`: which Leptos/DOM rules does any code here follow (SSR, effects, ...)?
- `porting.md`: how do we port from react-aria (upstream headers, deviation blocks)?
- `testing.md`: how are native and browser tests set up, written and run?
- `design-collections.md`: why is the collections layer designed as it is?
- `atom-theme.md`: how is the optional atom theme ported and maintained?
- `lessons.md`: what did a Leptos/browser/tooling pitfall cost us once?
- `build-performance.md`: compile times and binary sizes: measurements, advice
- `consumers.md`: who uses leptonic, and what do they depend on?
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
    let UseButtonReturn { props, is_pressed, .. } = use_button(UseButtonInput {
        on_press: Some(on_press),
        ..UseButtonInput::default()
    });
    // `use_button` also sets inline styles: spread the attributes, apply the styles.
    let (attrs, styles) = props.into_parts();

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
| Elements wiring hooks together        | `leptonic/src/atoms/` (ported from react-aria-components)                       |
| Shared types and DOM utilities        | `leptonic/src/utils/` (ARIA types, i18n, focus, colors, dates, formatters, ...) |
| Atom theme (optional)                 | `leptonic-theme/scss/atoms/` (`documentation/atom-theme.md`)                    |
| Native test support                   | `leptonic/src/testing.rs` (test builds only; "Native Tests" in `testing.md`)    |
| Browser tests                         | `leptonic/tests/` driving `testing/test-app/` (`testing.md`)                    |

Leptonic has no prelude. Users import atoms and hooks from their module (`leptonic::atoms::button::Button`,
`leptonic::hooks::button::use_button`; a hook family's module exposes its hooks and their types); everything else
(`Out`, `ValueBinding`, `Locale`, `I18nProvider`, ARIA types, ...) is re-exported flat from `lib.rs`
(`leptonic::Locale`); the `utils` module is private (`conventions.md`, "No prelude; one path per public item").
