use leptos::{context::Provider, prelude::*};

use crate::{
    hooks::{DialogRole, IntoAttrs, UseDialogInput, UseDialogReturn, use_dialog},
    utils::{classes::Classes, styles::Styles},
};

/// Context provided by [`Dialog`] for child components like [`DialogTitle`] and [`DialogDescription`].
#[derive(Debug, Clone)]
pub struct DialogContext {
    /// The ID for the title element (wires `aria-labelledby`).
    pub title_id: String,
    /// The ID for the description element (wires `aria-describedby`).
    pub description_id: String,
}

/// A dialog container that provides ARIA semantics via `use_dialog`.
///
/// Renders a `<div>` with `role="dialog"` (or `role="alertdialog"`),
/// `aria-labelledby`, `aria-describedby`, and focus-on-mount behavior.
///
/// Children can use [`DialogTitle`] and [`DialogDescription`] to automatically
/// wire up the ARIA label/description associations.
#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn Dialog(
    /// Title text for `aria-labelledby`.
    #[prop(into, optional)]
    title: Option<String>,

    /// Description text for `aria-describedby`.
    #[prop(into, optional)]
    description: Option<String>,

    /// Accessible label. Suppresses `aria-labelledby` when set.
    #[prop(into, optional)]
    aria_label: Option<String>,

    /// Dialog role. Defaults to `Dialog`.
    #[prop(default = DialogRole::Dialog)]
    role: DialogRole,

    #[prop(into, optional)] classes: Classes,

    #[prop(into, optional)] styles: Styles,

    children: Children,
) -> impl IntoView {
    let UseDialogReturn {
        dialog_props,
        title_props,
        description_props,
        dialog_id: _,
    } = use_dialog(UseDialogInput {
        title,
        description,
        aria_label,
        role,
    });

    let ctx = DialogContext {
        title_id: title_props.id,
        description_id: description_props.id,
    };

    view! {
        <Provider value=ctx>
            <div {..dialog_props.into_attrs()} class=classes style=styles>
                {children()}
            </div>
        </Provider>
    }
}

/// Renders a title element wired to the parent [`Dialog`]'s `aria-labelledby`.
///
/// When used outside a `Dialog`, renders without an ID.
#[component]
pub fn DialogTitle(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<DialogContext>();
    let id = ctx.map(|c| c.title_id);
    view! { <div id=id class=classes style=styles>{children()}</div> }
}

/// Renders a description element wired to the parent [`Dialog`]'s `aria-describedby`.
///
/// When used outside a `Dialog`, renders without an ID.
#[component]
pub fn DialogDescription(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<DialogContext>();
    let id = ctx.map(|c| c.description_id);
    view! { <div id=id class=classes style=styles>{children()}</div> }
}
