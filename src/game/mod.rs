mod ball;
mod block;
mod input;
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
    app.insert_resource(GameStatus::default())
        .add_event::<state::ResetGameEvent>()
        .add_systems(
            Update,
            (
                // Handle input
                (input::joystick, input::reset_btn).chain(),
                // Playing
                (
                    ball::spawn_ball_if_empty,
                    ball::update_ball,
                    ball::collison_handle,
                    ball::remove_balls,
                    block::remove_blocks,
                    state::update_game_state,
                )
                    .run_if(run_if_playing)
                    .chain()
                    .after(input::reset_btn),
                // Reset the game and spawn
                state::reset_game,
                (
                    ball::spawn_ball_on_reset,
                    block::spawn_blocks,
                    player::spawn_player,
                    state::switch_reset,
                )
                    .chain()
                    .run_if(run_if_resetting),
                // .after(state::reset_game),
                // Rendering
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
                render::display_game_over.run_if(run_if_game_over),
                render::display_game_completed.run_if(run_if_completed),
            ),
        );
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

fn run_if_game_over(game_status: Res<GameStatus>) -> bool {
    game_status.state == GameState::GameOver
}

fn run_if_completed(game_status: Res<GameStatus>) -> bool {
    game_status.state == GameState::LevelCompleted
}

fn run_if_resetting(game_status: Res<GameStatus>) -> bool {
    game_status.state == GameState::Resetting
}
