use leptonic::{
    atoms::checkbox::{CheckboxButton, CheckboxField},
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
        state,
        id: None,
        element: TextFieldElement::Input,
        input_type: Signal::stored(InputType::Text),
        is_disabled: Signal::stored(false),
        is_read_only: Signal::stored(false),
        is_required: Signal::stored(false),
        is_invalid: Signal::stored(false),
        validate: None,
        validation_behavior: ValidationBehavior::default(),
        validation: None,
        name: None,
        form: None,
        placeholder: MaybeProp::default(),
        pattern: None,
        min_length: None,
        max_length: None,
        auto_complete: None,
        auto_capitalize: None,
        auto_correct: None,
        spell_check: None,
        input_mode: None,
        enter_key_hint: None,
        auto_focus: false,
        exclude_from_tab_order: false,
        label_id: None,
        aria_label: MaybeProp::default(),
        aria_labelledby: None,
        aria_describedby: None,
        aria_errormessage: None,
        aria_activedescendant: Signal::stored(None),
        aria_autocomplete: None,
        aria_haspopup: None,
        aria_controls: Signal::stored(None),
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
        on_key_up: None,
        shortcuts: None,
    });

    view! {
        <div {..panel.props.into_attrs()} class="demo-propagation-panel">
            <label {..field.label_props.into_attrs()}>"Search"</label>
            <input {..field.input_props.into_attrs()} class="demo-text-input"/>
        </div>
        <p class="demo-status">{last_handled}</p>
        <div class="demo-controls">
            <CheckboxField is_selected=lets_keys_bubble set_selected=set_lets_keys_bubble>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Let other keys of the field bubble"
                </CheckboxButton>
            </CheckboxField>
        </div>
    }
}
