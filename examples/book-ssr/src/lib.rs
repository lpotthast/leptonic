#![recursion_limit = "256"]

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

            #[route("/use-move-within")]
            pub mod use_move_within {}

            #[route("/use-hover")]
            pub mod use_hover {}

            #[route("/use-focus")]
            pub mod use_focus {}

            #[route("/use-button")]
            pub mod use_button {}

            #[route("/use-overlay")]
            pub mod use_overlay {}

            #[route("/use-anchor-link")]
            pub mod use_anchor_link {}

            #[route("/use-keyboard")]
            pub mod use_keyboard {}

            #[route("/use-interact-outside")]
            pub mod use_interact_outside {}

            #[route("/use-scroll-wheel")]
            pub mod use_scroll_wheel {}

            #[route("/use-prevent-scroll")]
            pub mod use_prevent_scroll {}

            #[route("/use-focus-within")]
            pub mod use_focus_within {}

            #[route("/use-focusable")]
            pub mod use_focusable {}

            #[route("/use-focus-ring")]
            pub mod use_focus_ring {}

            #[route("/use-focus-manager")]
            pub mod use_focus_manager {}

            #[route("/use-has-tabbable-child")]
            pub mod use_has_tabbable_child {}

            #[route("/selection")]
            pub mod selection {}

            #[route("/use-label")]
            pub mod use_label {}

            #[route("/use-checkbox")]
            pub mod use_checkbox {}

            #[route("/use-radio")]
            pub mod use_radio {}

            #[route("/use-text-field")]
            pub mod use_text_field {}

            #[route("/use-switch")]
            pub mod use_switch {}

            #[route("/use-slider")]
            pub mod use_slider {}

            #[route("/use-modal")]
            pub mod use_modal {}

            #[route("/use-tooltip")]
            pub mod use_tooltip {}

            #[route("/use-menu")]
            pub mod use_menu {}

            #[route("/use-listbox")]
            pub mod use_listbox {}

            #[route("/use-select")]
            pub mod use_select {}

            #[route("/use-combobox")]
            pub mod use_combobox {}

            #[route("/use-tabs")]
            pub mod use_tabs {}

            #[route("/use-table")]
            pub mod use_table {}

            #[route("/dnd")]
            pub mod dnd {}

            #[route("/use-disclosure")]
            pub mod use_disclosure {}

            #[route("/use-progress-bar")]
            pub mod use_progress_bar {}

            #[route("/use-breadcrumbs")]
            pub mod use_breadcrumbs {}

            #[route("/use-link")]
            pub mod use_link {}

            #[route("/use-meter")]
            pub mod use_meter {}

            #[route("/use-separator")]
            pub mod use_separator {}

            #[route("/use-tag")]
            pub mod use_tag {}

            #[route("/use-toolbar")]
            pub mod use_toolbar {}

            #[route("/use-tree")]
            pub mod use_tree {}

            #[route("/use-grid")]
            pub mod use_grid {}

            #[route("/use-popover")]
            pub mod use_popover {}
        }

        #[route("/atoms")]
        pub mod atoms {

            #[route("/button")]
            pub mod button {}

            #[route("/popover")]
            pub mod popover {}

            #[route("/anchor-link")]
            pub mod anchor_link {}

            #[route("/focus-scope")]
            pub mod focus_scope {}

            #[route("/focus-ring")]
            pub mod focus_ring {}

            #[route("/grid")]
            pub mod grid {}

            #[route("/slider")]
            pub mod slider {}

            #[route("/link")]
            pub mod link {}
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
