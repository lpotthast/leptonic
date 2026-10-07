use indoc::indoc;
use leptos::prelude::*;

use super::demos::toast::ToastAtomDemo;
use crate::{kit::*, routes};

/// A link to a section of the Toast Hooks page.
fn hooks_section(anchor: &str) -> String {
    format!("{}#{anchor}", routes::doc::toast::Hook.materialize())
}

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomToast() -> impl IntoView {
    view! {
        <DocPage title="Toast Atoms">
            <p>
                "The unstyled "<Code inline=true>"ToastRegion"</Code>" shows the toasts of a queue at the end of the page; "
                "you render each toast with "<Code inline=true>"Toast"</Code>" and its parts "<Code inline=true>"ToastContent"</Code>
                ", "<Code inline=true>"ToastTitle"</Code>", "<Code inline=true>"ToastDescription"</Code>" and "
                <Code inline=true>"ToastCloseButton"</Code>". See the "<Link href=routes::doc::Toast.materialize()>"Toast overview"</Link>
                " for concept guidance and keyboard interaction."
            </p>

            <ReactAria hook="Toast"/>

            <Section title="Hooks Used">
                <DocTable headers=&["Atom", "Built on"]>
                    <TableRow>
                        <TableCell><Code inline=true>"ToastRegion"</Code></TableCell>
                        <TableCell>
                            <Link href=hooks_section("use-toast-region")>"use_toast_region"</Link>" and "
                            <Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link>
                            ". It reads the toasts from a "<Link href=hooks_section("use-toast-state")>"ToastQueue"</Link>"."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Toast"</Code></TableCell>
                        <TableCell>
                            <Link href=hooks_section("use-toast")>"use_toast"</Link>" and "<Code inline=true>"use_focus_ring"</Code>
                            ". It hands the props of its parts to them through a context."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"ToastCloseButton"</Code></TableCell>
                        <TableCell><Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>" with the toast\u{2019}s close button input."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Example">
                <p>
                    "The atoms are in "<Code inline=true>"leptonic::atoms::toast"</Code>", the queue in "
                    <Code inline=true>"leptonic::hooks"</Code>". The region calls its children for each visible toast:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{atoms::toast::*, hooks::{ToastOptions, ToastQueue}};
                        use leptos::prelude::*;

                        // Usually created at the root of the app and provided as a context.
                        let queue = ToastQueue::<String>::new(None);
                        queue.add("Your changes were saved.".to_owned(), ToastOptions::default());

                        view! {
                            <ToastRegion queue=queue let:toast>
                                <Toast toast=toast.clone()>
                                    <ToastContent>
                                        <ToastTitle>"Saved"</ToastTitle>
                                        <ToastDescription>{toast.content.clone()}</ToastDescription>
                                    </ToastContent>
                                    <ToastCloseButton>"\u{00d7}"</ToastCloseButton>
                                </Toast>
                            </ToastRegion>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "Each upload adds a toast that closes after 8 seconds, unless the pointer is over the toasts or the focus "
                    "is in them. Up to three show at once, at the bottom of the window; press "<Keys keys="F6"/>
                    " to move the focus into them."
                </p>
                <Demo description="Upload toasts of the atoms in a region at the bottom of the window" source=include_str!("demos/toast.rs")>
                    <ToastAtomDemo/>
                </Demo>
            </Section>

            <Section title="ToastRegion">
                <p>
                    "The landmark showing the toasts, a "<Code inline=true>"<div>"</Code>" rendered at the end of the "
                    <Code inline=true>"<body>"</Code>" while the queue has visible toasts. It holds an "<Code inline=true>"<ol>"</Code>
                    " with an "<Code inline=true>"<li>"</Code>" per toast (both "<Code inline=true>"display: contents"</Code>
                    "), the newest first. Position it with your classes, e.g. fixed at the bottom of the window."
                </p>

                <Section title="Props" id="toast-region-props">
                    <ApiTable kind=ApiKind::Props of="ToastRegion">
                        <ApiRow name="queue" ty="ToastQueue<T>">"The queue whose visible toasts are shown. Required."</ApiRow>
                        <ApiRow name="children" ty="Fn(QueuedToast<T>) -> impl IntoView">
                            "Renders a toast, usually a "<Code inline=true>"Toast"</Code>". With "<Code inline=true>"let:toast"</Code>
                            ", the toast is bound for the children. Required."
                        </ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                            "Names the region. Default: \u{201c}1 notification.\u{201d}, \u{201c}2 notifications.\u{201d}, \u{2026}"
                        </ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"Ids of elements naming the region."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the region."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Toast">
                <p>
                    "A toast of the region around it, a "<Code inline=true>"<div>"</Code>" with "
                    <Code inline=true>"role=\"alertdialog\""</Code>", named by its "<Code inline=true>"ToastTitle"</Code>
                    " and described by its "<Code inline=true>"ToastDescription"</Code>". Its timeout runs while it is shown. "
                    "Outside a "<Code inline=true>"ToastRegion"</Code>" of its content type, it renders nothing and warns in "
                    "debug builds."
                </p>

                <Section title="Props" id="toast-props">
                    <ApiTable kind=ApiKind::Props of="atoms::toast::Toast">
                        <ApiRow name="toast" ty="QueuedToast<T>">"The toast the region passes to its children. Required."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the toast instead of its title."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the toast."</ApiRow>
                        <ApiRow name="children" ty="Children">"A "<Code inline=true>"ToastContent"</Code>" and a "<Code inline=true>"ToastCloseButton"</Code>". Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ToastContent">
                <p>
                    "A "<Code inline=true>"<div>"</Code>" with "<Code inline=true>"role=\"alert\""</Code>" around the title "
                    "and the description: screen readers announce it when the toast appears. Keep the close button outside "
                    "it, so it isn\u{2019}t announced with the message."
                </p>

                <Section title="Props" id="toast-content-props">
                    <ApiTable kind=ApiKind::Props of="ToastContent">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the content."</ApiRow>
                        <ApiRow name="children" ty="Children">"The title and the description. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ToastTitle">
                <p>"A "<Code inline=true>"<div>"</Code>" naming the toast."</p>

                <Section title="Props" id="toast-title-props">
                    <ApiTable kind=ApiKind::Props of="ToastTitle">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the title."</ApiRow>
                        <ApiRow name="children" ty="Children">"The title. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ToastDescription">
                <p>"A "<Code inline=true>"<div>"</Code>" describing the toast. It is optional; one per toast."</p>

                <Section title="Props" id="toast-description-props">
                    <ApiTable kind=ApiKind::Props of="ToastDescription">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the description."</ApiRow>
                        <ApiRow name="children" ty="Children">"The description. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ToastCloseButton">
                <p>
                    "The "<Code inline=true>"<button>"</Code>" closing the toast, named \u{201c}Close\u{201d}. Its content "
                    "is usually an icon."
                </p>

                <Section title="Props" id="toast-close-button-props">
                    <ApiTable kind=ApiKind::Props of="ToastCloseButton">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the button."</ApiRow>
                        <ApiRow name="children" ty="Children">"The button\u{2019}s content. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-focused, data-focus-visible" ty="true">
                        "The element has the focus (by keyboard). On "<Code inline=true>"ToastRegion"</Code>", "
                        <Code inline=true>"Toast"</Code>" and "<Code inline=true>"ToastCloseButton"</Code>"."
                    </ApiRow>
                    <ApiRow name="data-pressed, data-hovered, data-disabled" ty="true">"On "<Code inline=true>"ToastCloseButton"</Code>"."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms bring no styles. They render the classes "<Code inline=true>"leptonic-ToastRegion"</Code>", "
                    <Code inline=true>"leptonic-Toast"</Code>", "<Code inline=true>"leptonic-ToastContent"</Code>", "
                    <Code inline=true>"leptonic-ToastTitle"</Code>", "<Code inline=true>"leptonic-ToastDescription"</Code>
                    " and "<Code inline=true>"leptonic-ToastCloseButton"</Code>", each followed by the "
                    <Code inline=true>"classes"</Code>" you pass. The close button\u{2019}s content (an icon or a glyph) "
                    "is yours; the button is named \u{201c}Close\u{201d}. Place the region and stack the toasts: the newest "
                    "comes first in the DOM, so "<Code inline=true>"column-reverse"</Code>" shows it nearest to the bottom "
                    "edge. The region and the toasts receive focus; show it only through "
                    <Code inline=true>"data-focus-visible"</Code>". The book\u{2019}s demos use these rules:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .my-toast-region { position: fixed; right: 1em; bottom: 1em; display: flex; flex-direction: column-reverse; gap: 0.5em; }
                        .my-toast { display: flex; gap: 1em; width: 20em; max-width: 100%; padding: 1em; border: 1px solid var(--border); border-radius: 8px; background: var(--surface); }
                        .my-toast-title { font-weight: 600; }
                        .my-toast-close { padding: 0.25em; border: none; border-radius: 4px; background: none; }
                        .my-toast-close[data-hovered] { background: var(--border); }
                        .my-toast-region:focus, .my-toast:focus, .my-toast-close:focus { outline: none; }
                        .my-toast-region[data-focus-visible], .my-toast[data-focus-visible], .my-toast-close[data-focus-visible] { outline: 2px solid var(--focus); }
                    ")}
                </Code>
                <p>
                    "Leptonic also ships an optional atom theme that styles the default classes, for apps that don\u{2019}t "
                    "want to start from scratch: "<Code inline=true>"@use \"leptonic/leptonic-atoms\";"</Code>"."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Toast.materialize()>"Toast overview"</Link></li>
                <li><Link href=routes::doc::toast::Hook.materialize()>"Toast Hooks"</Link></li>
                <li><Link href=routes::doc::focus::UseLandmark.materialize()>"use_landmark"</Link></li>
                <li><Link href=routes::doc::button::Atom.materialize()>"Button Atom"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
