<script lang="ts">
    import { onMount } from "svelte";
    import { initializeSudoku, SudokuGame } from "./sudoku";
    import SudokuCell from "./SudokuCell.svelte";
    import NumberPad from "./NumberPad.svelte";

    export interface ServerMove {
        value: number;
        playerId: number;
        playerSlot: number;
    }

    interface Props {
        externalGame?: SudokuGame;
        multiplayer?: boolean;
        syncVersion?: number;
        serverMoves?: Map<number, ServerMove>;
        rejectedCell?: number | null;
        rejectedValue?: number | null;
        onMove?: (row: number, col: number, value: number) => void;
        onClear?: (row: number, col: number) => void;
    }

    let {
        externalGame,
        multiplayer = false,
        syncVersion = 0,
        serverMoves = new Map(),
        rejectedCell = null,
        rejectedValue = null,
        onMove,
        onClear,
    }: Props = $props();

    let game = $state<SudokuGame | undefined>(externalGame);
    let selectedCell = $state<number | null>(null);
    let difficulty = $state(1);
    let solved = $state(false);
    let lockedCells = $state(new Set<number>());
    let incorrectValues = $state(new Map<number, number>());
    let incorrectCell = $state<number | null>(null);
    let animationTimer: ReturnType<typeof setTimeout> | null = null;

    $effect(() => {
        syncVersion;

        if (externalGame) {
            game = externalGame;
        }

        incorrectValues = new Map();
    });

    $effect(() => {
        if (
            multiplayer &&
            rejectedCell !== null &&
            rejectedCell !== undefined
        ) {
            triggerIncorrectAnimation(
                rejectedCell,
                rejectedValue ?? 0,
            );
        }
    });

    onMount(async () => {
        await initializeSudoku();

        if (!game && !multiplayer) {
            startNewGame();
        }

        return () => {
            if (animationTimer) {
                clearTimeout(animationTimer);
            }
        };
    });

    function triggerIncorrectAnimation(
        cell: number,
        value: number,
    ) {
        incorrectCell = null;

        if (animationTimer) {
            clearTimeout(animationTimer);
        }

        /*
         * Force the class to be removed and re-added.
         * This guarantees the CSS animation restarts even if
         * the user makes the same mistake twice in a row.
         */
        requestAnimationFrame(() => {
            incorrectCell = cell;

            incorrectValues = new Map(
                incorrectValues,
            );

            incorrectValues.set(cell, value);
        });

        animationTimer = setTimeout(() => {
            incorrectCell = null;

            incorrectValues = new Map(
                incorrectValues,
            );

            incorrectValues.delete(cell);
        }, 420);
    }

    function startNewGame() {
        if (multiplayer) return;

        const newGame = SudokuGame.generate(difficulty);
        if (!newGame) return;

        game = newGame;
        selectedCell = null;
        solved = false;
        lockedCells = new Set();
        incorrectValues = new Map();
        incorrectCell = null;
    }

    function resetGame() {
        if (!game || multiplayer) return;

        game.reset();
        selectedCell = null;
        solved = false;
        lockedCells = new Set();
        incorrectValues = new Map();
        incorrectCell = null;
    }

    function getServerMove(index: number): ServerMove | undefined {
        return multiplayer ? serverMoves.get(index) : undefined;
    }

    function getDisplayedValue(index: number): number {
        if (!game) return 0;

        const serverMove = getServerMove(index);

        if (serverMove) {
            return serverMove.value;
        }

        const row = Math.floor(index / 9);
        const col = index % 9;

        return game.get_cell(row, col);
    }

    function selectCell(index: number) {
        if (!game || solved) return;

        const row = Math.floor(index / 9);
        const col = index % 9;

        /*
         * Given cells can still be selected so their number
         * can be highlighted across the board.
         */
        if (game.is_given(row, col)) {
            selectedCell = index;
            return;
        }

        if (multiplayer) {
            if (getDisplayedValue(index) !== 0) return;

            selectedCell = index;
            return;
        }

        if (lockedCells.has(index)) return;

        selectedCell = index;
    }

    function enterNumber(value: number) {
        if (!game || selectedCell === null || solved) return;
        if (lockedCells.has(selectedCell)) return;

        const row = Math.floor(selectedCell / 9);
        const col = selectedCell % 9;

        if (game.is_given(row, col)) return;

        if (multiplayer) {
            onMove?.(row, col, value);
            return;
        }

        /*
         * Wrong number:
         *
         * Don't mutate the board. Instead trigger the visual
         * feedback animation on the selected cell.
         */
        if (!game.is_correct(row, col, value)) {
            triggerIncorrectAnimation(
                selectedCell,
                value,
            );

            return;
        }

        const accepted = game.make_move(
            row,
            col,
            value,
        );

        if (!accepted) return;

        lockedCells.add(selectedCell);
        lockedCells = new Set(lockedCells);

        if (game.is_solved()) {
            solved = true;
        }
    }

    function clearCell() {
        if (!game || selectedCell === null || solved) return;
        if (lockedCells.has(selectedCell)) return;

        const row = Math.floor(selectedCell / 9);
        const col = selectedCell % 9;

        if (game.is_given(row, col)) return;

        if (multiplayer) {
            onClear?.(row, col);
            return;
        }

        game.clear_move(row, col);
    }

    function isHighlighted(index: number): boolean {
        if (selectedCell === null) return false;

        const selectedRow = Math.floor(
            selectedCell / 9,
        );

        const selectedCol =
            selectedCell % 9;

        const row = Math.floor(index / 9);
        const col = index % 9;

        if (
            row === selectedRow ||
            col === selectedCol
        ) {
            return true;
        }

        const selectedBoxRow =
            Math.floor(selectedRow / 3);

        const selectedBoxCol =
            Math.floor(selectedCol / 3);

        const boxRow =
            Math.floor(row / 3);

        const boxCol =
            Math.floor(col / 3);

        return (
            selectedBoxRow === boxRow &&
            selectedBoxCol === boxCol
        );
    }

    function isSameNumber(index: number): boolean {
        if (selectedCell === null) {
            return false;
        }

        const selectedValue =
            getDisplayedValue(selectedCell);

        if (selectedValue === 0) {
            return false;
        }

        return (
            getDisplayedValue(index) ===
            selectedValue
        );
    }
