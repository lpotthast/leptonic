/// Reusable inline-style functions for documentation page demos.
///
/// Each function returns a `Styles` instance built via the typed builder pattern,
/// using `Style` enum variants for type-safe CSS property names.
use leptonic::utils::styles::{Style::*, Styles};

/// Focus outline used in scoped `<style>` blocks.
/// Stays as a `&str` because it's used inside CSS class rules, not as an element `style=` attribute.
pub const FOCUS_OUTLINE_CSS: &str =
    "outline: 3px solid var(--brand-color, #e66956); outline-offset: 2px;";

/// Container for interactive demos.
pub fn demo_container() -> Styles {
    Styles::builder()
        .with((Border, "2px solid var(--brand-color)"))
        .with((Padding, "1.5em"))
        .with((BorderRadius, "8px"))
        .build()
}

/// Active variant (focus-within, focused state) — adds brand background.
pub fn demo_container_active() -> Styles {
    Styles::builder()
        .with((Border, "2px solid var(--brand-color)"))
        .with((Padding, "1.5em"))
        .with((BorderRadius, "8px"))
        .with((Background, "var(--brand-color-light, rgba(230, 105, 86, 0.1))"))
        .build()
}

/// Inactive variant — transparent background, gray border.
pub fn demo_container_inactive() -> Styles {
    Styles::builder()
        .with((Border, "2px solid #ccc"))
        .with((Padding, "1.5em"))
        .with((BorderRadius, "8px"))
        .with((Background, "transparent"))
        .build()
}

/// Standard button inside demos.
pub fn demo_button() -> Styles {
    Styles::builder()
        .with((Padding, "0.5em 1em"))
        .with((BorderRadius, "4px"))
        .with((Border, "1px solid #ccc"))
        .with((Cursor, "pointer"))
        .build()
}

/// Standard text input inside demos.
pub fn demo_input() -> Styles {
    Styles::builder()
        .with((Padding, "0.5em"))
        .with((Border, "1px solid #ccc"))
        .with((BorderRadius, "4px"))
        .build()
}

/// State indicator: active / true.
pub fn state_active() -> Styles {
    Styles::builder().with((Color, "green")).build()
}

/// State indicator: inactive / false.
pub fn state_inactive() -> Styles {
    Styles::builder().with((Color, "gray")).build()
}

/// `FormControl` row layout for checkbox + label controls.
pub fn form_control_row() -> Styles {
    Styles::builder()
        .with((FlexDirection, "row"))
        .with((AlignItems, "center"))
        .with((Gap, "0.5em"))
        .build()
}

/// Flex row with gap and wrap — for counter displays.
pub fn flex_row_gap() -> Styles {
    Styles::builder()
        .with((Display, "flex"))
        .with((Gap, "2em"))
        .with((FlexWrap, "wrap"))
        .with((MarginTop, "0.5em"))
        .build()
}

/// Flex row centered — for inline element groups.
pub fn flex_row_center() -> Styles {
    Styles::builder()
        .with((Display, "flex"))
        .with((Gap, "1em"))
        .with((AlignItems, "center"))
        .build()
}

/// Simple margin-top spacer.
pub fn margin_top_1em() -> Styles {
    Styles::builder().with((MarginTop, "1em")).build()
}

/// Bold title for demo containers.
pub fn demo_container_title() -> Styles {
    Styles::builder()
        .with((Margin, "0 0 1em 0"))
        .with((FontWeight, "bold"))
        .build()
}

/// Solid button style for focus-scope demos.
pub fn demo_button_solid() -> Styles {
    Styles::builder()
        .with((Padding, "0.75em 1.5em"))
        .with((BorderRadius, "4px"))
        .with((Border, "2px solid #333"))
        .with((Cursor, "pointer"))
        .with((Background, "white"))
        .build()
}

/// Solid input style for focus-scope demos.
pub fn demo_input_solid() -> Styles {
    Styles::builder()
        .with((Padding, "0.75em"))
        .with((Border, "2px solid #333"))
        .with((BorderRadius, "4px"))
        .with((Width, "150px"))
        .with((Background, "white"))
        .build()
}

/// Event log `<pre>` element.
pub fn event_log() -> Styles {
    Styles::builder()
        .with((Width, "100%"))
        .with((Height, "15em"))
        .with((Overflow, "auto"))
        .with((Padding, "var(--typography-code-padding)"))
        .with((Border, "none"))
        .with((BorderRadius, "var(--typography-code-border-radius)"))
        .with((BackgroundColor, "var(--typography-code-background-color)"))
        .with((Color, "var(--typography-code-color)"))
        .build()
}
