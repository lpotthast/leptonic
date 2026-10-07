# Leptonic Architecture

See CLAUDE.md for the layer overview, feature flags, theme system, and build system. Leptonic is becoming hooks +
atoms + an optional CSS theme for the atoms (the components layer is being removed, see `PLAN.md`). This document
shows how the two remaining layers are used; the implementation patterns are in `hooks-implementation.md` (incl.
form validation and animation hooks) and `atoms-implementation.md`.

## Layer Examples

### Hooks

A hook adds behavior and accessibility to an element you render yourself:

```rust
use leptos::prelude::*;
use leptonic::hooks::{UseButtonInput, UseButtonReturn, use_button};

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
use leptonic::atoms::button::Button;

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

| Concern                               | Place                                                                                     |
|---------------------------------------|-------------------------------------------------------------------------------------------|
| Interaction, focus, ARIA, state logic | `leptonic/src/hooks/<family>/` (ported from react-aria/react-stately)                     |
| Elements wiring hooks together        | `leptonic/src/atoms/` (ported from react-aria-components)                                 |
| Shared types and DOM utilities        | `leptonic/src/utils/` (ARIA types, i18n, focus, collections helpers, ...)                 |
| Atom theme (optional)                 | `leptonic-theme/scss/atoms/` (`documentation/atom-theme.md`)                              |
| Native test support                   | `leptonic/src/testing.rs` (test builds only; "Native Tests" in `hooks-implementation.md`) |
| Browser tests                         | `leptonic/tests/` driving `testing/test-app/` (CLAUDE.md, "Browser Tests")                |
