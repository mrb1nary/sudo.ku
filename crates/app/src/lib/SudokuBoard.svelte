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

    $effect(() => {
        syncVersion;

        if (externalGame) {
            game = externalGame;
        }

        incorrectValues = new Map();
    });

    onMount(async () => {
        await initializeSudoku();

        if (!game && !multiplayer) {
            startNewGame();
        }

        /*
         * Keyboard controls
         *
         * 1-9       -> enter number
         * Backspace -> clear cell
         * Delete    -> clear cell
         */
        function handleKeyboard(event: KeyboardEvent) {
            /*
             * Don't steal keyboard input when the user is
             * interacting with another form control.
             */
            const target = event.target as HTMLElement | null;

            if (
                target?.tagName === "INPUT" ||
                target?.tagName === "TEXTAREA" ||
                target?.tagName === "SELECT" ||
                target?.isContentEditable
            ) {
                return;
            }

            if (event.key >= "1" && event.key <= "9") {
                event.preventDefault();

                enterNumber(
                    Number(event.key),
                );

                return;
            }

            if (
                event.key === "Backspace" ||
                event.key === "Delete"
            ) {
                event.preventDefault();

                clearCell();
            }
        }

        window.addEventListener(
            "keydown",
            handleKeyboard,
        );

        return () => {
            window.removeEventListener(
                "keydown",
                handleKeyboard,
            );
        };
    });

    function startNewGame() {
        if (multiplayer) return;

        const newGame =
            SudokuGame.generate(
                difficulty,
            );

        if (!newGame) return;

        game = newGame;
        selectedCell = null;
        solved = false;
        lockedCells = new Set();
        incorrectValues = new Map();
    }

    function resetGame() {
        if (!game || multiplayer) return;

        game.reset();

        selectedCell = null;
        solved = false;
        lockedCells = new Set();
        incorrectValues = new Map();
    }

    function getServerMove(
        index: number,
    ): ServerMove | undefined {
        return multiplayer
            ? serverMoves.get(index)
            : undefined;
    }

    function getDisplayedValue(
        index: number,
    ): number {
        if (!game) return 0;

        const serverMove =
            getServerMove(index);

        if (serverMove) {
            return serverMove.value;
        }

        const row =
            Math.floor(index / 9);

        const col =
            index % 9;

        return game.get_cell(
            row,
            col,
        );
    }

    function selectCell(index: number) {
        if (!game || solved) return;

        const row =
            Math.floor(index / 9);

        const col =
            index % 9;

        if (multiplayer) {
            if (
                getDisplayedValue(index) !== 0
            ) {
                return;
            }

            selectedCell = index;

            return;
        }

        if (game.is_given(row, col)) {
            return;
        }

        if (lockedCells.has(index)) {
            return;
        }

        selectedCell = index;
    }

    function enterNumber(value: number) {
        if (
            !game ||
            selectedCell === null ||
            solved
        ) {
            return;
        }

        if (lockedCells.has(selectedCell)) {
            return;
        }

        const row =
            Math.floor(selectedCell / 9);

        const col =
            selectedCell % 9;

        if (multiplayer) {
            onMove?.(
                row,
                col,
                value,
            );

            return;
        }

        if (
            !game.is_correct(
                row,
                col,
                value,
            )
        ) {
            return;
        }

        const accepted =
            game.make_move(
                row,
                col,
                value,
            );

        if (!accepted) return;

        lockedCells.add(
            selectedCell,
        );

        lockedCells =
            new Set(lockedCells);

        if (game.is_solved()) {
            solved = true;
        }
    }

    function clearCell() {
        if (
            !game ||
            selectedCell === null ||
            solved
        ) {
            return;
        }

        if (lockedCells.has(selectedCell)) {
            return;
        }

        const row =
            Math.floor(selectedCell / 9);

        const col =
            selectedCell % 9;

        if (multiplayer) {
            onClear?.(
                row,
                col,
            );

            return;
        }

        game.clear_move(
            row,
            col,
        );
    }

    function isHighlighted(
        index: number,
    ): boolean {
        if (selectedCell === null) {
            return false;
        }

        const selectedRow =
            Math.floor(
                selectedCell / 9,
            );

        const selectedCol =
            selectedCell % 9;

        const row =
            Math.floor(index / 9);

        const col =
            index % 9;

        if (
            row === selectedRow ||
            col === selectedCol
        ) {
            return true;
        }

        const selectedBoxRow =
            Math.floor(
                selectedRow / 3,
            );

        const selectedBoxCol =
            Math.floor(
                selectedCol / 3,
            );

        const boxRow =
            Math.floor(row / 3);

        const boxCol =
            Math.floor(col / 3);

        return (
            selectedBoxRow === boxRow &&
            selectedBoxCol === boxCol
        );
    }

    function isSameNumber(
        index: number,
    ): boolean {
        if (selectedCell === null) {
            return false;
        }

        const selectedValue =
            getDisplayedValue(
                selectedCell,
            );

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
                        value={displayedValue}
                        given={game.is_given(row, col)}
                        selected={selectedCell === index}
                        highlighted={isHighlighted(index)}
                        sameNumber={isSameNumber(index)}
                        incorrect={false}
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
                    <option value={0}>
                        Easy
                    </option>

                    <option value={1}>
                        Medium
                    </option>

                    <option value={2}>
                        Hard
                    </option>
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
            repeat(
                9,
                minmax(0, 1fr)
            );

        grid-template-rows:
            repeat(
                9,
                minmax(0, 1fr)
            );

        width:
                min(
                        90vw,
                        540px
                );

        aspect-ratio: 1;
    }

    .controls {
        display: flex;

        gap: 0.75rem;

        align-items: center;
    }

    select,
    .controls button {
        padding:
                0.6rem
                0.9rem;

        border-radius: 6px;

        border: 1px solid
        var(--border);

        background:
                var(--button-bg);

        color:
                var(--button-text);

        font-size: 0.9rem;
    }

    .controls button {
        cursor: pointer;
    }

    .controls button:hover {
        background:
                var(--button-hover);
    }

    .solved {
        margin: 0;

        color:
                var(--success);

        font-weight: 600;
    }

    .loading {
        text-align: center;

        color:
                var(--text-muted);
    }
</style>