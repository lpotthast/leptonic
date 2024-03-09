pub mod button;
pub mod calendar;
pub mod focus;
pub mod hover;
pub mod r#move;
pub mod overlay;
pub mod popover;
pub mod press;
pub mod scroll;
pub mod tooltip;

pub mod prelude {
    pub use super::button::use_button;
    pub use super::button::UseButtonProps;
    pub use super::button::UseButtonReturn;
    pub use super::focus::use_focus;
    pub use super::focus::UseFocusInput;
    pub use super::focus::UseFocusReturn;
    pub use super::hover::use_hover;
    pub use super::hover::UseHoverInput;
    pub use super::hover::UseHoverReturn;
    pub use super::overlay::use_overlay;
    pub use super::overlay::use_overlay_position;
    pub use super::overlay::use_overlay_trigger;
    pub use super::overlay::UseOverlayInput;
    pub use super::overlay::UseOverlayPositionInput;
    pub use super::overlay::UseOverlayPositionReturn;
    pub use super::overlay::UseOverlayReturn;
    pub use super::overlay::UseOverlayTriggerInput;
    pub use super::overlay::UseOverlayTriggerReturn;
    pub use super::press::use_press;
    pub use super::press::UsePressInput;
    pub use super::press::UsePressReturn;
    pub use super::r#move::use_move;
    pub use super::r#move::UseMoveInput;
    pub use super::r#move::UseMoveReturn;
}

fn disable_text_selection(element: web_sys::Element) {
    match element.set_attribute("data-disable-user-select", "true") {
        Ok(_ok) => {}
        Err(err) => {
            tracing::warn!(?err, "Could not set 'data-disable-user-select' attribute.");
        }
    }
}

fn restore_text_selection(element: web_sys::Element) {
    match element.remove_attribute("data-disable-user-select") {
        Ok(_ok) => {}
        Err(err) => {
            tracing::warn!(
                ?err,
                "Could not remove 'data-disable-user-select' attribute."
            );
        }
    }
}
