use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn ModalConfirmDemo() -> impl IntoView {
    let (show_confirm_modal, set_show_confirm_modal) = signal(false);

    view! {
        <p><Button on_press=move |_| set_show_confirm_modal.set(true)>"Show confirmation modal"</Button></p>

        <ConfirmModal
            show_when=show_confirm_modal
            requires_confirmation_of="ok".to_owned()
            on_accept=move || set_show_confirm_modal.set(false)
            on_cancel=move || set_show_confirm_modal.set(false)
        />
    }
}

#[component]
pub fn ConfirmModal<A, C>(
    #[prop(into)] show_when: Signal<bool>,
    requires_confirmation_of: String,
    on_accept: A,
    on_cancel: C,
) -> impl IntoView
where
    A: Fn() + Send + Sync + Copy + 'static,
    C: Fn() + Send + Sync + Copy + 'static,
{
    let required = StoredValue::new(requires_confirmation_of);

    let (input, set_input) = signal(String::new());

    let confirmed = move || required.with_value(|r| input.with(|i| r == i));
    let disabled = Signal::derive(move || !confirmed());

    let on_accept = move || {
        set_input.update(std::string::String::clear);
        on_accept();
    };
    let on_cancel = move || {
        set_input.update(std::string::String::clear);
        on_cancel();
    };

    view! {
        <Modal show_when=show_when on_close=move |()| on_cancel()>
            <ModalHeader><ModalTitle>"Delete repository?"</ModalTitle></ModalHeader>
            <ModalBody>
                "Please enter \""{required.get_value()}"\" to confirm."
                <TextInput get=input set=set_input/>
            </ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_press=move |_| on_accept() disabled=disabled color=ButtonColor::Danger>"Confirm"</Button>
                    <Button on_press=move |_| on_cancel() color=ButtonColor::Secondary>"Cancel"</Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>
    }
}
