use leptonic::components::prelude::*;
use leptos::prelude::*;

const COLORS: [(ButtonColor, &str); 6] = [
    (ButtonColor::Primary, "Primary"),
    (ButtonColor::Secondary, "Secondary"),
    (ButtonColor::Success, "Success"),
    (ButtonColor::Info, "Info"),
    (ButtonColor::Warn, "Warn"),
    (ButtonColor::Danger, "Danger"),
];

#[component]
pub fn ButtonColorsDemo() -> impl IntoView {
    let last_pressed = RwSignal::new(None::<&str>);

    view! {
        <ButtonWrapper>
            {COLORS
                .into_iter()
                .map(|(color, name)| {
                    view! { <Button color=color on_press=move |_| last_pressed.set(Some(name))>{name}</Button> }
                })
                .collect_view()}
        </ButtonWrapper>
        <p class="demo-status">
            {move || match last_pressed.get() {
                Some(name) => format!("Last pressed: {name}."),
                None => "No button pressed yet.".to_owned(),
            }}
        </p>
    }
}
