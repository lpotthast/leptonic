use crate::prelude::*;
use leptos::*;

#[component]
pub fn Quicksearch<T, IV>(
    cx: Scope,
    trigger: T,
    #[prop(into)] query: Callback<String, Vec<QuicksearchOption>>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
) -> impl IntoView
where
    T: Fn(Scope, WriteSignal<bool>) -> IV + 'static,
    IV: IntoView + 'static,
{
    let (show_modal, set_show_modal) = create_signal(cx, false);
    view! { cx,
        <leptonic-quicksearch id=id class=class style=style>
            { trigger(cx, set_show_modal) }
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
    cx: Scope,
    #[prop(into)] set_quicksearch: WriteSignal<bool>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! { cx,
        <leptonic-quicksearch-trigger id=id class=class style=style on:click=move |_| set_quicksearch.set(true)>
            { children(cx) }
        </leptonic-quicksearch-trigger>
    }
}

#[derive(Debug, Clone)]
pub struct QuicksearchOption {
    pub view: Callback<Scope, View>,
    pub on_select: Callback<()>,
}

#[component]
fn QuicksearchModal<C>(
    cx: Scope,
    #[prop(into)] show_when: Signal<bool>,
    #[prop(into)] query: Callback<String, Vec<QuicksearchOption>>,
    on_cancel: C,
) -> impl IntoView
where
    C: Fn() + Copy + 'static,
{
    let (input, set_input) = create_signal(cx, "".to_owned());

    let options = move || query.call(input.get());

    let g_keyboard_event: GlobalKeyboardEvent = expect_context::<GlobalKeyboardEvent>(cx);
    create_effect(cx, move |_old| {
        if let Some(e) = g_keyboard_event.read_signal.get() {
            if show_when.get_untracked() && e.key().as_str() == "Escape" {
                on_cancel();
            }
        }
    });

    view! { cx,
        <Modal show_when=show_when class="quicksearch-modal">
            <ModalHeader>
                <Input
                    get=input
                    set=set_input
                    label="Search"
                    class="search-input"
                    should_be_focused=show_when
                    prepend=view! {cx, ""}.into_view(cx)
                />
            </ModalHeader>
            <ModalBody>
                <leptonic-quicksearch-results>
                    { move || options().into_iter().map(|option| view! {cx,
                        <leptonic-quicksearch-result on:click=move |_| {
                                option.on_select.call(());
                                on_cancel();
                            }>
                            { option.view.call(cx) }
                        </leptonic-quicksearch-result>
                    }).collect_view(cx) }
                </leptonic-quicksearch-results>
            </ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_click=move |_| (on_cancel)() color=ButtonColor::Secondary>"Cancel"</Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>
    }
}
