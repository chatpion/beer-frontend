use orbtk::prelude::*;

// hacky button
widget!(TinyButton {
    font_size: f64,
    text: String16
});


impl Template for TinyButton {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("HackyButton").child(
            Button::new().style("tiny_button")
                .font_size(id)
                .text(id)
                .height(16)
                .max_width(16)
                .spacing(0)
                .build(ctx)
        )
    }
}

#[derive(Default, AsAny)]
pub struct NumericTextBoxState {
    value: isize,
    min_value: isize,
    max_value: isize
}

impl State for NumericTextBoxState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {

    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {

    }
}

widget!(NumericTextBox<NumericTextBoxState> {
    min_value: isize,
    max_value: isize,
    text: String16,
    suffix: String16
});

impl Template for NumericTextBox {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("NumericTextBox")
            .child(Stack::new().orientation("horizontal")
                .child(TextBox::new()
                    .text(("text", id))
                    .max_width(50)
                    .max_height(40)
                    .width(50)
                    .margin((5, 0, 5, 0))
                    .build(ctx))
                .child(TextBlock::new()
                    .text(("suffix", id))
                    .font_size(25)
                    .max_width(15)
                    .max_height(40)
                    .build(ctx))
                .child(Stack::new()
                    .orientation("vertical")
                    .child(TinyButton::new()
                        .text("+")
                        .build(ctx))
                    .child(TinyButton::new()
                        .text("-")
                        .build(ctx))
                    .build(ctx))
                .build(ctx))
    }
}
