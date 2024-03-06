use leptonic::components::anchor::AnchorLink;
use leptos::*;

use crate::app::AppLayoutContext;

pub enum Toc {
    Leaf {
        title: &'static str,
        link: &'static str,
    },
    Group {
        title: &'static str,
        link: &'static str,
        inner: Vec<Toc>,
    },
    List {
        inner: Vec<Toc>,
    },
}

#[component]
pub fn Toc(toc: Toc) -> impl IntoView {
    let app_layout_context = expect_context::<AppLayoutContext>();
    view! {
        <nav
            id="toc"
            style:display=move || match app_layout_context.is_medium.get() {
                true => "none",
                false => "initial",
            }
        >
            <h2 style="margin-top: 0; margin-bottom: 1em;">
                "Contents"
            </h2>
            <TocEntry toc level=1/>
        </nav>
    }
}

#[component]
pub fn TocEntry(toc: Toc, level: usize) -> impl IntoView {
    view! {
            {match toc {
                Toc::Leaf { title, link } => view! {
                    <li data-level=level>
                        <AnchorLink href=link>
                            { title }
                        </AnchorLink>
                    </li>
                }.into_view(),
                Toc::Group { title, link, inner } => view! {
                    <li data-level=level>
                        <AnchorLink href=link>
                            { title }
                        </AnchorLink>
                        <TocEntry toc=Toc::List { inner } level=level+1/>
                    </li>
                }.into_view(),
                Toc::List { inner } => view!{
                    <ul>
                        { inner.into_iter().map(|toc| view! {
                            <TocEntry toc=toc level=level/>
                        }).collect_view() }
                    </ul>
                }.into_view(),
            }}
    }
}
