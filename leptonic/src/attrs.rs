//! Spreading hook props onto elements.

use std::fmt::Debug;

use crate::utils::styles::Styles;

/// Trait for converting hook `*Props` types into spreadable attribute tuples.
///
/// All `*Props` types returned by hooks implement this trait. Call `.into_attrs()`
/// to convert props into an attribute tuple that can be spread onto elements
/// using Leptos's spreading syntax (`<div {..props.into_attrs()}/>`).
pub trait IntoAttrs: Debug {
    /// The concrete attributes tuple type produced by this conversion.
    type Attrs;

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    fn into_attrs(self) -> Self::Attrs;
}

/// Wrapper for hook props that include styles.
///
/// Does **not** implement [`IntoAttrs`] or any Leptos `Attribute` trait,
/// so it cannot be spread directly. Callers must call [`.into_parts()`](Self::into_parts)
/// to obtain both the spreadable attributes and the [`Styles`] that must be
/// merged with any user-provided styles before being applied via `style=`.
#[derive(Debug)]
pub struct PropsWithStyles<P: IntoAttrs> {
    pub(crate) props: P,
    pub(crate) styles: Styles,
}

impl<P: IntoAttrs> PropsWithStyles<P> {
    pub fn new(props: P, styles: Styles) -> Self {
        Self { props, styles }
    }

    /// Consume self, converting the inner props to spreadable attributes
    /// and returning them alongside the hook's styles. For spreading onto an element; use
    /// [`into_inner`](Self::into_inner) to change the props further first.
    pub fn into_parts(self) -> (P::Attrs, Styles) {
        (self.props.into_attrs(), self.styles)
    }

    /// Extract the inner props and styles.
    pub fn into_inner(self) -> (P, Styles) {
        (self.props, self.styles)
    }
}
