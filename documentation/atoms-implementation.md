# Atom Implementation Patterns

Atoms live in `leptonic/src/atoms/` and sit between hooks and components. They wrap hooks into
single-element Leptos components.

## Single-Element Rule

Every atom renders exactly **one** HTML element. This keeps them composable and gives consumers
full control over the surrounding DOM structure.

## Wrapping Hooks

An atom calls one or more hooks, destructures the return, and spreads attributes onto its element:

```rust
#[component]
pub fn Button(
    #[prop(into)] on_press: Callback<PressEvent>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] classes: Classes,
    children: Children,
) -> impl IntoView {
    let UseButtonReturn { props, .. } = use_button(UseButtonInput {
        disabled,
        use_press_input: UsePressInput { on_press, ..Default::default() },
        use_hover_input: UseHoverInput { disabled, ..Default::default() },
        use_focus_ring_input: UseFocusRingInput { disabled, ..Default::default() },
        ..Default::default()
    });

    view! {
        <button {..props.into_attrs()} class=classes>
            { children() }
        </button>
    }
}
```

## Props Design

Atoms expose a curated subset of hook inputs as component props — not every hook input needs a
corresponding atom prop. Common patterns:

- `disabled: Signal<bool>` — passed through to hook(s).
- `on_press`, `on_hover_start`, etc. — key callbacks exposed as props.
- `classes: Classes` and `styles: Styles` — for consumer styling.
- Internal hook configuration uses sensible defaults.

## No CSS Classes

Atoms are headless by design. They add no `class="leptonic-*"` attributes and no
`data-variant` / `data-color` design tokens. Only minimal inline styles are allowed when
absolutely necessary (e.g., slider track positioning). This makes atoms suitable as building
blocks for custom design systems.

## Context Pattern (Complex Atoms)

When an atom comprises multiple cooperating elements (e.g., Slider with track, thumb, marks):

1. The root atom creates shared state and provides it via `Provider<SliderCtx>`.
2. Child atoms (`SliderTrack`, `SliderThumb`, `SliderOutput`) read state via
   `expect_context::<SliderCtx>()`.
3. The root atom still delegates hook logic to `use_slider_state`, `use_slider`, etc.

## Reference Implementations

| Pattern                      | File                                |
|------------------------------|-------------------------------------|
| Simple hook wrapper          | `leptonic/src/atoms/button.rs`      |
| Context-based composition    | `leptonic/src/atoms/slider.rs`      |
| Router integration           | `leptonic/src/atoms/link.rs`        |
| Overlay with portal          | `leptonic/src/atoms/popover.rs`     |
