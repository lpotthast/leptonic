use leptonic::{components::prelude::*, hooks::LinkTarget};
use leptos::prelude::*;

use super::Section;

/// "Based on react-aria's `useX`." with a link to the react-aria documentation of `hook`.
#[component]
pub fn ReactAria(
    /// Name of the react-aria hook, e.g. `"useButton"`.
    hook: &'static str,
) -> impl IntoView {
    view! {
        <p>
            "Based on react-aria\u{2019}s "
            <LinkExt href=format!("https://react-spectrum.adobe.com/react-aria/{hook}.html") target=LinkTarget::_Blank>
                {hook}
            </LinkExt>
            "."
        </p>
    }
}

/// "Based on react-aria's `X`." with a link to the react-aria source file at `path`, for parts of react-aria without a
/// documentation page (e.g. `PressResponder`). Use [`ReactAria`] whenever a documentation page exists.
#[component]
pub fn ReactAriaSource(
    /// Path below `packages/react-aria/src/`, e.g. `"interactions/PressResponder.tsx"`.
    path: &'static str,
) -> impl IntoView {
    let file = path.rsplit('/').next().unwrap_or(path);
    let name = file.split('.').next().unwrap_or(file);
    view! {
        <p>
            "Based on react-aria\u{2019}s "
            <LinkExt
                href=format!("https://github.com/adobe/react-spectrum/blob/main/packages/react-aria/src/{path}")
                target=LinkTarget::_Blank
            >
                {name}
            </LinkExt>
            "."
        </p>
    }
}

/// The closing "See Also" section of a page. Children are `<li>` links.
#[component]
pub fn SeeAlso(children: Children) -> impl IntoView {
    view! {
        <Section title="See Also">
            <ul>{children()}</ul>
        </Section>
    }
}
