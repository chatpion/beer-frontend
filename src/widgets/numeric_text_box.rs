use orbtk::prelude::*;


static ID_INPUT: &str = "numeric_text_box_input";


enum Action {
    Inc, 
    Dec,
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

    input_entity: Entity
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
        ctx.get_widget(self.input_entity).set::<String16>("text", String16::from(self.value.to_string()));
    }
}

impl State for NumericTextBoxState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.value = 0;
        self.max_value = *ctx.widget().get::<usize>("max_value") as i32;
        self.neg_value = *ctx.widget().get::<bool>("neg_value"); 
        self.min_value = self.min_value();
        self.input_entity = ctx.entity_of_child(ID_INPUT).expect("NumericBoxTextState.init(): the child input could not be found!");
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
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
                Action::Dec => self.add_step_value(self.value - 1, ctx)
            }
        }
        self.action = None;
    }
}

widget!(NumericTextBox<NumericTextBoxState> {
    neg_value: bool,
    max_value: usize,
    max: f64,
    text: String16,
    suffix: String16,
    underflow: bool,
    overflow: bool,
    should_inc: bool,
    should_dec: bool
});


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
            .child(Stack::new().orientation("horizontal")
                .child(TextBox::new()
                    .id(ID_INPUT)
                    .text(("text", id))
                    .max_width(50)
                    .max_height(40)
                    .width(50)
                    .margin((2, 0, 2, 0))
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


