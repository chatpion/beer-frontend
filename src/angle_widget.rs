use orbtk::prelude::*;
use super::angle_event::{AngleEvent};
use super::data::Angle;
use super::widgets::numeric_text_box::NumericTextBox;


// --- KEYS ---

static HIGH_INPUT: &str = "high_input";
static MID_INPUT: &str = "mid_input";
static LOW_INPUT: &str = "low_input";


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AngleType {
    RightAsc, Declination, None
}

impl Default for AngleType {
    fn default() -> Self {
        AngleType::None
    }
}

enum Action {
    Update
}

#[derive(Default, AsAny)]
pub struct AngleViewState {
    angle: Angle,
    action: Option<Action>,
    angle_type: AngleType,

    high_input: Entity,
    mid_input: Entity,
    low_input: Entity
}

fn first_symbol(first_angle: bool) -> &'static str {
    if first_angle {"°"} else {"h"}
}

fn first_max_value(first_angle: bool) -> usize {
    if first_angle { 360 } else { 24 }
}

impl AngleViewState { 
    fn action(&mut self, action: Action) {
        self.action = Some(action);
    }

    fn send_event(&mut self, ctx: &mut Context) {
        let event = match self.angle_type {
            AngleType::RightAsc => Some(AngleEvent::UpdateRightAsc(self.angle)),
            AngleType::Declination => Some(AngleEvent::UpdateDeclination(self.angle)),
            AngleType::None => None
        }; 
        if let Some(e) = event {
            ctx.push_event(e);
        }
    } 

    fn handle_carries(&mut self, ctx: &mut Context) {
        if *ctx.get_widget(self.low_input).get::<bool>("underflow") {
            ctx.get_widget(self.low_input).set("underflow", false);
            ctx.get_widget(self.mid_input).set("should_dec", true);
            self.send_event(ctx);
        }
        if *ctx.get_widget(self.low_input).get::<bool>("overflow") {
            ctx.get_widget(self.low_input).set("overflow", false);
            ctx.get_widget(self.mid_input).set("should_inc", true);
            self.send_event(ctx);
        }
        if *ctx.get_widget(self.mid_input).get::<bool>("underflow") {
            ctx.get_widget(self.mid_input).set("underflow", false);
            ctx.get_widget(self.high_input).set("should_dec", true);
            self.send_event(ctx);
        }
        if *ctx.get_widget(self.mid_input).get::<bool>("overflow") {
            ctx.get_widget(self.mid_input).set("overflow", false);
            ctx.get_widget(self.high_input).set("should_inc", true);
            self.send_event(ctx);
        }
    }

    fn check_validity(&mut self, ctx: &mut Context) {
        let valid = *ctx.get_widget(self.low_input).get::<bool>("valid") 
            && *ctx.get_widget(self.mid_input).get::<bool>("valid")
            && *ctx.get_widget(self.high_input).get::<bool>("valid");

        println!("{}", valid);

        ctx.widget().set::<bool>("valid", valid);
    }
}


impl State for AngleViewState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        // initialize the inputs to zero
        self.angle = Angle(0, 0, 0);
        angle_view(ctx.widget()).set_value1("0");
        angle_view(ctx.widget()).set_value2("0");
        angle_view(ctx.widget()).set_value3("0");

        self.angle_type = *ctx.widget().get::<AngleType>("angle_type");

        // set variables according to the kind of angle (hour or degree)
        let first_angle = *ctx.widget().get::<bool>("first_angle");
        angle_view(ctx.widget()).set_value1_suffix(first_symbol(first_angle));
        angle_view(ctx.widget()).set_value1_max(first_max_value(first_angle));

        // fetch the inputs
        self.high_input = ctx.entity_of_child(HIGH_INPUT)
            .expect("AngleViewState.init(): the child high input could not be found!");
        self.mid_input = ctx.entity_of_child(MID_INPUT)
            .expect("AngleViewState.init(): the child mid input could not be found!");
        self.low_input = ctx.entity_of_child(LOW_INPUT)
            .expect("AngleViewState.init(): the child low input could not be found!");
    } 

    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        match &self.action {
            Some(Update) => self.send_event(ctx), 
            _ => ()
        }
        self.action = None;

        self.handle_carries(ctx);
        self.check_validity(ctx);
    }
}

widget!(AngleView<AngleViewState> {
    /// If true, then value1 is an angle, else it is an hour
    first_angle: bool,
    angle_type: AngleType,

    // automatically set
    value1_suffix: String16,
    value1_max: usize,
    value1: String16,
    value2: String16,
    value3: String16,
    valid: bool
});


impl Template for AngleView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("Angleview").child(
            Stack::new().orientation("horizontal")
            .child(
                NumericTextBox::new()
                    .id(HIGH_INPUT)
                    .text(("value1", id))
                    .suffix(("value1_suffix", id))
                    .neg_value(true)
                    .max_value(("value1_max", id))
                    .margin((5, 0, 5, 0))
                    .max_width(80)
                    .build(ctx)
            ).child(
                NumericTextBox::new()
                    .id(MID_INPUT)
                    .text(("value2", id))
                    .suffix("\'")
                    .neg_value(false)
                    .max_value(5)
                    .margin((5, 0, 5, 0))
                    .max_width(80)
                    .build(ctx)
            ).child(
                NumericTextBox::new()
                    .id(LOW_INPUT)
                    .text(("value3", id))
                    .suffix("\"")
                    .neg_value(false)
                    .max_value(5)
                    .margin((5, 0, 5, 0))
                    .max_width(80)
                    .build(ctx)
            )
            .build(ctx)
        )
    }
}
