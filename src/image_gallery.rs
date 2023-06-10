use tracing::error;
use yew::{html::Scope, prelude::*};

use crate::prelude::*;

// Implement in leptonic

pub enum Msg {
    ListFiles,
    ListFilesResponse(Result<ListFilesResponse, RequestError>),
    Selected(FileResource),
    Reload,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub api_base_url: String,
    pub show_file_names: bool,
    pub on_link: Option<Callback<Option<Scope<CrudImageGallery>>>>,
    pub on_select: Option<Callback<FileResource>>,
}

pub struct CrudImageGallery {
    resources: Vec<FileResource>,
    selected: Option<FileResource>,
}

impl Component for CrudImageGallery {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        if let Some(on_link) = &ctx.props().on_link {
            on_link.emit(Some(ctx.link().clone()));
        };
        ctx.link().send_message(Msg::ListFiles);
        Self {
            resources: Vec::new(),
            selected: None,
        }
    }

    fn destroy(&mut self, ctx: &Context<Self>) {
        if let Some(on_link) = &ctx.props().on_link {
            on_link.emit(None);
        };
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ListFiles => {
                let base = ctx.props().api_base_url.clone();
                ctx.link().send_future(async move {
                    Msg::ListFilesResponse(
                        // No authentication required when accessing '/public'!
                        request_get::<ListFilesResponse>(format!("{}/public", base), None).await,
                    )
                });
                false
            }
            Msg::ListFilesResponse(result) => {
                match result {
                    Ok(files) => {
                        if let Some(err) = files.error {
                            error!("Could not list files: {:?}", err);
                        } else {
                            self.resources = files.files;
                        }
                    }
                    Err(err) => error!("Could not list files: {}", err),
                }
                true
            }
            Msg::Selected(resource) => {
                self.selected = Some(resource.clone());
                if let Some(on_select) = &ctx.props().on_select {
                    on_select.emit(resource);
                }
                true
            }
            Msg::Reload => {
                ctx.link().send_message(Msg::ListFiles);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={"gallery"}>
                {
                    self.resources.iter()
                        .map(|resource| {
                            let is_selected = match &self.selected {
                                Some(selected) => selected == resource,
                                None => false,
                            };
                            let cloned = resource.clone();
                            html! {
                                <div
                                    class={classes!("img-wrapper", is_selected.then(|| "selected"))}
                                    onclick={ctx.link().callback(move |_| Msg::Selected(cloned.clone()))}
                                >
                                    <img
                                        src={format!("{}/public/{}", ctx.props().api_base_url.clone(), urlencoding::encode(resource.name.as_str()))}
                                        alt={resource.name.clone()}
                                    />
                                    if ctx.props().show_file_names {
                                        <span>{resource.name.clone()}</span>
                                    }
                                </div>
                            }
                        })
                        .collect::<Html>()
                }
            </div>
        }
    }
}
