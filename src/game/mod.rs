mod ball;
mod block;
mod player;
mod render;
pub mod resources;

use ball::ProjectilePlugin;
use block::BlockPlugin;
use player::PlayerPlugin;
use render::RenderPlugin;

use bevy::prelude::*;
use defmt::info;
use esp_hal::time::Duration;

use bevy_ecs::component::Component;
use embedded_graphics::prelude::Point;
use resources::GameStatus;

#[derive(Component, Default)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Default)]
pub struct Position(pub Point);

pub fn start_game(mut app: App) -> ! {
    app.insert_non_send_resource(GameStatus::default());
    app.add_plugins((PlayerPlugin, BlockPlugin, ProjectilePlugin, RenderPlugin));
    info!("running app");
    app.run();
    loop {
        // info!("updating game");
        app.update();
        super::blocking_delay(Duration::from_millis(50));
    }
}
