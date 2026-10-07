use leptonic::{
    components::prelude::*,
    hooks::{
        IntoAttrs, LandmarkRole, UseFocusWithinInput, UseLandmarkInput, UseLandmarkReturn,
        use_focus_within, use_landmark,
    },
    utils::CapturedElement,
};
use leptos::prelude::*;

/// A landmark of the mail app: F6 and Shift+F6 move between them.
#[component]
fn MailLandmark(
    role: LandmarkRole,
    label: &'static str,
    /// The landmark the focus is in, shown below the demo.
    current: RwSignal<Option<&'static str>>,
    children: Children,
) -> impl IntoView {
    let element = CapturedElement::new();
    let UseLandmarkReturn { props } = use_landmark(
        UseLandmarkInput {
            aria_label: label.into(),
            role,
            aria_labelledby: None,
            focus: None,
        },
        element,
    );
    let focus_within = use_focus_within(UseFocusWithinInput {
        on_focus_within_change: Some(Callback::new(move |is_within: bool| {
            if is_within {
                current.set(Some(label));
            } else if current.get_untracked() == Some(label) {
                current.set(None);
            }
        })),
        ..UseFocusWithinInput::default()
    });

    view! {
        <div {..props.into_attrs()} {..focus_within.props.into_attrs()} {..element.attr()} class="demo-landmark">
            <p class="demo-landmark-title">{label}</p>
            {children()}
        </div>
    }
}

#[component]
pub fn LandmarkDemo() -> impl IntoView {
    let current = RwSignal::new(None::<&'static str>);
    let folder = RwSignal::new("Inbox");
    let query = RwSignal::new(String::new());

    view! {
        <div class="demo-landmarks">
            <MailLandmark role=LandmarkRole::Navigation label="Folders" current=current>
                {["Inbox", "Sent", "Archive"]
                    .map(|name| view! {
                        <Button variant=ButtonVariant::Flat on_press=move |_| folder.set(name)>{name}</Button>
                    })
                    .collect_view()}
            </MailLandmark>
            <MailLandmark role=LandmarkRole::Search label="Mail search" current=current>
                <SearchField aria_label="Search mail" value=query set_value=query/>
            </MailLandmark>
            <MailLandmark role=LandmarkRole::Region label="Messages" current=current>
                <p>{move || format!("3 messages in {}.", folder.get())}</p>
                <Button>"Reply"</Button>
            </MailLandmark>
        </div>

        <p class="demo-status">
            {move || current.get().map_or_else(
                || "The focus is outside the landmarks.".to_owned(),
                |landmark| format!("The focus is in the landmark \u{201c}{landmark}\u{201d}."),
            )}
        </p>
    }
}
