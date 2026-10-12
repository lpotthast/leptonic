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
        form::{TextFieldElement, UseTextFieldInputProps},
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
    /// The input's attributes, for each rendered input (one at a time; an input may mount again,
    /// e.g. in a `Show`, or a text field may switch between `Input` and `TextArea`).
    attrs: Arc<dyn Fn() -> AnyAttribute + Send + Sync>,
    /// A text field's element kind and value, which a [`TextArea`] can render too.
    text_field: Option<TextFieldParts>,
    pub state: InputState,
}

/// What an [`Input`] or [`TextArea`] of a text field needs beyond its attributes.
#[derive(Clone, Copy)]
struct TextFieldParts {
    /// Which element the field renders (`use_text_field`'s `element`): set by the element that
    /// mounts (react-aria-components' `TextField` sets `inputElementType` from its input ref).
    element: RwSignal<TextFieldElement>,
    /// The field's value: a textarea's initial content.
    value: Signal<String>,
}

impl TextFieldParts {
    /// Tells the field which element renders it.
    fn rendered_as(self, element: TextFieldElement) {
        if self.element.get_untracked() != element {
            self.element.set(element);
        }
    }
}

impl InputContext {
    /// The input of a field with any attributes (e.g. a field built from hooks), cloned for each
    /// rendered input.
    pub fn new<A>(attrs: A, state: InputState) -> Self
    where
        A: Attribute + Clone + Send + Sync + 'static,
    {
        Self {
            attrs: Arc::new(move || attrs.clone().into_any_attr()),
            text_field: None,
            state,
        }
    }

    /// The input of a [`use_text_field`](fn@crate::hooks::form::use_text_field) field, which can
    /// also be a [`TextArea`]: `element` is the hook's `element`, set by the `Input` or
    /// `TextArea` that mounts.
    pub fn text_field(
        props: UseTextFieldInputProps,
        element: RwSignal<TextFieldElement>,
        state: InputState,
    ) -> Self {
        let value = props.value_property;
        Self {
            text_field: Some(TextFieldParts { element, value }),
            ..Self::new(props.into_attrs(), state)
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
    if let Some(text_field) = ctx.text_field {
        text_field.rendered_as(TextFieldElement::Input);
    }
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
    let Some((ctx, text_field)) = use_clearable_context::<InputContext>()
        .and_then(|ctx| ctx.text_field.map(|text_field| (ctx, text_field)))
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
    // Before the attributes are built: `type`, `pattern` and the `value` attribute only exist on
    // inputs (react-aria: `inputElementType`); a textarea renders its value as its content.
    text_field.rendered_as(TextFieldElement::TextArea);
    let initial_value = text_field.value.get_untracked();
    let state = ctx.state;
    let hover = use_hover(UseHoverInput {
        is_disabled: state.is_disabled,
        ..UseHoverInput::default()
    });

    Either::Left(view! {
        <textarea
            {..(ctx.attrs)()}
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
