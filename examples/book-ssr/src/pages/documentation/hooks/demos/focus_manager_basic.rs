use leptonic::{
    IntoAttrs,
    atoms::{
        button::Button,
        checkbox::{CheckboxButton, CheckboxField},
    },
    hooks::focus::{
        CreateFocusManagerReturn, FocusManagerOptions, Focusability, create_focus_manager,
    },
};
use leptos::{prelude::*, web_sys};
use send_wrapper::SendWrapper;

#[component]
pub fn FocusManagerBasicDemo() -> impl IntoView {
    let wrap = RwSignal::new(true);
    let tabbable_only = RwSignal::new(false);
    // The buttons below take focus when pressed, so the focus manager starts from the element
    // focused last inside the container instead of `document.activeElement`.
    let last_focused = StoredValue::new(None::<SendWrapper<web_sys::Element>>);
    let (status, set_status) = signal(String::from("Nothing focused yet."));

    let CreateFocusManagerReturn {
        focus_manager,
        props,
    } = create_focus_manager();

    let options = move || FocusManagerOptions {
        from: last_focused.with_value(|el| el.as_ref().map(|el| (**el).clone())),
        wrap: wrap.get_untracked(),
        focusability: if tabbable_only.get_untracked() {
            Focusability::Tabbable
        } else {
            Focusability::Focusable
        },
        ..Default::default()
    };
    let report = move |focused: Option<web_sys::Element>| {
        set_status.set(match focused {
            Some(el) => format!(
                "Focused \u{201c}{}\u{201d}.",
                el.text_content().unwrap_or_default().trim()
            ),
            None => "No element to focus.".to_string(),
        });
    };

    let first = focus_manager;
    let previous = focus_manager;
    let next = focus_manager;
    let last = focus_manager;

    view! {
        <div class="demo-controls demo-mb-1">
            <Button on_press=move |_| report(first.focus_first(options())) classes="demo-btn">"Focus first"</Button>
            <Button on_press=move |_| report(previous.focus_previous(options())) classes="demo-btn">"Focus previous"</Button>
            <Button on_press=move |_| report(next.focus_next(options())) classes="demo-btn">"Focus next"</Button>
            <Button on_press=move |_| report(last.focus_last(options())) classes="demo-btn">"Focus last"</Button>
        </div>

        <div
            class="demo-focus-row demo-focus-managed"
            on:focusin=move |ev| last_focused.set_value(Some(SendWrapper::new(event_target::<web_sys::Element>(&ev))))
            {..props.into_attrs()}
        >
            <button type="button" class="demo-focus-item">"Bold"</button>
            <button type="button" class="demo-focus-item">"Italic"</button>
            <button type="button" tabindex="-1" class="demo-focus-item">"Strikethrough"</button>
            <button type="button" class="demo-focus-item">"Underline"</button>
        </div>

        <p class="demo-status">{move || status.get()}</p>

        <div class="demo-controls">
            <CheckboxField is_selected=wrap set_selected=wrap>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Wrap around"
                </CheckboxButton>
            </CheckboxField>
            <CheckboxField is_selected=tabbable_only set_selected=tabbable_only>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Tabbable only"
                </CheckboxButton>
            </CheckboxField>
        </div>
    }
}
