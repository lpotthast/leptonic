use indoc::indoc;
use leptos::prelude::*;

use super::demos::breadcrumbs::BreadcrumbsDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageUseBreadcrumbs() -> impl IntoView {
    view! {
        <DocPage title="Breadcrumbs Hooks">
            <p>
                <AnchorLink href="#use-breadcrumbs">"use_breadcrumbs"</AnchorLink>" names the list that holds the trail of "
                "links, and "<AnchorLink href="#use-breadcrumb-item">"use_breadcrumb_item"</AnchorLink>" provides the "
                "behavior of each link. See the "<Link href=routes::doc::Breadcrumbs.materialize()>"Breadcrumbs overview"</Link>
                " for concept guidance."
            </p>

            <ReactAria hook="useBreadcrumbs"/>

            <Section title="Demo">
                <p>
                    "The trail of this page. The last item is the current page: a disabled link without "
                    <Code inline=true>"href"</Code>". The links are styled through the ARIA attributes the hooks set."
                </p>

                <Demo description="Breadcrumb trail with a Disabled checkbox" source=include_str!("demos/breadcrumbs.rs") source_open=true>
                    <BreadcrumbsDemo/>
                </Demo>
            </Section>

            <Section title="use_breadcrumbs">
                <p>
                    "Spread "<Code inline=true>"props.into_attrs()"</Code>" onto the "<Code inline=true>"<ol>"</Code>" of the trail, "
                    "and wrap the list in a "<Code inline=true>"<nav>"</Code>" landmark."
                </p>

                <Section title="Input" id="use-breadcrumbs-input">
                    <ApiTable kind=ApiKind::Input of="UseBreadcrumbsInput">
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="\"Breadcrumbs\"">"Names the list of breadcrumbs."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-breadcrumbs-return">
                    <ApiTable kind=ApiKind::Return of="UseBreadcrumbsReturn">
                        <ApiRow name="props" ty="UseBreadcrumbsProps">"The "<Code inline=true>"aria-label"</Code>"."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-breadcrumbs-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use leptonic::{
                                IntoAttrs,
                                hooks::breadcrumbs::{UseBreadcrumbsInput, use_breadcrumbs},
                            };

                            let breadcrumbs = use_breadcrumbs(UseBreadcrumbsInput::default());

                            view! {
                                <nav aria-label="Breadcrumbs">
                                    <ol {..breadcrumbs.props.into_attrs()}>
                                        // One `<li>` per item.
                                    </ol>
                                </nav>
                            }
                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="use_breadcrumb_item">
                <p>
                    "An item is a "<Link href=format!("{}#use-link", routes::doc::link::Hook.materialize())>"use_link"</Link>
                    " link: call the hook once per item and spread its props onto the link element. The current item is "
                    "disabled: it keeps its element, but loses its "<Code inline=true>"href"</Code>" and gets "
                    <Code inline=true>"aria-current"</Code>" and "<Code inline=true>"aria-disabled"</Code>"."
                </p>

                <Section title="Input" id="use-breadcrumb-item-input">
                    <ApiTable kind=ApiKind::Input of="UseBreadcrumbItemInput">
                        <ApiRow name="link" ty="UseLinkInput" default="UseLinkInput::default()">
                            "The item\u{2019}s link: "<Code inline=true>"href"</Code>", "<Code inline=true>"is_disabled"</Code>", "
                            <Code inline=true>"on_press"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="is_current" ty="Signal<bool>" default="false">"Whether this item is the current page, usually the last one."</ApiRow>
                        <ApiRow name="current" ty="AriaCurrent" default="Page">"The current item\u{2019}s "<Code inline=true>"aria-current"</Code>"."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-breadcrumb-item-return">
                    <p>
                        "The hook returns "<Code inline=true>"use_link"</Code>"\u{2019}s "
                        <Link href=format!("{}#use-link-return", routes::doc::link::Hook.materialize())>"UseLinkReturn"</Link>"."
                    </p>
                </Section>

                <Section title="Example" id="use-breadcrumb-item-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use leptonic::{
                                hooks::{
                                    breadcrumbs::{UseBreadcrumbItemInput, use_breadcrumb_item},
                                    link::UseLinkInput,
                                },
                            };

                            let (attrs, styles) = use_breadcrumb_item(UseBreadcrumbItemInput {
                                link: UseLinkInput {
                                    href: Signal::stored(Some("/docs".to_owned())),
                                    ..UseLinkInput::default()
                                },
                                is_current: Signal::stored(true),
                                ..UseBreadcrumbItemInput::default()
                            })
                            .props
                            .into_parts();

                            view! { <li><a {..attrs} style=styles>"Docs"</a></li> }
                        "#)}
                    </Code>
                </Section>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Breadcrumbs.materialize()>"Breadcrumbs overview"</Link></li>
                <li><Link href=routes::doc::breadcrumbs::Atom.materialize()>"Breadcrumbs Atoms"</Link></li>
                <li><Link href=routes::doc::link::Hook.materialize()>"Link Hooks"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
