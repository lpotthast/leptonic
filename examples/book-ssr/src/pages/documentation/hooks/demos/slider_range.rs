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
pub fn SliderRangeDemo() -> impl IntoView {
    view! {
        <div style="padding: 1.5em; border: 1px solid #ddd; border-radius: 8px; margin: 1em 0;">
            <SliderAtom values=SliderValues::Uncontrolled(vec![20.0, 80.0]) styles=[(Display, "flex"), (AlignItems, "center"), (Gap, "1em")]>
                <LabelAtom styles=[(MinWidth, "80px"), (FontWeight, "500")]>
                    "Price Range"
                </LabelAtom>
                <SliderTrack styles=track_style()>
                    <SliderTrackFill styles=[(BackgroundColor, "#4a90d9"), (BorderRadius, "4px")]/>
                    <SliderThumb aria_label="Minimum Price" styles=thumb_style("#4a90d9")/>
                    <SliderThumb aria_label="Maximum Price" styles=thumb_style("#4a90d9")/>
                </SliderTrack>
                <SliderOutput let:attrs let:values>
                    <output {..attrs} style=Styles::from([(MinWidth, "80px"), (TextAlign, "right")])>
                        { move || {
                            let val1 = values.get().first().copied().unwrap_or(0.0);
                            let val2 = values.get().get(1).copied().unwrap_or(0.0);
                            format!("{val1:.0}% - {val2:.0}%")
                        } }
                    </output>
                </SliderOutput>
            </SliderAtom>
        </div>
    }
}
