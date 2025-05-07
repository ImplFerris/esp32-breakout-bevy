use bevy::prelude::*;

use super::{
    ball::Ball,
    block::Block,
    player::Player,
    resources::{GameState, GameStatus},
};

#[derive(Debug, Event)]
pub struct ResetGameEvent;

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

pub fn reset_game(
    mut commands: Commands,
    mut events: EventReader<ResetGameEvent>,
    ball_query: Query<Entity, With<Ball>>,
    block_query: Query<Entity, With<Block>>,
    player_query: Query<Entity, With<Player>>,
    mut game_status: ResMut<GameStatus>,
) {
    let Some(_) = events.read().next() else {
        return;
    };

    // Despawn all existing game entities
    ball_query
        .iter()
        .for_each(|entity| commands.entity(entity).despawn());
    block_query
        .iter()
        .for_each(|entity| commands.entity(entity).despawn());
    player_query
        .iter()
        .for_each(|entity| commands.entity(entity).despawn());

    game_status.state = GameState::Resetting;
    game_status.score = 0;
}

/// switch to playing mode
pub fn switch_reset(mut game_status: ResMut<GameStatus>) {
    game_status.state = GameState::Playing;
}
