# Leptonic Architecture

Leptonic follows a three-layer architecture that provides flexibility for different use cases.

## Layer Overview

| Layer          | Purpose                  | Design Tokens                                                                      | Feature    |
|----------------|--------------------------|------------------------------------------------------------------------------------|------------|
| **Hooks**      | Pure logic, no rendering | None                                                                               | hooks      |
| **Atoms**      | Hook wrappers, headless  | Minimal (absolutely necessary inline styles)                                       | atoms      |
| **Components** | Full Leptonic experience | Full (classes (`leptonic-btn`), additional data attributes (`data-variant`), etc.) | components |

## User Options

1. **Hooks only** - Maximum control, user handles everything
2. **Atoms** - Convenience components for easy compositing with full control over style system
3. **Components** - Ready- and easy-to-use, fully styled components (via additional `leptonic-theme` crate)

---

## Hooks (`leptonic/src/hooks/`)

Low-level interaction and accessibility logic.

- Pure functions returning props/attributes to spread on elements.
- Handle accessibility (e.g. by providing correct ARIA attributes, etc.).
- No rendering: You control the DOM.
- No CSS classes or other design tokens.

**Use when:** You need maximum flexibility and control over your elements.

### Example

```rust
use leptos::prelude::*;
use leptonic::hooks::{use_button, UseButtonInput, UsePressInput, UseHoverInput, UseFocusRingInput};

#[component]
fn MyButton(children: Children) -> impl IntoView {
    let UseButtonReturn { props, .. } = use_button(UseButtonInput {
        /* ... */
    });

    view! {
        <button {..props.into_attrs()}>
            { children() }
        </button>
    }
}
```

---

## Atoms (`leptonic/src/atoms/`)

Headless, single-element components that wrap hooks. Multiple atoms may be provided for one capability.

- Provide accessibility and interaction behavior out of the box without requiring manual hook setup.
- Only render one HTML element each: Easy control over the DOM hierarchy, easily (re-)composable and stylable.
- Only minimal design tokens (no classes or custom styling-related data attributes, just minimal inline styles).
- Easy to integrate into custom design systems.

**Use when:** Building custom design systems that need accessibility without Leptonic's visual design.

### Example

```rust
use leptos::prelude::*;
use leptonic::atoms::button::Button;

#[component]
fn MyButton(
    #[prop(into)] on_press: Callback<PressEvent>,
    children: Children
) -> impl IntoView {
    view! {
        <Button on_press>
            { children() }
        </Button>
    }
}
```

---

## Components (`leptonic/src/components/`)

Pre-built, styled components ready for production use.

- Include CSS classes for leptonic-theme (`leptonic-btn`, etc.)
- Include design tokens (`data-variant`, `data-color`, `data-size`)
- Feature-rich with complex behavior
- Built on atoms and hooks

**Use when:** You want Leptonic's design system with minimal configuration.

### Example

```rust
use leptonic::components::button::{Button, ButtonVariant, ButtonColor};

view! {
    <Button
        on_press=move |_press| { /* handle press */ }
        variant=ButtonVariant::Filled
        color=ButtonColor::Primary
    >
        "Click me"
    </Button>
}
```

---

## Feature Flags

The library supports feature flags to control which layers are included:

- `hooks` - Low-level interaction hooks (default)
- `atoms` - Headless base components (requires hooks)
- `components` - Full pre-built components (requires atoms)
- `full` - All features combined

Feature hierarchy: `hooks` -> `atoms` -> `components`
