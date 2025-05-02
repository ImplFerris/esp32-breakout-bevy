use bevy::prelude::*;
use embedded_graphics::prelude::{Point, Size};

use super::{
    resources::{AdcResource, DisplayResolution, JoyStickResource},
    Position,
};

pub const PLAYER_SPEED: i32 = 5;
pub const PLAYER_SIZE: Size = Size::new(40, 5);
const PLAYER_LIVES: u8 = 3;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, update_player);
    }
}

#[derive(Component)]
#[require(Position)]
pub struct Player {
    pub lives: u8,
}

fn spawn_player(mut commands: Commands, display_resolution: NonSendMut<DisplayResolution>) {
    commands.spawn((
        Player {
            lives: PLAYER_LIVES,
        },
        Position(Point::new(
            (display_resolution.width / 2 - PLAYER_SIZE.width / 2) as i32,
            (display_resolution.height - PLAYER_SIZE.height) as i32,
        )),
    ));
}

fn update_player(
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
        // info!("Moving left");
        position.0.x = (position.0.x - PLAYER_SPEED).max(0);
    } else if adc_value < 1500 {
        // info!("Moving right");
        let right_edge = display_resolution.width as i32 - PLAYER_SIZE.width as i32;
        position.0.x = (position.0.x + PLAYER_SPEED).min(right_edge);
    }
}
