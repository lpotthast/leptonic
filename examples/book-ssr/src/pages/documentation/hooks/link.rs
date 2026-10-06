use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    link_disabled::LinkDisabledDemo, link_external::LinkExternalDemo,
    link_internal::LinkInternalDemo, link_pressed::LinkPressedDemo,
    link_programmatic_focus::LinkProgrammaticFocusDemo, link_span::LinkSpanDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageUseLink() -> impl IntoView {
    view! {
        <DocPage title="use_link">
            <p>
                "The "<Code inline=true>"use_link"</Code>" hook makes an element behave and announce itself as a link, "
                "with the press handling, focus management and focus ring tracking of "
                <Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>". See the "
                <Link href=routes::doc::Link.materialize()>"Link overview"</Link>" for concept guidance."
            </p>

            <ReactAria hook="useLink"/>

            <Section title="Input">
                <p><Code inline=true>"UseLinkInput"</Code>" has no "<Code inline=true>"Default"</Code>", so you set every field."</p>

                <ApiTable kind=ApiKind::Input of="UseLinkInput">
                    <ApiRow name="href" ty="Option<String>">"The link target."</ApiRow>
                    <ApiRow name="target" ty="Option<LinkTarget>">
                        "Where to open the link, e.g. "<Code inline=true>"LinkTarget::_Blank"</Code>" for a new tab."
                    </ApiRow>
                    <ApiRow name="rel" ty="Vec<LinkRel>">
                        "Values of the "<Code inline=true>"rel"</Code>" attribute, such as "<Code inline=true>"LinkRel::NoOpener"</Code>"."
                    </ApiRow>
                    <ApiRow name="is_disabled" ty="Signal<bool>">"Whether the link is disabled."</ApiRow>
                    <ApiRow name="element_type" ty="LinkElementType">
                        "The element you spread the props onto, see "<a href="#element-types">"Element Types"</a>"."
                    </ApiRow>
                    <ApiRow name="aria_current" ty="Option<AriaCurrent>">
                        "Marks the link as the current item of a set, e.g. the current page in a navigation."
                    </ApiRow>
                    <ApiRow name="on_press" ty="Option<Callback<PressEvent>>">"Called when the link is pressed."</ApiRow>
                    <ApiRow name="on_press_start, on_press_end" ty="Option<Callback<PressEvent>>">
                        "Called when a press starts and ends, as in "
                        <Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link>"."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseLinkReturn">
                    <ApiRow name="props" ty="PropsWithStyles<UseLinkProps>">
                        "Attributes, event handlers and styles for the link element. Call "
                        <Code inline=true>"props.into_parts()"</Code>" to get "<Code inline=true>"(attrs, styles)"</Code>
                        ", then spread "<Code inline=true>"{..attrs}"</Code>" and set "<Code inline=true>"style=styles"</Code>"."
                    </ApiRow>
                    <ApiRow name="is_disabled" ty="Signal<bool>">"Whether the link is disabled."</ApiRow>
                    <ApiRow name="is_pressed" ty="Signal<bool>">"Whether the link is currently pressed."</ApiRow>
                    <ApiRow name="is_focus_visible" ty="Signal<bool>">
                        "Whether a focus ring should be shown (keyboard focus only). Also exposed as "
                        <Code inline=true>"data-focus-visible"</Code>"."
                    </ApiRow>
                    <ApiRow name="focus_handle" ty="FocusHandle">
                        "Focus the link programmatically with "<Code inline=true>"focus_handle.focus()"</Code>"."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        let link = use_link(UseLinkInput {
                            href: Some("https://example.com".to_string()),
                            target: Some(LinkTarget::_Blank),
                            rel: vec![LinkRel::NoOpener, LinkRel::NoReferrer],
                            is_disabled: Signal::default(),
                            element_type: LinkElementType::Anchor,
                            aria_current: None,
                            on_press: None,
                            on_press_start: None,
                            on_press_end: None,
                        });
                        let (attrs, styles) = link.props.into_parts();

                        view! {
                            <a {..attrs} style=styles>"External link"</a>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>"A link to a section of this page:"</p>
                <Demo description="Internal link to an anchor on the page" source=include_str!("demos/link_internal.rs")>
                    <LinkInternalDemo/>
                </Demo>

                <p>"A link opening another site in a new tab, with "<Code inline=true>"rel"</Code>" values:"</p>
                <Demo description="External link opening in a new tab" source=include_str!("demos/link_external.rs")>
                    <LinkExternalDemo/>
                </Demo>

                <p>"A link you can disable. The hook sets "<Code inline=true>"aria-disabled"</Code>", which the demo uses for styling:"</p>
                <Demo description="Link with a disabled toggle" source=include_str!("demos/link_disabled.rs")>
                    <LinkDisabledDemo/>
                </Demo>
            </Section>

            <Section title="Pressed State">
                <p>
                    <Code inline=true>"is_pressed"</Code>" is "<Code inline=true>"true"</Code>
                    " while the link is held down, for visual feedback. Press and hold the link:"
                </p>

                <Demo description="Link that shrinks while pressed" source=include_str!("demos/link_pressed.rs")>
                    <LinkPressedDemo/>
                </Demo>
            </Section>

            <Section title="Element Types">
                <p>
                    "Links don\u{2019}t have to be "<Code inline=true>"<a>"</Code>" elements. For any other element, the hook "
                    "adds "<Code inline=true>"role=\"link\""</Code>" and the keyboard handling of a link. In debug builds, it "
                    "checks that the element matches "<Code inline=true>"element_type"</Code>"."
                </p>

                <DocTable headers=&["LinkElementType", "Element"]>
                    <TableRow><TableCell><Code inline=true>"Anchor"</Code>" (default)"</TableCell><TableCell>"A native "<Code inline=true>"<a>"</Code>"."</TableCell></TableRow>
                    <TableRow><TableCell><Code inline=true>"Span"</Code></TableCell><TableCell>"A "<Code inline=true>"<span>"</Code>", gets "<Code inline=true>"role=\"link\""</Code>"."</TableCell></TableRow>
                    <TableRow><TableCell><Code inline=true>"Button"</Code></TableCell><TableCell>"A "<Code inline=true>"<button>"</Code>", gets "<Code inline=true>"role=\"link\""</Code>"."</TableCell></TableRow>
                </DocTable>

                <Demo description="Span acting as a link, counting presses" source=include_str!("demos/link_span.rs")>
                    <LinkSpanDemo/>
                </Demo>
            </Section>

            <Section title="Programmatic Focus">
                <p>"Move focus to the link with the returned "<Code inline=true>"FocusHandle"</Code>":"</p>

                <Demo description="Button that focuses a link" source=include_str!("demos/link_programmatic_focus.rs")>
                    <LinkProgrammaticFocusDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>"The hook sets:"</p>

                <ul>
                    <li><Code inline=true>"href"</Code>", "<Code inline=true>"target"</Code>" and "<Code inline=true>"rel"</Code>" from the input."</li>
                    <li><Code inline=true>"role=\"link\""</Code>" on elements other than "<Code inline=true>"<a>"</Code>"."</li>
                    <li><Code inline=true>"aria-current"</Code>" from the input, and "<Code inline=true>"aria-disabled"</Code>" while disabled."</li>
                    <li>
                        <Code inline=true>"tabindex"</Code>": "<Code inline=true>"0"</Code>", or none while disabled."
                    </li>
                    <li><Code inline=true>"data-focus-visible"</Code>" while the link has keyboard focus."</li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="Tab">"Focuses the link."</KeyRow>
                    <KeyRow keys="Enter">"Activates the link. Unlike buttons, links don\u{2019}t react to Space."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Link.materialize()>"Link overview"</Link></li>
                <li><Link href=routes::doc::link::UseAnchorLink.materialize()>"use_anchor_link"</Link></li>
                <li><Link href=routes::doc::link::LinkAtom.materialize()>"Link atom"</Link></li>
                <li><Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
