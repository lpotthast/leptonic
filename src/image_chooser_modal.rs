use std::rc::Rc;

use yew::prelude::*;
use yewdux::prelude::Dispatch;

use super::prelude::*;
use crate::stores::global_key_up::GlobalKeyUp;

// Implement in leptonic

pub enum Msg {
    Selected(FileResource),
    OnCancel,
    OnChoose,
    GlobalKeyUp(Rc<GlobalKeyUp>),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub api_base_url: String,
    pub on_cancel: Callback<()>,
    pub on_choose: Callback<FileResource>,
}

pub struct CrudImageChooserModal {
    _global_key_up_dispatch: Dispatch<GlobalKeyUp>,
    selected: Option<FileResource>,
}

impl Component for CrudImageChooserModal {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            _global_key_up_dispatch: Dispatch::subscribe(ctx.link().callback(Msg::GlobalKeyUp)),
            selected: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Selected(resource) => {
                self.selected = Some(resource);
                true
            }
            Msg::OnCancel => {
                ctx.props().on_cancel.emit(());
                false
            }
            Msg::OnChoose => {
                if let Some(selected) = self.selected.clone() {
                    ctx.props().on_choose.emit(selected);
                }
                false
            }
            Msg::GlobalKeyUp(state) => {
                if let Some(event) = state.latest_event() {
                    if event.key().as_str() == "Escape" {
                        ctx.props().on_cancel.emit(());
                    }
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={"crud-modal"}>
                <div class={"crud-modal-header"}>
                    <div class={"crud-modal-title"}>
                        {"Bild auswählen"}
                    </div>
                </div>

                <div class={"crud-modal-body"}>
                    <CrudImageGallery
                        api_base_url={ctx.props().api_base_url.clone()}
                        show_file_names={true}
                        on_select={ctx.link().callback(|resource| Msg::Selected(resource))}/>
                </div>

                <div class={"crud-modal-footer"}>
                    <div class={"crud-row"}>
                    <div class={"crud-col crud-col-flex-end"}>
                        <CrudBtnWrapper>
                            <CrudBtn name={"Abbrechen"} variant={Variant::Default} onclick={&ctx.link().callback(|_| Msg::OnCancel)}/>
                            <CrudBtn name={"Wählen"} variant={Variant::Primary} disabled={self.selected.is_none()} onclick={&ctx.link().callback(|_| Msg::OnChoose)}/>
                        </CrudBtnWrapper>
                    </div>
                    </div>
                </div>
            </div>
        }
    }
}
