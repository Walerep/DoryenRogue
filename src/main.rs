extern crate doryen_rs;
extern crate doryen_fov;

mod player;
mod entity;
mod level;
mod light;
mod noise;
//mod menu;

mod config;
mod doryen_rogue;

use doryen_rs::{App, AppOptions};

fn main() {
    let mut app = App::new(AppOptions {
        window_title: "doryen rogue".to_owned(),
        vsync: false,
        ..Default::default()
    });
    app.set_engine(Box::new(doryen_rogue::DoryenRogue::new()));
    app.run();
}
