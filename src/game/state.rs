use bevy::prelude::*;

use super::{
    block::Block,
    player::Player,
    resources::{GameState, GameStatus},
};

pub fn update_game_state(
    player: Query<&Player, (With<Player>,)>,
    blocks: Query<&Block, (With<Block>,)>,
    mut game_status: ResMut<GameStatus>,
) {
    if blocks.is_empty() {
        game_status.state = GameState::LevelCompleted;
        return;
    }

    let player = player.single().unwrap();
    if player.lives == 0 {
        game_status.state = GameState::GameOver;
        game_status.set_changed();
    }
}

pub fn handle_game_reset(mut game_status: ResMut<GameStatus>) {
    if !game_status.reset_game {
        return;
    }

    game_status.reset_game = false;
    game_status.state = GameState::Playing;
}
