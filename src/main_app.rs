use orbtk::prelude::*;
use crate::{
    widgets::{PadView, RotationView, PositionView},
    events::{UserEvent},
    custom_app::CustomApplication
};
use orbtk::theming::config::ThemeConfig;
use orbtk::theme::{COLORS_RON, DARK_THEME_RON, FONTS_RON};
use std::sync::mpsc;

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

    fn update(&mut self, registry: &mut Registry, _: &mut Context) {
        if let Some(e) = self.user_event {
            registry.get::<mpsc::Sender<UserEvent>>("sender").send(e).unwrap();
        }
    }
}

widget!(MainView<MainViewState> {});

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
                PositionView::new()
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


pub fn make_window(sx: mpsc::Sender<UserEvent>) {
    CustomApplication::new()
        .theme(theme())
        .window(sx, |ctx| {
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



