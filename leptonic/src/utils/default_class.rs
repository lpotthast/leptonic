use leptos_classes::{Classes, MergeStrategy};

/// An atom's classes: its default class (`leptonic-<AtomName>`, what the atom theme styles)
/// followed by the caller's `classes` (react-aria-components' `defaultClassName`).
///
/// The caller's classes add to the default instead of replacing it (react-aria-components: a
/// `className` replaces it), so a themed element can be adjusted with a class of its own. A caller
/// passing the default class again is fine.
#[must_use]
pub fn with_default_class(default: &'static str, classes: Classes) -> Classes {
    Classes::new()
        .add(default)
        .merge(classes, MergeStrategy::UnionConditions)
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn default_comes_first_and_duplicates_merge() {
        let classes = with_default_class(
            "leptonic-Button",
            Classes::new().add("primary").add("leptonic-Button"),
        );
        assert_that!(classes.to_class_string()).is_equal_to("leptonic-Button primary".to_owned());
    }
}
