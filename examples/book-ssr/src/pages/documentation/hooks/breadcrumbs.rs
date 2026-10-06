use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::breadcrumbs::BreadcrumbsDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseBreadcrumbs() -> impl IntoView {
    view! {
        <DocPage title="use_breadcrumbs">
            <p>
                "Breadcrumbs show where the current page sits in a hierarchy, as a trail of links from the top level down "
                "to the current page. "<Code inline=true>"use_breadcrumbs"</Code>" labels the navigation landmark that "
                "holds the trail, and "<Code inline=true>"use_breadcrumb_item"</Code>" provides the behavior of each link."
            </p>

            <ReactAria hook="useBreadcrumbs"/>

            <Section title="Demo">
                <p>"The trail of this page. The last item is the current page and is not a link."</p>

                <Demo description="Breadcrumb trail with a disabled toggle" source=include_str!("demos/breadcrumbs.rs") source_open=true>
                    <BreadcrumbsDemo/>
                </Demo>
            </Section>

            <Section title="use_breadcrumbs">
                <p>"Spread "<Code inline=true>"nav_props.into_attrs()"</Code>" onto the "<Code inline=true>"<nav>"</Code>" element around the list."</p>

                <Section title="Input" id="use-breadcrumbs-input">
                    <ApiTable kind=ApiKind::Input of="UseBreadcrumbsInput">
                        <ApiRow name="label" ty="Option<String>" default="Some(\"Breadcrumbs\")">
                            "The accessible name of the navigation ("<Code inline=true>"aria-label"</Code>")."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-breadcrumbs-return">
                    <ApiTable kind=ApiKind::Return of="UseBreadcrumbsReturn">
                        <ApiRow name="nav_props" ty="UseBreadcrumbsProps">"A generated "<Code inline=true>"id"</Code>" and the "<Code inline=true>"aria-label"</Code>"."</ApiRow>
                        <ApiRow name="nav_id" ty="String">"The id of the navigation element."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_breadcrumb_item">
                <p>
                    "Call it once per item and spread "<Code inline=true>"link_props.into_attrs()"</Code>" onto the link. "
                    "The current item keeps its element but loses its "<Code inline=true>"href"</Code>"."
                </p>

                <Section title="Input" id="use-breadcrumb-item-input">
                    <ApiTable kind=ApiKind::Input of="UseBreadcrumbItemInput">
                        <ApiRow name="href" ty="Option<String>" default="None">"The link target. Ignored for the current item."</ApiRow>
                        <ApiRow name="is_current" ty="bool" default="false">"Whether this item is the current page, usually the last one."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the item is disabled."</ApiRow>
                        <ApiRow name="on_press" ty="Option<Callback<()>>" default="None">
                            "Called when the item is clicked or activated with Enter or Space, unless it is current or disabled."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-breadcrumb-item-return">
                    <ApiTable kind=ApiKind::Return of="UseBreadcrumbItemReturn">
                        <ApiRow name="link_props" ty="UseBreadcrumbLinkProps">
                            <Code inline=true>"href"</Code>", "<Code inline=true>"aria-current"</Code>", "
                            <Code inline=true>"aria-disabled"</Code>", "<Code inline=true>"tabindex"</Code>
                            " and the click and keyboard handlers of the link."
                        </ApiRow>
                        <ApiRow name="item_props" ty="UseBreadcrumbItemProps">
                            "Props for the list item. They are currently empty."
                        </ApiRow>
                        <ApiRow name="is_current" ty="bool">"Whether this is the current item."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Accessibility">
                <ul>
                    <li>"The "<Code inline=true>"<nav>"</Code>" landmark gets an "<Code inline=true>"aria-label"</Code>" (\u{201c}Breadcrumbs\u{201d} by default)."</li>
                    <li>
                        "The current item gets "<Code inline=true>"aria-current=\"page\""</Code>", no "<Code inline=true>"href"</Code>
                        " and "<Code inline=true>"tabindex=\"-1\""</Code>", so it is skipped when tabbing."
                    </li>
                    <li>"Disabled items get "<Code inline=true>"aria-disabled=\"true\""</Code>" and ignore clicks and keys."</li>
                    <li>"Hide visual separators from assistive technology with "<Code inline=true>"aria-hidden"</Code>", as the demo does."</li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="Tab">"Moves focus to the next breadcrumb link."</KeyRow>
                    <KeyRow keys="Enter / Space">"Calls "<Code inline=true>"on_press"</Code>" of the focused item."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Navigation.materialize()>"Navigation overview"</Link></li>
                <li><Link href=routes::doc::link::UseLink.materialize()>"use_link"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
