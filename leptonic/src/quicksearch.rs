use crate::prelude::*;
use leptos::*;

#[component]
pub fn Quicksearch(
    #[prop(into)] trigger: ViewCallback<WriteSignal<bool>>,
    #[prop(into)] query: Callback<String, Vec<QuicksearchOption>>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView {
    let (show_modal, set_show_modal) = create_signal(false);
    view! {
        <leptonic-quicksearch id=id class=class style=style>
            { trigger.call(set_show_modal) }
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
    #[prop(into)] query: Callback<String, Vec<QuicksearchOption>>,
    #[prop(into)] on_cancel: Producer<()>,
) -> impl IntoView {
    let on_cancel = StoredValue::new(on_cancel);

    let (input, set_input) = create_signal(String::new());

    let options = move || query.call(input.get());

    let g_keyboard_event: GlobalKeyboardEvent = expect_context::<GlobalKeyboardEvent>();
    create_effect(move |_old| {
        if let Some(e) = g_keyboard_event.read_signal.get() {
            if show_when.get_untracked() && e.key().as_str() == "Escape" {
                on_cancel.get_value().call(());
            }
        }
    });

    view! {
        <Modal show_when=show_when class="quicksearch-modal">
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
                                option.on_select.call(());
                                on_cancel.get_value().call(());
                            }>
                            { option.view.call(()) }
                        </leptonic-quicksearch-result>
                    }).collect_view() }
                </leptonic-quicksearch-results>
            </ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_click=move |_| on_cancel.get_value().call(()) color=ButtonColor::Secondary>"Cancel"</Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>
    }
}
