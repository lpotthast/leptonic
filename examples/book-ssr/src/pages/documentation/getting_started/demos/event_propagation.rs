use leptonic::{
    components::prelude::Checkbox,
    hooks::*,
    utils::{Propagation, key::KeyboardKey},
};
use leptos::prelude::*;

#[component]
pub fn EventPropagationDemo() -> impl IntoView {
    let (lets_keys_bubble, set_lets_keys_bubble) = signal(false);
    let (last_handled, set_last_handled) = signal("Nothing handled yet.");

    // The panel handles Escape, wherever the focus is inside it.
    let panel = use_keyboard(UseKeyboardInput {
        on_key_down: Some(Callback::new(move |e: KeyboardEventWrapper| {
            if e.key() == KeyboardKey::Escape {
                set_last_handled.set("The panel handled Escape.");
            } else {
                e.continue_propagation();
            }
        })),
        ..UseKeyboardInput::default()
    });

    // The field handles Enter. Its other keys reach the panel only if the handler continues their propagation.
    let state = use_text_field_state(UseTextFieldStateInput::default());
    let field = use_text_field(UseTextFieldInput {
        has_label: true.into(),
        on_key_down: Some(Callback::new(move |e: KeyboardEventWrapper| {
            if e.key() == KeyboardKey::Enter {
                set_last_handled.set("The field handled Enter.");
            } else if lets_keys_bubble.get_untracked() {
                e.continue_propagation();
            }
        })),
        ..UseTextFieldInput::new(state)
    });

    view! {
        <div {..panel.props.into_attrs()} class="demo-propagation-panel">
            <label {..field.label_props.into_attrs()}>"Search"</label>
            <input {..field.input_props.into_attrs()} class="demo-text-input"/>
        </div>
        <p class="demo-status">{last_handled}</p>
        <div class="demo-controls">
            <Checkbox is_selected=lets_keys_bubble set_selected=set_lets_keys_bubble>
                "Let other keys of the field bubble"
            </Checkbox>
        </div>
    }
}
