use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    anchor_link::AnchorLinkDemo, link_disabled::LinkDisabledDemo, link_external::LinkExternalDemo,
    link_internal::LinkInternalDemo, link_pressed::LinkPressedDemo,
    link_programmatic_focus::LinkProgrammaticFocusDemo, link_span::LinkSpanDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageUseLink() -> impl IntoView {
    view! {
        <DocPage title="Link Hooks">
            <p>
                <AnchorLink href="#use-link">"use_link"</AnchorLink>" makes an element behave and announce itself as a "
                "link, with press, hover and focus handling. "<AnchorLink href="#use-anchor-link">"use_anchor_link"</AnchorLink>
                " builds on it for links to an element on the current page. See the "
                <Link href=routes::doc::Link.materialize()>"Link overview"</Link>" for concept guidance."
            </p>

            <ReactAria hook="useLink"/>

            <Section title="use_link">
                <p>"Spread the returned props onto the link element: a native "<Code inline=true>"<a>"</Code>" or any other element."</p>

                <Section title="Input" id="use-link-input">
                    <p>"Start from "<Code inline=true>"UseLinkInput::default()"</Code>" and set what you need."</p>

                    <ApiTable kind=ApiKind::Input of="UseLinkInput">
                        <ApiRow name="href" ty="Signal<Option<String>>" default="None">
                            "The link target. Rendered on anchors only, and not while the link is disabled: a disabled link can\u{2019}t "
                            "be followed in any way."
                        </ApiRow>
                        <ApiRow name="target" ty="LinkTarget" default="Same">
                            "Where to open the link, e.g. "<Code inline=true>"LinkTarget::Blank"</Code>" for a new tab."
                        </ApiRow>
                        <ApiRow name="rel" ty="Vec<LinkRel>" default="empty">
                            "Values of the "<Code inline=true>"rel"</Code>" attribute. With "<Code inline=true>"LinkTarget::Blank"</Code>", "
                            <Code inline=true>"LinkRel::NoOpener"</Code>" is added automatically."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the link is disabled."</ApiRow>
                        <ApiRow name="element_type" ty="LinkElementType" default="Anchor">
                            "The element you spread the props onto, see "<AnchorLink href="#element-types">"Element Types"</AnchorLink>"."
                        </ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the link when its content doesn\u{2019}t."</ApiRow>
                        <ApiRow name="aria_current" ty="Signal<Option<AriaCurrent>>" default="None">
                            "Marks the link as the current item of a set, e.g. the current page in a navigation."
                        </ApiRow>
                        <ApiRow name="on_press" ty="Option<Callback<PressEvent>>" default="None">"Called when the link is pressed."</ApiRow>
                        <ApiRow name="on_press_start, on_press_end" ty="Option<Callback<PressEvent>>" default="None">
                            "Called when a press starts and ends, as in "
                            <Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link>"."
                        </ApiRow>
                        <ApiRow name="on_press_change" ty="Option<Callback<bool>>" default="None">"Called when the pressed state changes."</ApiRow>
                        <ApiRow name="on_hover_start, on_hover_end" ty="Option<Callback<HoverStartEvent>>, Option<Callback<HoverEndEvent>>" default="None">
                            "Called when a pointer starts or stops hovering the link."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-link-return">
                    <ApiTable kind=ApiKind::Return of="UseLinkReturn">
                        <ApiRow name="props" ty="PropsWithStyles<UseLinkProps>">
                            "Attributes, event handlers and styles for the link element. Call "
                            <Code inline=true>"props.into_parts()"</Code>" to get "<Code inline=true>"(attrs, styles)"</Code>
                            ", then spread "<Code inline=true>"{..attrs}"</Code>" and set "<Code inline=true>"style=styles"</Code>"."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>">"Whether the link is disabled."</ApiRow>
                        <ApiRow name="is_pressed" ty="Signal<bool>">"Whether the link is currently pressed."</ApiRow>
                        <ApiRow name="is_hovered" ty="Signal<bool>">"Whether a pointer hovers the link."</ApiRow>
                        <ApiRow name="is_focused" ty="Signal<bool>">"Whether the link has focus."</ApiRow>
                        <ApiRow name="is_focus_visible" ty="Signal<bool>">
                            "Whether a focus ring should be shown (keyboard focus only). Also exposed as "
                            <Code inline=true>"data-focus-visible"</Code>"."
                        </ApiRow>
                        <ApiRow name="focus_handle" ty="FocusHandle">
                            "Focus the link programmatically with "<Code inline=true>"focus_handle.focus()"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-link-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use leptonic::hooks::{LinkTarget, UseLinkInput, use_link};

                            let link = use_link(UseLinkInput {
                                href: Signal::stored(Some("https://leptos.dev".to_owned())),
                                target: LinkTarget::Blank,
                                ..UseLinkInput::default()
                            });
                            let (attrs, styles) = link.props.into_parts();

                            view! {
                                <a {..attrs} style=styles>"Leptos"</a>
                            }
                        "#)}
                    </Code>
                </Section>

                <Section title="Demo" id="use-link-demo">
                    <p>"A link to another page of the app:"</p>
                    <Demo description="Link to the Link overview" source=include_str!("demos/link_internal.rs")>
                        <LinkInternalDemo/>
                    </Demo>

                    <p>"A link opening another site in a new tab, with a "<Code inline=true>"rel"</Code>" value:"</p>
                    <Demo description="External link opening in a new tab" source=include_str!("demos/link_external.rs")>
                        <LinkExternalDemo/>
                    </Demo>

                    <p>
                        "A link you can disable. Disabled, it has no "<Code inline=true>"href"</Code>" and "
                        <Code inline=true>"aria-disabled=\"true\""</Code>", which the demo uses for styling:"
                    </p>
                    <Demo description="Link with a Disabled checkbox" source=include_str!("demos/link_disabled.rs")>
                        <LinkDisabledDemo/>
                    </Demo>
                </Section>

                <Section title="Pressed State">
                    <p>
                        <Code inline=true>"is_pressed"</Code>" is "<Code inline=true>"true"</Code>
                        " while the link is held down. The demo renders it as "<Code inline=true>"data-pressed"</Code>
                        " for its styles. Press and hold the link:"
                    </p>

                    <Demo description="Link that shrinks while pressed, with its pressed state" source=include_str!("demos/link_pressed.rs")>
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
                        <TableRow>
                            <TableCell><Code inline=true>"Other"</Code></TableCell>
                            <TableCell>
                                "Any other element (a "<Code inline=true>"<span>"</Code>", \u{2026}): gets "<Code inline=true>"role=\"link\""</Code>
                                " and, unless disabled, "<Code inline=true>"tabindex=\"0\""</Code>"."
                            </TableCell>
                        </TableRow>
                    </DocTable>

                    <Demo description="Span acting as a link, counting presses" source=include_str!("demos/link_span.rs")>
                        <LinkSpanDemo/>
                    </Demo>
                </Section>

                <Section title="Programmatic Focus">
                    <p>"Move focus to the link with the returned "<Code inline=true>"focus_handle"</Code>":"</p>

                    <Demo description="Button that focuses a link" source=include_str!("demos/link_programmatic_focus.rs")>
                        <LinkProgrammaticFocusDemo/>
                    </Demo>
                </Section>
            </Section>

            <Section title="use_anchor_link">
                <p>
                    <Code inline=true>"use_anchor_link"</Code>" makes a link that scrolls to an element on the current page "
                    "and puts the element\u{2019}s id into the URL. It is a "<AnchorLink href="#use-link">"use_link"</AnchorLink>" link with an "
                    "anchor as its "<Code inline=true>"href"</Code>"."
                </p>

                <Section title="Input" id="use-anchor-link-input">
                    <p>
                        "Create it with "<Code inline=true>"UseAnchorLinkInput::new(href)"</Code>" and change the rest with "
                        "struct update syntax."
                    </p>

                    <ApiTable kind=ApiKind::Input of="UseAnchorLinkInput">
                        <ApiRow name="href" ty="Href">
                            "Required. The element to link to, by id: "<Code inline=true>"Href::from(\"#my-section\")"</Code>" or "
                            <Code inline=true>"Href::new(\"my-section\")"</Code>" (the "<Code inline=true>"#"</Code>
                            " is optional). "<Code inline=true>"fragment()"</Code>" returns the id, "<Code inline=true>"as_str()"</Code>
                            " the href with its "<Code inline=true>"#"</Code>"."
                        </ApiRow>
                        <ApiRow name="scroll_behavior" ty="Option<ScrollBehavior>" default="Some(Smooth)">
                            <Code inline=true>"Smooth"</Code>" (instant while reduced motion is preferred) or "
                            <Code inline=true>"Instant"</Code>" scrolling to the target. "
                            <Code inline=true>"None"</Code>" doesn\u{2019}t scroll; only the URL changes."
                        </ApiRow>
                        <ApiRow name="link" ty="UseLinkInput" default="UseLinkInput::default()">
                            "The link\u{2019}s other settings, as for "<AnchorLink href="#use-link-input">"use_link"</AnchorLink>
                            " (e.g. "<Code inline=true>"is_disabled"</Code>", "<Code inline=true>"aria_label"</Code>" for a bare "
                            <Code inline=true>"#"</Code>"). Its "<Code inline=true>"href"</Code>" is replaced by the anchor\u{2019}s; its "
                            <Code inline=true>"on_press"</Code>" runs after scrolling."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-anchor-link-return">
                    <p>
                        "The hook returns "<AnchorLink href="#use-link-return">"use_link"</AnchorLink>"\u{2019}s "
                        <Code inline=true>"UseLinkReturn"</Code>"."
                    </p>
                </Section>

                <Section title="Example" id="use-anchor-link-example">
                    <Code language=Language::Rust>
                        {indoc!(r##"
                            use leptonic::hooks::{UseAnchorLinkInput, use_anchor_link};

                            let link = use_anchor_link(UseAnchorLinkInput::new("#returns"));
                            let (attrs, styles) = link.props.into_parts();

                            view! { <a {..attrs} style=styles>"Returns"</a> }
                        "##)}
                    </Code>
                </Section>

                <Section title="Demo" id="use-anchor-link-demo">
                    <p>"Press the "<Code inline=true>"#"</Code>" link to scroll to the paragraph it points to."</p>

                    <Demo description="Anchor link scrolling smoothly to a paragraph, with a Disabled checkbox" source=include_str!("demos/anchor_link.rs") source_open=true>
                        <AnchorLinkDemo/>
                    </Demo>
                </Section>

                <Section title="Behavior" id="use-anchor-link-behavior">
                    <p>"When the link is pressed and not disabled, the hook:"</p>

                    <ol>
                        <li>"prevents the browser\u{2019}s default navigation,"</li>
                        <li>"scrolls the element with the anchor\u{2019}s id into view, if "<Code inline=true>"scroll_behavior"</Code>" is set,"</li>
                        <li>"replaces the URL hash with "<Code inline=true>"history.replaceState"</Code>", so no history entry is added and the page doesn\u{2019}t reload,"</li>
                        <li>"calls your "<Code inline=true>"on_press"</Code>"."</li>
                    </ol>
                </Section>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Link.materialize()>"Link overview"</Link></li>
                <li><Link href=routes::doc::link::Atom.materialize()>"Link Atoms"</Link></li>
                <li><Link href=routes::doc::link::Component.materialize()>"Link Components"</Link></li>
                <li><Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
