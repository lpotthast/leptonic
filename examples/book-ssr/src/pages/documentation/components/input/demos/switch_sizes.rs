use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn SwitchSizesDemo() -> impl IntoView {
    view! {
        <div class="demo-control-stack">
            <Switch size=SwitchSize::Small>"Small"</Switch>
            <Switch size=SwitchSize::Normal default_selected=true>"Normal"</Switch>
            <Switch size=SwitchSize::Big>"Big"</Switch>
        </div>
    }
}
