use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageModal(cx: Scope) -> impl IntoView {
    let (show_modal, set_show_modal) = create_signal(cx, false);
    let (show_modal2, set_show_modal2) = create_signal(cx, false);
    let (show_input_modal, set_show_input_modal) = create_signal(cx, false);
    let (last_input, set_last_input) = create_signal(cx, Option::<String>::None);

    let (a_text, set_a_text) = create_signal(cx, String::new());
    let a = Signal::derive(cx, move || {
        let t = a_text.get();
        match t.len() {
            0 => None,
            _ => Some(t),
        }
    });
    let show_a = Signal::derive(cx, move || a.get().is_some());

    view! { cx,
        <H1>"Modals"</H1>

        <Button on_click=move |_| set_show_modal.set(true)>"Show Modal"</Button>
        <Button on_click=move |_| set_show_input_modal.set(true)>"Show Input Modal"</Button>

        <Input get=a_text set=move |v| set_a_text.set(v)/>
        <Modal show_when=show_a>
            <ModalHeader><ModalTitle>"Sure?"</ModalTitle></ModalHeader>
            <ModalBody>"This ia a test modal. Input: " { move || format!("{}", a.get().unwrap_or_default()) }</ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_click=move |_| set_a_text.set(String::new()) color=ButtonColor::Danger>"Accept"</Button>
                    <Button on_click=move |_| set_a_text.set(String::new()) color=ButtonColor::Secondary>"Cancel"</Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>

        <Modal show_when=show_modal>
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

        <Modal show_when=show_modal2>
            <ModalHeader><ModalTitle>"Next one"</ModalTitle></ModalHeader>
            <ModalBody>"This overlays..."</ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_click=move |_| set_show_modal2.set(false) color=ButtonColor::Secondary>"Back"</Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>

        <InputModal
            show_when=show_input_modal
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

        <H2>"Styling"</H2>

        <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

        <Code>
            {indoc!(r#"
                --modal-color
                --modal-background-color
                --modal-padding
                --modal-font-size
                --modal-header-padding
                --modal-body-padding
                --modal-footer-padding
                --modal-border-radius
                --modal-box-shadow
            "#)}
        </Code>
    }
}

#[component]
pub fn InputModal<A, C>(
    cx: Scope,
    #[prop(into)] show_when: Signal<bool>,
    on_accept: A,
    on_cancel: C,
) -> impl IntoView
where
    A: Fn(String) + Copy + 'static,
    C: Fn() + Copy + 'static,
{
    let (input, set_input) = create_signal(cx, "".to_owned());

    view! { cx,
        <Modal show_when=show_when>
            <ModalHeader><ModalTitle>"Delete repository"</ModalTitle></ModalHeader>
            <ModalBody>
                "Please enter \"ok\" to delete the repository."
                <Input get=input set=move |v| set_input.set(v)/>
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
