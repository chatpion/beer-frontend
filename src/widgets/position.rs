use orbtk::prelude::*;

use crate::{
    widgets::angle::{AngleView},
    events::user::{UserEvent, UserEventHandler},
    data::{Angle}
};


static LONGITUDE: &str = "longitude_input";
static LATITUDE: &str = "latitude_input";


static BTN_TEXT_VALID: &str = "Valider la position du téléscope";
static BTN_TEXT_INVALID: &str = "Coordonnées invalides";


#[derive(Default, AsAny)]
pub struct PositionViewState {
    longitude_input: Entity,
    latitude_input: Entity,

    should_check_validity: bool,
    button_pressed: bool
}

impl PositionViewState {
    // used to trigger an update
    fn check_validity(&mut self) {
        self.should_check_validity = true;
    }

    // used to trigger an update
    fn click(&mut self) {
        self.button_pressed = true;
    }
}

impl State for PositionViewState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.longitude_input = ctx.entity_of_child(LONGITUDE)
            .expect("PositionViewState.init(): the child longitude_input could not be found!");
        self.latitude_input = ctx.entity_of_child(LATITUDE)
            .expect("PositionViewState.init(): the child latitude_input could not be found!");

        self.should_check_validity = true;
        self.button_pressed = false;
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        let valid = *ctx.get_widget(self.longitude_input).get::<bool>("valid")
            && *ctx.get_widget(self.latitude_input).get::<bool>("valid");
        
        ctx.widget().set::<bool>("valid", valid);
        ctx.widget().set::<String16>("btn_text", if valid { BTN_TEXT_VALID.into() } else { BTN_TEXT_INVALID.into() });

        self.should_check_validity = false;

        if self.button_pressed {
            let lo = *ctx.get_widget(self.longitude_input).get::<Angle>("angle");
            let la = *ctx.get_widget(self.latitude_input).get::<Angle>("angle");
            ctx.push_event(UserEvent::Position(lo, la));
        }
        self.button_pressed = false;
    }
}


widget!(PositionView<PositionViewState> {
    valid: bool, 
    btn_text: String16
});


impl PositionView {
    pub fn on_user_event<H: Fn(&mut StatesContext, &UserEvent) -> bool + 'static>(
        self,
        handler: H,
    ) -> Self {
        self.insert_handler(UserEventHandler {
            handler: Rc::new(handler),
        })
    }
}


impl Template for PositionView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("PositionView")
            .valid(true)
            .btn_text(BTN_TEXT_VALID)
            .child(Stack::new().spacing(10.0).child(
                TextBlock::new()
                    .text("Position du téléscope")
                    .font_size(25)
                    .build(ctx)
            ).child(
                Stack::new().orientation("horizontal")
                    .child(
                        TextBlock::new()
                            .text("Longitude : ")
                            .build(ctx)
                    )
                    .child(
                        AngleView::new()
                            .id(LONGITUDE)
                            .first_angle(true)
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
                            .text("Latitude")
                            .build(ctx)
                    ).child(
                        AngleView::new()
                            .id(LATITUDE)
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
                    .on_click(move |states, _| {
                        state(id, states).click();
                        true
                    })
                    .build(ctx)
            ).build(ctx)
        )
    }
}

// helper to request MainViewState
fn state<'a>(id: Entity, states: &'a mut StatesContext) -> &'a mut PositionViewState {
    states.get_mut(id)
}
