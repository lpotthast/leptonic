// Upstream: react-aria-components/src/Input.tsx @ 99e6102368
// Upstream: react-aria-components/src/TextArea.tsx @ 99e6102368
use std::sync::Arc;

use leptos::{
    attr::{
        Attribute,
        any_attribute::{AnyAttribute, IntoAnyAttribute},
    },
    prelude::*,
};

use crate::{
    hooks::{IntoAttrs, UseHoverInput, UseTextFieldInputProps, use_hover},
    utils::{
        classes::Classes, data_attributes::flag, default_class::with_default_class, styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The field computes the focus state (`use_text_field`'s focus ring) and passes it through
//   `InputContext`, instead of `Input` running its own `useFocusRing`. Reason: the field's hook
//   already tracks focus; a second focus ring on the same element would duplicate it.
// - Render props become `data-*` attributes.
//
// =============================================================================

/// The state a field's input shows in its data attributes.
#[derive(Debug, Clone, Copy)]
pub struct InputState {
    pub is_disabled: Signal<bool>,
    pub is_invalid: Signal<bool>,
    pub is_focused: Signal<bool>,
    pub is_focus_visible: Signal<bool>,
}

/// What a field provides to its [`Input`] (or [`TextArea`]): the input's attributes from the
/// field's hook, and its state.
#[derive(Clone)]
pub struct InputContext {
    attrs: Arc<dyn Fn() -> AnyAttribute + Send + Sync>,
    /// A text field's props, which a [`TextArea`] can render too.
    text_field: Option<StoredValue<UseTextFieldInputProps>>,
    pub state: InputState,
}

impl InputContext {
    /// The input of a field with any attributes (e.g. a field built from hooks): `attrs` creates
    /// them for each rendered input.
    pub fn new<A>(attrs: impl Fn() -> A + Send + Sync + 'static, state: InputState) -> Self
    where
        A: Attribute + Send + 'static,
    {
        Self {
            attrs: Arc::new(move || attrs().into_any_attr()),
            text_field: None,
            state,
        }
    }

    /// The input of a [`use_text_field`](crate::hooks::use_text_field) field, which can also be
    /// a [`TextArea`].
    pub fn text_field(props: UseTextFieldInputProps, state: InputState) -> Self {
        let attrs_props = props.clone();
        Self {
            text_field: Some(StoredValue::new(props)),
            ..Self::new(move || attrs_props.clone().into_attrs(), state)
        }
    }
}

/// The `<input>` of the field around it.
///
/// Data attributes: `data-focused`, `data-focus-visible`, `data-hovered`, `data-disabled`,
/// `data-invalid`.
///
/// Default class: `leptonic-Input`.
#[component]
pub fn Input(
    /// The `<input>` element, e.g. to focus it from a shortcut (react-aria-components: a
    /// forwarded `ref`).
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Input>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let classes = with_default_class("leptonic-Input", classes);
    let ctx = expect_context::<InputContext>();
    let state = ctx.state;
    let hover = use_hover(UseHoverInput {
        is_disabled: state.is_disabled,
        ..UseHoverInput::default()
    });

    view! {
        <input
            {..(ctx.attrs)()}
            {..hover.props.into_attrs()}
            {..state_attributes(state, hover.is_hovered)}
            node_ref=node_ref
            class=classes
            style=styles
        />
    }
}

/// The `<textarea>` of the field around it (for multi-line text).
///
/// Data attributes: as [`Input`].
///
/// Default class: `leptonic-TextArea`.
#[component]
pub fn TextArea(
    /// The `<textarea>` element (react-aria-components: a forwarded `ref`).
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Textarea>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let classes = with_default_class("leptonic-TextArea", classes);
    let ctx = expect_context::<InputContext>();
    let state = ctx.state;
    let Some(text_field) = ctx.text_field else {
        crate::utils::dev_warn!("A <TextArea> needs a text field: use an <Input>.");
        return None;
    };
    let mut props = text_field.get_value();
    // `type`, `pattern` and the `value` attribute only exist on inputs (react-aria:
    // `inputElementType`); a textarea renders its initial value as its content.
    let initial_value = props.value.take();
    props.r#type = Signal::stored(None);
    props.pattern = None;
    let hover = use_hover(UseHoverInput {
        is_disabled: state.is_disabled,
        ..UseHoverInput::default()
    });

    Some(view! {
        <textarea
            {..props.into_attrs()}
            {..hover.props.into_attrs()}
            {..state_attributes(state, hover.is_hovered)}
            node_ref=node_ref
            class=classes
            style=styles
        >
            {initial_value}
        </textarea>
    })
}

/// The data attributes of an input.
fn state_attributes(
    state: InputState,
    is_hovered: Signal<bool>,
) -> impl Attribute + Clone + 'static {
    (
        data_attribute("data-focused", state.is_focused),
        data_attribute("data-focus-visible", state.is_focus_visible),
        data_attribute("data-hovered", is_hovered),
        data_attribute("data-disabled", state.is_disabled),
        data_attribute("data-invalid", state.is_invalid),
    )
}

/// A `data-*` flag attribute (see [`flag`]).
fn data_attribute(
    name: &'static str,
    signal: Signal<bool>,
) -> leptos::attr::custom::CustomAttr<&'static str, impl Fn() -> Option<&'static str> + Send + Clone>
{
    leptos::attr::custom::custom_attribute(name, flag(signal))
}
