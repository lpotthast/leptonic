use tracing::{error, warn};
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_bootstrap_icons::v1_10_3::Bi;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct CrudQuickSearchOption {
    pub name: String,
    pub icon: Option<Bi>,
}

pub enum Msg {
    SetFilter(String),
    FilterKeyDown(KeyboardEvent),
    FilterKeyUp(KeyboardEvent),
    FilterChanged(Event),
    PreselectPreviousOption,
    PreselectNextOption,
    PreselectedKeyDown(KeyboardEvent),
    Select(CrudQuickSearchOption),
    FocusInput,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub options: Vec<CrudQuickSearchOption>,
    pub onselect: Callback<CrudQuickSearchOption>,
}

pub struct CrudQuickSearch {
    input_id: String,
    filter: String,
    filtered_options: Vec<CrudQuickSearchOption>,
    preselected_option: Option<CrudQuickSearchOption>,
}

impl CrudQuickSearch {
    pub fn filter_options<'a>(
        options: &Vec<CrudQuickSearchOption>,
        filter: &'a str,
    ) -> Vec<CrudQuickSearchOption> {
        match filter.to_lowercase().as_str() {
            "" => vec![],
            filter => options
                .clone()
                .into_iter()
                .filter(|it| it.name.to_lowercase().contains(filter))
                .collect(),
        }
    }

    pub fn is_preselected(&self, option: &CrudQuickSearchOption) -> bool {
        match &self.preselected_option {
            Some(preselected) => preselected.name == option.name,
            None => false,
        }
    }
}

impl Component for CrudQuickSearch {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let filter = "".to_owned();
        let filtered_options = Self::filter_options(&ctx.props().options, &filter);
        Self {
            input_id: format!("id-{}", Uuid::new_v4()),
            filter,
            filtered_options,
            preselected_option: None,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        ctx.link().send_message(Msg::SetFilter(self.filter.clone()));
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetFilter(filter) => {
                self.filter = filter;
                self.filtered_options = Self::filter_options(&ctx.props().options, &self.filter);
                // Reset preselected_option to None if no longer present in the options vec.
                if let Some(preselected) = &self.preselected_option {
                    if !self
                        .filtered_options
                        .iter()
                        .any(|option| option.name == preselected.name)
                    {
                        self.preselected_option = None;
                    }
                }
                true
            }
            Msg::FilterKeyDown(event) => match event.key().as_str() {
                "ArrowUp" => {
                    event.prevent_default();
                    event.stop_propagation();
                    false
                }
                "ArrowDown" => {
                    event.prevent_default();
                    event.stop_propagation();
                    false
                }
                _ => false
            },
            Msg::FilterKeyUp(event) => match event.key().as_str() {
                "ArrowUp" => {
                    ctx.link().send_message(Msg::PreselectPreviousOption);
                    false
                }
                "ArrowDown" => {
                    ctx.link().send_message(Msg::PreselectNextOption);
                    false
                }
                "Enter" => {
                    if let Some(preselected) = &self.preselected_option {
                        ctx.link().send_message(Msg::Select(preselected.clone()));
                    }
                    false
                }
                _ => match keyboard_event_target_as::<web_sys::HtmlInputElement>(event) {
                    Ok(input) => {
                        ctx.link().send_message(Msg::SetFilter(input.value()));
                        true
                    }
                    Err(err) => {
                        error!("Could not get input value: {}", err);
                        false
                    }
                },
            },
            Msg::FilterChanged(event) => {
                match event_target_as::<web_sys::HtmlInputElement>(event) {
                    Ok(input) => {
                        ctx.link().send_message(Msg::SetFilter(input.value()));
                        true
                    }
                    Err(err) => {
                        error!("Could not get input value: {}", err);
                        false
                    }
                }
            }
            Msg::PreselectPreviousOption => {
                self.preselected_option = match &self.preselected_option {
                    Some(preselected) => {
                        self.filtered_options
                            .iter()
                            .enumerate()
                            .find_map(|(i, option)| {
                                if preselected.name == option.name {
                                    self.filtered_options
                                        .get(if i == 0 {
                                            self.filtered_options.len() - 1
                                        } else {
                                            i - 1
                                        })
                                        .cloned()
                                } else {
                                    None
                                }
                            })
                    }
                    None => self.filtered_options.last().cloned(),
                };
                true
            }
            Msg::PreselectNextOption => {
                self.preselected_option = match &self.preselected_option {
                    Some(preselected) => {
                        self.filtered_options
                            .iter()
                            .enumerate()
                            .find_map(|(i, option)| {
                                if preselected.name == option.name {
                                    self.filtered_options
                                        .get(if i == self.filtered_options.len() - 1 {
                                            0
                                        } else {
                                            i + 1
                                        })
                                        .cloned()
                                } else {
                                    None
                                }
                            })
                    }
                    None => self.filtered_options.first().cloned(),
                };
                true
            }
            Msg::PreselectedKeyDown(event) => match event.key().as_str() {
                "Enter" => {
                    if let Some(preselected) = &self.preselected_option {
                        ctx.link().send_message(Msg::Select(preselected.clone()));
                    }
                    false
                }
                _ => false
            }
            Msg::Select(option) => {
                ctx.props().onselect.emit(option);
                ctx.link().send_message(Msg::SetFilter("".to_owned()));
                true
            }
            Msg::FocusInput => {
                if let Some(input) = gloo::utils::document().get_element_by_id(&self.input_id) {
                    match input.unchecked_into::<HtmlElement>().focus() {
                        Ok(_) => {}
                        Err(_) => warn!(
                            "Could not set focus to input element with id '{}'",
                            self.input_id
                        ),
                    }
                } else {
                    warn!("Did not find element with id '{}'", self.input_id);
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={"crud-quicksearch"}>
                <div class={"input-with-btn"}>
                    <input
                        type={"text"}
                        id={self.input_id.clone()}
                        class={"crud-input-field"}
                        placeholder={"Suchen..."}
                        autocomplete={"off"}
                        value={self.filter.clone()}
                        onkeydown={ctx.link().callback(Msg::FilterKeyDown)}
                        onkeyup={ctx.link().callback(Msg::FilterKeyUp)}
                        onchange={ctx.link().callback(Msg::FilterChanged)} />

                    if self.filter.is_empty() {
                        <CrudBtn icon={Bi::Search} name={""} onclick={ctx.link().callback(|_| Msg::FocusInput)}/>
                    } else {
                        <CrudBtn icon={Bi::XLg} name={""} onclick={ctx.link().callback(|_| Msg::SetFilter("".to_owned()))}/>
                    }
                </div>

                if !self.filtered_options.is_empty() {
                    <div class={"dropdown"}>
                    {
                        self.filtered_options.iter().map(|it| {
                            let it_cloned = it.clone();
                            html! {
                                <div class={classes!("option", self.is_preselected(&it).then(|| "preselected"))}
                                onclick={ctx.link().callback(move |_| Msg::Select(it_cloned.clone()))}
                                onkeydown={ctx.link().callback(Msg::PreselectedKeyDown)}>
                                    {it.name.clone()}
                                </div>
                            }
                        }).collect::<Html>()
                    }
                    </div>
                }
            </div>
        }
    }
}
