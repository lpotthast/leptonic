use leptos::prelude::*;

/// The theme's text styles, shown on plain text. In an app, write the elements themselves (`<h1>`, `<p>`):
/// the theme styles them without classes.
#[component]
pub fn TypographyDemo() -> impl IntoView {
    view! {
        <ul class="demo-type-scale">
            <li class="demo-type-h1">"Heading 1"</li>
            <li class="demo-type-h2">"Heading 2"</li>
            <li class="demo-type-h3">"Heading 3"</li>
            <li class="demo-type-h4">"Heading 4"</li>
            <li class="demo-type-h5">"Heading 5"</li>
            <li class="demo-type-h6">"Heading 6"</li>
            <li class="demo-type-p">"Paragraph text, as in a long description."</li>
        </ul>
    }
}
