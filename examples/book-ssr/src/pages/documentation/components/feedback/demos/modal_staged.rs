use leptonic::components::prelude::*;
use leptos::prelude::*;

/// Where the sign-up is: not started, at one of its two steps, or done.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Step {
    Closed,
    Account,
    Newsletter,
    Done,
}

#[component]
pub fn ModalStagedDemo() -> impl IntoView {
    let (step, set_step) = signal(Step::Closed);
    let is_at = move |at: Step| Signal::derive(move || step.get() == at);

    view! {
        <Button on_press=move |_| set_step.set(Step::Account)>"Sign up"</Button>
        <p class="demo-status">
            {move || match step.get() {
                Step::Closed => "Not signed up yet.",
                Step::Account | Step::Newsletter => "Signing up\u{2026}",
                Step::Done => "Signed up.",
            }}
        </p>

        // Dismissing the first step cancels the sign-up.
        <Modal is_open=is_at(Step::Account) set_open=move |open: bool| if !open { set_step.set(Step::Closed) }>
            <ModalHeader><ModalTitle>"Step 1 of 2: Account"</ModalTitle></ModalHeader>
            <ModalBody>"Create an account with your email address."</ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_press=move |_| set_step.set(Step::Closed) color=ButtonColor::Secondary>"Cancel"</Button>
                    <Button on_press=move |_| set_step.set(Step::Newsletter)>"Next"</Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>

        // Dismissing the second step goes back to the first.
        <Modal is_open=is_at(Step::Newsletter) set_open=move |open: bool| if !open { set_step.set(Step::Account) }>
            <ModalHeader><ModalTitle>"Step 2 of 2: Newsletter"</ModalTitle></ModalHeader>
            <ModalBody>"Get a short email about new releases once a month."</ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_press=move |_| set_step.set(Step::Account) color=ButtonColor::Secondary>"Back"</Button>
                    <Button on_press=move |_| set_step.set(Step::Done)>"Finish"</Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>
    }
}
