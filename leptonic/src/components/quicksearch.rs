use crate::{
    components::{
        button::{Button, ButtonColor, ButtonWrapper},
        input::TextInput,
        modal::{Modal, ModalBody, ModalFooter, ModalHeader},
    },
    prelude::{Consumer, GlobalKeyboardEvent, Producer, ViewProducer},
    utils::callback::ViewCallback,
};
use leptos::*;

#[component]
pub fn Quicksearch(
    #[prop(into)] trigger: ViewCallback<WriteSignal<bool>>,
    #[prop(into)] query: Consumer<String, Vec<QuicksearchOption>>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView {
    let (show_modal, set_show_modal) = create_signal(false);
    view! {
        <leptonic-quicksearch id=id class=class style=style>
            { trigger.render(set_show_modal) }
            <QuicksearchModal
                show_when=show_modal
                query=query
                on_cancel=move || set_show_modal.set(false)
            />
        </leptonic-quicksearch>
    }
}

#[component]
pub fn QuicksearchTrigger(
    #[prop(into)] set_quicksearch: WriteSignal<bool>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! {
        <leptonic-quicksearch-trigger id=id class=class style=style on:click=move |_| set_quicksearch.set(true)>
            { children() }
        </leptonic-quicksearch-trigger>
    }
}

#[derive(Debug, Clone)]
pub struct QuicksearchOption {
    pub label: Oco<'static, str>,
    pub view: ViewProducer,
    pub on_select: Producer<()>,
}

#[component]
fn QuicksearchModal(
    #[prop(into)] show_when: Signal<bool>,
    #[prop(into)] query: Consumer<String, Vec<QuicksearchOption>>,
    #[prop(into)] on_cancel: Producer<()>,
) -> impl IntoView {
    let (input, set_input) = create_signal(String::new());

    let options = move || query.consume(input.get());

    let g_keyboard_event: GlobalKeyboardEvent = expect_context::<GlobalKeyboardEvent>();
    create_effect(move |_old| {
        if let Some(e) = g_keyboard_event.read_signal.get() {
            if show_when.get_untracked() && e.key().as_str() == "Escape" {
                on_cancel.produce();
            }
        }
    });

    let cancel = Callback::new(move |_| on_cancel.produce());

    view! {
        <Modal show_when=show_when on_escape=move || on_cancel.produce() class="quicksearch-modal">
            <ModalHeader>
                <TextInput
                    get=input
                    set=set_input
                    placeholder="Search"
                    class="search-input"
                    should_be_focused=show_when
                    prepend=().into_view()
                />
            </ModalHeader>
            <ModalBody style="overflow: auto;">
                <leptonic-quicksearch-results>
                    { move || options().into_iter().map(|option| view! {
                        <leptonic-quicksearch-result on:click=move |_| {
                                option.on_select.produce();
                                on_cancel.produce();
                            }>
                            { option.view.produce() }
                        </leptonic-quicksearch-result>
                    }).collect_view() }
                </leptonic-quicksearch-results>
            </ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_press=cancel color=ButtonColor::Secondary>"Cancel"</Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>
    }
}
