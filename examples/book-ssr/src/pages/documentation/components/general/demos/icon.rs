use leptonic::components::icon::Icon;
use leptonic::prelude::*;
use leptos::prelude::*;

#[component]
pub fn IconDemo() -> impl IntoView {
    view! {
        <Icon icon=icondata::BsFolderFill attr:style="font-size: 6em;"/>
        <Icon icon=icondata::BsFolder attr:style="font-size: 6em;"/>
    }
}
