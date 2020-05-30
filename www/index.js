import { Simulation } from "ising-model-2d";
import { memory } from "ising-model-2d/ising_model_2d_bg";

/*
 * Constants
 */

const SIMULATION_WIDTH = 500;
const SIMULATION_HEIGHT = 500;

const PLOT_WIDTH = 400;
const PLOT_HEIGHT = 200;

const SPIN_WIDTH = 2; // px
const SPIN_HEIGHT = 2; // px

const SPIN_UP_COLOR = "#FF0000";
const SPIN_DOWN_COLOR = "#0000FF";

const COUPLING = 0.44;
const MAGNETIC_FIELD = 0.001;

/*
 * Initialize Control Panel
 */

document.getElementById("simulation-width").value = SIMULATION_WIDTH;
document.getElementById("simulation-height").value = SIMULATION_HEIGHT;

document.getElementById("plot-width").value = PLOT_WIDTH;
document.getElementById("plot-height").value = PLOT_HEIGHT;

document.getElementById("coupling-constant").value = COUPLING;
document.getElementById("initial-magnetic-field").value = MAGNETIC_FIELD;

let running = false;

const startStopButton = document.getElementById("start-stop-button");

startStopButton.addEventListener("click", event => {
    if (running) {
        startStopButton.textContent = "Start";
    } else {
        startStopButton.textContent = "Stop";
    }
    running = !running;
});

/*
 * Draw Plot
 */

const plot = document.getElementById("simulation-canvas");
plot.width = PLOT_WIDTH * SPIN_WIDTH;
plot.height = PLOT_HEIGHT * SPIN_HEIGHT;

const context = plot.getContext("2d");

const simulation = Simulation.new(SIMULATION_WIDTH,
                                  SIMULATION_HEIGHT,
                                  COUPLING,
                                  MAGNETIC_FIELD);

function getIndex(row, column) {
    return row * plot.width + column;
}

function isSpinUp(index, array) {
    let byte = Math.floor(index / 8);
    let mask = 1 << (index % 8);
    return (array[byte] & mask) === mask;
}

function drawSpins() {
    const spinsPtr = simulation.spins();
    const spins = new Uint8Array(memory.buffer, spinsPtr, plot.width * plot.height / 8);

    context.beginPath();

    for (let row = 0; row < plot.height; row++) {
        for (let col = 0; col < plot.width; col++) {
            const index = getIndex(row, col);

            context.fillStyle = isSpinUp(index, spins) ? 
                SPIN_UP_COLOR : SPIN_DOWN_COLOR;

            context.fillRect(
                col * (SPIN_HEIGHT+ 1) + 1,
                row * (SPIN_WIDTH + 1) + 1,
                SPIN_HEIGHT + 1,
                SPIN_WIDTH + 1 
            );
        }
    }

    context.stroke();
}

drawSpins();

/*
 * Animation
 */

const FRAME_TIME = 0.001;
let last_frame_update = null;

function renderLoop(timestamp) {
    if (!last_frame_update) last_frame_update = timestamp;
    let elapsed_time = timestamp - last_frame_update;

    drawSpins();

    if (elapsed_time >= FRAME_TIME) {
        simulation.update_spins();
        last_frame_update = timestamp;
    }

    requestAnimationFrame(renderLoop);
}

requestAnimationFrame(renderLoop);
