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
        .with((
            Background,
            "var(--brand-color-light, rgba(230, 105, 86, 0.1))",
        ))
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

/// Primary action button (brand-colored).
pub fn demo_button_primary() -> Styles {
    Styles::builder()
        .with((Padding, "0.75em 1.5em"))
        .with((BorderRadius, "8px"))
        .with((Cursor, "pointer"))
        .with((Background, "var(--brand-color)"))
        .with((Color, "white"))
        .with((Border, "none"))
        .with((FontSize, "1em"))
        .build()
}

/// Danger action button (red).
pub fn demo_button_danger() -> Styles {
    Styles::builder()
        .with((Padding, "0.75em 1.5em"))
        .with((BorderRadius, "8px"))
        .with((Cursor, "pointer"))
        .with((Background, "#dc3545"))
        .with((Color, "white"))
        .with((Border, "none"))
        .with((FontSize, "1em"))
        .build()
}

/// Secondary action button (gray).
pub fn demo_button_secondary() -> Styles {
    Styles::builder()
        .with((Padding, "0.75em 1.5em"))
        .with((BorderRadius, "8px"))
        .with((Cursor, "pointer"))
        .with((Background, "#6c757d"))
        .with((Color, "white"))
        .with((Border, "none"))
        .with((FontSize, "1em"))
        .build()
}

/// Fixed fullscreen semi-transparent modal backdrop.
pub fn modal_backdrop() -> Styles {
    Styles::builder()
        .with((Position, "fixed"))
        .with((Inset, "0"))
        .with((Background, "rgba(0,0,0,0.5)"))
        .with((ZIndex, "1000"))
        .with((Display, "flex"))
        .with((AlignItems, "center"))
        .with((JustifyContent, "center"))
        .build()
}

/// White card panel for modal content.
pub fn modal_panel() -> Styles {
    Styles::builder()
        .with((Background, "white"))
        .with((Padding, "2em"))
        .with((BorderRadius, "12px"))
        .with((MaxWidth, "400px"))
        .with((Width, "90%"))
        .with((BoxShadow, "0 4px 20px rgba(0,0,0,0.3)"))
        .build()
}

/// Modal title styling.
pub fn modal_title() -> Styles {
    Styles::builder()
        .with((Margin, "0 0 0.5em 0"))
        .with((Color, "#333"))
        .build()
}

/// Danger variant modal title.
pub fn modal_title_danger() -> Styles {
    Styles::builder()
        .with((Margin, "0 0 0.5em 0"))
        .with((Color, "#dc3545"))
        .build()
}

/// Modal description text.
pub fn modal_description() -> Styles {
    Styles::builder()
        .with((Margin, "0 0 1.5em 0"))
        .with((Color, "#666"))
        .build()
}

/// Inline confirm/action button inside modal.
pub fn modal_action_button_primary() -> Styles {
    Styles::builder()
        .with((Padding, "0.5em 1em"))
        .with((BorderRadius, "4px"))
        .with((Cursor, "pointer"))
        .with((Background, "var(--brand-color)"))
        .with((Color, "white"))
        .with((Border, "none"))
        .build()
}

/// Danger action button inside modal.
pub fn modal_action_button_danger() -> Styles {
    Styles::builder()
        .with((Padding, "0.5em 1em"))
        .with((BorderRadius, "4px"))
        .with((Cursor, "pointer"))
        .with((Background, "#dc3545"))
        .with((Color, "white"))
        .with((Border, "none"))
        .build()
}

/// Dark button for tooltip demos.
pub fn demo_button_dark() -> Styles {
    Styles::builder()
        .with((Padding, "0.5em 1em"))
        .with((BorderRadius, "6px"))
        .with((Cursor, "pointer"))
        .with((Background, "#555"))
        .with((Color, "white"))
        .with((Border, "none"))
        .with((FontSize, "0.9em"))
        .build()
}

/// Popover content panel (white bg, border, shadow).
pub fn popover_panel() -> Styles {
    Styles::builder()
        .with((Background, "white"))
        .with((Border, "1px solid #ccc"))
        .with((BorderRadius, "8px"))
        .with((Padding, "1em"))
        .with((BoxShadow, "0 4px 12px rgba(0,0,0,0.15)"))
        .with((ZIndex, "1000"))
        .with((MaxWidth, "300px"))
        .build()
}

/// Overlay basic demo panel (fixed, centered, white bg, shadow).
pub fn overlay_panel() -> Styles {
    Styles::builder()
        .with((Position, "fixed"))
        .with((Top, "50%"))
        .with((Left, "50%"))
        .with((Transform, "translate(-50%, -50%)"))
        .with((Background, "white"))
        .with((Border, "1px solid #ccc"))
        .with((BorderRadius, "8px"))
        .with((Padding, "1.5em"))
        .with((BoxShadow, "0 4px 12px rgba(0,0,0,0.15)"))
        .with((ZIndex, "1000"))
        .with((MaxWidth, "300px"))
        .build()
}

/// Flex center container for demos.
pub fn demo_flex_center() -> Styles {
    Styles::builder()
        .with((Display, "flex"))
        .with((JustifyContent, "center"))
        .with((Padding, "2em"))
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
