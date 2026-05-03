use leptonic::{
    atoms::{
        label::Label as LabelAtom,
        slider::{Slider as SliderAtom, SliderOutput, SliderThumb, SliderTrack, SliderTrackFill},
    },
    hooks::*,
};
use leptos::prelude::*;
use leptos_styles::{
    Style::{
        AlignItems, Background, BackgroundColor, Border, BorderRadius, BoxShadow, Cursor, Display,
        FontWeight, Gap, Height, MinWidth, Position, TextAlign, Width,
    },
    Styles,
};

fn track_style() -> Styles {
    Styles::from([
        (Display, "flex"),
        (Width, "100%"),
        (Height, "8px"),
        (Background, "#ddd"),
        (BorderRadius, "4px"),
        (Position, "relative"),
        (Cursor, "pointer"),
    ])
}

fn thumb_style(color: &'static str) -> Styles {
    Styles::builder()
        .with(Width, "20px")
        .with(Height, "20px")
        .with(BackgroundColor, color)
        .with(BorderRadius, "50%")
        .with(Border, "2px solid white")
        .with(BoxShadow, "0 2px 4px rgba(0,0,0,0.2)")
        .with(Cursor, "grab")
        .build()
}

#[component]
pub fn SliderDisabledDemo() -> impl IntoView {
    view! {
        <div style="padding: 1.5em; border: 1px solid #ddd; border-radius: 8px; margin: 1em 0;">
            <SliderAtom values=SliderValues::Uncontrolled(vec![30.0]) disabled=true styles=[(Display, "flex"), (AlignItems, "center"), (Gap, "1em")]>
                <LabelAtom styles=[(MinWidth, "80px"), (FontWeight, "500")]>
                    "Disabled"
                </LabelAtom>
                <SliderTrack styles=track_style()>
                    <SliderTrackFill styles=[(BackgroundColor, "#999"), (BorderRadius, "4px")]/>
                    <SliderThumb styles=thumb_style("#999")/>
                </SliderTrack>
                <SliderOutput let:attrs let:values>
                    <output {..attrs} style=Styles::from([(MinWidth, "50px"), (TextAlign, "right")])>
                        { move || {
                            let val = values.get().first().copied().unwrap_or(0.0);
                            format!("{val:.0}%")
                        } }
                    </output>
                </SliderOutput>
            </SliderAtom>
        </div>
    }
}
