use leptonic::prelude::*;
use leptos::*;

use crate::pages::documentation::doc_root::DocRoutes;

#[component]
pub fn PageOverview() -> impl IntoView {
    view! {
        <H1>"Overview"</H1>

        <P>
            "Leptonic is a rich component library for the "<LinkExt href="https://leptos.dev/" target=LinkExtTarget::Blank>"Leptos"</LinkExt>" web framework."
        </P>

        <P>
            "It provides \"ready to be used\" components for capturing user input through buttons, inputs fields, select inputs, sliders, date & time inputs or even a rich text editor. "
            "Well known mechanisms for providing user-feedback are also available, ranging from modal windows, toast messages and alerts to progress indicators. "
            "Leptonic also includes components helping you lay out all these elements on your pages. These include stacks, a full grid system, tabs and collapsibles as well as components for app bars and side drawers. "
            "Common tasks such as linking to other parts in or outside your site or including a plethora of icons are also made simple. "
        </P>

        <P>"Explore the available components and other features using the side menu to get acquainted with what Leptonic has to offer."</P>

        <P>
            "If you want to dive right in, follow our " <Link href=DocRoutes::Installation>"Installation"</Link> " instructions."
        </P>
    }
}
