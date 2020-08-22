mod data;
mod widgets;
mod events;

use orbtk::prelude::*;
use std::collections::VecDeque;
use widgets::pad::{PadView};
use widgets::rotation::{RotationView};
use data::{Angle, Direction};
use events::user::{UserEvent, UserEventHandler};
use orbtk::theming::config::ThemeConfig;
use orbtk::theme::{COLORS_RON, DARK_THEME_RON, FONTS_RON};

static EXT: &str = include_str!("../res/theme.ron");

fn theme() -> Theme {
    Theme::from_config(
        ThemeConfig::from(DARK_THEME_RON)
            .extend(ThemeConfig::from(EXT))
            .extend(ThemeConfig::from(COLORS_RON))
            .extend(ThemeConfig::from(FONTS_RON))
    )
}

#[derive(Default, AsAny)]
pub struct MainViewState {
    user_event: Option<UserEvent>
}

impl MainViewState {
    fn register_event(&mut self, evt: UserEvent) {
        self.user_event = Some(evt);
    }
}

impl State for MainViewState {
    fn init(&mut self, _: &mut Registry, _: &mut Context) {
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(e) = self.user_event {
            println!("{:?}", e);
            ctx.widget().get_mut::<UserEventQueue>("user_event_queue").push_back(e);
        }
    }
}

type UserEventQueue = VecDeque<UserEvent>;

widget!(MainView<MainViewState> {
    user_event_queue: UserEventQueue
});

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView").width(212).height(700).child(
            Stack::new().spacing(10.0).child(
                RotationView::new()
                    .on_user_event(move |states, evt| {
                        state(id, states).register_event(*evt);
                        true
                    })
                    .build(ctx)
            ).child(
                PadView::new()
                    .on_user_event(move |states, evt| {
                        state(id, states).register_event(*evt);
                        true
                    })
                    .build(ctx)
            )
            .build(ctx)
        )
    }
}


fn main() {
    Application::new()
        .theme(theme())
        .window(|ctx| {
            Window::new()
                .title("OrbTk - minimal example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}

// helper to request MainViewState
fn state<'a>(id: Entity, states: &'a mut StatesContext) -> &'a mut MainViewState {
    states.get_mut(id)
}
