use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::anchor_link::AnchorLinkDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseAnchorLink() -> impl IntoView {
    view! {
        <DocPage title="use_anchor_link">
            <p>
                "The "<Code inline=true>"use_anchor_link"</Code>" hook makes a link that scrolls to an element on the "
                "current page and puts its id into the URL. It has the press handling, focus management and focus ring "
                "tracking of "<Link href=routes::doc::link::UseLink.materialize()>"use_link"</Link>". See the "
                <Link href=routes::doc::Link.materialize()>"Link overview"</Link>" for concept guidance."
            </p>

            <Section title="Input">
                <p>
                    <Code inline=true>"UseAnchorLinkInput"</Code>" has no "<Code inline=true>"Default"</Code>
                    ", so you set every field."
                </p>

                <ApiTable kind=ApiKind::Input of="UseAnchorLinkInput">
                    <ApiRow name="href" ty="Href">
                        "The element to link to, by id: "<Code inline=true>"Href::from(\"#my-section\")"</Code>" or "
                        <Code inline=true>"Href::new(\"my-section\")"</Code>" (the "<Code inline=true>"#"</Code>
                        " is optional). "<Code inline=true>"fragment()"</Code>" returns the id, "<Code inline=true>"as_str()"</Code>
                        " the href with its "<Code inline=true>"#"</Code>"."
                    </ApiRow>
                    <ApiRow name="scroll_behavior" ty="Option<ScrollBehavior>">
                        <Code inline=true>"Smooth"</Code>" or "<Code inline=true>"Instant"</Code>" scrolling to the target. "
                        <Code inline=true>"None"</Code>" doesn\u{2019}t scroll; only the URL changes."
                    </ApiRow>
                    <ApiRow name="is_disabled" ty="Signal<bool>">"Whether the link is disabled."</ApiRow>
                    <ApiRow name="element_type" ty="LinkElementType">
                        "The element you spread the props onto. Elements other than "<Code inline=true>"<a>"</Code>
                        " get "<Code inline=true>"role=\"link\""</Code>"."
                    </ApiRow>
                    <ApiRow name="description" ty="Option<Oco<'static, str>>">
                        "Accessible name ("<Code inline=true>"aria-label"</Code>"). Set it when the link has no descriptive "
                        "text, such as a bare "<Code inline=true>"#"</Code>"."
                    </ApiRow>
                    <ApiRow name="on_press" ty="Option<Callback<PressEvent>>">"Called after scrolling and updating the URL."</ApiRow>
                    <ApiRow name="on_press_start, on_press_end" ty="Option<Callback<PressEvent>>">
                        "Called when a press starts and ends."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseAnchorLinkReturn">
                    <ApiRow name="props" ty="PropsWithStyles<UseAnchorLinkProps>">
                        "Attributes, event handlers and styles for the link element. Call "
                        <Code inline=true>"props.into_parts()"</Code>" to get "<Code inline=true>"(attrs, styles)"</Code>
                        ", then spread "<Code inline=true>"{..attrs}"</Code>" and set "<Code inline=true>"style=styles"</Code>"."
                    </ApiRow>
                    <ApiRow name="is_pressed" ty="Signal<bool>">"Whether the link is currently pressed."</ApiRow>
                    <ApiRow name="is_focus_visible" ty="Signal<bool>">"Whether a focus ring should be shown (keyboard focus only)."</ApiRow>
                    <ApiRow name="focus_handle" ty="FocusHandle">"Focus the link programmatically."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Demo">
                <p>"Press the "<Code inline=true>"#"</Code>" link to scroll to the target below it."</p>

                <Demo description="Anchor link scrolling smoothly to a target, with a disabled toggle" source=include_str!("demos/anchor_link.rs") source_open=true>
                    <AnchorLinkDemo/>
                </Demo>
            </Section>

            <Section title="Behavior">
                <p>"When the link is pressed and not disabled, the hook:"</p>

                <ol>
                    <li>"prevents the browser\u{2019}s default navigation,"</li>
                    <li>"scrolls the element with the anchor\u{2019}s id into view, if "<Code inline=true>"scroll_behavior"</Code>" is set,"</li>
                    <li>"replaces the URL hash with "<Code inline=true>"history.replaceState"</Code>", so no history entry is added and the page doesn\u{2019}t reload,"</li>
                    <li>"calls your "<Code inline=true>"on_press"</Code>"."</li>
                </ol>
            </Section>

            <Section title="Accessibility">
                <p>"The hook sets:"</p>

                <ul>
                    <li><Code inline=true>"href"</Code>" pointing to the anchor."</li>
                    <li><Code inline=true>"role=\"link\""</Code>" on elements other than "<Code inline=true>"<a>"</Code>"."</li>
                    <li><Code inline=true>"aria-label"</Code>" from "<Code inline=true>"description"</Code>"."</li>
                    <li><Code inline=true>"aria-disabled"</Code>" while disabled, and a "<Code inline=true>"tabindex"</Code>" managed by "<Link href=routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link>"."</li>
                    <li><Code inline=true>"data-focus-visible"</Code>" while the link has keyboard focus."</li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="Tab">"Focuses the link."</KeyRow>
                    <KeyRow keys="Enter">"Activates the link."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Link.materialize()>"Link overview"</Link></li>
                <li><Link href=routes::doc::link::UseLink.materialize()>"use_link"</Link></li>
                <li><Link href=routes::doc::link::AnchorLinkAtom.materialize()>"AnchorLink atom"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
