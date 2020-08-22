use orbtk::prelude::*;


static ID_INPUT: &str = "numeric_text_box_input";


enum Action {
    Inc, 
    Dec
}


#[derive(Default, AsAny)]
pub struct NumericTextBoxState {
    /// true if negative values are allowed
    neg_value: bool,
    
    min_value: i32,
    /// max
    max_value: i32,

    /// current value, clamped between 0 and max_value if neg_value, else -max_value and max_value
    value: i32,

    action: Option<Action>,

    input_entity: Entity, 

    valid: bool
}

impl NumericTextBoxState {
    fn action(&mut self, action: Action) {
        self.action = Some(action);
    }

    fn min_value(&self) -> i32 {
        if self.neg_value {
            -(self.max_value as i32) + 1
        } else {
            0
        }
    }

    fn clamp_value(&mut self) {
        if self.value < self.min_value {
            self.value = self.min_value;
        } else if self.value >= self.max_value {
            self.value = self.max_value - 1;
        }
    }

    fn add_step_value(&mut self, new_value: i32, ctx: &mut Context) {
        if new_value < self.min_value {
            ctx.widget().set("underflow", true);
            self.value = self.max_value - 1;
        } else if new_value >= self.max_value {
            ctx.widget().set("overflow", true);
            self.value = self.min_value;
        } else {
            self.value = new_value;
        }
    }

    fn check_validity(&mut self, ctx: &mut Context) {
        let text = ctx.widget().get::<String16>("text").clone().as_string();
        self.value = match text.parse::<i32>() {
            Ok(v) => {self.valid = true; v}
            Err(_) => {self.valid = false; self.value}
        };
        self.clamp_value();
        ctx.widget().set::<bool>("valid", self.valid);
    }
}

impl State for NumericTextBoxState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.value = 0;
        self.max_value = *ctx.widget().get::<usize>("max_value") as i32;
        self.neg_value = *ctx.widget().get::<bool>("neg_value"); 
        self.min_value = self.min_value();
        self.input_entity = ctx.entity_of_child(ID_INPUT).expect("NumericBoxTextState.init(): the child input could not be found!");

        self.valid = false;
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.check_validity(ctx);

        if self.valid {
            if *ctx.widget().get::<bool>("should_inc") {
                self.add_step_value(self.value + 1, ctx);
            }
            if *ctx.widget().get::<bool>("should_dec") {
                self.add_step_value(self.value - 1, ctx);
            }

            ctx.widget().set("should_inc", false);
            ctx.widget().set("should_dec", false);

            if let Some(action) = &self.action {
                match action {
                    Action::Inc => self.add_step_value(self.value + 1, ctx),
                    Action::Dec => self.add_step_value(self.value - 1, ctx), 
                }
            }
            self.action = None;
        
            ctx.widget().set::<String16>("text", String16::from(self.value.to_string()));
            ctx.widget().set::<i32>("value", self.value);

            ctx.widget().set::<Brush>("background", "#3b434a".into());
        } else {
            ctx.widget().set::<Brush>("background", "#ff0000".into());
        }
    }
}

widget!(NumericTextBox<NumericTextBoxState> {
    /// true if value can be negative
    neg_value: bool,

    /// max possible value (exclusive upper bound)
    max_value: usize,

    value: i32,

    /// text of the TextBox
    text: String16,

    /// text after the TextBox. Used for the units
    suffix: String16,

    /// true if an underflow happened
    underflow: bool,

    /// true if an overflow happened
    overflow: bool,

    /// true if value must be increased (used for carries)
    should_inc: bool,

    /// true if value must be decreased (used for carries)
    should_dec: bool,

    /// background of the TextBox
    background: Brush,

    /// true if TextBox is focused
    focused: bool,

    /// true if value is valid (i.e. is a number and within the correct bounds)
    valid: bool
});


/// Generate a button increasing or decreasing the value
fn generate_mod_button(
    ctx: &mut BuildContext,
    id: Entity,
    positive: bool) -> Entity {
    Button::new()
        .style("button_small")
        .icon(if positive {material_icons_font::MD_KEYBOARD_ARROW_UP} else {material_icons_font::MD_KEYBOARD_ARROW_DOWN})
        .on_click(move |ctx, _| {
            if positive {
                state(id, ctx).action(Action::Inc)
            } else {
                state(id, ctx).action(Action::Dec)
            }
            true
        })
        .max_width(14)
        .height(15)
        .build(ctx)
}

impl Template for NumericTextBox {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("NumericTextBox")
            .overflow(false)
            .underflow(false)
            .should_inc(false)
            .should_dec(false)
            .background("#3b434a")
            .child(Stack::new().orientation("horizontal")
                .child(TextBox::new()
                    .id(ID_INPUT)
                    .text(("text", id))
                    .max_width(50)
                    .max_height(40)
                    .width(50)
                    .margin((2, 0, 2, 0))
                    .background(id)
                    .on_changed_filter(vec!["text, focus"])
                    .lost_focus_on_activation(true)
                    .focused(id)
                    .build(ctx))
                .child(TextBlock::new()
                    .text(("suffix", id))
                    .font_size(25)
                    .max_width(15)
                    .max_height(40)
                    .margin((2, 0, 2, 0))
                    .build(ctx))
                .child(Stack::new()
                    .orientation("vertical")
                    .child(generate_mod_button(ctx, id, true))
                    .child(generate_mod_button(ctx, id, false))
                    .build(ctx))
                .build(ctx))
    }
}

// helper to request MainViewState
fn state<'a>(id: Entity, states: &'a mut StatesContext) -> &'a mut NumericTextBoxState {
    states.get_mut(id)
}


