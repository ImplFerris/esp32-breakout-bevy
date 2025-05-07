use bevy_ecs::resource::Resource;
use embedded_graphics::image::ImageRaw;
use embedded_graphics::pixelcolor::BinaryColor;
use esp_hal::gpio::Input;
use esp_hal::i2c::master::I2c;
use esp_hal::rng::Rng;
use ssd1306::{
    mode::BufferedGraphicsMode, prelude::I2CInterface, size::DisplaySize128x64, Ssd1306,
};

pub type DisplayType<'a> = Ssd1306<
    I2CInterface<I2c<'a, esp_hal::Blocking>>,
    DisplaySize128x64,
    BufferedGraphicsMode<DisplaySize128x64>,
>;

// const VRX_PIN: u8 = 13;
// const VRY_PIN: u8 = 14;
const BTN_PIN: u8 = 32;

const HEART_SPRITE: [u8; 8] = [0x00, 0x6e, 0xff, 0xef, 0x7e, 0x3c, 0x18, 0x00];
pub const HEART_SPRITE_WIDTH: u32 = 8;
pub const RAW_HEART_SPRITE: ImageRaw<'static, BinaryColor> =
    ImageRaw::<BinaryColor>::new(&HEART_SPRITE, HEART_SPRITE_WIDTH);

type VrxPin = esp_hal::analog::adc::AdcPin<esp_hal::gpio::GpioPin<13>, esp_hal::peripherals::ADC2>;
type VryPin = esp_hal::analog::adc::AdcPin<esp_hal::gpio::GpioPin<14>, esp_hal::peripherals::ADC2>;

#[derive(Resource)]
pub struct JoyStickResource<'a> {
    pub vrx_pin: VrxPin,
    pub vry_pin: VryPin,
    pub btn: Input<'a>,
}

pub type Adc<'a> = esp_hal::analog::adc::Adc<'a, esp_hal::peripherals::ADC2, esp_hal::Blocking>;

#[derive(Resource)]
pub struct AdcResource<'a> {
    pub adc: Adc<'a>,
}

#[derive(Resource)]
pub struct DisplayResource<'a> {
    pub display: DisplayType<'a>,
}

#[derive(Resource)]
pub struct DisplayResolution {
    pub width: u32,
    pub height: u32,
}

#[derive(Resource)]
pub struct RandResource {
    pub rng: Rng,
}

#[derive(Default, PartialEq, Debug)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    LevelCompleted,
    GameOver,
    Resetting,
}

#[derive(Resource, Default)]
pub struct GameStatus {
    pub state: GameState,
    pub score: u32,
}
