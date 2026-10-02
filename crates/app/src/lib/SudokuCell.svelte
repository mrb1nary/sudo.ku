<script lang="ts">
    interface Props {
        value: number;
        notes?: number[];
        given: boolean;
        selected: boolean;
        highlighted: boolean;
        sameNumber: boolean;
        incorrect: boolean;
        correct: boolean;
        locked: boolean;
        serverFilled: boolean;
        playerId?: number | null;
        onclick: () => void;
    }

    let {
        value,
        notes=[],
        given,
        selected,
        highlighted,
        sameNumber,
        incorrect,
        correct,
        locked,
        serverFilled,
        playerId = null,
        onclick,
    }: Props = $props();
</script>

<button
    class="cell"
    class:selected
    class:given
    class:highlighted
    class:same-number={sameNumber}
    class:incorrect
    class:correct
    class:locked
    class:server-filled={serverFilled}
    class:player-one={serverFilled && playerId === 1}
    class:player-two={serverFilled && playerId === 2}
    type="button"
    {onclick}
>
    {#if value !== 0}
    <span class="cell-value">
        {value}
    </span>
    {:else if notes.length > 0}
    <span class="cell-notes">
        {#each Array.from({ length: 9 }) as _, index}
            <span class:note-active={notes.includes(index + 1)}>
                {notes.includes(index + 1)
                    ? index + 1
                    : ""}
            </span>
        {/each}
    </span>
    {/if}
</button>

<style>
    .cell {
        position: relative;

        display: flex;
        align-items: center;
        justify-content: center;

        width: 100%;
        height: 100%;

        min-width: 0;
        min-height: 0;

        padding: 0;

        border: 1px solid var(--border);

        background: var(--cell-bg);
        color: var(--given-text);

        font-size: clamp(1rem, 4vw, 1.35rem);
        font-weight: 500;

        cursor: pointer;

        transition:
            background-color 120ms ease,
            color 120ms ease,
            box-shadow 120ms ease,
            transform 120ms ease;
    }

    .cell-value {
        display: block;

        color: inherit;

        font-size: inherit;
        font-weight: inherit;
        line-height: 1;

        user-select: none;
    }

    .cell-notes {
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        grid-template-rows: repeat(3, 1fr);

        width: 100%;
        height: 100%;

        padding: 8%;
        box-sizing: border-box;

        color: var(--text-muted);
        font-size: clamp(0.45rem, 1.8vw, 0.7rem);
        font-weight: 600;
        line-height: 1;
    }

    .cell-notes span {
        display: flex;
        align-items: center;
        justify-content: center;

        min-width: 0;
        min-height: 0;

        user-select: none;
    }

    .cell-notes .note-active {
        color: var(--text);
    }

    .cell-notes {
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        grid-template-rows: repeat(3, 1fr);

        width: 100%;
        height: 100%;

        padding: 8%;
        box-sizing: border-box;

        color: var(--text-muted);
        font-size: clamp(0.45rem, 1.8vw, 0.7rem);
        font-weight: 600;
        line-height: 1;
    }

    .cell-notes span {
        display: flex;
        align-items: center;
        justify-content: center;

        min-width: 0;
        min-height: 0;

        user-select: none;
    }

    .cell-notes .note-active {
        color: var(--text);
    }

    .cell:hover {
        background: var(--cell-hover);
    }

    .cell.highlighted {
        background: var(--cell-highlighted);
    }

    .cell.same-number {
        background: var(--cell-same-number);
    }

    .cell.selected {
        z-index: 2;

        background: var(--cell-selected);

        box-shadow:
            inset 0 0 0 2px var(--accent),
            0 0 0 1px color-mix(
                in srgb,
                var(--accent) 35%,
                transparent
            );
    }

    .cell.correct .cell-value {
        color: var(--success);
        font-weight: 750;
    }

    .cell.selected:hover {
        background: var(--cell-selected);
    }

    .cell.given {
        color: var(--text);
        font-weight: 750;
    }

    .cell.server-filled.player-one .cell-value {
        color: #60a5fa;
        font-weight: 750;
    }

    .cell.server-filled.player-two .cell-value {
        color: #fb923c;
        font-weight: 750;
    }

    /* Given cells are always white */
    .cell.given .cell-value {
        color: #ffffff !important;
        font-weight: 750;
    }



    .cell.server-filled.player-one .cell-value {
        color: #60a5fa;
        font-weight: 750;
    }

    .cell.server-filled.player-two .cell-value {
        color: #fb923c;
        font-weight: 750;
    }

    .cell.server-filled:not(.player-one):not(.player-two),
        .cell-value {
        color: var(--player-text);
        font-weight: 700;
    }

    .cell.locked {
        cursor: default;
    }

    /*
     * Wrong answer feedback.
     *
     * This animation belongs here because .cell is rendered
     * by SudokuCell.svelte. Putting the keyframes/class in
     * SudokuBoard.svelte does not style this child component's
     * scoped DOM.
     */
    .cell.incorrect {
        color: var(--error);
        font-weight: 700;

        animation:
            incorrect-shake 180ms ease-in-out,
            incorrect-glow 450ms ease-out;
    }

    .cell.incorrect .cell-value {
        color: var(--error);
        font-weight: 750;
    }

    @keyframes incorrect-shake {
        0%,
        100% {
            transform: translateX(0);
        }

        25% {
            transform: translateX(-3px);
        }

        50% {
            transform: translateX(0);
        }

        75% {
            transform: translateX(3px);
        }
    }

    @keyframes incorrect-glow {
        0% {
            box-shadow:
                inset 0 0 0 2px var(--error),
                0 0 14px var(--error);
        }

        60% {
            box-shadow:
                inset 0 0 0 2px var(--error),
                0 0 10px var(--error);
        }

        100% {
            box-shadow:
                inset 0 0 0 0 transparent,
                0 0 0 transparent;
        }
    }

    .cell.selected.server-filled.player-one,
        .cell-value {
        color: #60a5fa;
    }

    .cell.selected.server-filled.player-two,
        .cell-value {
        color: #fb923c;
    }

    .cell.selected.given {
        background: var(--cell-selected);
    }

    .cell:nth-child(3n) {
        border-right: 2px solid var(--border-strong);
    }

    .cell:nth-child(3n + 1) {
        border-left: 2px solid var(--border-strong);
    }

    .cell:nth-child(n + 19):nth-child(-n + 27),
    .cell:nth-child(n + 46):nth-child(-n + 54),
    .cell:nth-child(n + 73):nth-child(-n + 81) {
        border-bottom: 2px solid var(--border-strong);
    }

    .cell:nth-child(-n + 9),
    .cell:nth-child(n + 28):nth-child(-n + 36),
    .cell:nth-child(n + 55):nth-child(-n + 63) {
        border-top: 2px solid var(--border-strong);
    }
</style>
