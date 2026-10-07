//! Building blocks for documentation pages.
//!
//! Pages are written with these components instead of raw headings and tables, so that the table of contents, anchor
//! links, table layout and the Markdown export stay consistent. See "Writing pages" in
//! `documentation/documentation-strategy.md`.

#[cfg(test)]
mod api_check;
mod code;
mod demo;
mod demo_styles;
mod disclosure;
mod icon;
mod link;
mod members;
mod page;
mod reference;
mod section;
mod table;

pub use code::{Code, Language};
pub use demo::Demo;
pub use disclosure::Disclosure;
pub use icon::Icon;
/// Where a [`Link`] opens, and its relationship to the page.
pub use leptonic::hooks::{LinkRel, LinkTarget};
pub use link::{AnchorLink, Link};
pub use members::SectionMembers;
pub use page::{DocPage, DocPageHeader};
pub use reference::{ReactAria, ReactAriaSource, SeeAlso, UpstreamPackage};
pub use section::Section;
pub use table::{
    ApiKind, ApiRow, ApiTable, DocTable, KeyRow, KeyboardTable, Keys, TableCell, TableRow,
};

