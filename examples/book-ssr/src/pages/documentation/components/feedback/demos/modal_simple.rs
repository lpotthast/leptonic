use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn ModalSimpleDemo() -> impl IntoView {
    let is_open = RwSignal::new(false);
    let (times_read, set_times_read) = signal(0_u32);

    view! {
        <Button on_press=move |_| is_open.set(true)>"What\u{2019}s new?"</Button>
        <p class="demo-status">
            {move || match times_read.get() {
                0 => "You haven\u{2019}t read the release notes yet.".to_owned(),
                1 => "You read the release notes once.".to_owned(),
                n => format!("You read the release notes {n} times."),
            }}
        </p>

        // `set_open` runs whenever the modal closes: through "Got it", Escape or a click outside.
        <Modal
            is_open=is_open
            set_open=move |open: bool| {
                if !open {
                    set_times_read.update(|n| *n += 1);
                }
                is_open.set(open);
            }
        >
            <ModalHeader><ModalTitle>"What\u{2019}s new"</ModalTitle></ModalHeader>
            <ModalBody>"Modals now keep focus inside and return it to the button that opened them."</ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_press=move |_| { set_times_read.update(|n| *n += 1); is_open.set(false); }>"Got it"</Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>
    }
}
