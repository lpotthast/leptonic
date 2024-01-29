use cfg_if::cfg_if;
use http::status::StatusCode;
use leptonic::prelude::*;
use leptos::*;
use thiserror::Error;

#[cfg(feature = "ssr")]
use leptos_axum::ResponseOptions;

#[derive(Clone, Debug, Error)]
pub enum AppError {
    #[error("Not Found")]
    NotFound,
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

// A basic function to display errors served by the error boundaries.
// Feel free to do more complicated things here than just displaying the error.
#[component]
pub fn ErrorTemplate(
    #[prop(optional)] outside_errors: Option<Errors>,
    #[prop(optional)] errors: Option<RwSignal<Errors>>,
) -> impl IntoView {
    let errors = match outside_errors {
        Some(e) => create_rw_signal(e),
        None => match errors {
            Some(e) => e,
            None => panic!("No Errors found and we expected errors!"),
        },
    };
    // Get Errors from Signal
    let errors = errors.get_untracked();

    // Downcast lets us take a type that implements `std::error::Error`
    let errors: Vec<AppError> = errors
        .into_iter()
        .filter_map(|(_k, v)| v.downcast_ref::<AppError>().cloned())
        .collect();
    println!("Errors: {errors:#?}");

    let num_errors = errors.len();

    // Only the response code for the first error is actually sent from the server
    // this may be customized by the specific application
    cfg_if! { if #[cfg(feature="ssr")] {
        let response = use_context::<ResponseOptions>();
        if let Some(response) = response {
            response.set_status(errors[0].status_code());
        }
    }}

    view! {
        <Box style="display: flex; flex-direction: column; align-items:center;">
            <H1>{match num_errors {
                1 => "Error",
                _ => "Errors",
            }}</H1>

            <For
                each=move || { errors.clone().into_iter().enumerate() }
                key=|(index, _error)| *index
                children=move |(_index, error)| {
                    // let error_string = error.to_string();
                    // let error_code= error.status_code();
                    match error {
                        AppError::NotFound => view! {
                            <P>"404 - Not Found"</P>
                        },
                    }
                }
            />

            <LinkButton href="/">
                "Back"
            </LinkButton>
        </Box>
    }
}
