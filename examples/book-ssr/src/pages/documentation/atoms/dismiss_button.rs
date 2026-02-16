use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageAtomDismissButton() -> impl IntoView {
    view! {
        <Article>
            <h1 id="dismiss-button" class="anchor">
                "DismissButton"
                <AnchorLink href="#dismiss-button" description="Direct link to article header"/>
            </h1>

            <p>
                "A visually hidden button that allows screen reader users to dismiss an overlay. "
                "Place at the start and/or end of overlay content (popovers, modals, trays) "
                "to provide an accessible dismiss mechanism for users who cannot press Escape, "
                "such as mobile VoiceOver users."
            </p>

            <h2 id="usage" class="anchor">
                "Usage"
                <AnchorLink href="#usage" description="Direct link to usage"/>
            </h2>

            <Code>
                {indoc!(r#"
                    use leptonic::atoms::dismiss_button::DismissButton;

                    view! {
                        <div class="my-overlay">
                            <DismissButton on_dismiss=move |_| close_overlay() />
                            // ... overlay content ...
                            <DismissButton on_dismiss=move |_| close_overlay() />
                        </div>
                    }
                "#)}
            </Code>

            <p>
                "The button is visually hidden but accessible to screen readers. "
                "It renders as a "<code>"<button>"</code>" with "<code>"tabindex=\"-1\""</code>
                " so it is only reachable via screen reader navigation, not regular Tab key navigation."
            </p>

            <h2 id="props" class="anchor">
                "Props"
                <AnchorLink href="#props" description="Direct link to props"/>
            </h2>

            <ul>
                <li><code>"on_dismiss: Option<Callback<()>>"</code>" - Callback invoked when the dismiss button is activated."</li>
                <li><code>"aria_label: Option<&'static str>"</code>" - Custom accessible label. Defaults to \"Dismiss\"."</li>
            </ul>

            <h2 id="react-aria" class="anchor">
                "React-aria Reference"
                <AnchorLink href="#react-aria" description="Direct link to react-aria reference"/>
            </h2>

            <p>
                "Based on react-aria's "
                <code>"DismissButton"</code>
                " component. See the "
                <LinkExt href="https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/overlays/src/DismissButton.tsx" target=leptonic::hooks::LinkTarget::_Blank>
                    "source"
                </LinkExt>
                " for reference."
            </p>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "DismissButton", link: "#dismiss-button" },
                Toc::Leaf { title: "Usage", link: "#usage" },
                Toc::Leaf { title: "Props", link: "#props" },
                Toc::Leaf { title: "React-aria Reference", link: "#react-aria" },
            ]
        }/>
    }
}
