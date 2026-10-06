//! Heading levels (`<h1>` to `<h6>`).

use leptos::{either::EitherOf6, html, prelude::*};

/// The level of a heading element.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum HeadingLevel {
    H1,
    #[default]
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl HeadingLevel {
    /// A heading element of this level with `children`; spread attributes onto it with
    /// `add_any_attr`.
    pub fn render(self, children: Children) -> impl IntoView + AddAnyAttr {
        match self {
            Self::H1 => EitherOf6::A(html::h1().child(children())),
            Self::H2 => EitherOf6::B(html::h2().child(children())),
            Self::H3 => EitherOf6::C(html::h3().child(children())),
            Self::H4 => EitherOf6::D(html::h4().child(children())),
            Self::H5 => EitherOf6::E(html::h5().child(children())),
            Self::H6 => EitherOf6::F(html::h6().child(children())),
        }
    }
}
