#![no_std]
#![no_main]

use core::sync::atomic::Ordering;

use bevy::app::App;
use bevy::platform_support::{sync::atomic::AtomicU64, time::Instant as BevyInstant};
use bevy::DefaultPlugins;
use esp32_breakout_bevy::game::resources::RandResource;
use esp_hal::analog::adc::{Adc, AdcConfig};
use esp_hal::gpio::{Input, InputConfig, Pull};
use esp_hal::main;
use esp_hal::rng::Rng;
use esp_hal::time::Rate;
use esp_hal::{analog::adc::Attenuation, clock::CpuClock};
use esp_println as _;

use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

use esp32_breakout_bevy as lib;
use lib::game::{
    resources::{AdcResource, DisplayResolution, DisplayResource, JoyStickResource},
    start_game,
};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    esp_println::println!("Panic occurred: {:?}", info);

    loop {}
}

extern crate alloc;

#[main]
fn main() -> ! {
    // generator version: 0.3.1

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 120 * 1024);
    // esp_alloc::psram_allocator!(peripherals.PSRAM, esp_hal::psram);
    esp_alloc::heap_allocator!(#[link_section = ".dram2_uninit"] size: 94000);

    // Initialize the OLED Display
    let i2c_bus = esp_hal::i2c::master::I2c::new(
        peripherals.I2C0,
        esp_hal::i2c::master::Config::default().with_frequency(Rate::from_khz(400)),
    )
    .expect("failed to initialize I2C")
    .with_scl(peripherals.GPIO18)
    .with_sda(peripherals.GPIO23);

    let interface = I2CDisplayInterface::new(i2c_bus);

    // initialize the display
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().expect("failed to init display");
    let (display_width, display_height) = display.dimensions();

    unsafe { BevyInstant::set_elapsed(elapsed_time) };

    let mut adc2_config = AdcConfig::new();

    let vrx_pin = adc2_config.enable_pin(peripherals.GPIO13, Attenuation::_11dB);
    let vry_pin = adc2_config.enable_pin(peripherals.GPIO14, Attenuation::_11dB);
    let input_btn = Input::new(
        peripherals.GPIO32,
        InputConfig::default().with_pull(Pull::Up),
    );

    let adc = Adc::new(peripherals.ADC2, adc2_config);

    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .insert_non_send_resource(DisplayResource { display })
        .insert_non_send_resource(DisplayResolution {
            width: u32::from(display_width),
            height: u32::from(display_height),
        })
        .insert_non_send_resource(JoyStickResource {
            vrx_pin,
            vry_pin,
            btn: input_btn,
        })
        .insert_non_send_resource(AdcResource { adc })
        .insert_non_send_resource(RandResource {
            rng: Rng::new(peripherals.RNG),
        });
    start_game(app)
}

static ELAPSED: AtomicU64 = AtomicU64::new(0);
fn elapsed_time() -> core::time::Duration {
    core::time::Duration::from_nanos(ELAPSED.load(Ordering::Relaxed))
}
