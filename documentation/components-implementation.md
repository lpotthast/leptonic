# Component Implementation Patterns

Components live in `leptonic/src/components/` and provide the full "batteries included" experience.
They build on hooks and/or atoms, adding design tokens, CSS classes, and complex behavior.

## CSS Class Convention

Components add a CSS class following the `leptonic-{component}` pattern, targeted by the
`leptonic-theme` SCSS stylesheets:

```rust
view! {
    <button class="leptonic-btn" data-variant=variant data-color=color>
        { children() }
    </button>
}
```

## Design Token Props

Components accept variant/color/size props that map to data attributes:

```rust
#[component]
pub fn Button(
    #[prop(into, optional)] variant: ButtonVariant,   // -> data-variant="filled"
    #[prop(into, optional)] color: ButtonColor,       // -> data-color="primary"
    #[prop(into, optional)] size: ButtonSize,         // -> data-size="small"
    // ...
) -> impl IntoView { ... }
```

Theme SCSS uses these data attributes as selectors:

```scss
.leptonic-btn[data-variant="filled"][data-color="primary"] {
    background: var(--brand-color);
}
```

## State Management

- **Collection state**: `RwSignal<Vec<T>>` for lists (toasts, shown modals).
- **Global context**: `provide_context()` for app-wide state access (e.g., `Toasts`).
- **Mutation API**: Named methods (`push`, `remove`, `clear`) instead of raw signal writes.
- **Auto-cleanup**: `set_timeout` for auto-dismissing toasts, overlays, etc.

## Reference Implementations

| Pattern                      | File                                        |
|------------------------------|---------------------------------------------|
| Design tokens                | `leptonic/src/components/button.rs`         |
| Global context + collection  | `leptonic/src/components/toast.rs`          |
| Complex state + overlays     | `leptonic/src/components/modal.rs`          |
| Flexible output (`Out<T>`)   | `leptonic/src/components/date_selector.rs`  |
