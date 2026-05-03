use leptos::{attr, attr::Attr};

use crate::{hooks::IntoAttrs, utils::aria::AriaModal};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/overlays/src/useModal.ts

//
// ## DIFFERENT BEHAVIOR
//
// - aria-modal vs aria-hidden
//   React-aria's `useModal` aria-hides all content *outside* the modal by
//   setting `aria-hidden` on sibling DOM trees (via `ariaHideOutside()`) and
//   uses a `ModalProvider` context to track nested modals. This lets screen
//   readers ignore everything outside the modal.
//
//   Leptonic instead sets `aria-modal="true"` on the modal element itself.
//   Modern browsers/AT already honour `aria-modal` and hide outside content,
//   so the `aria-hidden` approach is unnecessary for our target environments.
//   Combined with `aria_hide_outside` (called by `use_modal_backdrop` for
//   defense in depth), this replaces React-aria's `ModalProvider` system.
//
// ## OMITTED FEATURES
//
// - `ModalProvider` / `ModalContext`: Not needed. `aria-modal="true"` (this
//   hook) plus `aria_hide_outside` (in `use_modal_backdrop`) together replace
//   the provider-based aria-hidden propagation.
//
// Note: `ariaHideOutside()` IS implemented — it lives in `use_modal_backdrop`,
// not in this hook. See `use_modal_backdrop.rs` for details.
//

/// Input parameters for the `use_modal` hook.
///
/// This hook marks an element as a modal for assistive technology by setting
/// `aria-modal="true"`. For dismiss behavior (Escape key, outside click) and
/// overlay stacking, use `use_modal_backdrop` (which delegates to `use_overlay`).
/// For ARIA role and labeling, use `use_dialog`.
#[derive(Debug, Clone, Copy, Default)]
pub struct UseModalInput {
    /// Whether the modal behavior is disabled.
    /// When `true`, `aria-modal` is not set.
    pub is_disabled: bool,
}

/// The return value of the `use_modal` hook.
pub struct UseModalReturn {
    /// Props for the modal element. Call `.into_attrs()` for view spreading.
    pub modal_props: UseModalProps,
}

/// Props from `use_modal` that can be converted to spreadable attributes.
#[derive(Debug)]
pub struct UseModalProps {
    pub aria_modal: Option<AriaModal>,
}

impl IntoAttrs for UseModalProps {
    type Attrs = UseModalAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::AriaModal, self.aria_modal),)
    }
}

/// These attributes must be spread onto the modal element: `<div {..attrs} />`
pub type UseModalAttrs = (Attr<attr::AriaModal, Option<AriaModal>>,);

/// Marks an element as a modal for assistive technology.
///
/// Sets `aria-modal="true"` on the element so that screen readers treat content
/// outside the modal as hidden.
///
/// This hook only handles the aria-modal marker. Combine with:
/// - `use_modal_backdrop` for dismiss behavior (Escape, outside click) and scroll prevention
/// - `use_dialog` for ARIA role, labeling, and focus-on-mount
/// - `FocusScope` atom for focus trapping and restoration
///
/// # Example
///
/// ```ignore
/// let UseModalReturn { modal_props } = use_modal(UseModalInput { is_disabled: false });
///
/// view! {
///     <div {..modal_props.into_attrs()}>
///         "Modal content"
///     </div>
/// }
/// ```
pub fn use_modal(input: UseModalInput) -> UseModalReturn {
    let aria_modal = if input.is_disabled {
        None
    } else {
        Some(AriaModal::True)
    };

    UseModalReturn {
        modal_props: UseModalProps { aria_modal },
    }
}
