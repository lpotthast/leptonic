use leptonic::{atoms::prelude::LinkButton, components::prelude as components, jiff::civil::Date};
use leptos::prelude::*;
use leptos_meta::{Meta, Title};

use crate::{
    app::{BookToast, BookToasts, MainLandmark, SITE_DESCRIPTION},
    kit::{Code, Icon, Language, Link},
    routes,
};

#[component]
pub fn PageWelcome() -> impl IntoView {
    view! {
        <Title text="Leptonic \u{2013} accessible UI building blocks for Leptos"/>
        <Meta name="description" content=SITE_DESCRIPTION/>

        <MainLandmark class="book-welcome">
            <div class="book-welcome-intro">
                <h1 class="book-welcome-title">"Leptonic"</h1>
                <p class="book-welcome-tagline">
                    "Accessible UI building blocks for Leptos: hooks, atoms and themed components."
                </p>
                <div class="book-welcome-actions">
                    <LinkButton href=routes::doc::Installation.materialize() classes="book-button">
                        "Get started"
                    </LinkButton>
                    <LinkButton
                        href=routes::doc::Overview.materialize()
                        classes="book-button"
                        attr:data-variant="secondary"
                    >
                        "Read the overview"
                    </LinkButton>
                </div>
                <Code language=Language::Shell classes="book-welcome-install">"cargo add leptonic --features full"</Code>
            </div>

            <Showcase/>

            <ul class="book-welcome-features">
                <Feature icon=icondata::BsUniversalAccess title="Accessible" href=routes::doc::Accessibility.materialize()>
                    "Keyboard, pointer, touch and screen reader interaction, ported from react-aria\u{2019}s "
                    "battle-tested hooks."
                </Feature>
                <Feature icon=icondata::BsLayers title="Three layers" href=routes::doc::Architecture.materialize()>
                    "Hooks for full control, unstyled atoms for your own design system, or themed components that "
                    "work out of the box."
                </Feature>
                <Feature icon=icondata::BsBraces title="Rust all the way" href=routes::doc::Ssr.materialize()>
                    "Typed APIs, server-side rendering with hydration, and internationalization without a JavaScript "
                    "runtime."
                </Feature>
            </ul>
        </MainLandmark>
    }
}

/// One of the welcome page's selling points: an icon, a linked title and a sentence.
#[component]
fn Feature(
    icon: icondata::Icon,
    title: &'static str,
    href: String,
    children: Children,
) -> impl IntoView {
    view! {
        <li class="book-welcome-feature">
            <Icon icon=icon classes="book-welcome-feature-icon"/>
            <h2><Link href>{title}</Link></h2>
            <p>{children()}</p>
        </li>
    }
}

/// A few of leptonic's themed components in a small form, live: the save button shows a toast.
#[component]
fn Showcase() -> impl IntoView {
    use components::{Button, ButtonColor, ButtonVariant, DatePicker, Slider, Switch, TextField};

    let toasts = expect_context::<BookToasts>();
    let project = RwSignal::new("Rocket".to_owned());
    let launch = RwSignal::new(None::<Date>);
    let notify = RwSignal::new(true);
    let volume = RwSignal::new(60_u8);

    let save = move |_| {
        let name = project.get_untracked();
        let body = match launch.get_untracked() {
            Some(date) => format!(
                "\u{201c}{name}\u{201d} launches on {}.",
                date.strftime("%B %-d, %Y")
            ),
            None => format!("\u{201c}{name}\u{201d} has no launch date yet."),
        };
        toasts.show(BookToast {
            title: "Saved".to_owned(),
            description: body,
        });
    };

    view! {
        <section class="book-welcome-showcase" aria-labelledby="book-welcome-showcase-title">
            <h2 id="book-welcome-showcase-title" class="book-welcome-showcase-title">
                "Try them: these are leptonic\u{2019}s components"
            </h2>
            <div class="book-welcome-showcase-form">
                <TextField label="Project" value=project set_value=project/>
                <DatePicker<Date> label="Launch date" value=launch set_value=launch/>
                <div class="book-welcome-showcase-volume">
                    <span aria-hidden="true">{move || format!("Volume: {}%", volume.get())}</span>
                    <Slider value=volume set_value=volume min_value=0 max_value=100 aria_label="Volume"/>
                </div>
                <Switch is_selected=notify set_selected=notify>"Email me before the launch"</Switch>
            </div>
            <div class="book-welcome-showcase-actions">
                <Button on_press=save>"Save"</Button>
                <Button
                    variant=ButtonVariant::Outlined
                    color=ButtonColor::Secondary
                    on_press=move |_| {
                        project.set("Rocket".to_owned());
                        launch.set(None);
                        notify.set(true);
                        volume.set(60);
                    }
                >
                    "Reset"
                </Button>
            </div>
        </section>
    }
}
