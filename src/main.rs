use std::collections::HashSet;

use crate::{ behavior::{ Behavior, GremlinDrag }, sprite::DesktopGremlin };

pub mod sprite;
pub mod ui;
pub mod utils;
pub mod behavior;

fn main() {
    let mut app = DesktopGremlin::new(None).unwrap();
    let mut behaviors: Vec<Box<dyn Behavior>> = vec![Box::new(GremlinDrag::setup())];
    // app.register_behaviors(behaviors);
    loop {
        app.update();
        for behavior in behaviors.iter_mut() {
            behavior.update(&mut app, &HashSet::new());
        }
        if let false = *app.should_exit.lock().unwrap() {
            break;
        }
    }
    app.go();
}
