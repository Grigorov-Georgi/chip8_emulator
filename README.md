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

Example: ``` cargo run ./games/BLINKY```

## Game controls

PONG - ```cargo run ./games/PONG```

| CHIP-8 Key | Keyboard Key | Action                            |
|------------|--------------|-----------------------------------|
| 1          | `1`          | Move Player 1 (left paddle) up    |
| 4          | `Q`          | Move Player 1 (left paddle) down  |
| C          | `4`          | Move Player 2 (right paddle) up   |
| D          | `R`          | Move Player 2 (right paddle) down |

TETRIS - ```cargo run ./games/TETRIS```

| CHIP-8 Key | Keyboard Key | Action            |
|------------|--------------|-------------------|   
| 4          | `W`          | 	Move piece left  |  
| 6          | `E`          | Move piece right  |
| 1          | `Q`          | Rotate piece      |
| 5          | `S`          | Move piece down   |

BLINKY - ```cargo run ./games/BLINKY```

| CHIP-8 Key | Keyboard Key | Action     |
|------------|--------------|------------|
| 4          | `A`          | Move left  |
| 6          | `S`          | Move right |
| 3          | `3`          | Move up    |
| E          | `E`          | Move down  |

## Information Sources
This project is based on the guide provided here: https://github.com/aquova/chip8-book

## TODO

### Required
- [ ] improve error handling
- [X] fix the keyboard input
- [X] add keyboard instructions to the README.md file

### Optional
- [ ] add audio support
- [ ] make the emulator runnable in the browser

