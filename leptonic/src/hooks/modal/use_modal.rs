// Upstream: react-aria/src/overlays/useModal.tsx @ 99e6102368
use leptos::{attr, attr::Attr, prelude::*};

use crate::{hooks::IntoAttrs, utils::aria::AriaModal};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## DIFFERENT BEHAVIOR
// - Sets `aria-modal="true"` on the element instead of react-aria's `data-ismodal` marker for
//   its `ModalProvider`, which hides the content of parent providers with `aria-hidden`.
//   `use_modal_backdrop` makes everything outside the modal inert (`aria_hide_outside`), which
//   replaces the provider system.
// - The modal atoms (`ModalContent`) don't use this hook: as react-aria's `useDialog`, they set no
//   `aria-modal` (WebKit bug 211934: Safari then focuses the first focusable element on its own);
//   the inert content outside makes the modal modal.
//
// ## OMITTED FEATURES
// - `ModalProvider`, `OverlayProvider`, `OverlayContainer` and `useModalProvider` (see above).
//
// =============================================================================

/// Input parameters for the `use_modal` hook.
#[derive(Debug, Clone, Copy, Default)]
pub struct UseModalInput {
    /// Whether the modal behavior is disabled: then `aria-modal` is not set. Default: `false`.
    pub is_disabled: Signal<bool>,
}

/// The return value of the `use_modal` hook.
pub struct UseModalReturn {
    /// Props for the modal element. Call `.into_attrs()` for view spreading.
    pub modal_props: UseModalProps,
}

/// Props from `use_modal` that can be converted to spreadable attributes.
#[derive(Debug)]
pub struct UseModalProps {
    pub aria_modal: Signal<Option<AriaModal>>,
}

impl IntoAttrs for UseModalProps {
    type Attrs = UseModalAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::AriaModal, self.aria_modal),)
    }
}

/// These attributes must be spread onto the modal element: `<div {..attrs} />`
pub type UseModalAttrs = (Attr<attr::AriaModal, Signal<Option<AriaModal>>>,);

/// Marks an element as a modal for assistive technology: `aria-modal="true"`, so that screen
/// readers treat the content outside it as hidden.
///
/// Prefer an element with a `dialog` role for `aria-modal` (on an element without a role, it is
/// invalid ARIA), and mind WebKit bug 211934: Safari focuses the first focusable element inside an
/// `aria-modal` element. The modal atoms don't use this hook; `use_modal_backdrop` hides the content
/// outside the modal (inert), which makes it modal for everyone.
///
/// # Example
///
/// ```ignore
/// let UseModalReturn { modal_props } = use_modal(UseModalInput::default());
///
/// view! {
///     <div role="dialog" {..modal_props.into_attrs()}>
///         "Modal content"
///     </div>
/// }
/// ```
pub fn use_modal(input: UseModalInput) -> UseModalReturn {
    let UseModalInput { is_disabled } = input;
    UseModalReturn {
        modal_props: UseModalProps {
            aria_modal: Signal::derive(move || (!is_disabled.get()).then_some(AriaModal::True)),
        },
    }
}
