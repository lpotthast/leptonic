use leptos::*;

use crate::prelude::*;

#[component]
pub fn Root<T>(cx: Scope, default_theme: T, children: Children) -> impl IntoView
where
    T: Theme + 'static,
{
    view! {cx,
        <ThemeProvider theme=create_signal_ls(cx, "theme", default_theme)>
            <ToastRoot>
                <ModalRoot>
                    <Box style="min-height: 100vh; min-width: 100vw; display: flex; flex-direction: row;">
                        { children(cx) }
                    </Box>
                </ModalRoot>
            </ToastRoot>
        </ThemeProvider>
    }
}
