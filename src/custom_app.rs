//! This module contains the base elements of an OrbTk application (Application, WindowBuilder and Window).

use std::sync::mpsc;

use orbtk::prelude::*;
use orbtk::shell::prelude::{Shell, ShellRequest, WindowSettings, WindowRequest};

use orbtk::{
    theming::Theme,
};

use crate::events::UserEvent;

/// The `Application` represents the entry point of an OrbTk based application.
pub struct CustomApplication {
    // shells: Vec<Shell<WindowAdapter>>,
    request_sender: mpsc::Sender<ShellRequest<WindowAdapter>>,
    shell: Shell<WindowAdapter>,
    name: Box<str>,
    theme: Theme,
}

impl Default for CustomApplication {
    fn default() -> Self {
        CustomApplication::from_name("orbtk_application")
    }
}

impl CustomApplication {
    /// Creates a new application.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the default theme for the application. Could be changed per window.
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Create a new application with the given name.
    pub fn from_name(name: impl Into<Box<str>>) -> Self {
        let (sender, receiver) = mpsc::channel();

        CustomApplication {
            request_sender: sender,
            name: name.into(),
            shell: Shell::new(receiver),
            theme: orbtk::theme::dark_theme(),
        }
    }

    /// Creates a new window and add it to the application.
    pub fn window<F: Fn(&mut BuildContext) -> Entity + 'static>(mut self, sender: mpsc::Sender<UserEvent>, create_fn: F) -> Self {
        let (adapter, settings, receiver) = custom_create_window(
            self.name.clone(),
            self.theme.clone(),
            self.request_sender.clone(),
            create_fn,
            sender
        );

        self.shell
            .create_window_from_settings(settings, adapter)
            .request_receiver(receiver)
            .build();

        self
    }

    /// Starts the application and run it until quit is requested.
    pub fn run(mut self) {
        self.shell.run();
    }
}


/// Creates a `WindowAdapter` and a `WindowSettings` object from a window builder closure.
pub fn custom_create_window<F: Fn(&mut BuildContext) -> Entity + 'static>(
    app_name: impl Into<String>,
    theme: Theme,
    request_sender: mpsc::Sender<ShellRequest<WindowAdapter>>,
    create_fn: F,
    user_sender: mpsc::Sender<UserEvent>
) -> (WindowAdapter, WindowSettings, mpsc::Receiver<WindowRequest>) {
    let app_name = app_name.into();
    let mut world: World<Tree, StringComponentStore, RenderContext2D> =
        World::from_stores(Tree::default(), StringComponentStore::default());

    let (sender, receiver) = mpsc::channel();

    let registry = Rc::new(RefCell::new(Registry::new()));

    if app_name.is_empty() {
        registry
            .borrow_mut()
            .register("settings", Settings::default());
    } else {
        registry
            .borrow_mut()
            .register("settings", Settings::new(app_name.clone()));
    };

    registry
        .borrow_mut()
        .register("sender", user_sender);

    let context_provider = ContextProvider::new(sender, request_sender, app_name);

    let window = {
        let overlay = Overlay::new().build(&mut BuildContext::new(
            world.entity_component_manager(),
            &context_provider.render_objects,
            &context_provider.layouts,
            &context_provider.handler_map,
            &mut *context_provider.states.borrow_mut(),
            &theme,
            &context_provider.event_queue,
        ));

        {
            let tree: &mut Tree = world.entity_component_manager().entity_store_mut();
            tree.set_overlay(overlay);
        }

        let window = create_fn(&mut BuildContext::new(
            world.entity_component_manager(),
            &context_provider.render_objects,
            &context_provider.layouts,
            &context_provider.handler_map,
            &mut *context_provider.states.borrow_mut(),
            &theme,
            &context_provider.event_queue,
        ));

        {
            let tree: &mut Tree = world.entity_component_manager().entity_store_mut();
            tree.set_root(window);
        }

        window
    };

    let constraint = *world
        .entity_component_manager()
        .component_store()
        .get::<Constraint>("constraint", window)
        .unwrap();

    let position = *world
        .entity_component_manager()
        .component_store()
        .get::<Point>("position", window)
        .unwrap();

    let mut fonts = HashMap::new();
    fonts.insert(
        "Roboto-Regular".to_string(),
        orbtk::theme::fonts::ROBOTO_REGULAR_FONT,
    );
    fonts.insert(
        "Roboto-Medium".to_string(),
        orbtk::theme::fonts::ROBOTO_MEDIUM_FONT,
    );
    fonts.insert(
        "MaterialIcons-Regular".to_string(),
        orbtk::theme::fonts::MATERIAL_ICONS_FONT,
    );

    let settings = WindowSettings {
        title: world
            .entity_component_manager()
            .component_store()
            .get::<String>("title", window)
            .unwrap()
            .clone(),
        borderless: *world
            .entity_component_manager()
            .component_store()
            .get::<bool>("borderless", window)
            .unwrap(),
        resizeable: *world
            .entity_component_manager()
            .component_store()
            .get::<bool>("resizeable", window)
            .unwrap(),
        always_on_top: *world
            .entity_component_manager()
            .component_store()
            .get::<bool>("always_on_top", window)
            .unwrap(),
        position: (position.x(), position.y()),
        size: (constraint.width(), constraint.height()),
        fonts,
    };

    let mut global = Global::default();
    global.theme = theme;

    world
        .entity_component_manager()
        .component_store_mut()
        .register("global", window, global);
    world
        .entity_component_manager()
        .component_store_mut()
        .register(
            "bounds",
            window,
            Rectangle::from((0.0, 0.0, constraint.width(), constraint.height())),
        );

    world.register_init_system(InitSystem::new(context_provider.clone(), registry.clone()));

    world.register_cleanup_system(CleanupSystem::new(
        context_provider.clone(),
        registry.clone(),
    ));

    world
        .create_system(EventStateSystem::new(
            context_provider.clone(),
            registry.clone(),
        ))
        .with_priority(0)
        .build();

    world
        .create_system(LayoutSystem::new(context_provider.clone()))
        .with_priority(1)
        .build();

    world
        .create_system(PostLayoutStateSystem::new(
            context_provider.clone(),
            registry,
        ))
        .with_priority(2)
        .build();

    world
        .create_system(RenderSystem::new(context_provider.clone()))
        .with_priority(3)
        .build();

    (
        WindowAdapter::new(world, context_provider),
        settings,
        receiver,
    )
}
