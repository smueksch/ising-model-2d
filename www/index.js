import * as wasm from "ising-model-2d";

/*
 * Constants
 */

const SIMULATION_WIDTH = 1000;
const SIMULATION_HEIGHT = 1000;

const PLOT_WIDTH = 400;
const PLOT_HEIGHT = 200;

const SPIN_WIDTH = 2; // px
const SPIN_HEIGHT = 2; // px

/*
 * Initialize Control Panel
 */

document.getElementById("simulation-width").value = SIMULATION_WIDTH;
document.getElementById("simulation-height").value = SIMULATION_HEIGHT;

document.getElementById("plot-width").value = PLOT_WIDTH;
document.getElementById("plot-height").value = PLOT_HEIGHT;

const startStopButton = document.getElementById("start-stop-button");

const canvas = document.getElementById("simulation-canvas");
canvas.width = PLOT_WIDTH;
canvas.height = PLOT_HEIGHT;

const context = canvas.getContext("2d");
context.fillStyle = "#000000";
context.fillRect(0, 0, PLOT_WIDTH, PLOT_HEIGHT);

let running = false;

startStopButton.addEventListener("click", event => {
    if (running) {
        startStopButton.textContent = "Start";
    } else {
        startStopButton.textContent = "Stop";
    }
    running = !running;
});
