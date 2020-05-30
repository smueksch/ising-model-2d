import * as wasm from "ising-model-2d";

const startStopButton = document.getElementById("start-stop-button");

const canvas = document.getElementById("simulation-canvas");
canvas.width = 200
canvas.height = 200

const context = canvas.getContext("2d");
context.fillStyle = "#000000";
context.fillRect(0, 0, 200, 200);

let running = false;

startStopButton.addEventListener("click", event => {
    if (running) {
        startStopButton.textContent = "Start";
    } else {
        startStopButton.textContent = "Stop";
    }
    running = !running;
});
