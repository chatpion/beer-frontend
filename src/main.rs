mod angle_widget;
mod angle_event;
mod data;
mod widgets;
mod events;

use orbtk::prelude::*;
use std::collections::VecDeque;
use angle_widget::{AngleView};
use data::{Angle};
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Up, Down, Left, Right
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum UserEvent {
    Zero,
    Rotate(Angle, Angle),
    UpdatePos(Angle, Angle),
    BeginMove(Direction),
    EndMove(Direction),
    R
}



#[derive(Default, AsAny)]
pub struct MainViewState {
    // directional pad
    pressed_btn: Option<Direction>,

    // geographical position
    position: Angle, 

    // space position
    target: Angle,

    action: Vec<Option<UserEvent>>
}

impl MainViewState {
    fn action(&mut self, action: Option<UserEvent>) {
        self.action.push(action);
    }
}

impl State for MainViewState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.action = vec![];
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        
        for a in self.action.drain(..) {
            match a {
                Some(UserEvent::BeginMove(d)) => {
                    println!("Begin move {:?}", d);
                    self.pressed_btn = Some(d);
                },
                Some(UserEvent::EndMove(d)) => {
                    println!("End move {:?}", d);
                    self.pressed_btn = None;
                },
                Some(UserEvent::Zero) => println!("Zero !"), 
                Some(UserEvent::R) => (), 
                _ => ()
            }
            if let Some(e) = a {
                ctx.widget().get_mut::<UserEventQueue>("userEventQueue").push_back(e)
            }
        }
    }
}

type UserEventQueue = VecDeque<UserEvent>;

widget!(MainView<MainViewState> {
    userEventQueue: UserEventQueue
});

fn generate_pad_button(
    ctx: &mut BuildContext, 
    id: Entity,
    dir: Option<Direction>,
    column: usize,
    row: usize) -> Entity {
    let mut button = Button::new()
        .min_size(72, 72)
        .on_mouse_down(move |states, _| -> bool {
            state(id, states).action(
                match dir {
                    Some(d) => Some(UserEvent::BeginMove(d)),
                    None => Some(UserEvent::Zero)
                });
            true
        })
        .on_mouse_up(move |states, _| -> bool {
            if let Some(current) = state(id, states).pressed_btn {
                if let Some(d) = dir {
                    if d != current { 
                        return false; 
                    }
                    state(id, states).action(Some(UserEvent::EndMove(d)));  
                }
            }
            false
        }) 
        .attach(Grid::column(column))
        .attach(Grid::row(row));
    
    if let Some(d) = dir {
        button = button.icon(image_path(d));
    } else {
        button = button.text("0")
    }

    button.build(ctx)
}

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let btn_size = 72;

        self.name("MainView").width(212).height(700).child(
            Stack::new().spacing(10.0).child(
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
                        AngleView::new().first_angle(false).build(ctx)
                    )
                    .build(ctx)
            ).child(
                Stack::new().orientation("horizontal")
                    .child(
                        TextBlock::new()
                            .text("Déclinaison")
                            .build(ctx)
                    ).child(
                        AngleView::new().first_angle(true).build(ctx)
                    ).build(ctx)
            ).child(
                Grid::new() 
                    .rows(Rows::create().push(btn_size).push(btn_size).push(btn_size))
                    .columns(Columns::create().push(btn_size).push(btn_size).push(btn_size))
                    .child(generate_pad_button(ctx, id, None, 1, 1))
                    .child(generate_pad_button(ctx, id, Some(Direction::Up), 1, 0))
                    .child(generate_pad_button(ctx, id, Some(Direction::Down), 1, 2))
                    .child(generate_pad_button(ctx, id, Some(Direction::Left), 0, 1))
                    .child(generate_pad_button(ctx, id, Some(Direction::Right), 2, 1))
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

fn image_path(dir: Direction) -> &'static str {
    match dir {
        Direction::Up => material_icons_font::MD_ARROW_UPWARD,
        Direction::Down => material_icons_font::MD_ARROW_DOWNWARD,
        Direction::Right => material_icons_font::MD_ARROW_FORWARD,
        Direction::Left => material_icons_font::MD_ARROW_BACK
    }
}
