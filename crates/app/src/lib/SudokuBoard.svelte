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

    /*
     * Svelte only tracks the JavaScript object reference.
     *
     * SudokuGame is a mutable WASM object, so calling
     * game.make_move() changes the Rust-side state without
     * changing this JS reference.
     *
     * boardVersion is therefore an explicit render signal.
     * It is NEVER changed from inside an $effect.
     */
    let game = $state<SudokuGame | undefined>(externalGame);
    let selectedCell = $state<number | null>(null);
    let difficulty = $state(1);
    let solved = $state(false);
    let lockedCells = $state(new Set<number>());
    let incorrectValues = $state(new Map<number, number>());
    let boardVersion = $state(0);
    let incorrectTimers = new Map<number, number>();

    /*
     * Multiplayer synchronization only.
     *
     * syncVersion changes when the parent receives an authoritative
     * server update. We update the local game reference only when the
     * reference actually changed.
     *
     * Do not mutate boardVersion from this effect.
     */
    $effect(() => {
        if (!multiplayer) return;

        syncVersion;

        if (externalGame && game !== externalGame) {
            game = externalGame;
            selectedCell = null;
            solved = false;
            lockedCells = new Set<number>();
        }

        const nextIncorrect = new Map<number, number>();

        if (
            rejectedCell !== null &&
            rejectedValue !== null
        ) {
            nextIncorrect.set(
                rejectedCell,
                rejectedValue,
            );
        }

        incorrectValues = nextIncorrect;
        boardVersion += 1;
    });

    onMount(async () => {
        try {
            await initializeSudoku();

            if (!game && !multiplayer) {
                startNewGame();
            }
        } catch (error) {
            console.error(
                "Failed to initialize Sudoku:",
                error,
            );
        }

        function handleKeydown(event: KeyboardEvent) {
            const target = event.target as HTMLElement | null;

            if (
                target?.tagName === "INPUT" ||
                target?.tagName === "SELECT" ||
                target?.tagName === "TEXTAREA" ||
                target?.isContentEditable
            ) {
                return;
            }

            if (/^[1-9]$/.test(event.key)) {
                event.preventDefault();
                enterNumber(Number(event.key));
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
            handleKeydown,
        );

        return () => {
            window.removeEventListener(
                "keydown",
                handleKeydown,
            );
        };
    });

    function startNewGame() {
        if (multiplayer) return;

        for (const timer of incorrectTimers.values()) {
            window.clearTimeout(timer);
        }

        incorrectTimers.clear();

        const newGame = SudokuGame.generate(difficulty);

        if (!newGame) return;

        game = newGame;
        selectedCell = null;
        solved = false;
        lockedCells = new Set<number>();
        incorrectValues = new Map();
    }

    function resetGame() {
        if (!game || multiplayer) return;

        for (const timer of incorrectTimers.values()) {
            window.clearTimeout(timer);
        }

        incorrectTimers.clear();

        game.reset();

        selectedCell = null;
        solved = false;
        lockedCells = new Set<number>();
        incorrectValues = new Map();

        boardVersion += 1;
    }

    function getServerMove(
        index: number,
    ): ServerMove | undefined {
        return multiplayer
            ? serverMoves.get(index)
            : undefined;
    }

    function getDisplayedValue(index: number): number {
        if (!game) return 0;

        /*
         * Wrong single-player answers are not written into the
         * WASM board. Keep the attempted value in UI state so
         * the red feedback digit can actually be rendered.
         */
        const incorrectValue = incorrectValues.get(index);

        if (incorrectValue !== undefined) {
            return incorrectValue;
        }

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

        if (multiplayer) {
            if (getDisplayedValue(index) !== 0) {
                return;
            }

            selectedCell = index;
            return;
        }

        if (game.is_given(row, col)) return;
        if (lockedCells.has(index)) return;

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

        const row = Math.floor(selectedCell / 9);
        const col = selectedCell % 9;

        if (multiplayer) {
            onMove?.(row, col, value);
            return;
        }

        /*
         * Wrong answers are not written into the WASM board.
         * Keep the attempted digit in UI state temporarily so
         * the user can see it and get red visual feedback.
         */
        if (!game.is_correct(row, col, value)) {
            const cell = selectedCell;

            // Cancel the previous removal timer for this cell.
            const previousTimer = incorrectTimers.get(cell);

            if (previousTimer !== undefined) {
                window.clearTimeout(previousTimer);
            }

            const nextIncorrect = new Map(incorrectValues);
            nextIncorrect.set(cell, value);

            incorrectValues = nextIncorrect;
            boardVersion += 1;

            const timer = window.setTimeout(() => {
                const current = incorrectValues.get(cell);

                // Don't remove a newer wrong attempt.
                if (current !== value) {
                    return;
                }

                const next = new Map(incorrectValues);
                next.delete(cell);

                incorrectValues = next;
                incorrectTimers.delete(cell);

                boardVersion += 1;
            }, 650);

            incorrectTimers.set(cell, timer);

            return;
        }
        const accepted = game.make_move(
            row,
            col,
            value,
        );

        if (!accepted) return;

        /*
         * Lock the correctly entered cell.
         */
        lockedCells.add(selectedCell);
        lockedCells = new Set(lockedCells);

        /*
         * IMPORTANT:
         *
         * game.make_move() mutated the WASM object, but the
         * `game` reference itself did not change.
         *
         * Incrementing boardVersion forces the keyed board
         * subtree to render again and read the new WASM value.
         */
        boardVersion += 1;

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

        const row = Math.floor(selectedCell / 9);
        const col = selectedCell % 9;

        if (multiplayer) {
            onClear?.(row, col);
            return;
        }

        /*
         * clear_move() also mutates the existing WASM object,
         * so explicitly invalidate the board render.
         */
        game.clear_move(row, col);
        boardVersion += 1;
    }

    function isHighlighted(index: number): boolean {
        if (selectedCell === null) {
            return false;
        }

        const selectedRow = Math.floor(
            selectedCell / 9,
        );
        const selectedCol = selectedCell % 9;

        const row = Math.floor(index / 9);
        const col = index % 9;

        if (
            row === selectedRow ||
            col === selectedCol
        ) {
            return true;
        }

        const selectedBoxRow = Math.floor(
            selectedRow / 3,
        );
        const selectedBoxCol = Math.floor(
            selectedCol / 3,
        );

        const boxRow = Math.floor(row / 3);
        const boxCol = Math.floor(col / 3);

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
        {#key boardVersion}
            <div class="board">
                {#each Array.from({ length: 81 }) as _, index}
                    {@const row = Math.floor(index / 9)}
                    {@const col = index % 9}
                    {@const displayedValue = getDisplayedValue(index)}
                    {@const serverMove = getServerMove(index)}
                    {@const incorrectValue = incorrectValues.get(index)}

                    <SudokuCell
                        value={displayedValue}
                        given={game.is_given(row, col)}
                        selected={selectedCell === index}
                        highlighted={isHighlighted(index)}
                        sameNumber={isSameNumber(index)}
                        incorrect={
                            incorrectValue !== undefined
                        }
                        correct={
                            !multiplayer &&
                            lockedCells.has(index)
                        }
                        locked={
                            !multiplayer &&
                            lockedCells.has(index)
                        }
                        serverFilled={
                            multiplayer &&
                            !game.is_given(row, col) &&
                            serverMove !== undefined
                        }
                        playerId={serverMove?.playerSlot}
                        onclick={() => selectCell(index)}
                    />
                {/each}
            </div>
        {/key}

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
    }

    .loading {
        text-align: center;
        color: var(--text-muted);
    }
</style>
