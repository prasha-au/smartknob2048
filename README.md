# Smart Knob 2048

This contains the [2048 game](https://play2048.co/) implemented on a [stm32 smart knob](https://www.aliexpress.com/item/1005007045638144.html) off AliExpress.

More importantly, it shows:
- use of Embassy HAL for STM32
- a testable project structure
- using EXTI and SPI peripherals
- setup for RTT debugging


### Folder structure
- `/docs` - Contains docs for the smart knob hardware.
- `/app` - The microcontroller independent game logic.
- `/stm32` - Implementation of the game with STM32 hardware.


### Common commands
- `cargo run` - From within the `stm32` folder, this will build and flash the game to the smart knob with debug output.
- `cargo test` - This will run the tests in the `app` folder.

### Preview

https://github.com/user-attachments/assets/8c6c43cb-74ac-46b7-8c0e-e44a3132e120

