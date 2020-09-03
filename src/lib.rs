mod main_app;
mod data;
mod widgets;
mod events;
mod custom_app;

pub use main_app::make_window;
pub use events::UserEvent;

/*
use std::sync::mpsc;
use std::thread;

fn main() {
    let (sx, rx) = mpsc::channel();
    let handle = thread::spawn(|| {
        make_window(sx);
    });

    for _ in 0..5 {
        println!("Received {:?}", rx.recv().unwrap());
    }

    handle.join().unwrap();
}
*/
