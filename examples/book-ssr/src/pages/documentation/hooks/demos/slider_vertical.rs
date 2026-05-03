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

fn track_style(orientation: SliderOrientation) -> Styles {
    Styles::from([
        (Display, "flex"),
        (
            Width,
            match orientation {
                SliderOrientation::Horizontal => "100%",
                SliderOrientation::Vertical => "8px",
            },
        ),
        (
            Height,
            match orientation {
                SliderOrientation::Horizontal => "8px",
                SliderOrientation::Vertical => "100%",
            },
        ),
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
pub fn SliderVerticalDemo() -> impl IntoView {
    view! {
        <div style="padding: 1.5em; border: 1px solid #ddd; border-radius: 8px; margin: 1em 0;">
            <SliderAtom values=SliderValues::Uncontrolled(vec![60.0]) orientation=SliderOrientation::Vertical styles=[(Display, "flex"), (AlignItems, "center"), (Gap, "1em"), (Height, "150px")]>
                <LabelAtom styles=[(MinWidth, "80px"), (FontWeight, "500")]>
                    "Vertical"
                </LabelAtom>
                <SliderTrack styles=track_style(SliderOrientation::Vertical)>
                    <SliderTrackFill styles=[(BackgroundColor, "#9b59b6"), (BorderRadius, "4px")]/>
                    <SliderThumb styles=thumb_style("#9b59b6")/>
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

        <p>"Vertical range slider"</p>

        <div style="padding: 1.5em; border: 1px solid #ddd; border-radius: 8px; margin: 1em 0;">
            <SliderAtom values=SliderValues::Uncontrolled(vec![20.0, 60.0]) orientation=SliderOrientation::Vertical styles=[(Display, "flex"), (AlignItems, "center"), (Gap, "1em"), (Height, "150px")]>
                <LabelAtom styles=[(MinWidth, "80px"), (FontWeight, "500")]>
                    "Vertical"
                </LabelAtom>
                <SliderTrack styles=track_style(SliderOrientation::Vertical)>
                    <SliderTrackFill styles=[(BackgroundColor, "#9b59b6"), (BorderRadius, "4px")]/>
                    <SliderThumb styles=thumb_style("#9b59b6")/>
                    <SliderThumb styles=thumb_style("#9b59b6")/>
                </SliderTrack>
                <SliderOutput let:attrs let:values>
                    <output {..attrs} style=Styles::from([(MinWidth, "50px"), (TextAlign, "right")])>
                        { move || {
                            let val1 = values.get().first().copied().unwrap_or(0.0);
                            let val2 = values.get().get(1).copied().unwrap_or(0.0);
                            format!("{val1}% - {val2}% ")
                        } }
                    </output>
                </SliderOutput>
            </SliderAtom>
        </div>
    }
}
