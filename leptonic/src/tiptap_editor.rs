use leptos::*;
use leptos_icons::BsIcon;
use leptos_tiptap::*;

use crate::prelude::*;

#[component]
pub fn TiptapEditor(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] disabled: OptionalMaybeSignal<bool>,
    #[prop(into)] value: Signal<String>,
    #[prop(into, optional)] set_value: Option<Out<TiptapContent>>,
) -> impl IntoView {
    let (msg, set_msg) = create_signal(TiptapInstanceMsg::Noop);

    let (selection_state, set_selection_state) = create_signal(TiptapSelectionState::default());

    let instance_id = uuid::Uuid::now_v7();

    view! {
        <leptonic-tiptap-editor id=id class=class>
            { move || match disabled.get() {
                false => view! {
                    <leptonic-tiptap-menu>
                        { move || selection_state.with(|state| view! {
                            <Button class=MaybeSignal::from(format!("leptonic-tiptap-btn {}", if state.h1 { "active" } else { "" })) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::H1)>
                                "H1"
                            </Button>

                            <Button class=MaybeSignal::from(format!("leptonic-tiptap-btn {}", if state.h2 { "active" } else { "" })) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::H2)>
                                "H2"
                            </Button>

                            <Button class=MaybeSignal::from(format!("leptonic-tiptap-btn {}", if state.h3 { "active" } else { "" })) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::H3)>
                                "H3"
                            </Button>

                            <Button class=MaybeSignal::from(format!("leptonic-tiptap-btn {}", if state.h4 { "active" } else { "" })) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::H4)>
                                "H4"
                            </Button>

                            <Button class=MaybeSignal::from(format!("leptonic-tiptap-btn {}", if state.h5 { "active" } else { "" })) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::H5)>
                                "H5"
                            </Button>

                            <Button class=MaybeSignal::from(format!("leptonic-tiptap-btn {}", if state.h6 { "active" } else { "" })) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::H6)>
                                "H6"
                            </Button>

                            <Button class=MaybeSignal::from(format!("leptonic-tiptap-btn {}", if state.paragraph { "active" } else { "" })) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::Paragraph)>
                                <Icon icon=BsIcon::BsParagraph/>
                                "Paragraph"
                            </Button>

                            <Button class=MaybeSignal::from(format!("leptonic-tiptap-btn {}", if state.bold { "active" } else { "" })) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::Bold)>
                                <Icon icon=BsIcon::BsTypeBold/>
                                "Bold"
                            </Button>

                            <Button class=MaybeSignal::from(format!("leptonic-tiptap-btn {}", if state.italic { "active" } else { "" })) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::Italic)>
                                <Icon icon=BsIcon::BsTypeItalic/>
                                "Italic"
                            </Button>

                            <Button class=MaybeSignal::from(format!("leptonic-tiptap-btn {}", if state.strike { "active" } else { "" })) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::Strike)>
                                <Icon icon=BsIcon::BsTypeStrikethrough/>
                                "Strike"
                            </Button>

                            <Button class=MaybeSignal::from(format!("leptonic-tiptap-btn {}", if state.blockquote { "active" } else { "" })) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::Blockquote)>
                                <Icon icon=BsIcon::BsBlockquoteLeft/>
                                "Blockquote"
                            </Button>

                            <Button class=MaybeSignal::from(format!("leptonic-tiptap-btn {}", if state.highlight { "active" } else { "" })) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::Highlight)>
                                <Icon icon=BsIcon::BsBrightnessAltHigh/>
                                "Highlight"
                            </Button>

                            <Button class=MaybeSignal::from(format!("leptonic-tiptap-btn {}", if state.align_left { "active" } else { "" })) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::AlignLeft)>
                                <Icon icon=BsIcon::BsTextLeft/>
                                "left"
                            </Button>

                            <Button class=MaybeSignal::from(format!("leptonic-tiptap-btn {}", if state.align_center { "active" } else { "" })) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::AlignCenter)>
                                <Icon icon=BsIcon::BsTextCenter/>
                                "center"
                            </Button>

                            <Button class=MaybeSignal::from(format!("leptonic-tiptap-btn {}", if state.align_right { "active" } else { "" })) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::AlignRight)>
                                <Icon icon=BsIcon::BsTextRight/>
                                "right"
                            </Button>

                            <Button class=MaybeSignal::from(format!("leptonic-tiptap-btn {}", if state.align_justify { "active" } else { "" })) size=ButtonSize::Small on_click=move |_| set_msg.set(TiptapInstanceMsg::AlignJustify)>
                                <Icon icon=BsIcon::BsJustify/>
                                "justify"
                            </Button>
                        }) }
                    </leptonic-tiptap-menu>
                }.into_view(),
                true => ().into_view(),
            } }
            <TiptapInstance
                id=instance_id.to_string()
                msg=msg
                disabled=match disabled.0 {
                    Some(sig) => sig,
                    None => MaybeSignal::Static(false),
                }
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
