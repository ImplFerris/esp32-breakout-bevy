use bevy::prelude::*;
use core::fmt::Write;
use embedded_graphics::{
    image::Image,
    mono_font::{
        ascii::{FONT_5X8, FONT_6X10},
        MonoTextStyleBuilder,
    },
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
    text::{Baseline, Text},
};
use heapless::String;

use super::{
    ball::{Ball, BALL_SIZE},
    block::{Block, BLOCK_SIZE},
    player::{Player, PLAYER_SIZE},
    resources::{
        DisplayResolution, DisplayResource, GameStatus, HEART_SPRITE_WIDTH, RAW_HEART_SPRITE,
    },
    Position,
};

pub fn clear_screen(mut display_res: NonSendMut<DisplayResource>) {
    let display = &mut display_res.display;

    display.clear_buffer();
    display
        .clear(BinaryColor::Off)
        .expect("failed to clear display");
}

pub fn render_game(
    mut display_res: NonSendMut<DisplayResource>,
    blocks: Query<&Position, With<Block>>,
    player: Query<&Position, With<Player>>,
    balls: Query<&Position, With<Ball>>,
) {
    let display = &mut display_res.display;

    for position in blocks {
        let style = PrimitiveStyleBuilder::new()
            .fill_color(BinaryColor::On)
            .build();
        let rect = embedded_graphics::primitives::Rectangle::new(position.0, BLOCK_SIZE);

        rect.into_styled(style)
            .draw(display)
            .expect("failed to draw block");
    }

    let style = PrimitiveStyleBuilder::new()
        .fill_color(BinaryColor::On)
        .build();

    if let Ok(player_position) = player.single() {
        let rect = Rectangle::new(player_position.0, PLAYER_SIZE);
        rect.into_styled(style).draw(display).unwrap();
    }

    for position in balls {
        let rect = embedded_graphics::primitives::Rectangle::new(position.0, BALL_SIZE);

        let style = PrimitiveStyleBuilder::new()
            .fill_color(BinaryColor::On)
            .build();

        rect.into_styled(style).draw(display).unwrap();
    }

    display.flush().expect("failed to flush");
}

pub fn print_score(mut display_res: NonSendMut<DisplayResource>, game_status: ResMut<GameStatus>) {
    let display = &mut display_res.display;

    let mut score_text: String<16> = String::new();
    write!(score_text, "Score: {}", game_status.score).unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_5X8)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline(&score_text, Point::new(0, 0), text_style, Baseline::Top)
        .draw(display)
        .unwrap();
}

pub fn print_lives(
    display_resolution: NonSendMut<DisplayResolution>,
    mut player: Query<&mut Player, With<Player>>,
    mut display_res: NonSendMut<DisplayResource>,
) {
    let display = &mut display_res.display;

    let Ok(player) = player.single_mut() else {
        return;
    };

    let img_width = HEART_SPRITE_WIDTH;
    let lives_x = (display_resolution.width - img_width * player.lives as u32) - img_width;
    for i in 0..player.lives {
        let x = lives_x + i as u32 * img_width;

        let image = Image::new(&RAW_HEART_SPRITE, Point::new(x as i32, 0));
        image.draw(display).unwrap();
    }
}

pub fn display_game_over(
    mut display_res: NonSendMut<DisplayResource>,
    game_status: ResMut<GameStatus>,
) {
    let mut title: String<20> = String::new();
    write!(title, "You died! Score: {}", game_status.score).unwrap();

    let display = &mut display_res.display;
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    let text_width = title.len() as i32 * FONT_6X10.character_size.width as i32;
    let text_height = FONT_6X10.character_size.height as i32;

    // Get display dimensions
    let (width, height) = display.dimensions();

    // Calculate top-left position to center the text
    let x = (width as i32 - text_width) / 2;
    let y = (height as i32 - text_height) / 2;

    Text::with_baseline(&title, Point::new(x, y), text_style, Baseline::Top)
        .draw(display)
        .unwrap();
    display.flush().expect("failed to flush display");
}

pub fn display_game_completed(
    mut display_res: NonSendMut<DisplayResource>,
    game_status: ResMut<GameStatus>,
) {
    let mut title: String<20> = String::new();
    write!(title, "You win! Score: {}", game_status.score).unwrap();

    let display = &mut display_res.display;
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    let text_width = title.len() as i32 * FONT_6X10.character_size.width as i32;
    let text_height = FONT_6X10.character_size.height as i32;

    // Get display dimensions
    let (width, height) = display.dimensions();

    // Calculate top-left position to center the text
    let x = (width as i32 - text_width) / 2;
    let y = (height as i32 - text_height) / 2;

    Text::with_baseline(&title, Point::new(x, y), text_style, Baseline::Top)
        .draw(display)
        .unwrap();
    display.flush().expect("failed to flush display");
}

pub fn display_welcome(mut display_res: NonSendMut<DisplayResource>) {
    let display = &mut display_res.display;

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    let title = "Mom... I'm Game dev now";
    let text_width = title.len() as i32 * FONT_6X10.character_size.width as i32;
    let text_height = FONT_6X10.character_size.height as i32;

    // Get display dimensions
    let (width, height) = display.dimensions();

    // Calculate top-left position to center the text
    let x = (width as i32 - text_width) / 2;
    let y = (height as i32 - text_height) / 2;

    Text::with_baseline(title, Point::new(x, y), text_style, Baseline::Top)
        .draw(display)
        .expect("failed to draw welcome text");

    display.flush().expect("failed to flush display");
}
