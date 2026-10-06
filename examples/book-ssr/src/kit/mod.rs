//! Building blocks for documentation pages.
//!
//! Pages are written with these components instead of raw headings and tables, so that the table of contents, anchor
//! links, table layout and the Markdown export stay consistent. See "Writing pages" in
//! `documentation/documentation-strategy.md`.

#[cfg(test)]
mod api_check;
mod demo;
mod demo_styles;
mod disclosure;
mod members;
mod page;
mod reference;
mod section;
mod table;
mod theme;

pub use demo::Demo;
pub use disclosure::Disclosure;
/// The rows of a [`DocTable`].
pub use leptonic::components::table::{TableCell, TableRow};
pub use members::SectionMembers;
pub use page::{DocPage, DocPageHeader};
pub use reference::{ReactAria, ReactAriaSource, SeeAlso, UpstreamPackage};
pub use section::Section;
pub use table::{ApiKind, ApiRow, ApiTable, DocTable, KeyRow, KeyboardTable, Keys};
pub use theme::CssVariables;

pub use crate::theme_scss;
