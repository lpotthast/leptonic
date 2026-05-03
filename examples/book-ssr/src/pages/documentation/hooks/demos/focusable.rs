use leptonic::{
    atoms::focus_ring::FocusRing,
    components::prelude::*,
    hooks::*,
    utils::Propagation,
    utils::{classes::Classes, css::em},
};
use leptos::prelude::*;

#[component]
pub fn FocusableDemo() -> impl IntoView {
    let (disabled, set_disabled) = signal(false);
    let (exclude_from_tab, set_exclude_from_tab) = signal(false);
    let (focus_count, set_focus_count) = signal(0);
    let (blur_count, set_blur_count) = signal(0);
    let (is_focused, set_is_focused) = signal(false);
    let (key_events, set_key_events) = signal(Vec::<String>::new());

    let UseFocusableReturn {
        props,
        focus_handle,
    } = use_focusable(UseFocusableInput {
        disabled: disabled.into(),
        auto_focus: false,
        exclude_from_tab_order: exclude_from_tab.into(),
        on_focus: Some(Callback::new(move |_| {
            set_focus_count.update(|c| *c += 1);
        })),
        on_blur: Some(Callback::new(move |_| {
            set_blur_count.update(|c| *c += 1);
        })),
        on_focus_change: Some(Callback::new(move |focused: bool| {
            set_is_focused.set(focused);
        })),
        on_key_down: Some(Callback::new(move |e: KeyboardEventWrapper| {
            set_key_events.update(|events| {
                events.push(format!("Key: {}", e.key()));
                if events.len() > 5 {
                    events.remove(0);
                }
            });
            e.continue_propagation();
        })),
        on_key_up: None,
    });
    let attrs = props.into_attrs();

    view! {
        <div class=Classes::from("demo-flex-center-row")>
            <FocusRing>
                <div
                    {..attrs}
                    role="button"
                    style="
                        display: inline-flex;
                        align-items: center;
                        justify-content: center;
                        border: 2px solid var(--brand-color);
                        padding: 1em 2em;
                        border-radius: 8px;
                        cursor: pointer;
                        background: var(--brand-color-light, rgba(230, 105, 86, 0.1));
                        outline: none;
                        transition: all 0.2s;
                    "
                >
                    "Custom Focusable Element"
                </div>
            </FocusRing>

            <button
                on:click=move |_| focus_handle.focus()
                class=Classes::from("demo-btn")
            >
                "Click to focus"
            </button>
        </div>

        <Stack orientation=StackOrientation::Vertical spacing=em(0.5) attr:style="margin-top: 1em;">
            <FormControl classes="demo-form-row">
                <Checkbox checked=disabled set_checked=set_disabled />
                <Label>"Disabled"</Label>
            </FormControl>

            <FormControl classes="demo-form-row">
                <Checkbox checked=exclude_from_tab set_checked=set_exclude_from_tab />
                <Label>"Exclude from tab order (tabindex=-1)"</Label>
            </FormControl>
        </Stack>

        <div class=Classes::from("demo-flex-gap")>
            <p>"Focus count: " { move || focus_count.get() }</p>
            <p>"Blur count: " { move || blur_count.get() }</p>
            <p class=Classes::builder().with_toggle(is_focused.get(), "demo-state-active", "demo-state-inactive").build()>
                { move || if is_focused.get() { "Focused" } else { "Not focused" } }
            </p>
        </div>

        <p>"Last key events: " { move || {
            let events = key_events.get();
            if events.is_empty() {
                "(none)".to_string()
            } else {
                events.join(", ")
            }
        }}</p>
    }
}
