mod use_anchor_link;
mod use_link;

pub use use_anchor_link::*;
pub use use_link::*;

use leptos::oco::Oco;
use std::fmt;

#[cfg(debug_assertions)]
pub(crate) fn debug_validate_element_type(element_type: LinkElementType, el: &web_sys::Element) {
    let actual = el.tag_name().to_uppercase();
    let expected = match element_type {
        LinkElementType::Anchor => "A",
        LinkElementType::Span => "SPAN",
        LinkElementType::Button => "BUTTON",
    };
    debug_assert_eq!(
        actual, expected,
        "element_type is {element_type:?} but actual element is <{actual}>. \
         Update element_type to match the element used in the view."
    );
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum LinkTarget {
    /// Opens the linked document in a new window or tab.
    _Blank,
    /// Opens the linked document in the same frame as it was clicked (this is the default).
    #[default]
    _Self,
    /// Opens the linked document in the parent frame.
    _Parent,
    /// Opens the linked document in the full body of the window.
    _Top,
    /// Opens the linked document in the frame with the given name.
    Frame { with_name: Oco<'static, str> },
}

impl LinkTarget {
    /// Returns the corresponding HTML `target` attribute string.
    pub(crate) fn to_oco(&self) -> Oco<'static, str> {
        match &self {
            LinkTarget::_Blank => Oco::Borrowed("_blank"),
            LinkTarget::_Self => Oco::Borrowed("_self"),
            LinkTarget::_Parent => Oco::Borrowed("_parent"),
            LinkTarget::_Top => Oco::Borrowed("_top"),
            LinkTarget::Frame { with_name } => with_name.clone(),
        }
    }
}

impl fmt::Display for LinkTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_oco().as_str())
    }
}

/// Valid `rel` attribute values for `<a>` elements.
///
/// See the [HTML spec](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/rel)
/// for details on each value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LinkRel {
    Alternate,
    Author,
    Bookmark,
    External,
    Help,
    License,
    Me,
    Next,
    NoFollow,
    NoOpener,
    NoReferrer,
    Opener,
    Prev,
    PrivacyPolicy,
    Search,
    Tag,
    TermsOfService,
}

impl fmt::Display for LinkRel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Alternate => f.write_str("alternate"),
            Self::Author => f.write_str("author"),
            Self::Bookmark => f.write_str("bookmark"),
            Self::External => f.write_str("external"),
            Self::Help => f.write_str("help"),
            Self::License => f.write_str("license"),
            Self::Me => f.write_str("me"),
            Self::Next => f.write_str("next"),
            Self::NoFollow => f.write_str("nofollow"),
            Self::NoOpener => f.write_str("noopener"),
            Self::NoReferrer => f.write_str("noreferrer"),
            Self::Opener => f.write_str("opener"),
            Self::Prev => f.write_str("prev"),
            Self::PrivacyPolicy => f.write_str("privacy-policy"),
            Self::Search => f.write_str("search"),
            Self::Tag => f.write_str("tag"),
            Self::TermsOfService => f.write_str("terms-of-service"),
        }
    }
}

/// Joins a slice of `LinkRel` values into a space-separated string
/// suitable for the HTML `rel` attribute. Returns `None` if the slice is empty.
#[must_use]
pub fn link_rel_to_string(rels: &[LinkRel]) -> Option<String> {
    if rels.is_empty() {
        None
    } else {
        Some(
            rels.iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" "),
        )
    }
}
