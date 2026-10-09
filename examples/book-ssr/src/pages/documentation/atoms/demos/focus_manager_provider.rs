use leptonic::{
    IntoAttrs, KeyboardKey, KeyboardShortcuts, Shortcut,
    atoms::{button::Button, focus_manager::FocusManagerProvider},
    hooks::{
        focus::{FocusManager as Manager, FocusManagerOptions},
        interactions::{UseKeyboardInput, use_keyboard},
    },
};
use leptos::prelude::*;

#[component]
pub fn FocusManagerProviderDemo() -> impl IntoView {
    let (action, set_action) = signal(None::<&'static str>);

    view! {
        <FocusManagerProvider classes="demo-focus-manager-buttons" let:manager>
            <EditButtons manager set_action/>
        </FocusManagerProvider>

        <p class="demo-status">
            {move || match action.get() {
                Some(action) => format!("Last action: {action}."),
                None => "No action yet.".to_string(),
            }}
        </p>
    }
}

/// Buttons between which the arrow keys, Home and End move focus, wrapping around at the ends.
#[component]
fn EditButtons(manager: Manager, set_action: WriteSignal<Option<&'static str>>) -> impl IntoView {
    let wrap = || FocusManagerOptions {
        wrap: true,
        ..FocusManagerOptions::default()
    };
    let (next, previous, first, last) =
        (manager, manager, manager, manager);

    let keyboard = use_keyboard(UseKeyboardInput {
        shortcuts: Some(
            KeyboardShortcuts::new()
                .on(Shortcut::new(KeyboardKey::ArrowRight), move |_| {
                    next.focus_next(wrap());
                })
                .on(Shortcut::new(KeyboardKey::ArrowLeft), move |_| {
                    previous.focus_previous(wrap());
                })
                .on(Shortcut::new(KeyboardKey::Home), move |_| {
                    first.focus_first(FocusManagerOptions::default());
                })
                .on(Shortcut::new(KeyboardKey::End), move |_| {
                    last.focus_last(FocusManagerOptions::default());
                }),
        ),
        allow_repeats: true,
        ..Default::default()
    });

    view! {
        <div role="group" aria-label="Edit" class="demo-focus-row" {..keyboard.props.into_attrs()}>
            <Button on_press=move |_| set_action.set(Some("cut")) classes="demo-btn">"Cut"</Button>
            <Button on_press=move |_| set_action.set(Some("copy")) classes="demo-btn">"Copy"</Button>
            <Button on_press=move |_| set_action.set(Some("paste")) classes="demo-btn">"Paste"</Button>
        </div>
    }
}
