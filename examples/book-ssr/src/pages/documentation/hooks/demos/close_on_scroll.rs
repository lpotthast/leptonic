use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;
use leptos_element_capture::CapturedElement;

/// `use_close_on_scroll`: scrolling the list that holds the trigger closes the open overlay.
#[component]
pub fn CloseOnScrollDemo() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let closed_by_scrolling = RwSignal::new(0_u32);
    let trigger = CapturedElement::new();

    use_close_on_scroll(UseCloseOnScrollInput {
        is_open: is_open.into(),
        trigger_element: trigger,
        on_close: Callback::new(move |()| {
            closed_by_scrolling.update(|count| *count += 1);
            set_is_open.set(false);
        }),
    });

    view! {
        // Focusable, so that keyboard users can scroll it too.
        <div class="demo-close-on-scroll-list" tabindex="0" role="region" aria-label="Orders">
            <p>"Order 1041"</p>
            <p>"Order 1042"</p>
            <span {..trigger.attr()}>
                <Button on_press=move |_| set_is_open.update(|open| *open = !*open)>
                    {move || if is_open.get() { "Hide details" } else { "Show details" }}
                </Button>
            </span>
            <p>"Order 1043"</p>
            <p>"Order 1044"</p>
            <p>"Order 1045"</p>
            <p>"Order 1046"</p>
        </div>

        <Show when=move || is_open.get()>
            <div class="demo-popover demo-mt-half">"Order 1042 shipped on Monday."</div>
        </Show>

        <p class="demo-status">
            {move || {
                let state = if is_open.get() { "Scroll the list to close the details." } else { "The details are closed." };
                match closed_by_scrolling.get() {
                    1 => format!("{state} Closed by scrolling 1 time."),
                    count => format!("{state} Closed by scrolling {count} times."),
                }
            }}
        </p>
    }
}
