use leptos::prelude::*;
use leptos_tiptap::*;

use crate::utils::classes::Classes;
use crate::{
    components::{
        button::{Button, ButtonSize},
        icon::Icon,
    },
    Out,
};

#[component]
pub fn TiptapEditor(
    #[prop(into)] value: Signal<String>,
    #[prop(into, optional)] set_value: Option<Out<TiptapContent>>,
    #[prop(into, optional)] disabled: Signal<bool>,
) -> impl IntoView {
    let (msg, set_msg) = signal(TiptapInstanceMsg::Noop);

    let (selection_state, set_selection_state) = signal(TiptapSelectionState::default());

    let instance_id = uuid::Uuid::now_v7();

    view! {
        <leptonic-tiptap-editor>
            { move || match disabled.get() {
                false => view! {
                    <leptonic-tiptap-menu>
                        { move || selection_state.with(|state| view! {
                            <Button
                                classes=Classes::builder().with("leptonic-tiptap-btn").with(("active", state.h1)).build()
                                size=ButtonSize::Small
                                on_press=move |_| set_msg.set(TiptapInstanceMsg::H1)
                            >
                                "H1"
                            </Button>

                            <Button
                                classes=Classes::builder().with("leptonic-tiptap-btn").with(("active", state.h2)).build()
                                size=ButtonSize::Small
                                on_press=move |_| set_msg.set(TiptapInstanceMsg::H2)
                            >
                                "H2"
                            </Button>

                            <Button
                                classes=Classes::builder().with("leptonic-tiptap-btn").with(("active", state.h3)).build()
                                size=ButtonSize::Small
                                on_press=move |_| set_msg.set(TiptapInstanceMsg::H3)
                            >
                                "H3"
                            </Button>

                            <Button
                                classes=Classes::builder().with("leptonic-tiptap-btn").with(("active", state.h4)).build()
                                size=ButtonSize::Small
                                on_press=move |_| set_msg.set(TiptapInstanceMsg::H4)
                            >
                                "H4"
                            </Button>

                            <Button
                                classes=Classes::builder().with("leptonic-tiptap-btn").with(("active", state.h5)).build()
                                size=ButtonSize::Small
                                on_press=move |_| set_msg.set(TiptapInstanceMsg::H5)
                            >
                                "H5"
                            </Button>

                            <Button
                                classes=Classes::builder().with("leptonic-tiptap-btn").with(("active", state.h6)).build()
                                size=ButtonSize::Small
                                on_press=move |_| set_msg.set(TiptapInstanceMsg::H6)
                            >
                                "H6"
                            </Button>

                            <Button
                                classes=Classes::builder().with("leptonic-tiptap-btn").with(("active", state.paragraph)).build()
                                size=ButtonSize::Small
                                on_press=move |_| set_msg.set(TiptapInstanceMsg::Paragraph)
                            >
                                <Icon icon=icondata::BsParagraph/>
                                "Paragraph"
                            </Button>

                            <Button
                                classes=Classes::builder().with("leptonic-tiptap-btn").with(("active", state.bold)).build()
                                size=ButtonSize::Small
                                on_press=move |_| set_msg.set(TiptapInstanceMsg::Bold)
                            >
                                <Icon icon=icondata::BsTypeBold/>
                                "Bold"
                            </Button>

                            <Button
                                classes=Classes::builder().with("leptonic-tiptap-btn").with(("active", state.italic)).build()
                                size=ButtonSize::Small
                                on_press=move |_| set_msg.set(TiptapInstanceMsg::Italic)
                            >
                                <Icon icon=icondata::BsTypeItalic/>
                                "Italic"
                            </Button>

                            <Button
                                classes=Classes::builder().with("leptonic-tiptap-btn").with(("active", state.strike)).build()
                                size=ButtonSize::Small
                                on_press=move |_| set_msg.set(TiptapInstanceMsg::Strike)
                            >
                                <Icon icon=icondata::BsTypeStrikethrough/>
                                "Strike"
                            </Button>

                            <Button
                                classes=Classes::builder().with("leptonic-tiptap-btn").with(("active", state.blockquote)).build()
                                size=ButtonSize::Small
                                on_press=move |_| set_msg.set(TiptapInstanceMsg::Blockquote)
                            >
                                <Icon icon=icondata::BsBlockquoteLeft/>
                                "Blockquote"
                            </Button>

                            <Button
                                classes=Classes::builder().with("leptonic-tiptap-btn").with(("active", state.highlight)).build()
                                size=ButtonSize::Small
                                on_press=move |_| set_msg.set(TiptapInstanceMsg::Highlight)
                            >
                                <Icon icon=icondata::BsBrightnessAltHigh/>
                                "Highlight"
                            </Button>

                            <Button
                                classes=Classes::builder().with("leptonic-tiptap-btn").with(("active", state.align_left)).build()
                                size=ButtonSize::Small
                                on_press=move |_| set_msg.set(TiptapInstanceMsg::AlignLeft)
                            >
                                <Icon icon=icondata::BsTextLeft/>
                                "left"
                            </Button>

                            <Button
                                classes=Classes::builder().with("leptonic-tiptap-btn").with(("active", state.align_center)).build()
                                size=ButtonSize::Small
                                on_press=move |_| set_msg.set(TiptapInstanceMsg::AlignCenter)
                            >
                                <Icon icon=icondata::BsTextCenter/>
                                "center"
                            </Button>

                            <Button
                                classes=Classes::builder().with("leptonic-tiptap-btn").with(("active", state.align_right)).build()
                                size=ButtonSize::Small
                                on_press=move |_| set_msg.set(TiptapInstanceMsg::AlignRight)
                            >
                                <Icon icon=icondata::BsTextRight/>
                                "right"
                            </Button>

                            <Button
                                classes=Classes::builder().with("leptonic-tiptap-btn").with(("active", state.align_justify)).build()
                                size=ButtonSize::Small
                                on_press=move |_| set_msg.set(TiptapInstanceMsg::AlignJustify)
                            >
                                <Icon icon=icondata::BsJustify/>
                                "justify"
                            </Button>
                        }) }
                    </leptonic-tiptap-menu>
                }.into_any(),
                true => ().into_any(),
            } }
            <TiptapInstance
                id=instance_id.to_string()
                msg=msg
                disabled=disabled
                value=value
                set_value=move |v| {
                    if let Some(set_value) = &set_value {
                        set_value.set(v);
                    }
                }
                on_selection_change=move |state| set_selection_state.set(state)
            />
        </leptonic-tiptap-editor>
    }
}

