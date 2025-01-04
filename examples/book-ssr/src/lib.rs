use leptos_routes::routes;

pub mod app;
pub mod pages;

#[routes]
pub mod routes {

    #[route("/")]
    pub mod root {}

    #[route("/doc")]
    pub mod doc {

        #[route("/overview")]
        pub mod overview {}

        #[route("/installation")]
        pub mod installation {}

        #[route("/themes")]
        pub mod themes {}

        #[route("/changelog")]
        pub mod changelog {}

        #[route("/hooks")]
        pub mod hooks {

            #[route("/use-press")]
            pub mod use_press {}

            #[route("/use-move")]
            pub mod use_move {}

            #[route("/use-hover")]
            pub mod use_hover {}

            #[route("/use-button")]
            pub mod use_button {}

            #[route("/use-overlay")]
            pub mod use_overlay {}

            #[route("/use-anchor-link")]
            pub mod use_anchor_link {}
        }

        #[route("/atoms")]
        pub mod atoms {

            #[route("/button")]
            pub mod button {}

            #[route("/popover")]
            pub mod popover {}

            #[route("/anchor-link")]
            pub mod anchor_link {}
        }

        #[route("/components")]
        pub mod components {

            // Layout
            #[route("/stack")]
            pub mod stack {}

            #[route("/grid")]
            pub mod grid {}

            #[route("/separator")]
            pub mod separator {}

            #[route("/skeleton")]
            pub mod skeleton {}

            #[route("/app-bar")]
            pub mod app_bar {}

            #[route("/drawer")]
            pub mod drawer {}

            #[route("/tabs")]
            pub mod tabs {}

            #[route("/table")]
            pub mod table {}

            #[route("/collapsible")]
            pub mod collapsible {}

            // Input
            #[route("/button")]
            pub mod button {}

            #[route("/input")]
            pub mod input {}

            #[route("/tiptap-editor")]
            pub mod tiptap_editor {}

            #[route("/date-time")]
            pub mod date_time {}

            #[route("/slider")]
            pub mod slider {}

            #[route("/select")]
            pub mod select {}

            #[route("/checkbox")]
            pub mod checkbox {}

            #[route("/radio")]
            pub mod radio {}

            #[route("/toggle")]
            pub mod toggle {}

            #[route("/color-picker")]
            pub mod color_picker {}

            // Feedback
            #[route("/alert")]
            pub mod alert {}

            #[route("/toast")]
            pub mod toast {}

            #[route("/modal")]
            pub mod modal {}

            #[route("/progress")]
            pub mod progress {}

            #[route("/popover")]
            pub mod popover {}

            #[route("/chip")]
            pub mod chip {}

            #[route("/kbd")]
            pub mod kbd {}

            // General
            #[route("/typography")]
            pub mod typography {}

            #[route("/icon")]
            pub mod icon {}

            #[route("/link")]
            pub mod link {}

            #[route("/callback")]
            pub mod callback {}
        }
    }

    #[route("/theme-editor")]
    pub mod theme_editor {}

    #[route("/not-found")]
    pub mod not_found {}
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;

    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default_with_config(
        tracing_wasm::WASMLayerConfigBuilder::default()
            .set_max_level(tracing::Level::DEBUG)
            .build(),
    );

    leptos::mount::hydrate_body(App);
}