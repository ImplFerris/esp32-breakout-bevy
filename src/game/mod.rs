mod ball;
mod block;
mod player;
mod render;
pub mod resources;
mod state;

use bevy::prelude::*;
use defmt::info;
use esp_hal::time::Duration;

use bevy_ecs::component::Component;
use embedded_graphics::prelude::Point;
use resources::{GameState, GameStatus};

#[derive(Component, Default)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Default)]
pub struct Position(pub Point);

pub fn start_game(mut app: App) -> ! {
    app.insert_resource(Time::<Fixed>::from_hz(10.0))
        .insert_resource(GameStatus::default())
        .add_systems(Startup, (block::spawn_blocks, player::spawn_player))
        .add_systems(
            Update,
            (
                (player::joystick_input, player::reset_input).chain(),
                (
                    ball::spawn_ball,
                    ball::update_ball,
                    ball::collison_handle,
                    ball::collison_handle,
                    ball::remove_balls,
                    block::remove_blocks,
                    state::update_game_state,
                )
                    .run_if(run_if_playing)
                    .chain()
                    .after(player::reset_input),
                state::handle_game_reset,
                render::clear_screen,
                (
                    render::print_lives,
                    render::print_score,
                    render::render_game,
                )
                    .run_if(run_if_playing)
                    .chain()
                    .after(render::clear_screen),
                render::display_welcome
                    .run_if(run_if_main_menu)
                    .after(render::clear_screen),
            ),
        );
    // app.add_plugins((PlayerPlugin, BlockPlugin, ProjectilePlugin, RenderPlugin));
    info!("running app");
    app.run();
    loop {
        // info!("updating game");
        app.update();
        super::blocking_delay(Duration::from_millis(50));
    }
}

fn run_if_playing(game_status: Res<GameStatus>) -> bool {
    game_status.state == GameState::Playing
}

fn run_if_main_menu(game_status: Res<GameStatus>) -> bool {
    game_status.state == GameState::MainMenu
}
