use leptos::prelude::*;

use crate::{
    components::{
        button::{Button, ButtonColor, ButtonWrapper},
        input::TextInput,
        modal::{Modal, ModalBody, ModalFooter, ModalHeader},
    },
    prelude::{GlobalKeyboardEvent, ViewProducer},
    utils::callback::ViewCallback,
};

#[component]
pub fn Quicksearch(
    #[prop(into)] trigger: ViewCallback<WriteSignal<bool>>,
    #[prop(into)] query: Callback<String, Vec<QuicksearchOption>>,
) -> impl IntoView {
    let (show_modal, set_show_modal) = signal(false);
    view! {
        <div class="leptonic-quicksearch">
            {trigger.render(set_show_modal)}
            <QuicksearchModal
                show_when=show_modal
                query=query
                on_cancel=move || set_show_modal.set(false)
            />
        </div>
    }
}

#[component]
pub fn QuicksearchTrigger(
    #[prop(into)] set_quicksearch: WriteSignal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class="leptonic-quicksearch-trigger"
            on:click=move |_| {
                set_quicksearch.set(true);
            }
        >
            {children()}
        </div>
    }
}

#[derive(Debug, Clone)]
pub struct QuicksearchOption {
    pub label: Oco<'static, str>,
    pub view: ViewProducer,
    pub on_select: Callback<(), ()>,
}

#[component]
fn QuicksearchModal(
    #[prop(into)] show_when: Signal<bool>,
    #[prop(into)] query: Callback<String, Vec<QuicksearchOption>>,
    #[prop(into)] on_cancel: Callback<(), ()>,
) -> impl IntoView {
    let (input, set_input) = signal(String::new());

    let options = move || query.run(input.get());

    let g_keyboard_event: GlobalKeyboardEvent = expect_context::<GlobalKeyboardEvent>();
    Effect::new(move |_old| {
        if let Some(e) = g_keyboard_event.read_signal.get() {
            if show_when.get_untracked() && e.key().as_str() == "Escape" {
                on_cancel.run(());
            }
        }
    });

    let cancel = Callback::new(move |_| on_cancel.run(()));

    view! {
        <Modal show_when=show_when on_escape=move || on_cancel.run(()) classes="quicksearch-modal">
            <ModalHeader>
                <TextInput
                    get=input
                    set=set_input
                    placeholder=Oco::Borrowed("Search")
                    should_be_focused=show_when
                    attr:class="search-input"
                />
            </ModalHeader>
            <ModalBody attr:style="overflow: auto;">
                <div class="leptonic-quicksearch-results">
                    {move || {
                        options()
                            .into_iter()
                            .map(|option| {
                                view! {
                                    <div
                                        class="leptonic-quicksearch-result"
                                        on:click=move |_| {
                                            option.on_select.run(());
                                            on_cancel.run(());
                                        }
                                    >
                                        {option.view.produce()}
                                    </div>
                                }
                            })
                            .collect_view()
                    }}
                </div>
            </ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_press=cancel color=ButtonColor::Secondary>
                        "Cancel"
                    </Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>
    }
}
