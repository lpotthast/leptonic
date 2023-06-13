use leptos::*;
use leptos_tiptap::*;
use uuid::Uuid;

#[component]
pub fn TiptapEditor<C>(
    cx: Scope,
    value: ReadSignal<String>,
    set_value: C,
) -> impl IntoView
where
    C: Fn(TiptapContent) + 'static,
{
    let (msg, set_msg) = create_signal(cx, TiptapInstanceMsg::Noop);

    let (selection, set_selection) = create_signal(cx, TiptapSelectionState::default());

    let id = Uuid::new_v4();

    view! { cx,
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::H1)>"H1"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::H2)>"H2"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::H3)>"H3"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::H4)>"H4"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::H5)>"H5"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::H6)>"H6"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::Paragraph)>"Paragraph"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::Bold)>"Bold"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::Italic)>"Italic"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::Strike)>"Strike"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::Blockquote)>"Blockquote"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::Highlight)>"Highlight"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::AlignLeft)>"AlignLeft"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::AlignCenter)>"AlignCenter"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::AlignRight)>"AlignRight"</button>
        <button on:click=move |_| set_msg.set(TiptapInstanceMsg::AlignJustify)>"AlignJustify"</button>

        <TiptapInstance
            id=id.to_string()
            msg=msg
            disabled=false
            value=value
            set_value=set_value
            on_selection_change=move |state| set_selection.set(state)
            style="display: block; width: auto; height: auto; border: 1px solid; padding: 0.5em;"
        />
    }
}
