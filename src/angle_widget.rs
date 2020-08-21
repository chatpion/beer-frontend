use orbtk::prelude::*;
use super::angle_event::AngleEvent;
use super::data::Angle;
use super::widgets::numeric_text_box::NumericTextBox;

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
    angle_type: AngleType
}

fn first_symbol(first_angle: bool) -> &'static str {
    if first_angle {"°"} else {"h"}
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
}


impl State for AngleViewState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.angle = Angle(0, 0, 0);
        angle_view(ctx.widget()).set_value1("0");
        angle_view(ctx.widget()).set_value2("0");
        angle_view(ctx.widget()).set_value3("0");

        self.angle_type = *ctx.widget().get::<AngleType>("angle_type");

        let first_angle = *ctx.widget().get::<bool>("first_angle");
        angle_view(ctx.widget()).set_value1_suffix(first_symbol(first_angle));
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        match &self.action {
            Some(Update) => self.send_event(ctx), 
            _ => ()
        }
        self.action = None;
    }
}

widget!(AngleView<AngleViewState> {
    /// If true, then value1 is an angle, else it is an hour
    first_angle: bool,
    angle_type: AngleType,
    value1_suffix: String16,
    value1: String16,
    value2: String16,
    value3: String16
});


impl Template for AngleView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("Angleview").child(
            Stack::new().orientation("horizontal")
            .child(
                TextBox::new()
                    .text(("value1", id))
                    .max_width(50)
                    .margin((5, 0, 5, 0))
                    .build(ctx)
            ).child(
                TextBlock::new().text(("value1_suffix", id)).font_size(25).build(ctx)
            ).child(
                TextBox::new()
                    .text(("value2", id))
                    .max_width(50)
                    .margin((5, 0, 5, 0))
                    .build(ctx)
            ).child(
                TextBlock::new().text("'").font_size(25).build(ctx)
            ).child(
                NumericTextBox::new()
                    .text(("value3", id))
                    .suffix("\"")
                    .max_width(50)
                    .margin((5, 0, 5, 0))
                    .build(ctx)
            )/*.child(
                TextBlock::new().text("\"").font_size(25).build(ctx)
            )*/.build(ctx)
        )
    }
}
