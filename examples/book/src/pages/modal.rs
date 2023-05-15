use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageModal(cx: Scope) -> impl IntoView {
    let (show_modal, set_show_modal) = create_signal(cx, false);
    let (show_modal2, set_show_modal2) = create_signal(cx, false);
    let (show_input_modal, set_show_input_modal) = create_signal(cx, false);
    let (last_input, set_last_input) = create_signal(cx, Option::<String>::None);

    view! { cx,
        <H2>"Modals"</H2>

        <Button on_click=move |_| set_show_modal.set(true)>"Show Modal"</Button>
        <Button on_click=move |_| set_show_input_modal.set(true)>"Show Input Modal"</Button>

        <Modal display_if=show_modal>
            <ModalHeader><ModalTitle>"Sure?"</ModalTitle></ModalHeader>
            <ModalBody>"This ia a test modal."</ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_click=move |_| set_show_modal.set(false) color=ButtonColor::Danger>"Accept"</Button>
                    <Button on_click=move |_| {
                        //set_show_modal.set(false);
                        set_show_modal2.set(true);
                    } color=ButtonColor::Info>"Next"</Button>
                    <Button on_click=move |_| set_show_modal.set(false) color=ButtonColor::Secondary>"Cancel"</Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>

        <Modal display_if=show_modal2>
            <ModalHeader><ModalTitle>"Next one"</ModalTitle></ModalHeader>
            <ModalBody>"This overlays..."</ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_click=move |_| set_show_modal2.set(false) color=ButtonColor::Secondary>"Back"</Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>

        <InputModal
            display_if=show_input_modal
            on_accept=move |input| {
                set_last_input.set(Some(input));
                set_show_input_modal.set(false);
            }
            on_cancel=move || set_show_input_modal.set(false)
        />

        <div>
            "Last input: "
            { move || last_input.get() }
        </div>
    }
}

#[component]
pub fn InputModal<A, C>(cx: Scope, display_if: ReadSignal<bool>, on_accept: A, on_cancel: C) -> impl IntoView
where
    A: Fn(String) + Copy + 'static,
    C: Fn() + Copy + 'static,
{
    let (input, set_input) = create_signal(cx, "".to_owned());

    view! { cx,
        <Modal display_if=display_if>
            <ModalHeader><ModalTitle>"Delete repository"</ModalTitle></ModalHeader>
            <ModalBody>
                "Please enter \"ok\" to delete the repository."
                <Input get=input set=set_input/>
            </ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_click=move |_| (on_accept)("dummy".to_owned()) color=ButtonColor::Danger>"Accept"</Button>
                    <Button on_click=move |_| (on_cancel)() color=ButtonColor::Secondary>"Cancel"</Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>
    }
}
