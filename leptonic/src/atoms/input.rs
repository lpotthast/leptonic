// Upstream: react-aria-components/src/Input.tsx @ 99e6102368
// Upstream: react-aria-components/src/TextArea.tsx @ 99e6102368
use std::sync::Arc;

use leptos::{
    attr::{
        Attribute,
        any_attribute::{AnyAttribute, IntoAnyAttribute},
    },
    either::Either,
    ev,
    prelude::*,
};
use leptos_classes::Classes;

use crate::{
    IntoAttrs,
    hooks::{
        focus::use_focus_ring::{UseFocusRingInput, use_focus_ring},
        form::UseTextFieldInputProps,
        interactions::{UseHoverInput, use_hover},
    },
    utils::{
        data_attributes::flag, default_class::with_default_class,
        scoped_context::use_clearable_context, styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - In a field, the field computes the focus state (`use_text_field`'s focus ring) and passes it
//   through `InputContext`, instead of `Input` running its own `useFocusRing`. Reason: the field's
//   hook already tracks focus; a second focus ring on the same element would duplicate it.
//   Outside a field, `Input`/`TextArea` run their own focus ring and hover, as in
//   react-aria-components.
// - Outside a field, `is_disabled` and `is_invalid` props set `disabled` and `aria-invalid` (and
//   the data attributes); react-aria-components reads the DOM props `disabled` and
//   `aria-invalid`. In a field, the field decides both.
// - One `InputContext` serves `Input` and `TextArea` (react-aria-components: `InputContext` and
//   `TextAreaContext`); a `TextArea` takes it only from a text field and is a plain textarea
//   in other fields, as in react-aria-components, where only `TextField` provides
//   `TextAreaContext`.
// - Render props become `data-*` attributes.
//
// ## DIFFERENT BEHAVIOR
// - A disabled `TextArea` doesn't track hover (react-aria-components' `TextArea` doesn't pass
//   `isDisabled` to `useHover`, unlike its `Input`). Reason: `Input` and `TextArea` behave the
//   same.
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

    /// The input of a [`use_text_field`](fn@crate::hooks::form::use_text_field) field, which can also be
    /// a [`TextArea`].
    pub fn text_field(props: UseTextFieldInputProps, state: InputState) -> Self {
        let attrs_props = props.clone();
        Self {
            text_field: Some(StoredValue::new(props)),
            ..Self::new(move || attrs_props.clone().into_attrs(), state)
        }
    }
}

/// A text input. In a field (`TextField`, `ComboBox`, ...), the field's `<input>`, with the
/// field's attributes and state. Outside a field (or inside a field's popover), a plain `<input>`
/// tracking its own hover and focus.
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
    /// Outside a field: whether the input is disabled. In a field, the field decides.
    #[prop(into, optional)]
    is_disabled: Signal<bool>,
    /// Outside a field: whether the value is invalid (`aria-invalid`). In a field, the field
    /// decides.
    #[prop(into, optional)]
    is_invalid: Signal<bool>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let classes = with_default_class("leptonic-Input", classes);
    let Some(ctx) = use_clearable_context::<InputContext>() else {
        return Either::Right(view! {
            <input
                {..standalone_attributes(is_disabled, is_invalid)}
                node_ref=node_ref
                class=classes
                style=styles
            />
        });
    };
    let state = ctx.state;
    let hover = use_hover(UseHoverInput {
        is_disabled: state.is_disabled,
        ..UseHoverInput::default()
    });

    Either::Left(view! {
        <input
            {..(ctx.attrs)()}
            {..hover.props.into_attrs()}
            {..state_attributes(state, hover.is_hovered)}
            node_ref=node_ref
            class=classes
            style=styles
        />
    })
}

/// A multi-line text input. In a text field, the field's `<textarea>`, with the field's
/// attributes and state. Elsewhere (also in other fields), a plain `<textarea>` tracking its own
/// hover and focus.
///
/// Data attributes: as [`Input`].
///
/// Default class: `leptonic-TextArea`.
#[component]
pub fn TextArea(
    /// The `<textarea>` element (react-aria-components: a forwarded `ref`).
    #[prop(optional)]
    node_ref: NodeRef<leptos::html::Textarea>,
    /// Outside a text field: whether the textarea is disabled. In a text field, the field decides.
    #[prop(into, optional)]
    is_disabled: Signal<bool>,
    /// Outside a text field: whether the value is invalid (`aria-invalid`). In a text field, the
    /// field decides.
    #[prop(into, optional)]
    is_invalid: Signal<bool>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let classes = with_default_class("leptonic-TextArea", classes);
    let Some((state, text_field)) =
        use_clearable_context::<InputContext>().and_then(|ctx| Some((ctx.state, ctx.text_field?)))
    else {
        return Either::Right(view! {
            <textarea
                {..standalone_attributes(is_disabled, is_invalid)}
                node_ref=node_ref
                class=classes
                style=styles
            />
        });
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

    Either::Left(view! {
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

/// The attributes of an input outside a field: its own hover and focus ring
/// (react-aria-components: `useHover` and `useFocusRing` with `isTextInput`), `disabled`,
/// `aria-invalid` and the data attributes.
fn standalone_attributes(
    is_disabled: Signal<bool>,
    is_invalid: Signal<bool>,
) -> impl Attribute + 'static {
    let hover = use_hover(UseHoverInput {
        is_disabled,
        ..UseHoverInput::default()
    });
    let focus_ring = use_focus_ring(UseFocusRingInput {
        is_text_input: true,
        ..UseFocusRingInput::default()
    });
    let state = InputState {
        is_disabled,
        is_invalid,
        is_focused: focus_ring.is_focused,
        is_focus_visible: focus_ring.is_focus_visible,
    };
    (
        hover.props.into_attrs(),
        // The focus ring's own `data-focus-visible` comes with the other data attributes.
        focus_ring.props.on_focus.into_on(ev::focus),
        focus_ring.props.on_blur.into_on(ev::blur),
        state_attributes(state, hover.is_hovered),
        leptos::attr::disabled(is_disabled),
        leptos::attr::aria_invalid(flag(is_invalid)),
    )
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
