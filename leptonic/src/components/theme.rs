use leptos::prelude::*;

use crate::{
    atoms::theme::{LeptonicTheme, Theme, use_theme},
    components::switch::{Switch, SwitchIcons, SwitchVariant},
    utils::{classes::Classes, styles::Styles},
};

/// A theme's icon, shown by [`ThemeToggle`].
pub trait ThemeIcon: Theme {
    fn icon(&self) -> icondata::Icon;
}

impl ThemeIcon for LeptonicTheme {
    fn icon(&self) -> icondata::Icon {
        match self {
            Self::Light => icondata::BsSun,
            Self::Dark => icondata::BsMoon,
        }
    }
}

/// A switch between two themes: on selects `on`, off selects `off`. Shows their icons.
#[component]
pub fn ThemeToggle<T>(
    off: T,
    on: T,
    #[prop(optional)] variant: SwitchVariant,
    /// The switch's accessible name. Defaults to "`<on theme name>` theme".
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView
where
    T: ThemeIcon + 'static,
{
    let theme = use_theme::<T>()
        .expect("<ThemeToggle/> component should be nested within a <ThemeProvider/>.");
    let is_selected = Signal::derive(move || theme.theme().get() == on);
    let set_selected = move |selected: bool| theme.set_theme(if selected { on } else { off });
    let aria_label = MaybeProp::derive(move || {
        Some(
            aria_label
                .get()
                .unwrap_or_else(|| format!("{} theme", on.name())),
        )
    });

    view! {
        <Switch
            is_selected
            set_selected
            aria_label
            variant
            icons=SwitchIcons { on: on.icon(), off: off.icon() }
            classes=classes.add("leptonic-theme-toggle")
            styles
        />
    }
}
