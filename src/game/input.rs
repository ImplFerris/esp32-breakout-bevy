use bevy::prelude::*;

use super::{
    player::{Player, PLAYER_SIZE, PLAYER_SPEED},
    resources::{AdcResource, DisplayResolution, GameState, GameStatus, JoyStickResource},
    state::ResetGameEvent,
    Position,
};

pub fn joystick(
    mut joystick: NonSendMut<JoyStickResource>,
    mut adc_res: NonSendMut<AdcResource>,
    mut player: Query<&mut Position, With<Player>>,
    display_resolution: NonSendMut<DisplayResolution>,
) {
    let Ok(mut position) = player.single_mut() else {
        return;
    };

    let Ok(adc_value): Result<u16, _> = nb::block!(adc_res.adc.read_oneshot(&mut joystick.vry_pin))
    else {
        return;
    };

    if adc_value > 3000 {
        position.0.x = (position.0.x - PLAYER_SPEED).max(0);
    } else if adc_value < 1500 {
        let right_edge = display_resolution.width as i32 - PLAYER_SIZE.width as i32;
        position.0.x = (position.0.x + PLAYER_SPEED).min(right_edge);
    }
}

pub fn reset_btn(
    joystick: NonSendMut<JoyStickResource>,
    mut event_writer: EventWriter<ResetGameEvent>,
    game_status: ResMut<GameStatus>,
) {
    if joystick.btn.is_low() && game_status.state != GameState::Playing {
        event_writer.write(ResetGameEvent);
    }
}
