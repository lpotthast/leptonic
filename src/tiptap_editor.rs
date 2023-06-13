use leptos::*;
use leptos_tiptap::*;
use uuid::Uuid;

use crate::prelude::*;

#[component]
pub fn TiptapEditor<C>(cx: Scope, value: ReadSignal<String>, set_value: C) -> impl IntoView
where
    C: Fn(TiptapContent) + 'static,
{
    let (msg, set_msg) = create_signal(cx, TiptapInstanceMsg::Noop);

    let (selection, set_selection) = create_signal(cx, TiptapSelectionState::default());

    let id = Uuid::new_v4();

    view! { cx,
        <leptonic-tiptap-editor>
            <leptonic-tiptap-menu>
                <Button class=MaybeSignal::from("leptonic-tiptap-btn".to_owned()) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::H1)>"H1"</Button>
                <Button class=MaybeSignal::from("leptonic-tiptap-btn".to_owned()) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::H2)>"H2"</Button>
                <Button class=MaybeSignal::from("leptonic-tiptap-btn".to_owned()) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::H3)>"H3"</Button>
                <Button class=MaybeSignal::from("leptonic-tiptap-btn".to_owned()) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::H4)>"H4"</Button>
                <Button class=MaybeSignal::from("leptonic-tiptap-btn".to_owned()) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::H5)>"H5"</Button>
                <Button class=MaybeSignal::from("leptonic-tiptap-btn".to_owned()) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::H6)>"H6"</Button>
                <Button class=MaybeSignal::from("leptonic-tiptap-btn".to_owned()) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::Paragraph)>"Paragraph"</Button>
                <Button class=MaybeSignal::from("leptonic-tiptap-btn".to_owned()) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::Bold)>"Bold"</Button>
                <Button class=MaybeSignal::from("leptonic-tiptap-btn".to_owned()) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::Italic)>"Italic"</Button>
                <Button class=MaybeSignal::from("leptonic-tiptap-btn".to_owned()) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::Strike)>"Strike"</Button>
                <Button class=MaybeSignal::from("leptonic-tiptap-btn".to_owned()) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::Blockquote)>"Blockquote"</Button>
                <Button class=MaybeSignal::from("leptonic-tiptap-btn".to_owned()) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::Highlight)>"Highlight"</Button>
                <Button class=MaybeSignal::from("leptonic-tiptap-btn".to_owned()) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::AlignLeft)>"AlignLeft"</Button>
                <Button class=MaybeSignal::from("leptonic-tiptap-btn".to_owned()) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::AlignCenter)>"AlignCenter"</Button>
                <Button class=MaybeSignal::from("leptonic-tiptap-btn".to_owned()) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::AlignRight)>"AlignRight"</Button>
                <Button class=MaybeSignal::from("leptonic-tiptap-btn".to_owned()) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::AlignJustify)>"AlignJustify"</Button>
            </leptonic-tiptap-menu>

            <TiptapInstance
                id=id.to_string()
                msg=msg
                disabled=false
                value=value
                set_value=set_value
                on_selection_change=move |state| set_selection.set(state)
            />
        </leptonic-tiptap-editor>
    }
}
