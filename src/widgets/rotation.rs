use orbtk::prelude::*;

use crate::{
    widgets::angle::{AngleView, AngleType}
};


static RIGHT_ASC: &str = "right_asc_input";
static DECL: &str = "decl_input";


static BTN_TEXT_VALID: &str = "Tourner le téléscope";
static BTN_TEXT_INVALID: &str = "Angles de rotation invalides";


#[derive(Default, AsAny)]
pub struct RotationViewState {
    right_asc_input: Entity,
    decl_input: Entity,

    should_check_validity: bool
}

impl RotationViewState {
    fn check_validity(&mut self) {
        self.should_check_validity = true;
    }
}

impl State for RotationViewState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.right_asc_input = ctx.entity_of_child(RIGHT_ASC)
            .expect("RotationViewState.init(): the child right_asc_input could not be found!");
        self.decl_input = ctx.entity_of_child(DECL)
            .expect("RotationViewState.init(): the child decl_input could not be found!");

        self.should_check_validity = true;
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        let valid = *ctx.get_widget(self.right_asc_input).get::<bool>("valid")
            && *ctx.get_widget(self.decl_input).get::<bool>("valid");
        
        ctx.widget().set::<bool>("valid", valid);
        ctx.widget().set::<String16>("btn_text", if valid { BTN_TEXT_VALID.into() } else { BTN_TEXT_INVALID.into() });

        self.should_check_validity = false;
    }
}


widget!(RotationView<RotationViewState> {
    valid: bool, 
    btn_text: String16
});

impl Template for RotationView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("RotationView")
            .valid(true)
            .btn_text(BTN_TEXT_VALID)
            .child(Stack::new().spacing(10.0).child(
                TextBlock::new()
                    .text("Tourner le téléscope")
                    .build(ctx)
            ).child(
                Stack::new().orientation("horizontal")
                    .child(
                        TextBlock::new()
                            .text("Ascension droite : ")
                            .build(ctx)
                    )
                    .child(
                        AngleView::new()
                            .id(RIGHT_ASC)
                            .angle_type(AngleType::RightAsc)
                            .first_angle(false)
                            .on_changed_filter(vec!["valid"])
                            .on_changed(move |states, _, _| {
                                state(id, states).check_validity();
                            })
                            .build(ctx)
                    )
                    .build(ctx)
            ).child(
                Stack::new().orientation("horizontal")
                    .child(
                        TextBlock::new()
                            .text("Déclinaison")
                            .build(ctx)
                    ).child(
                        AngleView::new()
                            .id(DECL)
                            .angle_type(AngleType::Declination)
                            .first_angle(true)
                            .on_changed_filter(vec!["valid"])
                            .on_changed(move |states, _, _| {
                                state(id, states).check_validity();
                            })

                            .build(ctx)
                    ).build(ctx)
            ).child(
                Button::new()
                    .text(("btn_text", id))
                    .enabled(("valid", id))
                    .build(ctx)
            ).build(ctx)
        )
    }
}

// helper to request MainViewState
fn state<'a>(id: Entity, states: &'a mut StatesContext) -> &'a mut RotationViewState {
    states.get_mut(id)
}
