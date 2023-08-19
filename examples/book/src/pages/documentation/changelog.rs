use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageChangelog(cx: Scope) -> impl IntoView {
    view! { cx,
        <H1>"Changelog"</H1>

        <H2>"0.2.0"</H2>

        <H3>"Added:"</H3>
        <ul>
            <li>
                "Added the 'Out' type. An enum abstracting over 'Callback's and 'WriteSignal's."
                " Components can use this type when it is equally likely that a user will provide either of the previously mentioned types."
                " In case of the WriteSignal, the user just wants a new value to be stored."
                " In case of a callback, the user wants fine control over how a new value is handled."
                " The input component is the first one using it, as mentioned in the 'Changed' section."
            </li>
        </ul>

        <H3>"Changed:"</H3>
        <ul>
            <li>"The DateSelector components on_change prop now takes a Callback instead of a generic function."</li>
            <li>"Buttons of type 'outlined' now use --button-outlined-[color]-... variables for their styling."</li>
            <li>"Buttons of type 'filled' now use --button-filled-[color]-... variables for their styling."</li>
            <li>"Buttons of type 'outlined' and color 'primary' now use a dark text color, contrasting the default bright background."</li>
            <li>"When using an input of type 'Number', you now have to supply optional 'min', 'max' and 'step' values which are propagated to the underlying input element."</li>
            <li>"The inputs 'set' prop is now optional."</li>
            <li>"The inputs 'set' prop is no longer generic. It now expects an 'Out<String>', which can either be a 'WriteSignal' or a custom 'Callback'."</li>
        </ul>

        <H3>"Fixed:"</H3>
        <ul>
            <li>"A button with variants now properly respects its disabled state."</li>
            <li>"A button with variants now only triggers one of its actions (either main or variant) per interaction."</li>
            <li>"Buttons of type 'flat' and color 'info' now receive correct styling."</li>
        </ul>

        <H2>"0.1.0"</H2>

        <P>"Initial release."</P>

        <H3>"Added utilities:"</H3>
        <ul>
            <li>"Callback types"</li>
            <li>"OptionalMaybeSignal type"</li>
            <li>"Global event listener contexts"</li>
        </ul>

        <H3>"Added components:"</H3>
        <ul>
            <li>"Root component"</li>
            <li>"Skeleton component and styles"</li>
            <li>"Stack component and styles"</li>
            <li>"Grid component and styles"</li>
            <li>"Separator component and styles"</li>
            <li>"Tab components and styles"</li>
            <li>"Collapsible components and styles"</li>
            <li>"AppBar components and styles"</li>
            <li>"Drawer components and styles"</li>
            <li>"Button component and styles"</li>
            <li>"Input component and styles"</li>
            <li>"Date selector component and styles"</li>
            <li>"Slider component and styles"</li>
            <li>"Select component and styles"</li>
            <li>"Toggle component and styles"</li>
            <li>"Alert component and styles"</li>
            <li>"Toast component and styles"</li>
            <li>"Modal components and styles"</li>
            <li>"Progress component and styles"</li>
            <li>"Popover component and styles"</li>
            <li>"Chip component and styles"</li>
            <li>"Icon component and styles"</li>
            <li>"Link component and styles"</li>
            <li>"Anchor component and styles"</li>
            <li>"Typography components and styles"</li>
            <li>"Transition components and styles"</li>
        </ul>
    }
}
