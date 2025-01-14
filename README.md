# Chip-8 Emulator

A simple CHIP-8 emulator written in Rust.

## Desktop Application (runner)

1. Install Rust
   - Follow the instructions at https://www.rust-lang.org/tools/install to install Rust.
   
2. Install SDL2
   - Use Homebrew to install SDL2
   
```bash
brew install sdl2
```

- For additional configuration details, refer to: https://github.com/PistonDevelopers/rust-empty/issues/175.

3. Run
A few games are added to ```./games``` folder. To run a CHIP-8 ROM, use the following command:

```bash
cargo run ./games/{your_game}
``` 

Example: 

```bash
cargo run ./games/BLINKY
```

## Browser Application (wasm)

1. Build the wasm from the wasm folder [chip8_emulator/wasm] and move the resulted files into web folder

```bash
wasm-pack build --target web & mv pkg/wasm_bg.wasm ../web & mv pkg/wasm.js ../web
```

2. Start python server from web folder [chip8_emulator/web]
```bash
python3 -m http.server
```

3. Go to localhost:8000 and play

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

## Emulator Object - Emu

1. PC: program counter - keep the index of the current instruction.
2. RAM 
   1. ROM file data is copied into the RAM. 
   2. 4096 bytes (4KB) of size
3. Screen
   1. 64x32 monochrome display (1 bit per pixel)
4. V Registers
   1. 16 8-bit registers which the game can use as it pleases for much faster operation.
   2. Numbered from V0 to VF
5. I Register
   1. 16-bit
   2. used for indexing into RAM for reads and writes.
6. Stack
   1. 16-bit
   2. LIFO
   3. used while entering or exiting a subroutine, where the stack is used to know where you started so you can return after the routine ends
   4. std::collections::VecDeque is not used because it doesn't work for WebAssembly.
   5. used statically sized array and an index pointing the top ot the stack (sp - Stack Pointer)
7. Timers
   1. Delay Timer - register used by the system as a typical timer, counting down every cycle and performing some action if it hits 0
   2. Sound Timer - register used to count down every clock cycle, but upon hitting 0 emits a noise.
   3. Both registers are 8-bit

## Opcode Table

Chip-8 has 35 opcodes, which are all 2 bytes long and stored big-endian.
NNN: address
NN: 8-bit constant
N: 4-bit constant
X and Y: 4-bit register identifier
PC: Program Counter
I: 12-bit register
VN: One of the 16 available variables. N may be 0 to F (hexadecimal)

