use bevy::prelude::*;
use embedded_graphics::prelude::{Point, Size};

use super::{resources::DisplayResolution, Position};

const BLOCK_COLUMNS: usize = 6;
const BLOCK_ROWS: usize = 5;
pub const BLOCK_SIZE: Size = Size::new(20, 3);
const BLOCK_PADDING: i32 = 1;

#[derive(Component)]
pub struct Block {
    pub lives: u8,
}

pub fn spawn_blocks(mut commands: Commands, display_resolution: NonSendMut<DisplayResolution>) {
    let total_width =
        BLOCK_COLUMNS as i32 * (BLOCK_SIZE.width as i32 + BLOCK_PADDING) - BLOCK_PADDING;
    let start_x = (display_resolution.width as i32 - total_width) / 2;
    let start_y = 10;

    for row in 0..BLOCK_ROWS {
        for column in 0..BLOCK_COLUMNS {
            let x = start_x + column as i32 * (BLOCK_SIZE.width as i32 + BLOCK_PADDING);
            let y = start_y + row as i32 * (BLOCK_SIZE.height as i32 + BLOCK_PADDING);

            commands.spawn((Block { lives: 2 }, Position(Point { x, y })));
        }
    }
}

pub fn remove_blocks(mut commands: Commands, balls: Query<(Entity, &mut Block), With<Block>>) {
    for (entity, block) in balls.iter() {
        if block.lives == 0 {
            commands.entity(entity).despawn();
        }
    }
}
