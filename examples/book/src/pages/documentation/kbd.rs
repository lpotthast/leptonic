use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;
use strum::IntoEnumIterator;

#[component]
pub fn PageKbd(cx: Scope) -> impl IntoView {
    let all_keys = Key::iter();

    view! { cx,
        <H1>"Keyboard"</H1>

        <P>
            "Display labeled keyboard key-caps using the "<Code inline=true>"<Kbd>"</Code>" component."
            " Leptonic provides the "<Code inline=true>"Key"</Code>" enum which provides well-known keys and their display properties."
        </P>

        <Code>
            {indoc!(r#"
                <Kbd key=Key::Option/>
            "#)}
        </Code>

        <Kbd key=Key::Option/>

        <H2>"Shortcuts"</H2>

        <P>
            "You may use this component to display a hint to a keyboard shortcut your users can use to interact with your app."
            " As shortcuts mostly consist of two or more keys, Leptonic also provide the "<Code inline=true>"<KbdShortcut>"</Code>
            " component to make this task as easy as possible. Simply provide the keys which must be pressed in order to activate the shortcut."
        </P>

        <P>"Note that these component do not listen for key-presses. Their sole purpose is to unify rendering of key caps and shortcuts!"</P>

        <Code>
            {indoc!(r#"
                <KbdShortcut keys=[Key::Command, Key::Enter]/>
            "#)}
        </Code>

        <KbdShortcut keys=[Key::Command, Key::Enter]/>

        <P>"This could also be rendered manually using the following markup."</P>

        <Code>
            {indoc!(r#"
                <KbdShortcutRoot>
                    <Kbd key=Key::Command/>
                    <KbdConcatenate with="+"/>
                    <Kbd key=Key::Enter/>
                </KbdShortcutRoot>
            "#)}
        </Code>

        <KbdShortcutRoot>
            <Kbd key=Key::Command/>
            <KbdConcatenate with="+"/>
            <Kbd key=Key::Enter/>
        </KbdShortcutRoot>

        <H2>Keys</H2>

        <P>"Here is a list of all keys provided by the "<Code inline=true>"Key"</Code>" enum."</P>

        {
            all_keys
                .into_iter()
                .filter(|key| std::mem::discriminant(key) != std::mem::discriminant(&Key::Custom("")))
                .map(|key: Key| view! {cx,
                    <Kbd key=key/>
                })
                .collect_view(cx)
        }

        <P>"If you need custom content in a "<Code inline=true>"<Kbd>"</Code>" element, use the "<Code inline=true>"Key::Custom(String)"</Code>" variant."</P>

        <Code>
            {indoc!(r#"
                <Kbd key=Key::Custom("Foo")/>
            "#)}
        </Code>

        <Kbd key=Key::Custom("Foo")/>

        <H2>"Styling"</H2>

        <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

        <Code>
            {indoc!(r#"
                --leptonic-kbd-key-color
                --leptonic-kbd-key-background-color
                --leptonic-kbd-key-margin
                --leptonic-kbd-key-padding
                --leptonic-kbd-key-border-radius
                --leptonic-kbd-key-border-color
                --leptonic-kbd-concatenate-color
                --leptonic-kbd-concatenate-background-color
                --leptonic-kbd-concatenate-margin
                --leptonic-kbd-concatenate-padding
                --leptonic-kbd-concatenate-border-radius
            "#)}
        </Code>
    }
}
