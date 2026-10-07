use leptos::prelude::*;

use crate::{
    Out,
    atoms::{
        field::{Description, FieldError, Label, TextElement},
        input::{Input, TextArea},
        search_field::{
            SearchField as SearchFieldAtom, SearchFieldClearButton,
            SearchFieldProps as SearchFieldAtomProps,
        },
        text_field::{TextField as TextFieldAtom, TextFieldProps as TextFieldAtomProps},
    },
    components::icon::Icon,
    hooks::{InputMode, InputType, ValidateFn, ValidationBehavior},
    utils::{classes::Classes, styles::Styles},
};

/// The label, description and error message around a field's input. Built inside the field (the
/// parts read its contexts).
pub(crate) fn field_parts(
    label: MaybeProp<String>,
    description: MaybeProp<String>,
    input: impl FnOnce() -> AnyView,
) -> AnyView {
    let input = input();
    view! {
        {move || label.get().map(|label| view! { <Label classes="leptonic-field-label">{label}</Label> })}
        {input}
        {move || description.get().map(|description| view! {
            <Description element=TextElement::Div classes="leptonic-field-description">
                {description}
            </Description>
        })}
        <FieldError element=TextElement::Div classes="leptonic-field-error" />
    }
    .into_any()
}

/// A text field with its label, description and validation errors.
///
/// Its value starts at `default_value` and is reported through `on_change`; or it is `value`, and
/// changes go to `set_value` (e.g. both an `RwSignal<String>`). `multiline` renders a `<textarea>`.
#[allow(clippy::too_many_arguments, clippy::needless_pass_by_value)]
#[component]
pub fn TextField(
    /// The visible label. Without it, set `aria_label`.
    #[prop(into, optional)]
    label: MaybeProp<String>,
    #[prop(into, optional)] description: MaybeProp<String>,
    #[prop(into, optional)] default_value: String,
    #[prop(into, optional)] on_change: Option<Callback<String>>,
    /// The value (controlled): a value or any signal.
    #[prop(into, optional)]
    value: Option<Signal<String>>,
    /// Receives the new state: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_value: Option<Out<String>>,
    /// The `<input>`'s type; reactive, e.g. for a "show password" toggle.
    #[prop(into, optional)]
    input_type: Signal<InputType>,
    /// A `<textarea>` instead of an `<input>`.
    #[prop(optional)]
    multiline: bool,
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] is_read_only: Signal<bool>,
    #[prop(into, optional)] is_required: Signal<bool>,
    #[prop(into, optional)] is_invalid: Signal<bool>,
    #[prop(optional)] validate: Option<ValidateFn<String>>,
    #[prop(optional)] validation_behavior: Option<ValidationBehavior>,
    #[prop(into, optional)] name: Option<String>,
    #[prop(into, optional)] min_length: Option<u32>,
    #[prop(into, optional)] max_length: Option<u32>,
    /// The `autocomplete` hint, e.g. `"email"` or `"off"`.
    #[prop(into, optional)]
    auto_complete: Option<String>,
    #[prop(optional)] auto_focus: bool,
    /// The input's id.
    #[prop(into, optional)]
    id: Option<String>,
    #[prop(into, optional)] form: Option<String>,
    #[prop(into, optional)] pattern: Option<String>,
    #[prop(into, optional)] input_mode: Option<InputMode>,
    #[prop(into, optional)] aria_describedby: Option<String>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] on_focus_change: Option<Callback<bool>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let input = move || {
        if multiline {
            view! { <TextArea classes="leptonic-text-field-input" /> }.into_any()
        } else {
            view! { <Input classes="leptonic-text-field-input" /> }.into_any()
        }
    };
    let mut props = TextFieldAtomProps::builder()
        .default_value(default_value)
        .input_type(input_type)
        .placeholder(placeholder)
        .is_disabled(is_disabled)
        .is_read_only(is_read_only)
        .is_required(is_required)
        .is_invalid(is_invalid)
        .auto_focus(auto_focus)
        .aria_label(aria_label)
        .classes(classes.add("leptonic-text-field"))
        .styles(styles)
        .children(Box::new(move || field_parts(label, description, input)))
        .build();
    props.on_change = on_change;
    props.value = value;
    props.set_value = set_value;
    props.validate = validate;
    props.validation_behavior = validation_behavior;
    props.name = name;
    props.min_length = min_length;
    props.max_length = max_length;
    props.auto_complete = auto_complete;
    props.id = id;
    props.form = form;
    props.pattern = pattern;
    props.input_mode = input_mode;
    props.aria_describedby = aria_describedby;
    props.on_focus_change = on_focus_change;
    TextFieldAtom(props)
}

/// A search field with its label, description and a button clearing it (shown while it has a
/// value). Enter calls `on_submit`, Escape clears.
#[allow(clippy::too_many_arguments, clippy::needless_pass_by_value)]
#[component]
pub fn SearchField(
    /// The visible label. Without it, set `aria_label`.
    #[prop(into, optional)]
    label: MaybeProp<String>,
    #[prop(into, optional)] description: MaybeProp<String>,
    #[prop(into, optional)] default_value: String,
    #[prop(into, optional)] on_change: Option<Callback<String>>,
    /// The value (controlled): a value or any signal.
    #[prop(into, optional)]
    value: Option<Signal<String>>,
    /// Receives the new state: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_value: Option<Out<String>>,
    #[prop(into, optional)] on_submit: Option<Callback<String>>,
    #[prop(into, optional)] on_clear: Option<Callback<()>>,
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] is_read_only: Signal<bool>,
    #[prop(into, optional)] is_required: Signal<bool>,
    #[prop(into, optional)] is_invalid: Signal<bool>,
    #[prop(optional)] validate: Option<ValidateFn<String>>,
    #[prop(optional)] validation_behavior: Option<ValidationBehavior>,
    #[prop(into, optional)] name: Option<String>,
    #[prop(optional)] auto_focus: bool,
    /// The input's id.
    #[prop(into, optional)]
    id: Option<String>,
    #[prop(into, optional)] form: Option<String>,
    #[prop(into, optional)] pattern: Option<String>,
    #[prop(into, optional)] input_mode: Option<InputMode>,
    #[prop(into, optional)] aria_describedby: Option<String>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] on_focus_change: Option<Callback<bool>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let input = || {
        view! {
            <div class="leptonic-search-field-input">
                <Input classes="leptonic-text-field-input" />
                <SearchFieldClearButton classes="leptonic-search-field-clear">
                    <Icon icon=icondata::BsXCircleFill />
                </SearchFieldClearButton>
            </div>
        }
        .into_any()
    };
    let mut props = SearchFieldAtomProps::builder()
        .default_value(default_value)
        .placeholder(placeholder)
        .is_disabled(is_disabled)
        .is_read_only(is_read_only)
        .is_required(is_required)
        .is_invalid(is_invalid)
        .auto_focus(auto_focus)
        .aria_label(aria_label)
        .classes(
            classes
                .add("leptonic-text-field")
                .add("leptonic-search-field"),
        )
        .styles(styles)
        .children(Box::new(move || field_parts(label, description, input)))
        .build();
    props.on_change = on_change;
    props.value = value;
    props.set_value = set_value;
    props.on_submit = on_submit;
    props.on_clear = on_clear;
    props.validate = validate;
    props.validation_behavior = validation_behavior;
    props.name = name;
    props.id = id;
    props.form = form;
    props.pattern = pattern;
    props.input_mode = input_mode;
    props.aria_describedby = aria_describedby;
    props.on_focus_change = on_focus_change;
    SearchFieldAtom(props)
}
