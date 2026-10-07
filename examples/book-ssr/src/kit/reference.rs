use leptonic::hooks::LinkTarget;
use leptos::prelude::*;

use super::{Link, Section};

/// Where react-aria's documentation lives. Each page is at the path of its source file below
/// `packages/dev/s2-docs/pages/react-aria/` in the react-spectrum repository, without the `.mdx` extension.
const REACT_ARIA_DOCS: &str = "https://react-aria.adobe.com";

/// Hooks documented below the page of the component they belong to (`Button/useButton`), as (hook, component).
const COMPONENT_HOOKS: &[(&str, &str)] = &[
    ("useBreadcrumbs", "Breadcrumbs"),
    ("useButton", "Button"),
    ("useCalendar", "Calendar"),
    ("useCheckboxGroup", "CheckboxGroup"),
    ("useCheckbox", "Checkbox"),
    ("useColorArea", "ColorArea"),
    ("useColorField", "ColorField"),
    ("useColorSlider", "ColorSlider"),
    ("useColorSwatch", "ColorSwatch"),
    ("useColorWheel", "ColorWheel"),
    ("useComboBox", "ComboBox"),
    ("useDateField", "DateField"),
    ("useDatePicker", "DatePicker"),
    ("useDateRangePicker", "DateRangePicker"),
    ("useDisclosure", "Disclosure"),
    ("useGridList", "GridList"),
    ("useLink", "Link"),
    ("useListBox", "ListBox"),
    ("useMenu", "Menu"),
    ("useMeter", "Meter"),
    ("useModalOverlay", "Modal"),
    ("useNumberField", "NumberField"),
    ("usePopover", "Popover"),
    ("useProgressBar", "ProgressBar"),
    ("useRadioGroup", "RadioGroup"),
    ("useRangeCalendar", "RangeCalendar"),
    ("useSearchField", "SearchField"),
    ("useSelect", "Select"),
    ("useSeparator", "Separator"),
    ("useSlider", "Slider"),
    ("useSwitch", "Switch"),
    ("useTable", "Table"),
    ("useTabList", "Tabs"),
    ("useTagGroup", "TagGroup"),
    ("useTextField", "TextField"),
    ("useTimeField", "TimeField"),
    ("useToast", "Toast"),
    ("useToggleButtonGroup", "ToggleButtonGroup"),
    ("useToggleButton", "ToggleButton"),
    ("useToolbar", "Toolbar"),
    ("useTooltipTrigger", "Tooltip"),
];

/// Hooks and components with a top-level page (`usePress`, `FocusScope`).
const TOP_LEVEL_PAGES: &[&str] = &[
    "Calendar",
    "ColorSwatchPicker",
    "DateField",
    "DatePicker",
    "DateRangePicker",
    "FocusRing",
    "FocusScope",
    "I18nProvider",
    "PortalProvider",
    "RangeCalendar",
    "TimeField",
    "Toast",
    "Virtualizer",
    "VisuallyHidden",
    "useAsyncList",
    "useClipboard",
    "useCollator",
    "useContextMenu",
    "useDateFormatter",
    "useDrag",
    "useDraggableCollection",
    "useDrop",
    "useDroppableCollection",
    "useField",
    "useFilter",
    "useFocus",
    "useFocusRing",
    "useFocusVisible",
    "useFocusWithin",
    "useHover",
    "useId",
    "useIsSSR",
    "useKeyboard",
    "useLabel",
    "useLandmark",
    "useListData",
    "useListFormatter",
    "useLocale",
    "useLongPress",
    "useMove",
    "useNumberFormatter",
    "useObjectRef",
    "usePress",
    "useTreeData",
];

/// Hooks without a page of their own, documented by their component's page: (hook, page).
const DOCUMENTED_BY_COMPONENT: &[(&str, &str)] = &[("useTree", "Tree")];

/// The path of the react-aria documentation page of `name` (a hook or component), relative to [`REACT_ARIA_DOCS`].
fn doc_path(name: &str) -> Option<String> {
    COMPONENT_HOOKS
        .iter()
        .find(|(hook, _)| *hook == name)
        .map(|(hook, component)| format!("{component}/{hook}"))
        .or_else(|| TOP_LEVEL_PAGES.contains(&name).then(|| name.to_owned()))
        .or_else(|| {
            DOCUMENTED_BY_COMPONENT
                .iter()
                .find(|(hook, _)| *hook == name)
                .map(|(_, page)| (*page).to_owned())
        })
}

/// "Based on react-aria's `useX`." with a link to the react-aria documentation of `hook`.
///
/// A unit test checks that every `hook` the pages name has a documentation page.
#[component]
pub fn ReactAria(
    /// Name of the react-aria hook (or component), e.g. `"useButton"`.
    hook: &'static str,
) -> impl IntoView {
    let href = doc_path(hook).map_or_else(
        || {
            tracing::error!(
                "react-aria has no documentation page for {hook}: use `ReactAriaSource`"
            );
            format!("{REACT_ARIA_DOCS}/")
        },
        |path| format!("{REACT_ARIA_DOCS}/{path}"),
    );
    view! {
        <p>
            "Based on react-aria\u{2019}s "
            <Link href target=LinkTarget::Blank>{hook}</Link>
            "."
        </p>
    }
}

