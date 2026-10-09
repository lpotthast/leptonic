use leptonic::{
    CapturedElement,
    atoms::{
        button::Button,
        dialog::{Dialog, DialogTrigger},
        field::Label,
        input::Input,
        menu::{Menu, MenuItems, MenuTrigger},
        modal::{ModalBackdrop, ModalContent},
        popover::Popover,
        text_field::TextField,
    },
    hooks::{
        collections::{Key, use_collection},
        overlay::InteractOutsideFilter,
    },
};
use leptos::{prelude::*, web_sys};
use send_wrapper::SendWrapper;

use crate::pages::Section;

/// Overlays with their own open state, and their animation callbacks (react-aria-components'
/// `Popover.test.js`, `Dialog.test.js` and react-aria's `useModalOverlay.test.js` setups), one per
/// section. Every open state change and callback is appended to `#test-os-log` (`open:true`,
/// `open:false`, `<part>-enter`, `<part>-exit`).
/// - `popover-override`, `modal-override`: a popover and a dismissable modal open by their own
///   `is_open` inside a `DialogTrigger` (`#test-os-<section>-trigger`).
/// - `standalone`: a popover open by its own `is_open` at an anchor (`#test-os-anchor`), outside
///   any `DialogTrigger`.
/// - `popover-in-modal`: `#test-os-open-modal` opens a dismissable modal with a `DialogTrigger`
///   (`#test-os-popover-trigger`) of a popover inside.
/// - `modal-filter-close`, `modal-filter-keep`: open dismissable modals over a page-covering
///   backdrop (`.test-os-backdrop`) whose filter closes them on outside presses on the backdrop, or
///   never.
/// - `enter-exit`: `#test-os-open-animated` opens a modal whose backdrop (`backdrop`) and content
///   (`modal`) log `on_enter`/`on_exit`; the content's `on_exit` starts a 300ms animation
///   (`.test-os-slow-exit`), which keeps the modal rendered.
/// - `menu-modal`: the menu of `#test-os-menu-trigger` ("Add account", "Sign out") opens, by "Add
///   account", a modal with a sign-up form whose email field (the first input) has `auto_focus`.
#[component]
pub fn PageAtomOverlayState() -> impl IntoView {
    let log = RwSignal::new(Vec::<String>::new());
    let record = move |entry: &str| log.update(|log| log.push(entry.to_owned()));
    let on_open_change = Callback::new(move |open: bool| record(&format!("open:{open}")));
    let anchor = CapturedElement::new();
    let modal_open = RwSignal::new(false);
    let animated_open = RwSignal::new(false);
    let sign_up_open = RwSignal::new(false);
    let account_actions = use_collection(|b| {
        b.item("add", "Add account");
        b.item("sign-out", "Sign out");
    });
    let logged = move |entry: &'static str| {
        Callback::new(move |_: SendWrapper<web_sys::Element>| record(entry))
    };
    let slow_exit = Callback::new(move |element: SendWrapper<web_sys::Element>| {
        record("modal-exit");
        let _ = element.class_list().add_1("test-os-slow-exit");
    });

    view! {
        <h1>"Overlay state"</h1>
        <style>
            ".test-os-backdrop { position: fixed; inset: 0; }
            .test-os-slow-exit { animation: test-os-fade 300ms; }
            @keyframes test-os-fade { from { opacity: 1; } to { opacity: 0; } }"
        </style>
        <Section name="popover-override">
            <DialogTrigger>
                <Button id="test-os-popover-override-trigger">"Trigger"</Button>
                <Popover is_open=true on_open_change=on_open_change>
                    <Dialog aria_label="Popover">"A popover"</Dialog>
                </Popover>
            </DialogTrigger>
        </Section>
        <Section name="modal-override">
            <DialogTrigger>
                <Button id="test-os-modal-override-trigger">"Trigger"</Button>
                <ModalBackdrop
                    is_dismissable=true
                    is_open=true
                    on_open_change=on_open_change
                    classes="test-os-backdrop"
                >
                    <ModalContent>
                        <Dialog aria_label="Modal">"A modal"</Dialog>
                    </ModalContent>
                </ModalBackdrop>
            </DialogTrigger>
        </Section>
        <Section name="standalone">
            <button id="test-os-anchor" {..anchor.attr()}>
                "Anchor"
            </button>
            <Popover trigger=anchor is_open=true on_open_change=on_open_change>
                <Dialog aria_label="Standalone">"A standalone popover"</Dialog>
            </Popover>
        </Section>
        <Section name="popover-in-modal">
            <button id="test-os-open-modal" on:click=move |_| modal_open.set(true)>
                "Open modal"
            </button>
            <ModalBackdrop
                is_open=modal_open
                set_open=modal_open
                is_dismissable=true
                classes="test-os-backdrop"
            >
                <ModalContent>
                    <Dialog aria_label="Outer">
                        <p id="test-os-modal-text">"In the modal"</p>
                        <DialogTrigger>
                            <Button id="test-os-popover-trigger">"Popover"</Button>
                            <Popover>
                                <Dialog aria_label="Inner">"In the popover"</Dialog>
                            </Popover>
                        </DialogTrigger>
                    </Dialog>
                </ModalContent>
            </ModalBackdrop>
        </Section>
        <Section name="modal-filter-close">
            <ModalBackdrop
                default_open=true
                on_open_change=on_open_change
                is_dismissable=true
                should_close_on_interact_outside=InteractOutsideFilter::new(|target| {
                    target.class_list().contains("test-os-backdrop")
                })
                classes="test-os-backdrop"
            >
                <ModalContent>
                    <Dialog aria_label="Closing">"Closes"</Dialog>
                </ModalContent>
            </ModalBackdrop>
        </Section>
        <Section name="modal-filter-keep">
            <ModalBackdrop
                is_open=true
                on_open_change=on_open_change
                is_dismissable=true
                should_close_on_interact_outside=InteractOutsideFilter::new(|target| {
                    !target.class_list().contains("test-os-backdrop")
                })
                classes="test-os-backdrop"
            >
                <ModalContent>
                    <Dialog aria_label="Keeping">"Stays"</Dialog>
                </ModalContent>
            </ModalBackdrop>
        </Section>
        <Section name="enter-exit">
            <button id="test-os-open-animated" on:click=move |_| animated_open.set(true)>
                "Open animated"
            </button>
            <ModalBackdrop
                is_open=animated_open
                set_open=animated_open
                is_dismissable=true
                on_enter=logged("backdrop-enter")
                on_exit=logged("backdrop-exit")
                classes="test-os-backdrop"
            >
                <ModalContent
                    on_enter=logged("modal-enter")
                    on_exit=slow_exit
                    classes="test-os-animated"
                >
                    <Dialog aria_label="Animated">"Animated"</Dialog>
                </ModalContent>
            </ModalBackdrop>
        </Section>
        <Section name="menu-modal">
            <MenuTrigger>
                <Button id="test-os-menu-trigger">"Open menu"</Button>
                <Popover>
                    <Menu
                        collection=account_actions
                        on_action=move |key: Key| {
                            if key == Key::from("add") {
                                sign_up_open.set(true);
                            }
                        }
                    >
                        <MenuItems let:node>{node.text_value.to_string()}</MenuItems>
                    </Menu>
                </Popover>
            </MenuTrigger>
            <ModalBackdrop is_open=sign_up_open set_open=sign_up_open is_dismissable=true>
                <ModalContent>
                    <Dialog aria_label="Sign up">
                        <form>
                            <TextField auto_focus=true>
                                <Label>"Email"</Label>
                                <Input />
                            </TextField>
                            <TextField>
                                <Label>"Password"</Label>
                                <Input />
                            </TextField>
                        </form>
                    </Dialog>
                </ModalContent>
            </ModalBackdrop>
        </Section>
        <p>"Log: " <span id="test-os-log">{move || log.get().join(",")}</span></p>
    }
}
