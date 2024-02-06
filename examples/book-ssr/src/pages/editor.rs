use std::collections::HashMap;

use leptonic::components::input::TextInput;
use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum EditMode {
    Text,
}

type SettingSpec = (&'static str, EditMode, &'static str);

const TEXT_INPUT_SETTINGS_V0_3: &[(&str, EditMode, &str)] = &[
    ("--input-padding", EditMode::Text, "padding"),
    ("--input-color", EditMode::Text, "color"),
    (
        "--input-background-color",
        EditMode::Text,
        "background-color",
    ),
    ("--input-border", EditMode::Text, "border"),
    ("--input-border-bottom", EditMode::Text, "border-bottom"),
    ("--input-border-radius", EditMode::Text, "border-radius"),
    ("--input-min-height", EditMode::Text, "min-height"),
];
// --input-focused-border-color ?

struct Settings {
    map: HashMap<SettingSpec, Option<String>>,
}

impl Settings {
    pub fn new() -> Self {
        let mut map = HashMap::new();

        for s in TEXT_INPUT_SETTINGS_V0_3 {
            map.insert(*s, None);
        }

        Self { map }
    }

    pub fn to_style(&self) -> String {
        let mut style = String::new();
        for (s, v) in &self.map {
            if let Some(v) = v {
                style.push_str(&format!("{}: {};\n", s.0, v));
            }
        }
        style
    }
}

#[component]
pub fn ThemeEditor() -> impl IntoView {
    let (settings, set_settings) = create_signal(Settings::new());

    let style = Signal::derive(move || settings.with(|s| s.to_style()));

    view! {
        <div style="display: grid; grid-template-columns: 1fr 2fr 2fr; width: 100%; gap: 0.75em;">
            <div id="col1" style="display: flex; flex-direction: column;">
                <Components />
            </div>
            <div id="col2" style="display: flex; flex-direction: column; padding: 0.5em;">
                <Preview style />
                <Output style />
            </div>
            <div id="col3" style="display: flex; flex-direction: column;">
                <Settings>
                    <For
                        each=move || TEXT_INPUT_SETTINGS_V0_3
                        key=|e| e.2
                        children=move |setting| view! {
                            <Setting setting=*setting set_settings />
                        }
                    />
                </Settings>
            </div>
        </div>
    }
}

#[component]
fn Components() -> impl IntoView {
    view! {
        <div style="height: 100%; padding: 0.75em; border-right: 1px solid;">
            "Text Input"
        </div>
    }
}

#[component]
fn Preview(style: Signal<String>) -> impl IntoView {
    let (t, set_t) = create_signal(String::from("Text Input"));

    view! {
        <div style="padding-bottom: 1em; height: 50%; padding: 0.75em;">
            <div>"Preview:"</div>
            <div style=move || style.get()>
                <TextInput get=t set=set_t />
            </div>
        </div>
    }
}

#[component]
fn Settings(children: Children) -> impl IntoView {
    view! {
        <div style="height: 100%; border-left: 1px solid; padding: 0.75em;">
            {children()}
        </div>
    }
}

#[component]
fn Setting(setting: SettingSpec, set_settings: WriteSignal<Settings>) -> impl IntoView {
    let (padding, set_padding) = create_signal(String::from("inherit")); // TODO: what default?

    let setter = move |v: String| {
        set_padding.set(v.clone());
        set_settings.update(|settings| {
            if let Some(map_value) = settings.map.get_mut(&setting) {
                *map_value = Some(v);
            };
        })
    };

    view! {
        <div style="padding-bottom: 1em;">
            <span>{setting.2}</span>
            <TextInput get=padding set=setter />
        </div>
    }
}

#[component]
fn Output(style: Signal<String>) -> impl IntoView {
    view! {
        // <TiptapEditor value=style disabled=true />
        <div style="height: 50%; padding: 0.75em;" >
            <div>"Output:"</div>
            <div style="background-color: white; padding: 0.5em; white-space: pre-wrap;">
                { move || style.get() }
            </div>
        </div>
    }
}