/// The package of react-spectrum a ported file comes from.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum UpstreamPackage {
    /// Behavior and accessibility hooks.
    #[default]
    ReactAria,
    /// State hooks.
    ReactStately,
}

impl UpstreamPackage {
    fn dir(self) -> &'static str {
        match self {
            Self::ReactAria => "react-aria",
            Self::ReactStately => "react-stately",
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::ReactAria => "react-aria",
            Self::ReactStately => "react-stately",
        }
    }
}

/// "Based on react-aria's `X`." with a link to the source file at `path`, for parts of react-aria (or react-stately)
/// without a documentation page (e.g. `PressResponder`). Use [`ReactAria`] whenever a documentation page exists.
#[component]
pub fn ReactAriaSource(
    /// Path below `packages/<package>/src/`, e.g. `"interactions/PressResponder.tsx"`.
    path: &'static str,
    /// The package the file is in. Default: react-aria.
    #[prop(optional)]
    package: UpstreamPackage,
) -> impl IntoView {
    let file = path.rsplit('/').next().unwrap_or(path);
    let name = file.split('.').next().unwrap_or(file);
    let dir = package.dir();
    view! {
        <p>
            "Based on "{package.name()}"\u{2019}s "
            <Link
                href=format!("https://github.com/adobe/react-spectrum/blob/main/packages/{dir}/src/{path}")
                target=LinkTarget::Blank
            >
                {name}
            </Link>
            "."
        </p>
    }
}

/// The closing "See Also" section of a page. Children are `<li>` links.
#[component]
pub fn SeeAlso(children: Children) -> impl IntoView {
    view! {
        <Section title="See Also">
            <ul>{children()}</ul>
        </Section>
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use assertr::prelude::*;

    use super::{COMPONENT_HOOKS, DOCUMENTED_BY_COMPONENT, TOP_LEVEL_PAGES, doc_path};

    /// The `hook` of every `<ReactAria hook="..."/>` in `dir` and below.
    fn named_hooks(dir: &Path, found: &mut Vec<(String, String)>) {
        for entry in fs::read_dir(dir).expect("pages are readable").flatten() {
            let path = entry.path();
            if path.is_dir() {
                named_hooks(&path, found);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                let source = fs::read_to_string(&path).expect("page is readable");
                for rest in source.split("<ReactAria hook=\"").skip(1) {
                    let hook = rest.split('"').next().unwrap_or_default();
                    found.push((path.display().to_string(), hook.to_owned()));
                }
            }
        }
    }

    #[test]
    fn links_hooks_to_their_documentation_pages() {
        assert_that!(doc_path("useButton")).is_equal_to(Some("Button/useButton".to_owned()));
        assert_that!(doc_path("usePress")).is_equal_to(Some("usePress".to_owned()));
        assert_that!(doc_path("FocusScope")).is_equal_to(Some("FocusScope".to_owned()));
        assert_that!(doc_path("useTree")).is_equal_to(Some("Tree".to_owned()));
        assert_that!(doc_path("useSpinButton")).is_none();
    }

    /// Every `<ReactAria hook="..."/>` of the pages names a hook with a documentation page.
    #[test]
    fn every_named_hook_has_a_documentation_page() {
        let mut found = Vec::new();
        named_hooks(
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("src/pages"),
            &mut found,
        );
        assert_that!(found.len()).is_greater_than(0);
        let unknown: Vec<String> = found
            .into_iter()
            .filter(|(_, hook)| doc_path(hook).is_none())
            .map(|(page, hook)| {
                format!("{page}: {hook} (no react-aria page; use `ReactAriaSource`)")
            })
            .collect();
        assert_that!(unknown).is_empty();
    }

    /// The pages of the tables exist in the react-spectrum checkout next to this repository (`~/dev/react-spectrum`,
    /// see CLAUDE.md), if there is one: catches pages upstream moved or removed.
    #[test]
    fn documentation_pages_exist_upstream() {
        let Some(home) = std::env::var_os("HOME") else {
            return;
        };
        let pages =
            Path::new(&home).join("dev/react-spectrum/packages/dev/s2-docs/pages/react-aria");
        if !pages.is_dir() {
            return;
        }
        let missing: Vec<String> = COMPONENT_HOOKS
            .iter()
            .map(|(hook, component)| format!("{component}/{hook}"))
            .chain(TOP_LEVEL_PAGES.iter().map(|&page| page.to_owned()))
            .chain(
                DOCUMENTED_BY_COMPONENT
                    .iter()
                    .map(|(_, page)| (*page).to_owned()),
            )
            .filter(|page| !pages.join(format!("{page}.mdx")).is_file())
            .collect();
        assert_that!(missing).is_empty();
    }
}
