use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    anchor_link::AnchorLinkComponentDemo, link::LinkComponentDemo,
    link_button::LinkButtonComponentDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageLinkComponents() -> impl IntoView {
    view! {
        <DocPage title="Link Components">
            <p>
                "The themed "<Code inline=true>"Link"</Code>", "<Code inline=true>"AnchorLink"</Code>" and "
                <Code inline=true>"LinkButton"</Code>" components wrap the "
                <Link href=routes::doc::link::Atom.materialize()>"Link Atoms"</Link>" and add leptonic\u{2019}s theme. See the "
                <Link href=routes::doc::Link.materialize()>"Link overview"</Link>" for concept guidance."
            </p>

            <Demo description="A link to a page of the app and a link to another site" source=include_str!("demos/link.rs")>
                <LinkComponentDemo/>
            </Demo>

            <Section title="Link">
                <p>
                    "The "<Link href=format!("{}#link", routes::doc::link::Atom.materialize())>"Link"</Link>
                    " atom with the "<Code inline=true>"leptonic-link"</Code>" class, colored with "
                    <Code inline=true>"--link-color"</Code>". It renders leptos_router\u{2019}s "<Code inline=true>"<A>"</Code>
                    ", so it needs a surrounding "<Code inline=true>"<Router>"</Code>"."
                </p>

                <Section title="Props" id="link-props">
                    <ApiTable kind=ApiKind::Props of="components::link::Link">
                        <ApiRow name="href" ty="impl ToHref">"A route of your app or a URL. Required."</ApiRow>
                        <ApiRow name="target" ty="LinkTarget" default="Same">
                            "Where to open the link, e.g. "<Code inline=true>"LinkTarget::Blank"</Code>" for a new tab."
                        </ApiRow>
                        <ApiRow name="rel" ty="Vec<LinkRel>" default="empty">
                            "Values of the "<Code inline=true>"rel"</Code>" attribute. "<Code inline=true>"LinkRel::NoOpener"</Code>
                            " is added for "<Code inline=true>"LinkTarget::Blank"</Code>"."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the link can\u{2019}t be followed."</ApiRow>
                        <ApiRow name="current_match" ty="CurrentMatch" default="Prefix">
                            "When the link is the current page: "<Code inline=true>"Prefix"</Code>" on its route and below, "
                            <Code inline=true>"Exact"</Code>" only on its route."
                        </ApiRow>
                        <ApiRow name="replace" ty="bool" default="false">"Replace the current history entry instead of adding one."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the link when its content doesn\u{2019}t."</ApiRow>
                        <ApiRow name="on_press" ty="Option<Callback<PressEvent>>" default="None">"Called when the link is pressed."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Added to the link element."</ApiRow>
                        <ApiRow name="children" ty="ChildrenFn">"The link content. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="AnchorLink">
                <p>
                    "The "<Link href=format!("{}#anchorlink", routes::doc::link::Atom.materialize())>"AnchorLink"</Link>
                    " atom with the "<Code inline=true>"leptonic-anchor-link"</Code>" class. Without children it shows a "
                    <Code inline=true>"#"</Code>", the usual link next to a heading; name it with "
                    <Code inline=true>"aria_label"</Code>". In headings, the theme puts a space on either side."
                </p>

                <Demo description="A heading with a bare # link, and a text link to the same heading" source=include_str!("demos/anchor_link.rs")>
                    <AnchorLinkComponentDemo/>
                </Demo>

                <Section title="Props" id="anchor-link-props">
                    <ApiTable kind=ApiKind::Props of="components::link::AnchorLink">
                        <ApiRow name="href" ty="Href">
                            "The id of the element to link to: "<Code inline=true>"\"#returns\""</Code>" or "
                            <Code inline=true>"\"returns\""</Code>". Required."
                        </ApiRow>
                        <ApiRow name="scroll_behavior" ty="Option<ScrollBehavior>" default="Some(Smooth)">
                            "How to scroll to the element. "<Code inline=true>"None"</Code>" doesn\u{2019}t scroll; only the URL "
                            "fragment changes."
                        </ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                            "Names the link when its content doesn\u{2019}t, e.g. the bare "<Code inline=true>"#"</Code>"."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Added to the link element."</ApiRow>
                        <ApiRow name="children" ty="Option<Children>" default="#">"The link content."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="LinkButton">
                <p>
                    "A link that looks like a "<Link href=routes::doc::button::Component.materialize()>"Button Component"</Link>
                    ", with its variant, color and size. Use it instead of a button inside a link, which HTML doesn\u{2019}t allow. "
                    "It is the themed "<Link href=format!("{}#linkbutton", routes::doc::link::Atom.materialize())>"LinkButton"</Link>
                    " atom."
                </p>

                <Demo description="Two link buttons with a Disabled checkbox" source=include_str!("demos/link_button.rs")>
                    <LinkButtonComponentDemo/>
                </Demo>

                <Section title="Props" id="link-button-props">
                    <ApiTable kind=ApiKind::Props of="components::button::LinkButton">
                        <ApiRow name="href" ty="impl ToHref">"A route of your app or a URL. Required."</ApiRow>
                        <ApiRow name="target" ty="LinkTarget" default="Same">"Where to open the link."</ApiRow>
                        <ApiRow name="variant" ty="Signal<ButtonVariant>" default="Filled">
                            <Code inline=true>"Filled"</Code>", "<Code inline=true>"Outlined"</Code>" or "<Code inline=true>"Flat"</Code>"."
                        </ApiRow>
                        <ApiRow name="color" ty="Signal<ButtonColor>" default="Primary">"The button color."</ApiRow>
                        <ApiRow name="size" ty="Signal<ButtonSize>" default="Normal">"The button size."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the link can\u{2019}t be followed."</ApiRow>
                        <ApiRow name="current_match" ty="CurrentMatch" default="Prefix">
                            "When the link is the current page ("<Code inline=true>"aria-current=\"page\""</Code>")."
                        </ApiRow>
                        <ApiRow name="aria_haspopup, aria_expanded" ty="Option<Signal<Option<AriaHasPopup>>>, Option<Signal<Option<AriaExpanded>>>" default="None">
                            "Popup attributes, for a link that opens something."
                        </ApiRow>
                        <ApiRow name="on_hover_start, on_hover_end" ty="Option<Callback<HoverStartEvent>>, Option<Callback<HoverEndEvent>>" default="None">
                            "Called when a pointer starts or stops hovering the link."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Added to the link element."</ApiRow>
                        <ApiRow name="children" ty="ChildrenFn">"The content. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Styling">
                <p>
                    "The themed "<Code inline=true>"Link"</Code>" and "<Code inline=true>"AnchorLink"</Code>" read this CSS "
                    "variable; "<Code inline=true>"LinkButton"</Code>" reads the "
                    <Link href=format!("{}#styling", routes::doc::button::Component.materialize())>"button variables"</Link>
                    ". The theme gives the links no hover, pressed or disabled look; target the atoms\u{2019} data attributes ("
                    <Link href=format!("{}#data-attributes", routes::doc::link::Atom.materialize())>"Data Attributes"</Link>
                    ") for those."
                </p>
                <CssVariables prefix="--link-" scss=theme_scss!("link")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Link.materialize()>"Link overview"</Link></li>
                <li><Link href=routes::doc::link::Hook.materialize()>"Link Hooks"</Link></li>
                <li><Link href=routes::doc::link::Atom.materialize()>"Link Atoms"</Link></li>
                <li><Link href=routes::doc::button::Component.materialize()>"Button Component"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
