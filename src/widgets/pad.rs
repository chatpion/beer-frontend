use orbtk::prelude::*;

use crate::data::Direction;
use crate::events::{UserEvent, UserEventHandler};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Action {
    BeginMove(Direction),
    EndMove(Direction),
    Zero
}

#[derive(Default, AsAny)]
pub struct PadViewState {
    // directional pad
    pressed_btn: Option<Direction>,

    action: Vec<Action>
}

impl PadViewState {
    fn action(&mut self, action: Action) {
        self.action.push(action);
    }
}

impl State for PadViewState {
    fn init(&mut self, _: &mut Registry, _: &mut Context) {
        self.action = vec![];
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        for a in self.action.drain(..) {
            match a {
                Action::BeginMove(d) => {
                    ctx.push_event(UserEvent::BeginMove(d));
                    self.pressed_btn = Some(d);
                },
                Action::EndMove(d) => {
                    ctx.push_event(UserEvent::EndMove(d));
                    self.pressed_btn = None;
                },
                Action::Zero => ctx.push_event(UserEvent::Zero), 
            }
        }
    }
}

widget!(PadView<PadViewState> {});

impl PadView {
    pub fn on_user_event<H: Fn(&mut StatesContext, &UserEvent) -> bool + 'static>(
        self,
        handler: H,
    ) -> Self {
        self.insert_handler(UserEventHandler {
            handler: Rc::new(handler),
        })
    }
}

fn generate_pad_button(
    ctx: &mut BuildContext, 
    id: Entity,
    dir: Option<Direction>,
    column: usize,
    row: usize) -> Entity {
    let mut button = Button::new()
        .style("pad_button")
        .on_mouse_down(move |states, _| -> bool {
            state(id, states).action(
                match dir {
                    Some(d) => Action::BeginMove(d),
                    None => Action::Zero
                });
            true
        })
        .on_mouse_up(move |states, _| -> bool {
            if let Some(current) = state(id, states).pressed_btn {
                if let Some(d) = dir {
                    if d != current { 
                        return false; 
                    }
                    state(id, states).action(Action::EndMove(d));  
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

impl Template for PadView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let btn_size = 72;

        self.name("PadView").child(
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
    }
}

// helper to request MainViewState
fn state<'a>(id: Entity, states: &'a mut StatesContext) -> &'a mut PadViewState {
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

