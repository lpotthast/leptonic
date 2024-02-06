use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageModal() -> impl IntoView {
    let (show_simple_modal, set_show_simple_modal) = create_signal(false);

    let (show_staged_modal1, set_show_staged_modal1) = create_signal(false);
    let (show_staged_modal2, set_show_staged_modal2) = create_signal(false);

    let escape_staged_modal1 = Producer::new(move || set_show_staged_modal1.set(false));
    let escape_staged_modal2 = Producer::new(move || {
        set_show_staged_modal2.set(false);
        set_show_staged_modal1.set(true);
    });

    let (show_confirm_modal, set_show_confirm_modal) = create_signal(false);

    view! {
        <H1>"Modals"</H1>

        <P>"Create modals to ask critical questions or notify the user about an upcoming event which must be acknowledged in some way."</P>

        <Code>
            {indoc!(r#"
                let (show_simple_modal, set_show_simple_modal) = create_signal(false);

                view! {
                    <P><Button on_press=move |_| set_show_simple_modal.set(true)>"Show simple modal"</Button></P>
                    <Modal show_when=show_simple_modal>
                        <ModalHeader><ModalTitle>"Hello"</ModalTitle></ModalHeader>
                        <ModalBody>"This ia a simple modal."</ModalBody>
                        <ModalFooter>
                            <ButtonWrapper>
                                <Button on_press=move |_| set_show_simple_modal.set(false) color=ButtonColor::Secondary>"Cancel"</Button>
                            </ButtonWrapper>
                        </ModalFooter>
                    </Modal>
                }
            "#)}
        </Code>

        <P><Button on_press=move |_| set_show_simple_modal.set(true)>"Show simple modal"</Button></P>

        <Modal show_when=show_simple_modal on_escape=move || set_show_simple_modal.set(false)>
            <ModalHeader><ModalTitle>"Hello"</ModalTitle></ModalHeader>
            <ModalBody>"This ia a simple modal."</ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_press=move |_| set_show_simple_modal.set(false) color=ButtonColor::Secondary>"Cancel"</Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>

        <H2>"Stages"</H2>

        <P>"You can connect multiple modals by setting their visibility props appropriately."</P>

        <Code>
            {indoc!(r#"
                let (show_staged_modal1, set_show_staged_modal1) = create_signal(false);
                let (show_staged_modal2, set_show_staged_modal2) = create_signal(false);

                view! {
                    <P><Button on_press=move |_| set_show_staged_modal1.set(true)>"Show staged modal"</Button></P>

                    <Modal show_when=show_staged_modal1>
                        <ModalHeader><ModalTitle>"Sure?"</ModalTitle></ModalHeader>
                        <ModalBody>"This ia a test modal."</ModalBody>
                        <ModalFooter>
                            <ButtonWrapper>
                                <Button on_press=move |_| set_show_staged_modal2.set(true) color=ButtonColor::Info>"Next"</Button>
                                <Button on_press=move |_| set_show_staged_modal1.set(false) color=ButtonColor::Secondary>"Cancel"</Button>
                            </ButtonWrapper>
                        </ModalFooter>
                    </Modal>
            
                    <Modal show_when=show_staged_modal2>
                        <ModalHeader><ModalTitle>"Next one"</ModalTitle></ModalHeader>
                        <ModalBody>"This overlays..."</ModalBody>
                        <ModalFooter>
                            <ButtonWrapper>
                                <Button on_press=move |_| set_show_staged_modal2.set(false) color=ButtonColor::Secondary>"Back"</Button>
                            </ButtonWrapper>
                        </ModalFooter>
                    </Modal>            
                }
            "#)}
        </Code>

        <P><Button on_press=move |_| set_show_staged_modal1.set(true)>"Show staged modal"</Button></P>

        <Modal show_when=show_staged_modal1 on_escape=escape_staged_modal1>
            <ModalHeader><ModalTitle>"Sure?"</ModalTitle></ModalHeader>
            <ModalBody>"This ia a test modal."</ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_press=move |_| set_show_staged_modal2.set(true) color=ButtonColor::Info>"Next"</Button>
                    <Button on_press=move |_| set_show_staged_modal1.set(false) color=ButtonColor::Secondary>"Cancel"</Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>

        <Modal show_when=show_staged_modal2 on_escape=escape_staged_modal2>
            <ModalHeader><ModalTitle>"Next one"</ModalTitle></ModalHeader>
            <ModalBody>"This overlays..."</ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_press=move |_| set_show_staged_modal2.set(false) color=ButtonColor::Secondary>"Back"</Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>

        <H2>"React to user input"</H2>

        <P>"You can include arbitrary (reactive) children inside a modal. The next modal can only be accepted after entering \"ok\" in the presented input field."</P>

        <Code>
            {indoc!(r#"
                let (show_confirm_modal, set_show_confirm_modal) = create_signal(false);

                view! {
                    <P><Button on_press=move |_| set_show_confirm_modal.set(true)>"Show confirmation modal"</Button></P>
            
                    <ConfirmModal
                        show_when=show_confirm_modal
                        requires_confirmation_of="ok".to_owned()
                        on_accept=move || set_show_confirm_modal.set(false)
                        on_cancel=move || set_show_confirm_modal.set(false)
                    />  
                }
            "#)}
        </Code>

        <P>"Using:"</P>

        <Code>
            {indoc!(r#"
                #[component]
                pub fn ConfirmModal<A, C>(
                    #[prop(into)] show_when: Signal<bool>,
                    requires_confirmation_of: String,
                    on_accept: A,
                    on_cancel: C,
                ) -> impl IntoView
                where
                    A: Fn() + Copy + 'static,
                    C: Fn() + Copy + 'static,
                {
                    let required = store_value(requires_confirmation_of);
                
                    let (input, set_input) = create_signal(String::new());
                
                    let confirmed = move || required.with_value(|r| input.with(|i| r == i));
                    let disabled = Signal::derive(move || !confirmed());
                
                    let on_accept = move || {
                        set_input.update(|it| it.clear());
                        (on_accept)();
                    };
                    let on_cancel = move || {
                        set_input.update(|it| it.clear());
                        (on_cancel)();
                    };
                
                    view! {
                        <Modal show_when=show_when on_escape=move || (on_cancel)()>
                            <ModalHeader><ModalTitle>"Delete repository?"</ModalTitle></ModalHeader>
                            <ModalBody>
                                "Please enter \""{required.get_value()}"\" to confirm."
                                <TextInput get=input set=set_input/>
                            </ModalBody>
                            <ModalFooter>
                                <ButtonWrapper>
                                    <Button on_press=move |_| (on_accept)() disabled=disabled color=ButtonColor::Danger>"Confirm"</Button>
                                    <Button on_press=move |_| (on_cancel)() color=ButtonColor::Secondary>"Cancel"</Button>
                                </ButtonWrapper>
                            </ModalFooter>
                        </Modal>
                    }
                }
            "#)}
        </Code>

        <P><Button on_press=move |_| set_show_confirm_modal.set(true)>"Show confirmation modal"</Button></P>

        <ConfirmModal
            show_when=show_confirm_modal
            requires_confirmation_of="ok".to_owned()
            on_accept=move || set_show_confirm_modal.set(false)
            on_cancel=move || set_show_confirm_modal.set(false)
        />

        <H2>"Escape"</H2>

        <P>"Closing a modal through a press on " <KbdKey key=Key::Escape/> " or closing it through a click outside its rendered content can be considered \"commonly expected behavior\" of modals."</P>

        <P>
            "Modals may be used as critical gate-keepers. The API of them should minimize error-potential as much as possible. "
            "The author of a modal should therefore have full control, preferably at a centralized point, over all ways a modals display state may change."
            "As the modal component does neither owns it's display state nor knows what to do when the modal is shown or hidden "
            "(modals only know wether they should be rendered or not, reading their " <Code inline=true>"show_when"</Code> " prop) "
            "fully automated handling of escape keys is not possible."
        </P>

        <P>
            "Leptonic's" <Code inline=true>"Modal"</Code> " component will however automatically listen for an 'Escape' key press when the " <Code inline=true>"on_escape"</Code> " callback property is provided. "
            "This gives you explicit control over the effect this should have."
        </P>

        <P>
            "We snuck this in for all example modals (even the staged one). For our first example, it simply looks like this."
        </P>

        <Code>
            {indoc!(r#"
                <Modal show_when=show_simple_modal on_escape=move || set_show_simple_modal.set(false)>
                    <ModalHeader><ModalTitle>"Hello"</ModalTitle></ModalHeader>
                    <ModalBody>"This ia a simple modal."</ModalBody>
                    <ModalFooter>
                        <ButtonWrapper>
                            <Button on_press=move |_| set_show_simple_modal.set(false) color=ButtonColor::Secondary>"Cancel"</Button>
                        </ButtonWrapper>
                    </ModalFooter>
                </Modal>
            "#)} 
        </Code>

        <H2>"Styling"</H2>

        <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

        <Code>
            {indoc!(r"
                --modal-color
                --modal-background-color
                --modal-padding
                --modal-font-size
                --modal-header-padding
                --modal-body-padding
                --modal-footer-padding
                --modal-border-radius
                --modal-box-shadow
            ")}
        </Code>
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
    A: Fn() + Copy + 'static,
    C: Fn() + Copy + 'static,
{
    let required = store_value(requires_confirmation_of);

    let (input, set_input) = create_signal(String::new());

    let confirmed = move || required.with_value(|r| input.with(|i| r == i));
    let disabled = Signal::derive(move || !confirmed());

    let on_accept = move || {
        set_input.update(|it| it.clear());
        (on_accept)();
    };
    let on_cancel = move || {
        set_input.update(|it| it.clear());
        (on_cancel)();
    };

    view! {
        <Modal show_when=show_when on_escape=move || (on_cancel)()>
            <ModalHeader><ModalTitle>"Delete repository?"</ModalTitle></ModalHeader>
            <ModalBody>
                "Please enter \""{required.get_value()}"\" to confirm."
                <TextInput get=input set=set_input/>
            </ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_press=move |_| (on_accept)() disabled=disabled color=ButtonColor::Danger>"Confirm"</Button>
                    <Button on_press=move |_| (on_cancel)() color=ButtonColor::Secondary>"Cancel"</Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>
    }
}
