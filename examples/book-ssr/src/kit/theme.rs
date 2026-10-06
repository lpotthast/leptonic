use leptonic::components::prelude::{Code, Language};
use leptos::prelude::*;

/// Source of a leptonic theme stylesheet, e.g. `theme_scss!("button")` for `components/button.scss`.
///
/// Reads the copy that leptonic's build script places in `style/leptonic/`.
#[macro_export]
macro_rules! theme_scss {
    ($component:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/style/leptonic/components/",
            $component,
            ".scss"
        ))
    };
}

/// The CSS custom properties starting with `prefix` that the stylesheet `scss` reads, in order of first use.
///
/// Generated from the theme instead of listed by hand, so the list can't fall out of date.
#[component]
pub fn CssVariables(
    /// For example `"--button-"`.
    prefix: &'static str,
    /// The stylesheet, usually `theme_scss!("<component>")`.
    scss: &'static str,
) -> impl IntoView {
    view! { <Code language=Language::Css>{css_variables(prefix, scss).join("\n")}</Code> }
}

fn css_variables(prefix: &str, scss: &str) -> Vec<String> {
    let mut variables: Vec<String> = Vec::new();
    for (start, _) in scss.match_indices(prefix) {
        let name: String = scss[start..]
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
            .collect();
        if !variables.contains(&name) {
            variables.push(name);
        }
    }
    variables
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::css_variables;

    #[test]
    fn collects_unique_variables_in_order() {
        let scss = ".a { color: var(--btn-color); border: var(--btn-border-size) solid var(--btn-color); }";
        assert_that!(css_variables("--btn-", scss)).is_equal_to(vec![
            "--btn-color".to_owned(),
            "--btn-border-size".to_owned(),
        ]);
    }
}
