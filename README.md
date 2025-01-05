# Chip-8 Emulator

A simple CHIP-8 emulator written in Rust.

## Setup
1. Install Rust
   - Follow the instructions at https://www.rust-lang.org/tools/install to install Rust.
   
2. Install SDL2
   - Use Homebrew to install SDL2
   
```bash
brew install sdl2
```

   - For additional configuration details, refer to: https://github.com/PistonDevelopers/rust-empty/issues/175.

## Run
A few games are added to ```./games``` folder. To run a CHIP-8 ROM, use the following command:

```bash
cargo run ./games/{your_game}
``` 

Example: 

```bash
cargo run ./games/BLINKY
```

## Game controls

PONG
```bash
cargo run ./games/PONG
```

| CHIP-8 Key | Keyboard Key | Action                            |
|------------|--------------|-----------------------------------|
| 1          | `1`          | Move Player 1 (left paddle) up    |
| 4          | `Q`          | Move Player 1 (left paddle) down  |
| C          | `4`          | Move Player 2 (right paddle) up   |
| D          | `R`          | Move Player 2 (right paddle) down |

TETRIS
```bash
cargo run ./games/TETRIS
```

| CHIP-8 Key | Keyboard Key | Action            |
|------------|--------------|-------------------|   
| 4          | `W`          | 	Move piece left  |  
| 6          | `E`          | Move piece right  |
| 1          | `Q`          | Rotate piece      |
| 5          | `S`          | Move piece down   |

BLINKY

```bash
cargo run ./games/BLINKY
```

| CHIP-8 Key | Keyboard Key | Action     |
|------------|--------------|------------|
| 4          | `A`          | Move left  |
| 6          | `S`          | Move right |
| 3          | `3`          | Move up    |
| E          | `E`          | Move down  |

## Keyboard to Chip-8 Key Mapping

| Keyboard Key | CHIP-8 Key |
|--------------|------------|
| `1`          | `1`        |
| `2`          | `2`        |
| `3`          | `3`        |
| `4`          | `C`        |
| `Q`          | `4`        |
| `W`          | `5`        |
| `E`          | `6`        |
| `R`          | `D`        |
| `A`          | `7`        |
| `S`          | `8`        |
| `D`          | `9`        |
| `F`          | `E`        |
| `Z`          | `A`        |
| `X`          | `0`        |
| `C`          | `B`        |
| `V`          | `F`        |

## Information Sources
This project is based on the guide provided here: https://github.com/aquova/chip8-book

## Pre-Push Hook

This repository provides a `pre-push` hook located in the `.githooks` folder. It ensures that the code is properly 
formatted and linted before being pushed to the remote repository. To activate, configure Git to use the `.githooks`
directory as the hooks folder:

```bash
git config core.hooksPath .githooks
```

## TODO

### Required
- [ ] improve error handling
- [X] fix the keyboard input
- [X] add keyboard instructions to the README.md file

### Optional
- [ ] add audio support
- [ ] make the emulator runnable in the browser

