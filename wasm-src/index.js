import { Universe } from "./wasm_cgol";
import { memory } from "./wasm_cgol_bg";

const universe = new Universe();

const CELL_SIZE = 5;
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const DEAD = 0;
const ALIVE = 1;


const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * universe.height + 1;
canvas.width = (CELL_SIZE + 1) * universe.width + 1;

const ctx = canvas.getContext('2d');

const drawRect = (ctx, fill, row, col) => {
  ctx.fillStyle = fill;
  ctx.fillRect(
    row * (CELL_SIZE + 1) + 1,
    col * (CELL_SIZE + 1) + 1,
    CELL_SIZE,
    CELL_SIZE
  );
}

const drawCells = (cellArray) => {
  ctx.beginPath();

  for (let row = 0; row < universe.height; row++) {
    for (let col = 0; col < universe.width; col++) {
      let cell = cellArray[(row * universe.width) + col];
      if (cell == ALIVE) {
        drawRect(ctx, ALIVE_COLOR, row, col);
      } else {
        drawRect(ctx, DEAD_COLOR, row, col);
      }
    }
  }

  ctx.stroke();
}

const update = () => {
  drawCells(universe.as_array());
  universe.tick();
}

window.setInterval(update, 250);
