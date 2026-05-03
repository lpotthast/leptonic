pub trait ToStaticStrRepr {
    fn to_static_str_repr(&self) -> &'static str;
}

impl ToStaticStrRepr for bool {
    fn to_static_str_repr(&self) -> &'static str {
        match self {
            true => "true",
            false => "false",
        }
    }
}
