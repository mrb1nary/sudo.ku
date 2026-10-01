import init, { SudokuGame } from "./wasm/sudoku_wasm.js";

let initialized = false;

export async function initializeSudoku(): Promise<void> {
    if (initialized) {
        return;
    }

    await init();
    initialized = true;
}

export { SudokuGame };