/*

#[derive(Properties, PartialEq)]
pub struct Props {
    pub api_base_url: String,
    pub id: String,
    pub value: String,
    pub class: String,
    pub disabled: bool,
    pub onchange: Option<Callback<String>>,
}

pub struct CrudTipTapEditor {
    link: Option<Scope<TiptapInstance>>,
    choose_image: bool,
    selection_state: SelectionState,
}

choose_image: false,

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InstanceLinked(link) => {
                self.link = link;
                false
            }
            Msg::SelectionChanged(selection) => {
                self.selection_state = selection.state;
                false
            }
            Msg::ContentChanged(content) => {
                if let Some(onchange) = &ctx.props().onchange {
                    onchange.emit(content.content);
                }
                false
            }
            Msg::ChooseImage => {
                // Enables the chooser modal!
                self.choose_image = true;
                true
            }
            Msg::ChooseImageCanceled => {
                self.choose_image = false;
                true
            }
            Msg::ImageChosen(resource) => {
                self.choose_image = false;
                self.send_tiptap_msg(TiptapInstanceMsg::SetImage(resource));
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("tiptap-editor", ctx.props().disabled.then(|| "disabled"))}>

                <div class={"tiptap-menu"}>

                    <div class={"tiptap-btn"} onclick={ctx.link().callback(|_| Msg::ChooseImage)}>
                        <CrudIcon variant={Bi::Image}/>
                        {"image"}
                    </div>

                </div>

                // This is our TipTap instance!
                <TiptapInstance
                    id={ctx.props().id.clone()}
                    class={"tiptap-instance".to_owned()}
                    content={ctx.props().value.clone()}
                    disabled={ctx.props().disabled}
                    on_link={ctx.link().callback(|link: Option<Scope<TiptapInstance>>| Msg::InstanceLinked(link))}
                    on_selection_change={ctx.link().callback(Msg::SelectionChanged)}
                    on_content_change={ctx.link().callback(Msg::ContentChanged)}
                />

                {
                    match &self.choose_image {
                        true => html! {
                            <CrudModal>
                                <CrudImageChooserModal
                                    api_base_url={ctx.props().api_base_url.clone()}
                                    on_cancel={ctx.link().callback(|_| Msg::ChooseImageCanceled)}
                                    on_choose={ctx.link().callback(|res: FileResource| Msg::ImageChosen(ImageResource {
                                        title: res.name.clone(),
                                        alt: res.name,
                                        url: res.path,
                                    }))}
                                />
                            </CrudModal>
                        },
                        false => html! {}
                    }
                }
            </div>
        }
    }
}
 */
