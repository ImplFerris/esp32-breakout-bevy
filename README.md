# Breakout Game for ESP32 with OLED Display - Built in Rust Using the Bevy Engine
 
Breakout Game written in Rust for the ESP32 with an OLED display, Using the Bevy Engine.

## Hardware Requirements
- ESP32 (WROOM Dev Kit 1)
- SSD1306 OLED I2C 128x64 Display
- Joystick Module
- Jumper wires and breadboard
    
## Circuit

| ESP32 Pin | Component               |
|----------|-------------------------|
| GPIO 23  | SDA pin of OLED         |
| GPIO 18  | SCL pin of OLED         |
| 3.3V     | VCC pin of OLED         |
| GND      | GND pin of OLED         |
| 3.3V     | 5V pin of Joystick      |
| GPIO 32  | SW pin of Joystick      |
| GPIO 13  | VRX pin of Joystick (unused)    |
| GPIO 14  | VRY pin of Joystick     |

Note: I used only the VRY input for the player's movement and won't be tracking VRX.


## Related Tutorials

You can refer to the following tutorials in the "impl Rust on ESP32" book to learn how to use the joystick and OLED with the ESP32.

- [Using Joystick Module with ESP32](https://esp32.implrust.com/joystick/index.html)
- [Using OLED Display Module with ESP32](https://esp32.implrust.com/oled/index.html)

## Preview of the game.  

https://github.com/user-attachments/assets/0af3e458-e532-441c-86aa-8e010a55fd94
