use leptonic::{
    components::prelude::*,
    hooks::{Orientation, collections::Key},
};
use leptos::prelude::*;
use uuid::Uuid;

/// The variants and timeouts to choose from, with their radio labels.
const VARIANTS: [(&str, ToastVariant); 4] = [
    ("Success", ToastVariant::Success),
    ("Info", ToastVariant::Info),
    ("Warn", ToastVariant::Warn),
    ("Error", ToastVariant::Error),
];
const TIMEOUTS: [(&str, ToastTimeout); 3] = [
    ("3 seconds", ToastTimeout::DefaultDelay),
    ("15 seconds", ToastTimeout::CustomDelay(time::Duration::seconds(15))),
    ("Until closed", ToastTimeout::None),
];

/// The option whose label is selected.
fn chosen<T: Copy>(options: &[(&str, T)], selected: Option<&Key>) -> Option<T> {
    options
        .iter()
        .find(|(label, _)| selected == Some(&Key::from(*label)))
        .map(|(_, option)| *option)
}

#[component]
pub fn ToastCreationDemo() -> impl IntoView {
    let header = RwSignal::new("Saved".to_owned());
    let body = RwSignal::new("Your changes were saved.".to_owned());
    let variant = RwSignal::new(Some(Key::from(VARIANTS[0].0)));
    let timeout = RwSignal::new(Some(Key::from(TIMEOUTS[0].0)));

    let toasts = expect_context::<Toasts>();

    let create_toast = move |_| {
        let header = header.get_untracked();
        let body = body.get_untracked();
        toasts.push(Toast {
            id: Uuid::new_v4(),
            created_at: time::OffsetDateTime::now_utc(),
            variant: variant.with_untracked(|key| chosen(&VARIANTS, key.as_ref())).unwrap_or_default(),
            header: (move || header.clone()).into(),
            body: (move || body.clone()).into(),
            timeout: timeout
                .with_untracked(|key| chosen(&TIMEOUTS, key.as_ref()))
                .unwrap_or(ToastTimeout::DefaultDelay),
        });
    };

    view! {
        <div class="demo-form">
            <TextField label="Header" value=header set_value=header/>
            <TextField label="Body" value=body set_value=body/>

            <RadioGroup label="Variant" orientation=Orientation::Horizontal value=variant set_value=variant>
                {VARIANTS.iter().map(|(label, _)| view! { <Radio value=*label>{*label}</Radio> }).collect_view()}
            </RadioGroup>

            <RadioGroup label="Timeout" orientation=Orientation::Horizontal value=timeout set_value=timeout>
                {TIMEOUTS.iter().map(|(label, _)| view! { <Radio value=*label>{*label}</Radio> }).collect_view()}
            </RadioGroup>
        </div>

        <div class="demo-inline-controls">
            <Button on_press=create_toast>"Create Toast"</Button>
            <Button variant=ButtonVariant::Outlined on_press=move |_| toasts.clear()>"Clear All"</Button>
        </div>
        <p class="demo-status">
            {move || match toasts.toasts.with(Vec::len) {
                1 => "1 toast shown.".to_owned(),
                count => format!("{count} toasts shown."),
            }}
        </p>
    }
}
