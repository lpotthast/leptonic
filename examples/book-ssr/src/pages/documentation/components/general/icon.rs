use leptonic::{components::prelude::*, hooks::LinkTarget};
use leptos::prelude::*;

use super::demos::icon::IconDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageIcon() -> impl IntoView {
    view! {
        <DocPage title="Icon Component">
            <p>
                "Icons label actions and states with a small picture, e.g. in buttons, alerts and menus. The "
                <Code inline=true>"Icon"</Code>" component renders an SVG icon from the "
                <Link target=LinkTarget::Blank href="https://crates.io/crates/icondata">"icondata"</Link>
                " crate. Its readme lists the available icon packages. You can browse all icons at "
                <Link target=LinkTarget::Blank href="https://carlosted.github.io/icondata/">
                    "carlosted.github.io/icondata"
                </Link>"."
            </p>

            <p>
                "leptonic re-exports "<Code inline=true>"icondata"</Code>" in "<Code inline=true>"leptonic::prelude"</Code>
                ". The SVG data of every icon you use is embedded into your binary."
            </p>

            <Demo description="A decorative folder icon next to text and a labelled cloud icon" source=include_str!("demos/icon.rs")>
                <IconDemo/>
            </Demo>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="Icon">
                    <ApiRow name="icon" ty="Signal<icondata::Icon>">"The icon to render. Required."</ApiRow>
                    <ApiRow name="width, height" ty="MaybeProp<TextProp>" default="\"1em\"">
                        "The "<Code inline=true>"width"</Code>" and "<Code inline=true>"height"</Code>" attributes of the "
                        <Code inline=true>"<svg>"</Code>". The default theme stretches the "<Code inline=true>"<svg>"</Code>
                        " to the size of the icon element, which overrides them."
                    </ApiRow>
                    <ApiRow name="margin" ty="Option<Margin>" default="None">
                        "The margin around the icon, set as the "<Code inline=true>"--margin"</Code>" variable."
                    </ApiRow>
                    <ApiRow name="aria_label" ty="Option<Oco<'static, str>>" default="None">
                        "Names the icon and makes it an image ("<Code inline=true>"role=\"img\""</Code>"). Without it, the "
                        "icon is decorative and hidden from screen readers ("<Code inline=true>"aria-hidden"</Code>"), as "
                        "it should be next to text or inside a button that has a name."
                    </ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                        "Additional classes and styles."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>
                    "The icon renders as a "<Code inline=true>"<span class=\"leptonic-icon\">"</Code>" containing the "
                    <Code inline=true>"<svg>"</Code>". The default theme makes it 1rem wide and high, and stretches the "
                    <Code inline=true>"<svg>"</Code>" to that size."
                </p>
                <ul>
                    <li>
                        "Size icons by setting "<Code inline=true>"width"</Code>" and "<Code inline=true>"height"</Code>
                        " on the icon element, for example through a class."
                    </li>
                    <li>
                        "Color icons by setting "<Code inline=true>"color"</Code>". Icons fill with "
                        <Code inline=true>"currentColor"</Code>" unless the icon defines its own fill."
                    </li>
                    <li>
                        "Some icons render differently when you change their "<Code inline=true>"background-color"</Code>"."
                    </li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Layout.materialize()>"Content & Layout"</Link></li>
                <li><Link href=routes::doc::Button.materialize()>"Button"</Link>" (icon-only buttons)"</li>
                <li><Link target=LinkTarget::Blank href="https://docs.rs/icondata">"icondata documentation"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
