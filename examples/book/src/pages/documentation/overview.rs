use leptonic::prelude::*;
use leptos::*;

use crate::pages::documentation::doc_root::DocRoutes;

#[component]
pub fn PageOverview(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal_ls(cx, "count", 0u64);

    let increase_counter_by_one = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <Typography variant=TypographyVariant::H1>"Overview"</Typography>

        <P>"Leptonic is a component library for Leptos. This site was built using Leptonic."</P>
        <P>"Explore the available components and other features using the side menu."</P>
        <P>
            "If you want to dive right in, follow our " <Link href=DocRoutes::Installation>"Installation"</Link> " instructions."
        </P>

        <Button on_click=increase_counter_by_one>"Click Me: " {count}</Button>
    }
}
