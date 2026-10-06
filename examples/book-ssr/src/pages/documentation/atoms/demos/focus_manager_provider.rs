use leptonic::{
    atoms::prelude::*,
    components::prelude::{Button, ButtonVariant},
    hooks::{FocusManager as Manager, FocusManagerOptions, IntoAttrs, UseKeyboardInput, use_keyboard},
    utils::keyboard_shortcut::{KeyboardShortcuts, Shortcut},
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
    let (next, previous, first, last) = (manager.clone(), manager.clone(), manager.clone(), manager);

    let keyboard = use_keyboard(UseKeyboardInput {
        shortcuts: Some(
            KeyboardShortcuts::new()
                .on(Shortcut::key("ArrowRight"), move |_| {
                    next.focus_next(wrap());
                })
                .on(Shortcut::key("ArrowLeft"), move |_| {
                    previous.focus_previous(wrap());
                })
                .on(Shortcut::key("Home"), move |_| {
                    first.focus_first(FocusManagerOptions::default());
                })
                .on(Shortcut::key("End"), move |_| {
                    last.focus_last(FocusManagerOptions::default());
                }),
        ),
        allow_repeats: true,
        ..Default::default()
    });

    view! {
        <div role="group" aria-label="Edit" class="demo-focus-row" {..keyboard.props.into_attrs()}>
            <Button variant=ButtonVariant::Outlined on_press=move |_| set_action.set(Some("cut"))>"Cut"</Button>
            <Button variant=ButtonVariant::Outlined on_press=move |_| set_action.set(Some("copy"))>"Copy"</Button>
            <Button variant=ButtonVariant::Outlined on_press=move |_| set_action.set(Some("paste"))>"Paste"</Button>
        </div>
    }
}
