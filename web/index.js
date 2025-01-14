import init, * as wasm from "./wasm.js"

const WIDTH = 64
const HEIGHT = 32
const SCALE = 15
const TICKS_PER_FRAME = 10
let anim_frame = 0

const canvas = document.getElementById("canvas")
canvas.width = WIDTH * SCALE
canvas.height = HEIGHT * SCALE

const ctx = canvas.getContext("2d")
ctx.fillStyle = "black"
ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE)

const input = document.getElementById("fileinput")

async function run() {
    await init()
    let chip8 = new wasm.EmuWasm()

    document.addEventListener("keydown", function (evt) {
        chip8.keypress(evt, true)
    })

    document.addEventListener("keyup", function (evt) {
        chip8.keypress(evt, false)
    })

    const loadGameButton = document.getElementById("loadGameButton")
    const gameSelect = document.getElementById("gameSelect")

    loadGameButton.addEventListener("click", async () => {
        try {
            if (anim_frame != 0) {
                window.cancelAnimationFrame(anim_frame)
            }

            const response = await fetch(gameSelect.value)
            const romData = await response.arrayBuffer()

            const rom = new Uint8Array(romData)
            chip8.reset()
            chip8.load_game(rom)
            mainloop(chip8)

            console.log(`Loaded game from: ${gamePath}`);
        } catch (error) {
            console.error("Error loading game:", error);
        }
    })
}

function mainloop(chip8) {
    // Only draw every few ticks
    for (let i = 0; i < TICKS_PER_FRAME; i++) {
        chip8.tick()
    }
    chip8.tick_timers()

    // Clear the canvas before drawing
    ctx.fillStyle = "black"
    ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE)

    // Set the draw color back to white before we render our frame
    ctx.fillStyle = "white"
    chip8.draw_screen(SCALE)

    anim_frame = window.requestAnimationFrame(() => {
        mainloop(chip8)
    })

}

run().catch(console.error)