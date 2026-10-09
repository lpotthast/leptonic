// Upstream: react-aria/src/interactions/useFocusable.tsx @ 99e6102368
// Upstream: react-aria/test/interactions/Focusable.test.js @ 99e6102368
use leptos::{attr::Attr, ev, prelude::*};
use send_wrapper::SendWrapper;
use web_sys::FocusEvent;

use crate::{
    CapturedElement, IdRefs,
    hooks::{
        focus::{FocusableContextAttr, UseFocusableInput, UseFocusableProps, use_focusable},
        interactions::KeyboardEventWrapper,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## DIFFERENT BEHAVIOR
// - The child's `tabindex` is set on the client, in an Effect, once the child element is known
//   (react-aria merges `tabIndex={0}` into the child's props, so it is in the server HTML too):
//   the atom takes any child view, whose own `tabindex` it can only see on the element. The
//   server HTML has no `tabindex` (until hydration, the child is focusable only if it is
//   natively).
// - The development checks (an element child with an interactive role) run when the child is
//   mounted and warn (`dev_warn!`) instead of throwing for a missing ref.
//
// =============================================================================

/// Makes its child element focusable (react-aria's `Focusable`): the focus and keyboard handlers
/// go onto the child itself, which gets a `tabindex` where it has none and must have an
/// interactive role (or be an image). A surrounding trigger's [`FocusableContext`] applies to it,
/// e.g. a `TooltipTrigger`'s description.
///
/// ```ignore
/// <TooltipTrigger>
///     <Focusable>
///         <span role="img" aria-label="Info">"ⓘ"</span>
///     </Focusable>
///     <Tooltip>"More details"</Tooltip>
/// </TooltipTrigger>
/// ```
///
/// [`FocusableContext`]: crate::hooks::focus::FocusableContext
#[component]
pub fn Focusable<V>(
    /// Whether the element can't be focused.
    #[prop(into, optional)]
    is_disabled: Signal<bool>,
    /// Whether the element is skipped when tabbing (it stays focusable by click or script).
    #[prop(into, optional)]
    exclude_from_tab_order: Signal<bool>,
    /// Whether to focus the element when it mounts.
    #[prop(optional)]
    auto_focus: bool,
    #[prop(into, optional)] on_focus: Option<Callback<FocusEvent>>,
    #[prop(into, optional)] on_blur: Option<Callback<FocusEvent>>,
    #[prop(into, optional)] on_focus_change: Option<Callback<bool>>,
    #[prop(into, optional)] on_key_down: Option<Callback<KeyboardEventWrapper>>,
    #[prop(into, optional)] on_key_up: Option<Callback<KeyboardEventWrapper>>,
    /// The focusable element.
    children: TypedChildren<V>,
) -> impl IntoView
where
    V: IntoView + 'static,
{
    // Auto focus happens in `manage_child`, once the child has its `tabindex`.
    let focusable = use_focusable(UseFocusableInput {
        is_disabled,
        exclude_from_tab_order,
        on_focus,
        on_blur,
        on_focus_change,
        on_key_down,
        on_key_up,
        ..UseFocusableInput::default()
    })
    .props;
    let element = CapturedElement::new();
    manage_child(
        ChildKind::Focusable,
        element,
        focusable.tabindex,
        auto_focus,
    );
    let (focusable, on_keydown) = focusable_child_attrs(focusable, element, None);
    children.into_inner()().add_any_attr((focusable, on_keydown.into_on(ev::keydown)))
}

/// The atom putting its handlers onto its child: it checks the child differently.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ChildKind {
    Pressable,
    Focusable,
}

/// Makes the child of a [`Focusable`] or `Pressable` focusable: a `tabindex` of its own wins (as
/// upstream, where the child's props override), otherwise it follows `tabindex` (on the client).
/// Then focuses it when `auto_focus` is set (once it can take focus). In development, the child
/// must be an element with an interactive role, as upstream checks.
pub(crate) fn manage_child(
    kind: ChildKind,
    element: CapturedElement,
    tabindex: Signal<Option<i32>>,
    auto_focus: bool,
) {
    // The child element, and whether it had no `tabindex` of its own (decided per element: a
    // reactive child may be replaced).
    // Thread-safe storage that stays `None` on the server: a local one is dropped on whichever
    // thread finishes the response, which panics.
    let managed = StoredValue::new(None::<(SendWrapper<web_sys::Element>, bool)>);
    Effect::new(move |previous: Option<()>| {
        let Some(el) = element.get() else {
            // Only a missing child, not an unmounting one.
            if previous.is_none() {
                crate::utils::dev_warn!("<{kind:?}> needs an element as its child.");
            }
            return;
        };
        let el: web_sys::Element = (*el).clone();
        let known = managed.with_value(|managed| {
            managed
                .as_ref()
                .filter(|(managed_el, _)| **managed_el == el)
                .map(|(_, manage)| *manage)
        });
        let first = known.is_none();
        let manage = known.unwrap_or_else(|| {
            let manage = !el.has_attribute("tabindex");
            managed.set_value(Some((SendWrapper::new(el.clone()), manage)));
            if cfg!(debug_assertions) {
                check_role(kind, &el);
            }
            manage
        });
        if manage {
            let _ = match tabindex.get() {
                Some(index) => el.set_attribute("tabindex", &index.to_string()),
                None => el.remove_attribute("tabindex"),
            };
        }
        if first && auto_focus && previous.is_none() {
            crate::utils::focus::focus_safely(&el);
        }
    });
}

/// https://w3c.github.io/aria/#widget_roles
const WIDGET_ROLES: [&str; 19] = [
    "application",
    "button",
    "checkbox",
    "combobox",
    "gridcell",
    "link",
    "menuitem",
    "menuitemcheckbox",
    "menuitemradio",
    "option",
    "radio",
    "searchbox",
    "separator",
    "slider",
    "spinbutton",
    "switch",
    "tab",
    "textbox",
    "treeitem",
];

/// Further roles a focusable element may have: `aria-describedby` is announced on them too.
const FOCUSABLE_ROLES: [&str; 4] = ["tabpanel", "img", "meter", "progressbar"];

/// Upstream's development check: the child must announce itself as interactive.
fn check_role(kind: ChildKind, el: &web_sys::Element) {
    let native: &[&str] = match kind {
        ChildKind::Pressable => &[
            "button", "input", "select", "textarea", "a", "area", "summary",
        ],
        ChildKind::Focusable => &[
            "button", "input", "select", "textarea", "a", "area", "summary", "img", "svg",
        ],
    };
    if native.contains(&el.local_name().as_str()) {
        return;
    }
    match el.get_attribute("role") {
        None => crate::utils::dev_warn!("<{kind:?}> child must have an interactive ARIA role."),
        Some(role)
            if !WIDGET_ROLES.contains(&role.as_str())
                && (kind != ChildKind::Focusable || !FOCUSABLE_ROLES.contains(&role.as_str())) =>
        {
            crate::utils::dev_warn!(
                "<{kind:?}> child must have an interactive ARIA role. Got \"{role}\"."
            );
        }
        Some(_) => {}
    }
}

/// The attributes `use_focusable` gives the child (without `tabindex`, see [`manage_child`]):
/// focus handlers, the element capture, the description (the context's, and a press' long-press
/// description) and the context's attributes. The keydown
/// handler is returned separately, for chaining.
pub(crate) fn focusable_child_attrs(
    focusable: UseFocusableProps,
    element: CapturedElement,
    press_describedby: Option<Signal<Option<String>>>,
) -> (
    impl leptos::tachys::html::attribute::Attribute,
    crate::EventHandler<web_sys::KeyboardEvent>,
) {
    (
        (
            focusable.element_capture.chain(element.attr()),
            focusable.on_keyup.into_on(ev::keyup),
            focusable.on_focus.into_on(ev::focus),
            focusable.on_blur.into_on(ev::blur),
            // The press' long-press description and the context's, as `use_button` merges them.
            Attr(
                leptos::attr::AriaDescribedby,
                IdRefs::derive(
                    press_describedby
                        .into_iter()
                        .chain([focusable.context_aria_describedby]),
                ),
            ),
            FocusableContextAttr(focusable.context_attrs),
        ),
        focusable.on_keydown,
    )
}
