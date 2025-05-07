use bevy::prelude::*;
use embedded_graphics::prelude::{Point, Size};

use super::{resources::DisplayResolution, state::ResetGameEvent, Position};

pub const PLAYER_SPEED: i32 = 5;
pub const PLAYER_SIZE: Size = Size::new(40, 5);
const PLAYER_LIVES: u8 = 3;

#[derive(Component)]
#[require(Position)]
pub struct Player {
    pub lives: u8,
}

pub fn spawn_player(
    mut commands: Commands,
    display_resolution: NonSendMut<DisplayResolution>,
    mut events: EventReader<ResetGameEvent>,
) {
    let Some(_) = events.read().next() else {
        return;
    };

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
