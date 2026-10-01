<script lang="ts">
    interface Props {
        value: number;
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
    <span class="cell-value">
        {value === 0 ? "" : value}
    </span>
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

    .cell.selected:hover {
        background: var(--cell-selected);
    }

    .cell.given {
        color: var(--given-text);
        font-weight: 750;
    }

    .cell.correct .cell-value {
        color: var(--success);
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

    .cell.server-filled:not(.player-one):not(.player-two)
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

    .cell.selected.server-filled.player-one
        .cell-value {
        color: #60a5fa;
    }

    .cell.selected.server-filled.player-two
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
