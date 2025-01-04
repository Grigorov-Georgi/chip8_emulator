# Chip-8 Emulator

A simple CHIP-8 emulator written in Rust.

## Setup
1. Install Rust
   - Follow the instructions at https://www.rust-lang.org/tools/install to install Rust.
   
2. Install SDL2
   - Use Homebrew to install SDL2: ```brew install sdl2```
   - For additional configuration details, refer to: https://github.com/PistonDevelopers/rust-empty/issues/175.

## Run
A few games are added to ```./games``` folder. To run a CHIP-8 ROM, use the following command:

```cargo run ./games/{your_game}``` 

- Example:
``` cargo run ./games/BLINKY```)

## Information Sources
- This project is based on the guide provided here: https://github.com/aquova/chip8-book

## TODO

### Required
- [ ] improve error handling
- [ ] fix the keyboard input
- [ ] add keyboard instructions to the README.md file

### Optional
- [ ] add audio support
- [ ] make the emulator runnable in the browser

