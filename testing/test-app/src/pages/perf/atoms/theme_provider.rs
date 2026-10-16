//! The `ThemeProvider` alone, around a paragraph: uncontrolled (`.perf-theme`, inside the app's
//! own provider, so not the root one), its theme switched through its context, and controlled
//! (`.perf-theme-controlled`), for its performance test
//! (`leptonic/tests/perf_tests/atoms/theme_provider.rs`): both mounted at first, showing the
//! theme as `data-theme`. Its controls ([`Control`]) mount and unmount the uncontrolled one,
//! switch its theme through its context (`dark`, `light`), and switch the controlled one's
//! (`controlled-dark`, `controlled-light`).

use leptonic::atoms::theme::{LeptonicTheme, ThemeContext, ThemeProvider, use_theme};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let context = StoredValue::new(None::<ThemeContext<LeptonicTheme>>);
    let controlled = RwSignal::new(LeptonicTheme::Light);
    let switch = move |theme: LeptonicTheme| {
        if let Some(context) = context.get_value() {
            context.set_theme(theme);
        }
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("dark", move || switch(LeptonicTheme::Dark)),
        Control::action("light", move || switch(LeptonicTheme::Light)),
        Control::action("controlled-dark", move || {
            controlled.set(LeptonicTheme::Dark);
        }),
        Control::action("controlled-light", move || {
            controlled.set(LeptonicTheme::Light);
        }),
    ];
    view! {
        <h1>"ThemeProvider performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <ThemeProvider default_theme=LeptonicTheme::Light classes="perf-theme">
                <Themed context />
            </ThemeProvider>
        </Show>
        <ThemeProvider theme=controlled set_theme=controlled classes="perf-theme-controlled">
            <p>"Controlled"</p>
        </ThemeProvider>
    }
}

/// Keeps the provider's context for the controls.
#[component]
fn Themed(context: StoredValue<Option<ThemeContext<LeptonicTheme>>>) -> impl IntoView {
    context.set_value(use_theme::<LeptonicTheme>());
    view! { <p>"Themed"</p> }
}
