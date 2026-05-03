use leptonic::{
    atoms::slider::{
        Slider as SliderAtom, SliderOutput, SliderThumb, SliderTrack, SliderTrackFill,
    },
    hooks::SliderValues,
};
use leptos::prelude::*;
use leptos_styles::{
    Style::{
        AlignItems, Background, BackgroundColor, Border, BorderRadius, BoxShadow, Cursor, Display,
        Gap, Height, MinWidth, Position, TextAlign, Width,
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

fn thumb_style() -> Styles {
    Styles::builder()
        .with(Width, "20px")
        .with(Height, "20px")
        .with(BackgroundColor, "#4a90d9")
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
                <SliderTrack styles=track_style()>
                    <SliderTrackFill styles=[(BackgroundColor, "#4a90d9"), (BorderRadius, "4px")]/>
                    <SliderThumb aria_label="Minimum" styles=thumb_style()/>
                    <SliderThumb aria_label="Maximum" styles=thumb_style()/>
                </SliderTrack>
                <SliderOutput let:attrs let:values>
                    <output {..attrs} style=Styles::from([(MinWidth, "80px"), (TextAlign, "right")])>
                        { move || {
                            let vals = values.get();
                            let v1 = vals.first().copied().unwrap_or(0.0);
                            let v2 = vals.get(1).copied().unwrap_or(0.0);
                            format!("{v1:.0}% - {v2:.0}%")
                        } }
                    </output>
                </SliderOutput>
            </SliderAtom>
        </div>
    }
}
