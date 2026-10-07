use leptonic::{
    atoms::button::Button,
    hooks::{IntoAttrs, LandmarkRole, UseLandmarkInput, UseLandmarkReturn, use_landmark},
    utils::CapturedElement,
};
use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn LayoutAppBarDemo() -> impl IntoView {
    let (last_action, set_last_action) = signal(None::<&'static str>);

    // The page's banner landmark. This page has a second one, the book's own app bar, so it gets a name.
    let header = CapturedElement::new();
    let UseLandmarkReturn { props } = use_landmark(UseLandmarkInput {
        element: header,
        role: LandmarkRole::Banner,
        aria_label: "My App".into(),
        aria_labelledby: None,
        focus: None,
    });

    view! {
        // A scrolling frame standing in for the page: the bar sticks to its top.
        <div class="demo-app-bar-frame" role="region" aria-label="Scrolling page" tabindex="0">
            <header {..props.into_attrs()} {..header.attr()} class="demo-app-bar">
                <span class="demo-app-bar-title">"My App"</span>
                <div class="demo-app-bar-actions">
                    // Icon-only buttons are named with `aria_label`; the icons themselves are decorative.
                    <Button
                        on_press=move |_| set_last_action.set(Some("Notifications"))
                        aria_label="Notifications"
                        classes="demo-app-bar-button"
                    >
                        <Icon icon=icondata::BsBell/>
                    </Button>
                    <Button
                        on_press=move |_| set_last_action.set(Some("Log out"))
                        aria_label="Log out"
                        classes="demo-app-bar-button"
                    >
                        <Icon icon=icondata::BsBoxArrowRight/>
                    </Button>
                </div>
            </header>

            <div class="demo-app-bar-content">
                {(0..8).map(|_| view! { <div class="demo-app-bar-placeholder" aria-hidden="true"></div> }).collect_view()}
            </div>
        </div>
        <p class="demo-status">
            {move || match last_action.get() {
                Some(action) => format!("Last action: {action}."),
                None => "No action yet.".to_owned(),
            }}
        </p>
    }
}
