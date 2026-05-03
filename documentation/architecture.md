# Leptonic Architecture

See CLAUDE.md for the layer overview (hooks → atoms → components), feature flags, theme system, and build
system. This document covers architectural patterns not captured there.

## Layer Examples

### Hooks

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

### Atoms

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

### Components

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

## Form Validation Hooks

Three cooperating hooks handle form validation:

- `use_form_validation_state` — state management for multiple validation sources (controlled, server,
  client-side, native). `ValidationBehavior::Aria` vs `::Native` determines how validation is surfaced.
  Returns `ValidationResult` aggregating all sources.
- `use_form_validation` — DOM connection (side-effectual, no return value). Calls `setCustomValidity()` on
  the form element and listens for native validation events.
- `use_form_reset` — detects parent `<form>` reset events via `CapturedElement` from
  `leptos-element-capture`.

Field hooks (e.g., `use_text_field`, `use_checkbox`) compose these three internally.

## Animation System (`leptonic/src/hooks/animation/`)

CSS animation lifecycle hooks using the Web Animations API.

- `use_enter_animation` — accepts `CapturedElement` from `leptos-element-capture` plus an `is_ready` signal,
  returns `is_entering: Signal<bool>`.
  Watches element animations on mount and reports when they complete.
- `use_exit_animation` — accepts `CapturedElement` from `leptos-element-capture` plus an `is_open` signal,
  returns `is_exiting: Signal<bool>` and `exit_state: Signal<ExitState>`.
- `ExitState` enum: `Open` → `Exiting` → `Closed`.

Internally, `watch_animations()` uses `Element.getAnimations()` and `Promise.all(animation.finished)` to
detect animation completion. During SSR, no Web Animations API calls are made; enter reports `false`, exit
follows `is_open` directly.

**Purpose:** Coordinated enter/exit transitions for overlays, popovers, modals — keeping the element mounted
during exit animations.
