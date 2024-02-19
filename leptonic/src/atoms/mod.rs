use std::rc::Rc;

use leptos::Attribute;
use leptos_reactive::Oco;

pub mod button;
pub mod button_group;

pub mod prelude {
    pub use super::button::Button;
    pub use super::button_group::ButtonGroup;
}

trait AttributeExt {
    fn prepend(self, string: Oco<'static, str>) -> Self;
}

impl AttributeExt for Attribute {
    fn prepend(self, string: Oco<'static, str>) -> Self {
        match self {
            Attribute::String(s) => Attribute::String(Oco::Owned(format!("{string} {}", s))),
            Attribute::Fn(f) => Attribute::Fn(Rc::new(move || f().prepend(string.clone()))),
            Attribute::Option(o) => {
                Attribute::Option(o.map(|s| Oco::Owned(format!("{string} {}", s))))
            }
            Attribute::Bool(_) => panic!("Cannot prepend something to an Attribute::Bool."),
        }
    }
}
