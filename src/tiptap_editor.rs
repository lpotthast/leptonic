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

/*

use yew::{html::Scope, prelude::*};
use yew_bootstrap_icons::v1_10_3::Bi;
use yew_tiptap::{
    tiptap_instance::{Content, Selection, SelectionState, TiptapInstance},
    ImageResource,
};

// TODO: Build leptos-tiptap

type TiptapInstanceMsg = <TiptapInstance as Component>::Message;

use crate::prelude::*;

pub enum Msg {
    InstanceLinked(Option<Scope<TiptapInstance>>),
    SelectionChanged(Selection),
    ContentChanged(Content),
    H1,
    H2,
    H3,
    Paragraph,
    Bold,
    Italic,
    Strike,
    Blockquote,
    Highlight,
    AlignLeft,
    AlignCenter,
    AlignRight,
    AlignJustify,
    ChooseImage,
    ChooseImageCanceled,
    ImageChosen(ImageResource),
}

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

impl CrudTipTapEditor {
    fn send_tiptap_msg(&self, msg: TiptapInstanceMsg) -> bool {
        if let Some(link) = &self.link {
            link.send_message(msg);
            true
        } else {
            false
        }
    }
}

impl Component for CrudTipTapEditor {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            link: None,
            choose_image: false,
            selection_state: Default::default(),
        }
    }

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
            Msg::H1 => {
                self.send_tiptap_msg(TiptapInstanceMsg::H1);
                false
            }
            Msg::H2 => {
                self.send_tiptap_msg(TiptapInstanceMsg::H2);
                false
            }
            Msg::H3 => {
                self.send_tiptap_msg(TiptapInstanceMsg::H3);
                false
            }
            Msg::Paragraph => {
                self.send_tiptap_msg(TiptapInstanceMsg::Paragraph);
                false
            }
            Msg::Bold => {
                self.send_tiptap_msg(TiptapInstanceMsg::Bold);
                false
            }
            Msg::Italic => {
                self.send_tiptap_msg(TiptapInstanceMsg::Italic);
                false
            }
            Msg::Strike => {
                self.send_tiptap_msg(TiptapInstanceMsg::Strike);
                false
            }
            Msg::Blockquote => {
                self.send_tiptap_msg(TiptapInstanceMsg::Blockquote);
                false
            }
            Msg::Highlight => {
                self.send_tiptap_msg(TiptapInstanceMsg::Highlight);
                false
            }
            Msg::AlignLeft => {
                self.send_tiptap_msg(TiptapInstanceMsg::AlignLeft);
                false
            }
            Msg::AlignCenter => {
                self.send_tiptap_msg(TiptapInstanceMsg::AlignCenter);
                false
            }
            Msg::AlignRight => {
                self.send_tiptap_msg(TiptapInstanceMsg::AlignRight);
                false
            }
            Msg::AlignJustify => {
                self.send_tiptap_msg(TiptapInstanceMsg::AlignJustify);
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

                    <div class={classes!("tiptap-btn", self.selection_state.h1.then(|| "active"))} onclick={ctx.link().callback(|_| Msg::H1)}>
                        <CrudIcon variant={Bi::TypeH1}/>
                        {"h1"}
                    </div>

                    <div class={classes!("tiptap-btn", self.selection_state.h2.then(|| "active"))} onclick={ctx.link().callback(|_| Msg::H2)}>
                        <CrudIcon variant={Bi::TypeH2}/>
                        {"h2"}
                    </div>

                    <div class={classes!("tiptap-btn", self.selection_state.h3.then(|| "active"))} onclick={ctx.link().callback(|_| Msg::H3)}>
                        <CrudIcon variant={Bi::TypeH3}/>
                        {"h3"}
                    </div>

                    <div class={classes!("tiptap-btn", self.selection_state.paragraph.then(|| "active"))} onclick={ctx.link().callback(|_| Msg::Paragraph)}>
                        <CrudIcon variant={Bi::Paragraph}/>
                        {"paragraph"}
                    </div>

                    <div class={classes!("tiptap-btn", self.selection_state.bold.then(|| "active"))} onclick={ctx.link().callback(|_| Msg::Bold)}>
                        <CrudIcon variant={Bi::TypeBold}/>
                        {"bold"}
                    </div>

                    <div class={classes!("tiptap-btn", self.selection_state.italic.then(|| "active"))} onclick={ctx.link().callback(|_| Msg::Italic)}>
                        <CrudIcon variant={Bi::TypeItalic}/>
                        {"italic"}
                    </div>

                    <div class={classes!("tiptap-btn", self.selection_state.strike.then(|| "active"))} onclick={ctx.link().callback(|_| Msg::Strike)}>
                        <CrudIcon variant={Bi::TypeStrikethrough}/>
                        {"strike"}
                    </div>

                    <div class={classes!("tiptap-btn", self.selection_state.blockquote.then(|| "active"))} onclick={ctx.link().callback(|_| Msg::Blockquote)}>
                        <CrudIcon variant={Bi::BlockquoteLeft}/>
                        {"blockquote"}
                    </div>

                    <div class={classes!("tiptap-btn", self.selection_state.highlight.then(|| "active"))} onclick={ctx.link().callback(|_| Msg::Highlight)}>
                        <CrudIcon variant={Bi::BrightnessAltHigh}/>
                        {"highlight"}
                    </div>

                    <div class={classes!("tiptap-btn", self.selection_state.align_left.then(|| "active"))} onclick={ctx.link().callback(|_| Msg::AlignLeft)}>
                        <CrudIcon variant={Bi::TextLeft}/>
                        {"left"}
                    </div>

                    <div class={classes!("tiptap-btn", self.selection_state.align_center.then(|| "active"))} onclick={ctx.link().callback(|_| Msg::AlignCenter)}>
                        <CrudIcon variant={Bi::TextCenter}/>
                        {"center"}
                    </div>

                    <div class={classes!("tiptap-btn", self.selection_state.align_right.then(|| "active"))} onclick={ctx.link().callback(|_| Msg::AlignRight)}>
                        <CrudIcon variant={Bi::TextRight}/>
                        {"right"}
                    </div>

                    <div class={classes!("tiptap-btn", self.selection_state.align_justify.then(|| "active"))} onclick={ctx.link().callback(|_| Msg::AlignJustify)}>
                        <CrudIcon variant={Bi::Justify}/>
                        {"justify"}
                    </div>

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
