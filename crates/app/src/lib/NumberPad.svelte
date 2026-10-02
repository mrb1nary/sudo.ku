<script lang="ts">
    interface Props {
        onclick: (value: number) => void;
        onclear: () => void;
        remaining?: number[];
        notesEnabled?: boolean;
        onnotesToggle?: () => void;
    }

    let {
        onclick,
        onclear,
        remaining = [9, 9, 9, 9, 9, 9, 9, 9, 9],
        notesEnabled = false,
        onnotesToggle,
    }: Props = $props();
</script>

<div class="number-pad">
    <div class="numbers">
        {#each Array.from({ length: 9 }) as _, index}
            {@const number = index + 1}
            {@const left = remaining[index] ?? 9}
            {@const complete = left === 0}

            <button
                    class:complete
                    class="number"
                    type="button"
                    disabled={complete}
                    aria-label={
                    complete
                        ? `${number}, complete`
                        : `Enter ${number}, ${left} left`
                }
                    onclick={() => onclick(number)}
            >
                <span class="number-value">{number}</span>

                <span class="remaining">
                    {#if complete}
                        Complete
                    {:else}
                        {left} left
                    {/if}
                </span>
            </button>
        {/each}

        <button
                type="button"
                class:active={notesEnabled}
                class="notes-button"
                onclick={onnotesToggle}
                aria-pressed={notesEnabled}
        >
            <span class="notes-icon">✎</span>
            <span>Notes</span>
        </button>
    </div>


    <button
            class="clear"
            type="button"
            aria-label="Clear selected cell"
            onclick={onclear}
    >
        <span class="clear-icon" aria-hidden="true">×</span>
        <span>Clear</span>
    </button>

</div>

<style>
    .number-pad {
        display: flex;
        flex-direction: column;
        gap: 0.55rem;

        width: min(90vw, 540px);

        padding: 0.6rem;

        border: 1px solid var(--border);
        border-radius: 14px;

        background:
                linear-gradient(
                        145deg,
                        var(--surface-elevated),
                        var(--surface)
                );

        box-shadow:
                0 8px 24px rgba(0, 0, 0, 0.16),
                inset 0 1px 0 rgba(255, 255, 255, 0.025);
    }

    .numbers {
        display: grid;

        grid-template-columns:
            repeat(5, 1fr);

        gap: 0.5rem;
    }

    .number-pad button {
        display: flex;

        align-items: center;
        justify-content: center;

        border: 1px solid var(--border);
        border-radius: 10px;

        font: inherit;

        cursor: pointer;

        -webkit-tap-highlight-color: transparent;

        transition:
                transform 100ms ease,
                background 140ms ease,
                border-color 140ms ease,
                color 140ms ease,
                box-shadow 140ms ease;
    }

    .number {
        flex-direction: column;

        min-height: 62px;

        gap: 0.15rem;

        padding: 0.5rem;

        background:
                linear-gradient(
                        145deg,
                        var(--button-hover),
                        var(--button-bg)
                );

        color: var(--button-text);
    }

    .number-value {
        font-size: 1.15rem;
        font-weight: 750;
        line-height: 1;
    }

    .notes-button {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.4rem;

        width: 100%;
        padding: 0.65rem 0.8rem;

        border: 1px solid var(--border);
        border-radius: 8px;

        background: var(--button-bg);
        color: var(--button-text);

        font: inherit;
        font-weight: 650;

        cursor: pointer;

        transition:
                background-color 120ms ease,
                border-color 120ms ease,
                color 120ms ease,
                box-shadow 120ms ease;
    }

    .notes-button:hover {
        background: var(--button-hover);
    }

    .notes-button.active {
        border-color: var(--accent);
        background: var(--accent-soft);
        color: var(--accent);
        box-shadow: 0 0 0 1px var(--accent);
    }

    .notes-icon {
        font-size: 1rem;
        line-height: 1;
    }

    .remaining {
        min-height: 1em;

        color: var(--text-subtle);

        font-size: 0.68rem;
        font-weight: 600;

        line-height: 1;

        transition: color 140ms ease;
    }

    .number:hover:not(:disabled) {
        border-color: var(--accent);

        background:
                linear-gradient(
                        145deg,
                        var(--cell-highlighted),
                        var(--button-bg)
                );

        color: var(--accent);

        box-shadow:
                0 0 0 1px var(--accent-soft),
                0 5px 16px var(--accent-soft);
    }

    .number:hover:not(:disabled) .remaining {
        color: var(--accent);
    }

    .number:active:not(:disabled) {
        transform: scale(0.94);

        background: var(--cell-selected);

        box-shadow: none;
    }

    .number.complete {
        cursor: default;

        border-color: transparent;

        background:
                var(--surface);

        color: var(--text-subtle);

        opacity: 0.55;
    }

    .number.complete .remaining {
        color: var(--success);

        opacity: 0.8;
    }

    .number:focus-visible,
    .clear:focus-visible {
        outline: 2px solid var(--accent);

        outline-offset: 2px;
    }

    .clear {
        width: 100%;

        min-height: 42px;

        gap: 0.4rem;

        background: var(--button-bg);

        color: var(--text-muted);

        font-size: 0.88rem;
        font-weight: 650;
    }

    .clear:hover {
        border-color: var(--border-strong);

        background: var(--button-hover);

        color: var(--accent);

        box-shadow:
                0 0 0 1px var(--accent-soft);
    }

    .clear:active {
        transform: scale(0.98);
    }

    .clear-icon {
        display: inline-flex;

        align-items: center;
        justify-content: center;

        width: 18px;
        height: 18px;

        font-size: 1.25rem;
        font-weight: 400;

        line-height: 1;
    }

    @media (max-width: 420px) {
        .number-pad {
            gap: 0.45rem;
            padding: 0.5rem;
            border-radius: 12px;
        }

        .numbers {
            gap: 0.4rem;
        }

        .number {
            min-height: 57px;
            border-radius: 9px;
        }

        .number-value {
            font-size: 1.05rem;
        }

        .remaining {
            font-size: 0.62rem;
        }

        .clear {
            min-height: 40px;
        }
    }
</style>