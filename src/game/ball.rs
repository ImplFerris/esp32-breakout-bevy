use bevy::prelude::*;
use embedded_graphics::{
    prelude::{Point, Size},
    primitives::Rectangle,
};

use super::{
    block::{Block, BLOCK_SIZE},
    player::{Player, PLAYER_SIZE},
    resources::{DisplayResolution, GameStatus, RandResource},
    state::ResetGameEvent,
    Position, Velocity,
};

pub const BALL_SIZE: Size = Size::new(4, 4);
pub const BALL_SPEED: i32 = 1;

#[derive(Component)]
#[require(Velocity)]
pub struct Ball;

pub fn spawn_ball_if_empty(
    balls: Query<(&mut Position, &mut Velocity), With<Ball>>,
    commands: Commands,
    display_resolution: NonSendMut<DisplayResolution>,
    rand_res: NonSendMut<RandResource>,
) {
    if !balls.is_empty() {
        // Spawn ball only if it is empty
        return;
    }

    spawn_ball(commands, display_resolution, rand_res);
}

pub fn spawn_ball_on_reset(
    commands: Commands,
    display_resolution: NonSendMut<DisplayResolution>,
    rand_res: NonSendMut<RandResource>,
    mut event_reader: EventReader<ResetGameEvent>,
) {
    let Some(_) = event_reader.read().next() else {
        return;
    };

    spawn_ball(commands, display_resolution, rand_res);
}

pub fn spawn_ball(
    mut commands: Commands,
    display_resolution: NonSendMut<DisplayResolution>,
    mut rand_res: NonSendMut<RandResource>,
) {
    let rng = &mut rand_res.rng;
    let rand_velocity_x = ((rng.random() as i32 % 21) - 10).clamp(-1, 1);

    commands.spawn((
        Ball,
        Position(Point::new(
            (display_resolution.width / 2) as i32,
            (display_resolution.height / 2) as i32,
        )),
        Velocity {
            x: rand_velocity_x,
            y: -1,
        },
    ));
}

pub fn update_ball(
    balls: Query<(&mut Position, &mut Velocity), With<Ball>>,
    display_resolution: NonSendMut<DisplayResolution>,
) {
    for (mut position, mut velocity) in balls {
        position.0.x += velocity.x * BALL_SPEED;
        position.0.y += velocity.y * BALL_SPEED;

        if position.0.x < 0 {
            velocity.x = 1;
        } else if position.0.x > display_resolution.width as i32 - BALL_SIZE.width as i32 {
            velocity.x = -1;
        }

        if position.0.y < 0 {
            velocity.y = 1;
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn collison_handle(
    balls: Query<(&mut Position, &mut Velocity), With<Ball>>,
    mut blocks: Query<(&mut Block, &mut Position), (With<Block>, Without<Ball>, Without<Player>)>,
    mut player: Query<&mut Position, (With<Player>, Without<Ball>, Without<Block>)>,
    mut game_status: ResMut<GameStatus>,
) {
    let Ok(player_pos) = player.single_mut() else {
        return;
    };

    //TODO: Use bevy only to check the collison
    let player_rect = Rectangle::new(player_pos.0, PLAYER_SIZE);

    for (mut ball_position, mut ball_velocity) in balls {
        let mut ball_rect = Rectangle::new(ball_position.0, BALL_SIZE);
        if resolve_collison(&mut ball_rect, &mut ball_velocity, &player_rect) {
            ball_position.0 = ball_rect.top_left;
        }

        for (mut block, block_position) in blocks.iter_mut() {
            let block_rect = Rectangle::new(block_position.0, BLOCK_SIZE);
            if resolve_collison(&mut ball_rect, &mut ball_velocity, &block_rect) {
                ball_position.0 = ball_rect.top_left;
                block.lives = block.lives.saturating_sub(1);
                if block.lives == 0 {
                    game_status.score += 10;
                }
            }
        }
    }
}

pub fn remove_balls(
    mut commands: Commands,
    balls: Query<(Entity, &mut Position), With<Ball>>,
    mut player: Query<&mut Player, With<Player>>,
    display_resolution: NonSendMut<DisplayResolution>,
) {
    let mut removed_balls = 0;
    for (entity, position) in balls.iter() {
        if position.0.y > display_resolution.height as i32 {
            removed_balls += 1;
            commands.entity(entity).despawn();
        }
    }

    if removed_balls > 0 {
        let Ok(mut player) = player.single_mut() else {
            return;
        };

        player.lives = player.lives.saturating_sub(1);
    }
}

fn resolve_collison(a: &mut Rectangle, vel: &mut Velocity, b: &Rectangle) -> bool {
    let intersection = a.intersection(b);

    if intersection.size.width == 0 || intersection.size.height == 0 {
        return false;
    }

    let a_center = a.center();
    let b_center = b.center();
    let to = b_center - a_center;
    let to_signum = Point::new(to.x.signum(), to.y.signum());

    if intersection.size.width > intersection.size.height {
        a.top_left.y -= to_signum.y * intersection.size.height as i32;

        vel.y = match to_signum.y {
            1 => -vel.y.abs(),
            -1 => vel.y.abs(),
            0 => -vel.y,
            _ => vel.y,
        };
    } else {
        a.top_left.x -= to_signum.x * intersection.size.width as i32;
        vel.x = match to_signum.x {
            1 => -vel.x.abs(),
            -1 => vel.x.abs(),
            0 => -vel.x,
            _ => vel.x,
        };
    }

    true
}
