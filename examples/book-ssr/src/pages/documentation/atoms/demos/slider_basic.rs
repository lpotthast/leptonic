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
        .with(BackgroundColor, "var(--brand-color)")
        .with(BorderRadius, "50%")
        .with(Border, "2px solid white")
        .with(BoxShadow, "0 2px 4px rgba(0,0,0,0.2)")
        .with(Cursor, "grab")
        .build()
}

#[component]
pub fn SliderBasicDemo() -> impl IntoView {
    view! {
        <div style="padding: 1.5em; border: 1px solid #ddd; border-radius: 8px; margin: 1em 0;">
            <SliderAtom values=SliderValues::Uncontrolled(vec![50.0]) styles=[(Display, "flex"), (AlignItems, "center"), (Gap, "1em")]>
                <SliderTrack styles=track_style()>
                    <SliderTrackFill styles=[(BackgroundColor, "var(--brand-color)"), (BorderRadius, "4px")]/>
                    <SliderThumb styles=thumb_style()/>
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