| Opcode | Pseudo Code                                                              | Explanation                                                                                                                                                                                                                                                                                                                                                                                                      |
|--------|--------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 0000   | -                                                                        | Do nothing and move on to the next opcode                                                                                                                                                                                                                                                                                                                                                                        |
| 00E0   | disp_clear()                                                             | Clears the screen. Reset the screen buffer                                                                                                                                                                                                                                                                                                                                                                       |
| 00EE   | return;                                                                  | Returns form a subroutine                                                                                                                                                                                                                                                                                                                                                                                        |
| 1NNN   | goto NNN;                                                                | Jumps to address NNN                                                                                                                                                                                                                                                                                                                                                                                             |
| 2NNN   | *(0xNNN) ()                                                              | Calls subroutine at NNN                                                                                                                                                                                                                                                                                                                                                                                          |
| 3XNN   | if(Vx == NN)                                                             | Skips the next instruction if VX equals NN (usually the next instruction is a jump to skip <br/> a code block)                                                                                                                                                                                                                                                                                                   |
| 4XNN   | if(Vx != NN)                                                             | Skips the next instruction if VX does not equal to NN (usually the next instruction is a <br/> jump to skip a code block)                                                                                                                                                                                                                                                                                        |
| 5XY0   | if(Vx == Vy)                                                             | Skips the next instruction if VX equals VY (usually the next instruction is a jump to skip <br/> a code block)                                                                                                                                                                                                                                                                                                   |
| 6XNN   | Vx = NN                                                                  | Sets VX to NN                                                                                                                                                                                                                                                                                                                                                                                                    |
| 7XNN   | Vx += NN                                                                 | Adds NN to VX (carry flag is not changed)                                                                                                                                                                                                                                                                                                                                                                        |
| 8XY0   | Vx = Vy                                                                  | Sets VX to the value of VY                                                                                                                                                                                                                                                                                                                                                                                       |
| 8XY1   | Vx \|= Vy                                                                | Sets VX to VX or VY (bitwise OR operation)                                                                                                                                                                                                                                                                                                                                                                       |
| 8XY2   | Vx &= Vy                                                                 | Sets VX to VX and VY (bitwise AND operation)                                                                                                                                                                                                                                                                                                                                                                     |
| 8XY3   | Vx ^= Vy                                                                 | Sets VX to VX xor XY (bitwise XOR operation)                                                                                                                                                                                                                                                                                                                                                                     |
| 8XY4   | Vx += Vy                                                                 | Adds VY yo VX. VF is set to 1 when there's an overflow, amd to 0 when there is not                                                                                                                                                                                                                                                                                                                               |
| 8XY5   | Vx -= Vy                                                                 | VY is subtracted from VX. VF is set to 0 where there's an underflow, and 1 when there is not                                                                                                                                                                                                                                                                                                                     |
| 8XY6   | Vx >>= 1                                                                 | Shifts VX to the right by 1, then stores the least significant bit of VX prior to the shift <br/> into VF                                                                                                                                                                                                                                                                                                        |
| 8XY7   | Vx = Vy - Vx                                                             | Sets VX to VY minus VX. VF is set to 0 when there's an underflow, and 1 when there is not                                                                                                                                                                                                                                                                                                                        |
| 8XYE   | Vx <<= 1                                                                 | Shifts VX to the left by 1, then sets VF to 1 if the most significant bit of VX prior to <br/> that shift was set, or to 0 if it was unset                                                                                                                                                                                                                                                                       |
| 9XY0   | if(VX != Vy)                                                             | Skips the next instruction if VX does not equal VY. (Usually the next instruction is a jump <br/> to skip a code block)                                                                                                                                                                                                                                                                                          |
| ANNN   | I = NNN                                                                  | Sets I to the address NNN.                                                                                                                                                                                                                                                                                                                                                                                       |
| BNNN   | PC = V0 + NNN                                                            | Jumps to the address NNN plus V0                                                                                                                                                                                                                                                                                                                                                                                 |
| CNNN   | Vx = rand() & NN                                                         | Sets Vx to the result of a bitwise and operation on a random number (0 - 255) and NN                                                                                                                                                                                                                                                                                                                             |
| DXYN   | draw(Vx, Vy, N)                                                          | Draw a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N <br/> pixels. Each row of 8 pixels is read as bit-coded starting from memory location I; <br/> I value does not change after the execution of this instruction. As described above, <br/> VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, <br/> and to 0 if that does not happen |
| EX9E   | if(key() == Vx)                                                          | Skips the next instruction if the key stored in VX (only consider the lowest nibble) is <br/> pressed (usually the next instruction is a jump to skip <br/> a code block)                                                                                                                                                                                                                                        |
| EXA1   | if(key() != Vx)                                                          | Skips the next instruction if the key stored in VX (only consider the lowest nibble) is <br/> not pressed (usually the next instruction is a jump to skip a code block)                                                                                                                                                                                                                                          |
| FX07   | Vx = get_delay()                                                         | Sets VX to the value of the delay timer                                                                                                                                                                                                                                                                                                                                                                          |
| FX0A   | Vx = get_key()                                                           | A key press is awaited, and then stored in VX (blocking operation, all instruction halted <br/> until next key event, delay and sound timers should continue processing)                                                                                                                                                                                                                                         |
| FX15   | delay_timer(Vx)                                                          | Sets the delay timer to VX                                                                                                                                                                                                                                                                                                                                                                                       |
| FX18   | sound_timer(Vx)                                                          | Sets the sound timer to VX                                                                                                                                                                                                                                                                                                                                                                                       |
| FX1E   | I += Vx                                                                  | Adds VX to I. Vf is not affected                                                                                                                                                                                                                                                                                                                                                                                 |
| FX29   | I = sprite_addr[Vx]                                                      | Sets I to the location of the sprite for the character in VX (only consider the lowest <br/> nibble). Characters 0-F (in hexadecimal) are represented by a 4x5 font.                                                                                                                                                                                                                                             |
| FX33   | set_BCD(Vx) <br/> *(I) = BCD(3); <br/> *(I+1) = BCD(2); *(I+2) = BCD(1); | Stores the binary-coded decimal representation of VX, with the hundreds digit in memory <br/> at location in I, the tens digit at location I + 1, and <br/> the ones digit at location I + 2                                                                                                                                                                                                                     |
| FX55   | reg_dump(Vx, &I)                                                         | Stores from V0 to VX (including VX) in memory, starting at address I. The offset from I <br/> is increased by 1 for each value written, but I itself is left unmodified.                                                                                                                                                                                                                                         |
| FX65   | reg_load(Vx, &I)                                                         | Fills from V0 to VX (including VX) with values from memory, starting at address I. The <br/> offset from I is increased by 1 for each value read, but I itself is left unmodified                                                                                                                                                                                                                                |

1. Explanations for Complex Opcodes
   1. FX07 - await key press - TODO
   2. DXYN - Draw - TODO
   3. FX33 - BCD - TODO

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

