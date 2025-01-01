use crate::app::SegmentRenderer;
use crate::pages::documentation::doc_root::doc_routes;
use crate::pages::documentation::{article::Article, toc::Toc};
use leptonic::{atoms::link::AnchorLink, components::prelude::*};
use leptos::prelude::*;

#[component]
pub fn PageOverview() -> impl IntoView {
    view! {
        <Article>
            <h1 id="overview">
                "Overview"
                <AnchorLink href="#overview" description="Direct link to section: Overview"/>
            </h1>

            <p>
                "Leptonic is a rich component library for the "<LinkExt href="https://leptos.dev/" target=LinkExtTarget::Blank>"Leptos"</LinkExt>" web framework."
            </p>

            <p>
                "It provides \"ready to be used\" components for capturing user input through buttons, inputs fields, select inputs, sliders, date & time inputs or even a rich text editor. "
                "Well known mechanisms for providing user-feedback are also available, ranging from modal windows, toast messages and alerts to progress indicators. "
                "Leptonic also includes components helping you lay out all these elements on your pages. These include stacks, a full grid system, tabs and collapsibles as well as components for app bars and side drawers. "
                "Common tasks such as linking to other parts in or outside your site or including a plethora of icons are also made simple. "
            </p>

            <p>"Explore the available components and other features using the side menu to get acquainted with what Leptonic has to offer."</p>

            <p>
                "If you want to dive right in, follow our " <Link href=doc_routes::INSTALLATION.to_href()>"Installation"</Link> " instructions."
            </p>

            <h2 id="help">
                "Need help?"
                <AnchorLink href="#help" description="Need help?"/>
            </h2>

            <p>
                "If you get stuck at any point integrating or using Leptonic, these are things you may find helpful: "
                <ul>
                    <li>"Look for help in the Leptos "<LinkExt href="https://discord.gg/x8NhWWYTV2" target=LinkExtTarget::Blank>"Discord"</LinkExt>" server."</li>
                    <li>"If you think you encountered a bug, open a ticket in our " <LinkExt href="https://github.com/lpotthast/leptonic" target=LinkExtTarget::Blank>"repository"</LinkExt></li>
                    <li>"Compare the implementation of this book at "<LinkExt href="https://github.com/lpotthast/leptonic/tree/main/examples/book" target=LinkExtTarget::Blank>"GitHub"</LinkExt>" with what you currently have."</li>
                </ul>
            </p>

            <h2 id="contribute">
                "Contribute"
                <AnchorLink href="#contribute" description="Direct link to section: Contribute"/>
            </h2>

            <p>
                "If you have anything to say about Leptonic, be it a component you miss, a feature you feel missing, or a bug you encountered, "
                "feel free to contribute back to the project by reaching the community or us through"
                <ul>
                    <li>"the Leptos "<LinkExt href="https://discord.gg/x8NhWWYTV2" target=LinkExtTarget::Blank>"Discord"</LinkExt>" server or "</li>
                    <li>"the "<LinkExt href="https://github.com/lpotthast/leptonic/issues" target=LinkExtTarget::Blank>"Issues"</LinkExt>" section of the Leptonic repository."</li>
                </ul>
            </p>

            <p>
                "Writing a component library is a big undertaking. Feel free to contribute by writing new or updating existing code yourself. "
                "As more people are on board, more and better ideas, concepts and implementation details follow. Code contributions are always welcomed!"
            </p>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Overview", link: "#overview" },
                Toc::Leaf { title: "Need help?", link: "#help" },
                Toc::Leaf { title: "Contribute", link: "#contribute" },
            ]
        }/>
    }
}
