use indoc::indoc;
use leptonic::{components::prelude::*, utils::key::Key};
use leptos::prelude::*;

use super::demos::modal_confirm::ModalConfirmDemo;
use super::demos::modal_simple::ModalSimpleDemo;
use super::demos::modal_staged::ModalStagedDemo;
use crate::pages::documentation::demo_shell::DemoShell;
use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageModal() -> impl IntoView {
    view! {
        <Article>
            <h1 id="modal" class="anchor">
                "Modal"
                <AnchorLink href="#modal" description="Direct link to article header"/>
            </h1>

            <p>"Create modals to ask critical questions or notify the user about an upcoming event which must be acknowledged in some way."</p>

            <DemoShell source=include_str!("demos/modal_simple.rs")>
                <ModalSimpleDemo />
            </DemoShell>

            <h2 id="stages" class="anchor">
                "Stages"
                <AnchorLink href="#stages" description="Direct link to section: Stages"/>
            </h2>

            <p>"You can connect multiple modals by setting their visibility props appropriately."</p>

            <DemoShell source=include_str!("demos/modal_staged.rs")>
                <ModalStagedDemo />
            </DemoShell>

            <h2 id="react-to-user-input" class="anchor">
                "React to user input"
                <AnchorLink href="#react-to-user-input" description="Direct link to section: React to user input"/>
            </h2>

            <p>"You can include arbitrary (reactive) children inside a modal. The next modal can only be accepted after entering \"ok\" in the presented input field."</p>

            <DemoShell source=include_str!("demos/modal_confirm.rs")>
                <ModalConfirmDemo />
            </DemoShell>

            <h2 id="dismiss" class="anchor">
                "Dismissing modals"
                <AnchorLink href="#dismiss" description="Direct link to section: Dismissing modals"/>
            </h2>

            <p>"Closing a modal through a press on " <KbdKey key=Key::Escape/> " or closing it through a click outside its rendered content can be considered \"commonly expected behavior\" of modals."</p>

            <p>
                "The " <Code inline=true>"Modal"</Code> " component accepts an " <Code inline=true>"on_close"</Code> " callback that fires when the user presses Escape or clicks outside the modal (when " <Code inline=true>"is_dismissable"</Code> " is true, the default). "
                "Set " <Code inline=true>"is_dismissable=false"</Code> " and " <Code inline=true>"is_keyboard_dismiss_disabled=true"</Code> " to prevent any automatic dismissal."
            </p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    <Modal
                        show_when=show_simple_modal
                        on_close=move |()| set_show_simple_modal.set(false)
                    >
                        <ModalHeader><ModalTitle>"Hello"</ModalTitle></ModalHeader>
                        <ModalBody>"This is a simple modal."</ModalBody>
                        <ModalFooter>
                            <ButtonWrapper>
                                <Button on_press=move |_| set_show_simple_modal.set(false) color=ButtonColor::Secondary>"Cancel"</Button>
                            </ButtonWrapper>
                        </ModalFooter>
                    </Modal>
                "#)}
            </Code>

            <h2 id="accessibility" class="anchor">
                "Accessibility"
                <AnchorLink href="#accessibility" description="Direct link to section: Accessibility"/>
            </h2>

            <p>"The Modal component provides full accessibility out of the box:"</p>
            <ul>
                <li><Code inline=true>"role=\"dialog\""</Code>" with " <Code inline=true>"aria-modal=\"true\""</Code></li>
                <li>"Focus is trapped inside the modal and restored on close"</li>
                <li>"Use the " <Code inline=true>"title"</Code> " prop for automatic " <Code inline=true>"aria-labelledby"</Code></li>
                <li>"Set " <Code inline=true>"role=DialogRole::AlertDialog"</Code> " for critical prompts"</li>
            </ul>

            <h2 id="styling" class="anchor">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </h2>

            <p>"You may overwrite any of the following CSS variables to meet your styling needs."</p>

            <Code language=Language::Rust>
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
                Toc::Leaf { title: "Dismissing modals", link: "#dismiss" },
                Toc::Leaf { title: "Accessibility", link: "#accessibility" },
                Toc::Leaf { title: "Styling", link: "#styling" },
            ]
        }/>
    }
}