</script>

{#if game}
    <div class="game">
        <div class="board">
            {#each Array.from({ length: 81 }) as _, index}
                {@const row = Math.floor(index / 9)}
                {@const col = index % 9}
                {@const displayedValue = getDisplayedValue(index)}
                {@const serverMove = getServerMove(index)}

                <SudokuCell
                        value={
                        incorrectCell === index
                            ? incorrectValues.get(index) ?? displayedValue
                            : displayedValue
                    }
                        given={game.is_given(row, col)}
                        selected={selectedCell === index}
                        highlighted={isHighlighted(index)}
                        sameNumber={isSameNumber(index)}
                        incorrect={incorrectCell === index}
                        locked={
                        !multiplayer &&
                        lockedCells.has(index)
                    }
                        serverFilled={
                        multiplayer &&
                        !game.is_given(row, col) &&
                        serverMove !== undefined
                    }
                        playerId={
                        serverMove?.playerSlot
                    }
                        onclick={() =>
                        selectCell(index)
                    }
                />
            {/each}
        </div>

        <NumberPad
                onclick={enterNumber}
                onclear={clearCell}
        />

        {#if !multiplayer}
            <div class="controls">
                <select bind:value={difficulty}>
                    <option value={0}>Easy</option>
                    <option value={1}>Medium</option>
                    <option value={2}>Hard</option>
                </select>

                <button
                        type="button"
                        onclick={resetGame}
                >
                    Reset
                </button>

                <button
                        type="button"
                        onclick={startNewGame}
                >
                    New Game
                </button>
            </div>
        {/if}

        {#if solved}
            <p class="solved">
                🎉 Puzzle solved!
            </p>
        {/if}
    </div>
{:else}
    <p class="loading">
        Loading Sudoku...
    </p>
{/if}

<style>
    .game {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
    }

    .board {
        display: grid;

        grid-template-columns:
            repeat(9, minmax(0, 1fr));

        grid-template-rows:
            repeat(9, minmax(0, 1fr));

        width: min(90vw, 540px);

        aspect-ratio: 1;
    }

    .controls {
        display: flex;
        gap: 0.75rem;
        align-items: center;
    }

    select,
    .controls button {
        padding: 0.6rem 0.9rem;

        border-radius: 6px;
        border: 1px solid var(--border);

        background: var(--button-bg);
        color: var(--button-text);

        font-size: 0.9rem;
    }

    .controls button {
        cursor: pointer;
    }

    .controls button:hover {
        background: var(--button-hover);
    }

    .solved {
        margin: 0;

        color: var(--success);

        font-weight: 600;

        animation: solved-pop 300ms ease-out;
    }

    .loading {
        text-align: center;
        color: var(--text-muted);
    }

    @keyframes solved-pop {
        0% {
            transform: scale(0.9);
            opacity: 0;
        }

        70% {
            transform: scale(1.05);
            opacity: 1;
        }

        100% {
            transform: scale(1);
        }
    }

    @media (prefers-reduced-motion: reduce) {
        .solved {
            animation: none;
        }
    }
</style>