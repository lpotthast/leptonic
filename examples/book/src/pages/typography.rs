use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageTypography(cx: Scope) -> impl IntoView {
    view! { cx,
        <Typography variant=TypographyVariant::H2>"Typography"</Typography>

        <Separator />

        <Typography variant=TypographyVariant::H1>"Typography - H1"</Typography>
        <Typography variant=TypographyVariant::H2>"Typography - H2"</Typography>
        <Typography variant=TypographyVariant::H3>"Typography - H3"</Typography>
        <Typography variant=TypographyVariant::H4>"Typography - H4"</Typography>
        <Typography variant=TypographyVariant::H5>"Typography - H5"</Typography>
        <Typography variant=TypographyVariant::H6>"Typography - H6"</Typography>

        <Typography variant=TypographyVariant::Paragraph>"This is a paragraph"</Typography>

        <Typography variant=TypographyVariant::Code {  inline: false }>"Typography - Code"</Typography>

        <Typography variant=TypographyVariant::Paragraph>
            "This is a paragraph containing an "
            <Typography variant=TypographyVariant::Code {  inline: true }>"inlined"</Typography>
            " piece of code."
        </Typography>
    }
}
