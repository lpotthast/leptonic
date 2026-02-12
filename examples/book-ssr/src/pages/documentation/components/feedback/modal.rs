use indoc::indoc;
use leptonic::{components::prelude::*, utils::key::Key};
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageModal() -> impl IntoView {
    let (show_simple_modal, set_show_simple_modal) = signal(false);

    let (show_staged_modal1, set_show_staged_modal1) = signal(false);
    let (show_staged_modal2, set_show_staged_modal2) = signal(false);

    let escape_staged_modal1 = Callback::new(move |()| set_show_staged_modal1.set(false));
    let escape_staged_modal2 = Callback::new(move |()| {
        set_show_staged_modal2.set(false);
        set_show_staged_modal1.set(true);
    });

    let (show_confirm_modal, set_show_confirm_modal) = signal(false);

    view! {
        <Article>
            <h1 id="modal" class="anchor">
                "Modal"
                <AnchorLink href="#modal" description="Direct link to article header"/>
            </h1>

            <p>"Create modals to ask critical questions or notify the user about an upcoming event which must be acknowledged in some way."</p>

            <Code>
                {indoc!(r#"
                    let (show_simple_modal, set_show_simple_modal) = signal(false);

                    view! {
                        <p><Button on_press=move |_| set_show_simple_modal.set(true)>"Show simple modal"</Button></p>
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

            <p><Button on_press=move |_| set_show_simple_modal.set(true)>"Show simple modal"</Button></p>

            <Modal
                show_when=show_simple_modal
                on_escape=move || set_show_simple_modal.set(false)
                on_backdrop_interaction=move || set_show_simple_modal.set(false)
            >
                <ModalHeader><ModalTitle>"Hello"</ModalTitle></ModalHeader>
                <ModalBody>"This ia a simple modal."</ModalBody>
                <ModalFooter>
                    <ButtonWrapper>
                        <Button on_press=move |_| set_show_simple_modal.set(false) color=ButtonColor::Secondary>"Cancel"</Button>
                    </ButtonWrapper>
                </ModalFooter>
            </Modal>

            <h2 id="stages" class="anchor">
                "Stages"
                <AnchorLink href="#stages" description="Direct link to section: Stages"/>
            </h2>

            <p>"You can connect multiple modals by setting their visibility props appropriately."</p>

            <Code>
                {indoc!(r#"
                    let (show_staged_modal1, set_show_staged_modal1) = signal(false);
                    let (show_staged_modal2, set_show_staged_modal2) = signal(false);

                    view! {
                        <p><Button on_press=move |_| set_show_staged_modal1.set(true)>"Show staged modal"</Button></p>

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

            <p><Button on_press=move |_| set_show_staged_modal1.set(true)>"Show staged modal"</Button></p>

            <Modal
                show_when=show_staged_modal1
                on_escape=escape_staged_modal1
                on_backdrop_interaction=escape_staged_modal1
            >
                <ModalHeader><ModalTitle>"Sure?"</ModalTitle></ModalHeader>
                <ModalBody>"This ia a test modal."</ModalBody>
                <ModalFooter>
                    <ButtonWrapper>
                        <Button on_press=move |_| set_show_staged_modal2.set(true) color=ButtonColor::Info>"Next"</Button>
                        <Button on_press=move |_| set_show_staged_modal1.set(false) color=ButtonColor::Secondary>"Cancel"</Button>
                    </ButtonWrapper>
                </ModalFooter>
            </Modal>

            <Modal
                show_when=show_staged_modal2
                on_escape=escape_staged_modal2
                on_backdrop_interaction=escape_staged_modal2
            >
                <ModalHeader><ModalTitle>"Next one"</ModalTitle></ModalHeader>
                <ModalBody>"This overlays..."</ModalBody>
                <ModalFooter>
                    <ButtonWrapper>
                        <Button on_press=move |_| set_show_staged_modal2.set(false) color=ButtonColor::Secondary>"Back"</Button>
                    </ButtonWrapper>
                </ModalFooter>
            </Modal>

            <h2 id="react-to-user-input" class="anchor">
                "React to user input"
                <AnchorLink href="#react-to-user-input" description="Direct link to section: React to user input"/>
            </h2>

            <p>"You can include arbitrary (reactive) children inside a modal. The next modal can only be accepted after entering \"ok\" in the presented input field."</p>

            <Code>
                {indoc!(r#"
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
                "#)}
            </Code>

            <p>"Using:"</p>

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
                        let required = StoredValue::new(requires_confirmation_of);

                        let (input, set_input) = signal(String::new());

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

            <p><Button on_press=move |_| set_show_confirm_modal.set(true)>"Show confirmation modal"</Button></p>

            <ConfirmModal
                show_when=show_confirm_modal
                requires_confirmation_of="ok".to_owned()
                on_accept=move || set_show_confirm_modal.set(false)
                on_cancel=move || set_show_confirm_modal.set(false)
            />

            <h2 id="escape" class="anchor">
                "Handling escape and backdrop interactions "
                <AnchorLink href="#escape" description="Direct link to section: Escape and backdrop interactions"/>
            </h2>

            <p>"Closing a modal through a press on " <KbdKey key=Key::Escape/> " or closing it through a click outside its rendered content can be considered \"commonly expected behavior\" of modals."</p>

            <p>
                "Modals may be used as critical gate-keepers. The API of them should minimize error-potential as much as possible. "
                "The author of a modal should therefore have full control, preferably at a centralized point, over all ways a modals display state may change."
                "As the modal component does neither owns it's display state nor knows what to do when the modal is shown or hidden "
                "(modals only know whether they should be rendered or not, reading their " <Code inline=true>"show_when"</Code> " prop) "
                "fully automated handling of escape keys is not possible."
            </p>

            <p>
                "Leptonic's" <Code inline=true>"Modal"</Code> " component will however automatically listen for an 'Escape' key press when the " <Code inline=true>"on_escape"</Code> " callback property is provided. "
                "This gives you explicit control over the effect this should have."
            </p>

            <p>
                "The " <Code inline=true>"Modal"</Code> "'s " <Code inline=true>"on_backdrop_interaction"</Code> " callback property allows you to handle user interactions with the backdrop. "
                "In the current implementation, pressing the backdrop triggers the callback. "
                "This gives you explicit control over the effect this should have."
            </p>

            <p>
                "We snuck this in for all example modals (even the staged one). For our first example, it simply looks like this."
            </p>

            <Code>
                {indoc!(r#"
                    <Modal
                        show_when=show_simple_modal
                        on_escape=move || set_show_simple_modal.set(false)
                        on_backdrop_interaction=move || set_show_simple_modal.set(false)
                    >
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

            <h2 id="styling" class="anchor">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </h2>

            <p>"You may overwrite any of the following CSS variables to meet your styling needs."</p>

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
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Modal", link: "#modal" },
                Toc::Leaf { title: "Stages", link: "#stages" },
                Toc::Leaf { title: "React to user input", link: "#react-to-user-input" },
                Toc::Leaf { title: "Escape", link: "#escape" },
                Toc::Leaf { title: "Styling", link: "#styling" },
            ]
        }/>
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
        set_input.update(|it| it.clear());
        on_accept();
    };
    let on_cancel = move || {
        set_input.update(|it| it.clear());
        on_cancel();
    };

    view! {
        <Modal show_when=show_when on_escape=move || on_cancel()>
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
