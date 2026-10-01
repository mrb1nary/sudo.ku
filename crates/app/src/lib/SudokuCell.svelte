<script lang="ts">
    interface Props {
        value: number;
        given: boolean;
        selected: boolean;
        highlighted: boolean;
        sameNumber: boolean;
        incorrect: boolean;
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
                background-color 140ms ease,
                color 140ms ease,
                box-shadow 140ms ease,
                transform 100ms ease;
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

    /*
     * A successful player move gets a tiny pop when it
     * becomes locked on the board.
     */
    .cell.locked .cell-value {
        animation: number-enter 180ms ease-out;
    }

    .cell.server-filled .cell-value {
        animation: number-enter 180ms ease-out;
    }

    .cell.server-filled.player-one .cell-value {
        color: #60a5fa;
        font-weight: 750;
    }

    .cell.server-filled.player-two .cell-value {
        color: #fb923c;
        font-weight: 750;
    }

    .cell.server-filled:not(.player-one):not(.player-two) .cell-value,
    .cell.locked .cell-value {
        color: var(--player-text);
        font-weight: 700;
    }

    .cell.locked {
        color: var(--player-text);
        font-weight: 700;
        cursor: default;
    }

    /*
     * Wrong answer feedback.
     *
     * The class is intentionally animation-driven rather than
     * permanently changing the cell background.
     */
    .cell.incorrect {
        z-index: 4;

        color: var(--error);

        background:
                color-mix(
                        in srgb,
                        var(--error) 12%,
                        var(--cell-bg)
                );

        border-color:
                color-mix(
                        in srgb,
                        var(--error) 55%,
                        var(--border)
                );

        box-shadow:
                inset 0 0 0 2px
                color-mix(
                        in srgb,
                        var(--error) 55%,
                        transparent
                ),
                0 0 14px
                color-mix(
                        in srgb,
                        var(--error) 25%,
                        transparent
                );

        animation: wrong-answer 360ms ease-out;
    }

    .cell.incorrect .cell-value {
        color: var(--error);

        font-weight: 750;

        animation: wrong-number 360ms ease-out;
    }

    .cell.selected.server-filled.player-one .cell-value {
        color: #60a5fa;
    }

    .cell.selected.server-filled.player-two .cell-value {
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

    @keyframes wrong-answer {
        0% {
            transform: translateX(0);
        }

        15% {
            transform: translateX(-5px);
        }

        30% {
            transform: translateX(5px);
        }

        45% {
            transform: translateX(-4px);
        }

        60% {
            transform: translateX(4px);
        }

        75% {
            transform: translateX(-2px);
        }

        100% {
            transform: translateX(0);
        }
    }

    @keyframes wrong-number {
        0% {
            transform: scale(1);
        }

        35% {
            transform: scale(1.16);
        }

        100% {
            transform: scale(1);
        }
    }

    @keyframes number-enter {
        0% {
            transform: scale(0.72);
            opacity: 0.35;
        }

        65% {
            transform: scale(1.08);
            opacity: 1;
        }

        100% {
            transform: scale(1);
        }
    }

    @media (prefers-reduced-motion: reduce) {
        .cell,
        .cell-value {
            animation: none !important;

            transition: none;
        }
    }
</style>