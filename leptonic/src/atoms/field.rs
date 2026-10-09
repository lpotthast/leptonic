// Upstream: react-aria-components/src/Label.tsx @ 99e6102368
// Upstream: react-aria-components/src/Text.tsx @ 99e6102368
// Upstream: react-aria-components/src/FieldError.tsx @ 99e6102368
// Upstream: react-aria-components/test/FieldError.test.js @ 99e6102368
use std::sync::Arc;

use leptos::{
    either::{Either, EitherOf3},
    ev,
    prelude::*,
};
use leptos_classes::Classes;
use web_sys::MouseEvent;

use crate::{
    CapturedElement, EventHandler, IntoAttrs, SlotProps,
    hooks::form::{LabelElementType, UseLabelProps, ValidationResult, ValidityStateSnapshot},
    use_slot,
    utils::{
        default_class::with_default_class, dev_warn, scoped_context::use_clearable_context,
        styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `LabelContext` as upstream; one `FieldContext` replaces `TextContext` (slots `description`
//   and `errorMessage`) and `FieldErrorContext`. Reason: Leptos contexts are typed, so the slot
//   strings become separate `Description` and `FieldError` components.
// - `Text`'s `description` slot is the `Description` component. `elementType` strings become
//   `TextElement`.
// - Render props become plain children; `FieldError` shows the joined validation errors when it
//   has none. Its render function for the validation result is the `message` prop (returning the
//   text, not a view). Reason: a component's children aren't a function of its state in Leptos.
//
// =============================================================================

/// What a field atom (text field, checkbox group, select, ...) provides to its [`Description`]
/// and [`FieldError`] (beside a [`LabelContext`] for its [`Label`]). Field atoms built from hooks
/// provide it to use these parts.
#[derive(Debug, Clone)]
pub struct FieldContext {
    /// `description_props` of the field's hook.
    pub description: SlotProps,
    /// `error_message_props` of the field's hook.
    pub error_message: SlotProps,
    pub is_invalid: Signal<bool>,
    pub validation_errors: Signal<Vec<String>>,
    pub validation_details: Signal<ValidityStateSnapshot>,
}

/// The message of a [`FieldError`] for a validation result; `None` shows no error.
pub type FieldErrorMessage = Arc<dyn Fn(&ValidationResult) -> Option<String> + Send + Sync>;

/// What an atom with a visible label (a field, a progress bar, ...) provides to its [`Label`]: how
/// the label is rendered.
#[derive(Debug, Clone)]
pub struct LabelContext {
    /// `label_props` of the field's hook.
    pub props: UseLabelProps,
    /// The element the label is rendered as (`Span` for fields a `<label>` can't label: groups,
    /// select triggers).
    pub element_type: LabelElementType,
    /// Handles clicks on the label, for fields whose label isn't a `<label>` (focusing the field).
    pub on_click: EventHandler<MouseEvent>,
    /// Captures the rendered label (see [`LabelPresence`]).
    element: CapturedElement,
    /// The text a `Label` without children shows (react-aria-components: the context's
    /// `children`, e.g. a color slider's channel name).
    default_text: Option<Signal<String>>,
}

impl LabelContext {
    /// A `<label>` labelling its field natively.
    pub fn label(props: UseLabelProps) -> Self {
        Self {
            props,
            element_type: LabelElementType::Label,
            on_click: EventHandler::empty(),
            element: CapturedElement::new(),
            default_text: None,
        }
    }

    /// A `<span>`, for fields a `<label>` can't label.
    pub fn span(props: UseLabelProps) -> Self {
        Self {
            props,
            element_type: LabelElementType::Span,
            on_click: EventHandler::empty(),
            element: CapturedElement::new(),
            default_text: None,
        }
    }

    /// Reports the rendered label to `presence`.
    #[must_use]
    pub(crate) fn with_presence(self, presence: LabelPresence) -> Self {
        Self {
            element: presence.element,
            ..self
        }
    }

    /// Shows `text` in a `Label` without children.
    #[must_use]
    pub fn with_default_text(self, text: Signal<String>) -> Self {
        Self {
            default_text: Some(text),
            ..self
        }
    }

    /// Runs `on_click` when the label is clicked.
    #[must_use]
    pub fn with_on_click(self, on_click: EventHandler<MouseEvent>) -> Self {
        Self { on_click, ..self }
    }
}

/// Whether an atom's [`Label`] part is rendered, for its hook's `has_label` (react-aria-components:
/// `useSlot`). Until mounted, it guesses from the atom's ARIA props (a label is expected without
/// `aria-label` and `aria-labelledby`), so server-rendered HTML references a likely label; then it
/// follows the rendered label.
#[derive(Debug, Clone, Copy)]
pub(crate) struct LabelPresence {
    pub has_label: Signal<bool>,
    element: CapturedElement,
}

impl LabelPresence {
    pub(crate) fn new(aria_label: MaybeProp<String>, aria_labelledby: Option<&String>) -> Self {
        let guess = aria_label.get_untracked().is_none() && aria_labelledby.is_none();
        let element = CapturedElement::new();
        let mounted = RwSignal::new(false);
        // Effects run on the client only, after the children (and the label) are rendered.
        Effect::new(move || mounted.set(true));
        Self {
            has_label: Signal::derive(move || {
                if mounted.get() {
                    element.get().is_some()
                } else {
                    guess
                }
            }),
            element,
        }
    }
}

/// The element a [`Description`] or [`FieldError`] renders.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TextElement {
    #[default]
    Span,
    /// For block-level children (e.g. a `<ul>` of errors).
    Div,
}

/// The visible label of the atom around it (see [`LabelContext`]). Outside one, a plain `<label>`.
/// Without children, it shows the atom's default text, if it has one (e.g. a color slider's
/// channel name).
///
/// Default class: `leptonic-Label`.
#[component]
pub fn Label(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let classes = with_default_class("leptonic-Label", classes);
    let context = use_clearable_context::<LabelContext>();
    let default_text = context.as_ref().and_then(|context| context.default_text);
    let children = move || match children {
        Some(children) => Either::Left(children()),
        None => Either::Right(default_text),
    };
    match context {
        Some(LabelContext {
            props,
            element_type: LabelElementType::Span,
            on_click,
            element,
            ..
        }) => EitherOf3::A(view! {
            <span {..props.into_attrs()} {..(on_click.into_on(ev::click), element.attr())} class=classes style=styles>
                {children()}
            </span>
        }),
        Some(LabelContext {
            props,
            element_type: LabelElementType::Label,
            on_click,
            element,
            ..
        }) => EitherOf3::B(view! {
            <label {..props.into_attrs()} {..(on_click.into_on(ev::click), element.attr())} class=classes style=styles>
                {children()}
            </label>
        }),
        None => EitherOf3::C(view! { <label class=classes style=styles>{children()}</label> }),
    }
}

/// A description of the field around it, referenced by the field (`aria-describedby`) while
/// rendered.
///
/// Default class: `leptonic-Description`.
#[component]
pub fn Description(
    #[prop(optional)] element: TextElement,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-Description", classes);
    let props = use_clearable_context::<FieldContext>().map_or_else(
        || {
            // Outside a field, the id is referenced by nothing.
            dev_warn!(
                "A <Description> describes nothing outside a field (TextField, Checkbox, ...)."
            );
            use_slot("description").props
        },
        |ctx| ctx.description,
    );
    text(element, props, classes, styles, children())
}

/// The validation errors of the field around it, rendered only while it is invalid. It shows its
/// children, else `message` for the validation result (e.g. by its `validation_details`), else the
/// field's validation errors (and nothing when there are none).
///
/// Default class: `leptonic-FieldError`.
#[component]
pub fn FieldError(
    #[prop(optional)] element: TextElement,
    /// The message for the field's validation result; `None` shows no error.
    #[prop(optional)]
    message: Option<FieldErrorMessage>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let classes = with_default_class("leptonic-FieldError", classes);
    let Some(ctx) = use_clearable_context::<FieldContext>() else {
        dev_warn!("A <FieldError> shows nothing outside a field (TextField, Checkbox, ...).");
        return None;
    };
    let has_children = children.is_some();
    // The message, unless children replace it: only this text follows the validation.
    let message_text = {
        let (is_invalid, errors, details) = (
            ctx.is_invalid,
            ctx.validation_errors,
            ctx.validation_details,
        );
        Memo::new(move |_| {
            if has_children || !is_invalid.get() {
                return None;
            }
            match &message {
                Some(message) => message(&ValidationResult {
                    is_invalid: true,
                    validation_errors: errors.get(),
                    validation_details: details.get(),
                }),
                None => errors.with(|errors| (!errors.is_empty()).then(|| errors.join(" "))),
            }
        })
    };
    // The element is rendered (and its id referenced by the field) only while this changes, not
    // on every change of the errors.
    let is_shown = Memo::new(move |_| {
        ctx.is_invalid.get() && (has_children || message_text.with(Option::is_some))
    });
    let children = StoredValue::new(children);
    let error_message = ctx.error_message;
    Some(move || {
        is_shown.get().then(|| {
            let content = match children.with_value(|c| c.as_ref().map(|c| c())) {
                Some(children) => children,
                None => (move || message_text.get()).into_any(),
            };
            text(
                element,
                error_message.clone(),
                classes.clone(),
                styles.clone(),
                content,
            )
        })
    })
}

fn text(
    element: TextElement,
    props: SlotProps,
    classes: Classes,
    styles: Styles,
    children: AnyView,
) -> impl IntoView {
    let attrs = props.into_attrs();
    match element {
        TextElement::Span => Either::Left(view! {
            <span {..attrs} class=classes style=styles>{children}</span>
        }),
        TextElement::Div => Either::Right(view! {
            <div {..attrs} class=classes style=styles>{children}</div>
        }),
    }
}
