#![no_std]

use esp_hal::time::{Duration, Instant};

pub mod game;

pub fn blocking_delay(duration: Duration) {
    let delay_start = Instant::now();
    while delay_start.elapsed() < duration {}
}
