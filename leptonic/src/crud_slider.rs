use std::rc::Rc;

use tracing::{trace, warn};
use uuid::Uuid;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{
    event_functions::MouseEventFunctions,
    stores::{
        global_mouse_move::{GlobalMouseMove, GlobalMouseMoveRequired},
        global_mouse_up::GlobalMouseUp,
    },
};

pub enum Msg {
    MouseDown(MouseEvent),
    GlobalMouseMove(Rc<GlobalMouseMove>),
    GlobalMouseUp(Rc<GlobalMouseUp>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style {
    Block,
    Round,
}

// TODO: This con be computed statically!
impl From<Style> for Classes {
    fn from(size: Style) -> Self {
        match size {
            Style::Block => classes!(""),
            Style::Round => classes!("round"), // TODO: is this even necessary?
        }
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct CrudSliderProps {
    pub min: f64,
    pub max: f64,
    pub step: f64,
    #[prop_or_default]
    pub value: f64,
    #[prop_or(Style::Round)]
    pub style: Style,
    #[prop_or(false)]
    pub active: bool,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or_default]
    pub on_change: Callback<f64>,
}

pub struct CrudSlider {
    _global_mouse_up_dispatch: Dispatch<GlobalMouseUp>,
    _global_mouse_move_dispatch: Dispatch<GlobalMouseMove>,
    global_mouse_move_required_dispatch: Dispatch<GlobalMouseMoveRequired>,
    uuid: Uuid,
    id: String,
    listen_for_mouse_movement: bool,
    value: f64,
    min: f64,
    max: f64,
    step: f64,
}

impl CrudSlider {
    fn set_stepped_value(&mut self, new_value: f64) {
        self.value = (new_value / self.step).round() * self.step
    }

    fn get_range(&self) -> f64 {
        self.min.abs() + self.max.abs()
    }

    fn value_in_range(&self, percentage: f64) -> f64 {
        percentage * self.get_range() + self.min
    }

    fn percentage_in_range(&self) -> f64 {
        (self.value + self.min.abs()) / self.get_range()
    }
}

impl Component for CrudSlider {
    type Message = Msg;
    type Properties = CrudSliderProps;

    fn create(ctx: &Context<Self>) -> Self {
        let uuid = Uuid::new_v4();
        let id = format!("crud-slider-{}", uuid);
        Self {
            _global_mouse_up_dispatch: Dispatch::subscribe(ctx.link().callback(Msg::GlobalMouseUp)),
            _global_mouse_move_dispatch: Dispatch::subscribe(
                ctx.link().callback(Msg::GlobalMouseMove),
            ),
            global_mouse_move_required_dispatch: Dispatch::new(),
            uuid,
            id,
            listen_for_mouse_movement: false,
            value: ctx.props().value,
            min: ctx.props().min,
            max: ctx.props().max,
            step: ctx.props().step,
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {}

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GlobalMouseMove(gmm) => {
                if self.listen_for_mouse_movement {
                    if let Some(event) = gmm.latest_event() {
                        return self.process_event(event, |new_value| {
                            ctx.props().on_change.emit(new_value)
                        });
                    }
                }
                false
            }
            Msg::MouseDown(event) => {
                self.enable_global_mouse_move_tracking();
                self.listen_for_mouse_movement = true;
                self.process_event(&event, |new_value| ctx.props().on_change.emit(new_value))
            }
            Msg::GlobalMouseUp(_new_state) => {
                self.disable_global_mouse_move_tracking();
                self.listen_for_mouse_movement = false;
                false
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.value = ctx.props().value;
        self.min = ctx.props().min;
        self.max = ctx.props().max;
        self.step = ctx.props().step;

        if self.value < self.min {
            warn!(
                "Slider was given the value {} which is outside the current range {}-{}",
                self.value, self.min, self.max
            );
            self.value = self.min;
        }
        if self.value > self.max {
            warn!(
                "Slider was given the value {} which is outside the current range {}-{}",
                self.value, self.min, self.max
            );
            self.value = self.max;
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        trace!("render");
        let knob_transform = format!("left: {}%", self.percentage_in_range() * 100.0);
        html! {
            <div
                class={classes!(
                    "crud-slider",
                    "round",
                    ctx.props().style,
                    ctx.props().active.then(|| "active"),
                    ctx.props().disabled.then(|| "disabled")
                )}
                onmousedown={&ctx.link().callback(Msg::MouseDown)}
            >
                <div id={self.id.clone()} class={"bar"}>
                    <div class={"knob-wrapper"}>
                        <div class={"knob"} style={knob_transform}></div>
                    </div>
                </div>
            </div>
        }
    }
}

impl CrudSlider {
    fn enable_global_mouse_move_tracking(&mut self) {
        let uuid = self.uuid;
        self.global_mouse_move_required_dispatch
            .reduce_mut(move |state| state.require_by(uuid));
    }
    fn disable_global_mouse_move_tracking(&mut self) {
        let uuid = self.uuid;
        self.global_mouse_move_required_dispatch
            .reduce_mut(move |state| state.not_require_by(uuid));
    }
    /// Returns whether or not the component was really updated.
    /// Call the given on_change function if self.value was modified.
    fn process_event(&mut self, event: &MouseEvent, on_change: impl Fn(f64)) -> bool {
        if let Some(relative) = event.comp_rel_pos_percent_target_element_with_id(&self.id) {
            self.set_stepped_value(self.value_in_range(relative.0));
            on_change(self.value);
            true
        } else {
            false
        }
    }
}
