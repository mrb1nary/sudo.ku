<script lang="ts">
    import SudokuBoard, { type ServerMove } from "./SudokuBoard.svelte";
    import { SudokuGame } from "./sudoku";

    type BoardMode = "shared" | "separate";
    type GameResult = "win" | "loss" | "draw" | null;
    type EndReason = "mistakes" | "timeout" | "completed" | null;

    interface PlayerScore {
        player_id: number;
        score: number;
        mistakes_used: number;
        mistakes_allowed: number;
    }

    interface Props {
        game: SudokuGame;
        boardMode: BoardMode;
        playerOneName: string;
        playerTwoName: string;
        playerOneId: number | null;
        playerTwoId: number | null;
        yourPlayerId: number | null;
        scores: Map<number, PlayerScore>;
        mistakesUsed: number;
        mistakesAllowed: number;
        remainingSeconds: number | null;
        gameEnded: boolean;
        gameResult: GameResult;
        endReason: EndReason;
        status: string;
        syncVersion: number;
        serverMoves: Map<number, ServerMove>;
        rejectedCell: number | null;
        rejectedValue: number | null;
        onMove: (row: number, col: number, value: number) => void;
        onClear: (row: number, col: number) => void;
        formatTime: (seconds: number | null) => string;
    }

    let {
        game,
        boardMode,
        playerOneName,
        playerTwoName,
        playerOneId,
        playerTwoId,
        yourPlayerId,
        scores,
        mistakesAllowed,
        remainingSeconds,
        gameEnded,
        gameResult,
        endReason,
        status,
        syncVersion,
        serverMoves,
        rejectedCell,
        rejectedValue,
        onMove,
        onClear,
        formatTime,
    }: Props = $props();

    function scoreFor(id: number | null): PlayerScore | undefined {
        if (id === null) return undefined;
        return scores.get(id);
    }

    let initialTimerSeconds = $state<number | null>(null);

    $effect(() => {
        if (remainingSeconds !== null && initialTimerSeconds === null) {
            initialTimerSeconds = remainingSeconds;
        }

        if (remainingSeconds === null) {
            initialTimerSeconds = null;
        }
    });

    function timerProgress(): number {
        if (remainingSeconds === null || initialTimerSeconds === null) {
            return 0;
        }

        if (initialTimerSeconds <= 0) {
            return 0;
        }

        return Math.max(0, Math.min(1, remainingSeconds / initialTimerSeconds));
    }

    function isYou(id: number | null): boolean {
        return id !== null && id === yourPlayerId;
    }
</script>

<div class="game-page">
    <header class="game-header">
        <div class="players">
            <div class="player player-one">
                <div class="player-info">
                    <div class="player-name">
                        <strong>{playerOneName}</strong>

                        {#if isYou(playerOneId)}
                            <small>You</small>
                        {/if}
                    </div>

                    <div class="mobile-score">
                        <strong>{scoreFor(playerOneId)?.score ?? 0}</strong>
                        <span>
                            {scoreFor(playerOneId)?.mistakes_used ?? 0}/{scoreFor(playerOneId)?.mistakes_allowed ?? mistakesAllowed}
                        </span>
                    </div>
                </div>
            </div>

            <div class="versus">VS</div>

            <div class="player player-two">
                <div class="player-info">
                    <div class="player-name">
                        <strong>{playerTwoName}</strong>

                        {#if isYou(playerTwoId)}
                            <small>You</small>
                        {/if}
                    </div>

                    <div class="mobile-score">
                        <strong>{scoreFor(playerTwoId)?.score ?? 0}</strong>
                        <span>
                            {scoreFor(playerTwoId)?.mistakes_used ?? 0}/{scoreFor(playerTwoId)?.mistakes_allowed ?? mistakesAllowed}
                        </span>
                    </div>
                </div>
            </div>
        </div>

        <div class="game-meta">
            <span class="mode">
                {boardMode === "shared" ? "Shared Board" : "Separate Boards"}
            </span>

            {#if remainingSeconds !== null}
                <span class="timer">
                    <span aria-hidden="true">◷</span>
                    {formatTime(remainingSeconds)}
                </span>
            {/if}
        </div>

        {#if remainingSeconds !== null}
            <div
                class="timer-progress"
                class:timer-warning={timerProgress() <= 0.25}
                class:timer-critical={timerProgress() <= 0.1}
                aria-label={`Time remaining: ${formatTime(remainingSeconds)}`}
                role="progressbar"
                aria-valuemin="0"
                aria-valuemax="100"
                aria-valuenow={Math.round(timerProgress() * 100)}
            >
                <div
                    class="timer-progress-fill"
                    style={`width: ${timerProgress() * 100}%`}
                ></div>
            </div>
        {/if}
    </header>

    <div class="game-layout">
        <aside class="score-card player-one-card">
            <div class="score-player">
                <div>
                    <strong>{playerOneName}</strong>
                    {#if isYou(playerOneId)}
                        <small>You</small>
                    {/if}
                </div>
            </div>

            <div class="score-value">
                {scoreFor(playerOneId)?.score ?? 0}
            </div>

            <div class="mistakes-label">SCORE</div>

            <div class="score-divider"></div>

            <div class="mistakes-value">
                {scoreFor(playerOneId)?.mistakes_used ?? 0}
                <span>/ {scoreFor(playerOneId)?.mistakes_allowed ?? mistakesAllowed}</span>
            </div>

            <div class="mistakes-label">MISTAKES</div>
        </aside>

        <main class="board-area">
            {#if boardMode === "separate"}
                <p class="mode-description">
                    First player to finish their board wins.
                </p>
            {/if}

            {#if gameEnded}
                <div
                    class="result"
                    class:win={gameResult === "win"}
                    class:loss={gameResult === "loss"}
                    class:draw={gameResult === "draw"}
                >
                    {#if gameResult === "win"}
                        <strong>🏆 You Win!</strong>

                        {#if endReason === "mistakes"}
                            <span>Your opponent reached the mistake limit.</span>
                        {:else if endReason === "completed"}
                            <span>
                                {boardMode === "shared"
                                    ? "The shared puzzle was completed and the final score determined the result."
                                    : "You completed your board first."}
                            </span>
                        {/if}
                    {:else if gameResult === "loss"}
                        <strong>😔 You Lose</strong>

                        {#if endReason === "mistakes"}
                            <span>You reached the mistake limit.</span>
                        {:else if endReason === "completed"}
                            <span>Your opponent finished first.</span>
                        {/if}
                    {:else}
                        <strong>
                            {endReason === "timeout" ? "⏱️ Time's Up" : "🤝 Draw"}
                        </strong>

                        {#if endReason === "completed"}
                            <span>The final shared-board scores were tied.</span>
                        {:else}
                            <span>The game ended before the puzzle was completed.</span>
                        {/if}
                    {/if}
                </div>
            {/if}

            {#if !gameEnded && status}
                <p class="status">{status}</p>
            {/if}

            <div class="board-wrapper">
                <SudokuBoard
                    externalGame={game}
                    multiplayer={true}
                    syncVersion={syncVersion}
                    serverMoves={serverMoves}
                    rejectedCell={rejectedCell}
                    rejectedValue={rejectedValue}
                    onMove={onMove}
                    onClear={onClear}
                />
            </div>
        </main>

        <aside class="score-card player-two-card">
            <div class="score-player">
                <div>
                    <strong>{playerTwoName}</strong>
                    {#if isYou(playerTwoId)}
                        <small>You</small>
                    {/if}
                </div>
            </div>

            <div class="score-value">
                {scoreFor(playerTwoId)?.score ?? 0}
            </div>

            <div class="mistakes-label">SCORE</div>

            <div class="score-divider"></div>

            <div class="mistakes-value">
                {scoreFor(playerTwoId)?.mistakes_used ?? 0}
                <span>/ {scoreFor(playerTwoId)?.mistakes_allowed ?? mistakesAllowed}</span>
            </div>

            <div class="mistakes-label">MISTAKES</div>
        </aside>
    </div>
</div>

<style>
    .game-page {
        width: 100%;
        min-height: 100vh;

        padding: 1rem 1rem 1.5rem;

        display: flex;
        flex-direction: column;
        align-items: center;

        gap: 0.75rem;
    }

    .game-header {
        width: min(94vw, 620px);

        display: flex;
        flex-direction: column;
        align-items: center;

        gap: 0.4rem;
    }

    .players {
        display: none;

        width: min(94vw, 620px);

        grid-template-columns: 1fr auto 1fr;
        align-items: center;

        gap: 0.75rem;

        padding: 0.75rem 1rem;

        border: 1px solid var(--border);
        border-radius: 12px;

        background: var(--surface);
    }

    .player {
        display: flex;
        align-items: center;
        gap: 0.55rem;
        min-width: 0;
    }

    .player:last-child {
        justify-content: flex-end;
        text-align: right;
    }

    .player-info {
        min-width: 0;
        display: flex;
        flex-direction: column;
        gap: 0.1rem;
    }

    .player-name {
        display: flex;
        align-items: center;
        gap: 0.35rem;
        min-width: 0;
    }

    .player strong {
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .player small {
        color: var(--text-muted);
        font-size: 0.7rem;
        font-weight: 500;
    }

    .versus {
        color: var(--text-subtle);
        font-size: 0.75rem;
        font-weight: 800;
    }

    .mobile-score {
        display: none;
    }

    .game-meta {
        display: flex;
        justify-content: center;
        align-items: center;
        gap: 0.55rem;
        flex-wrap: wrap;
    }

    .mode,
    .timer {
        display: inline-flex;
        align-items: center;
        gap: 0.3rem;

        padding: 0.35rem 0.7rem;

        border: 1px solid var(--border);
        border-radius: 999px;

        background: var(--surface);
        color: var(--text-muted);

        font-size: 0.8rem;
        font-weight: 650;
    }

    .timer {
        color: var(--accent);
    }

    .timer-progress {
        width: min(100%, 640px);
        height: 5px;

        overflow: hidden;

        border: 1px solid var(--border);
        border-radius: 999px;

        background: var(--surface);
        box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.18);
    }

    .timer-progress-fill {
        height: 100%;

        border-radius: inherit;

        background: var(--accent);

        box-shadow: 0 0 10px var(--accent-soft);

        transition: width 500ms linear, background-color 180ms ease;
    }

    .timer-warning .timer-progress-fill {
        background: var(--error);
        box-shadow: 0 0 10px color-mix(in srgb, var(--error) 30%, transparent);
    }

    .timer-critical .timer-progress-fill {
        animation: timer-pulse 900ms ease-in-out infinite;
    }

    .game-layout {
        width: min(100%, 1540px);

        display: grid;
        grid-template-columns: 170px minmax(0, 1080px) 170px;
        justify-content: center;
        align-items: center;

        column-gap: 1.5rem;

        margin-top: 0.25rem;
    }

    .score-card {
        width: 100%;
        min-height: 230px;

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;

        padding: 1.25rem 1rem;

        border: 1px solid var(--border);
        border-radius: 14px;

        background:
            linear-gradient(
                145deg,
                var(--surface-elevated),
                var(--surface)
            );

        text-align: center;

        box-shadow:
            0 12px 32px rgba(0, 0, 0, 0.12),
            inset 0 1px 0 rgba(255, 255, 255, 0.02);
    }

    .player-one-card {
        border-color: color-mix(in srgb, #60a5fa 45%, var(--border));
    }

    .player-two-card {
        border-color: color-mix(in srgb, #fb923c 45%, var(--border));
    }

    .score-player {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.45rem;

        min-width: 0;
        max-width: 100%;
    }

    .score-player > div {
        min-width: 0;

        display: flex;
        flex-direction: column;
        align-items: flex-start;
    }

    .score-player strong {
        max-width: 135px;

        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;

        font-size: 0.95rem;
    }

    .score-player small {
        color: var(--text-muted);
        font-size: 0.68rem;
    }

    .score-value {
        margin-top: 1.15rem;

        color: var(--text);

        font-size: 2.2rem;
        font-weight: 750;
        line-height: 1;
    }

    .mistakes-label {
        margin-top: 0.4rem;

        color: var(--text-subtle);

        font-size: 0.6rem;
        font-weight: 800;
        letter-spacing: 0.12em;
    }

    .score-divider {
        width: 42px;
        height: 1px;

        margin: 1rem 0 0.55rem;

        background: var(--border);
    }

    .mistakes-value {
        color: var(--text);
        font-size: 1rem;
        font-weight: 700;
    }

    .mistakes-value span {
        color: var(--text-subtle);
        font-weight: 500;
    }

    .board-area {
        width: 100%;
        min-width: 0;

        display: flex;
        flex-direction: column;
        align-items: center;

        gap: 0.65rem;
    }

    .mode-description {
        margin: 0;
        color: var(--text-muted);
        font-size: 0.8rem;
    }

    .status {
        width: 100%;
        min-height: 1rem;

        margin: 0;

        color: var(--text-muted);
        font-size: 0.8rem;
        text-align: center;
    }

    .result {
        width: 100%;

        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 0.3rem;

        padding: 0.75rem 1rem;

        border: 1px solid var(--border);
        border-radius: 12px;

        background: var(--surface);

        text-align: center;
    }

    .result strong {
        font-size: 1.15rem;
    }

    .result span {
        color: var(--text-muted);
        font-size: 0.78rem;
    }

    .result.win strong {
        color: var(--success);
    }

    .result.loss strong {
        color: var(--error);
    }

    .result.draw strong {
        color: var(--accent);
    }

    .board-wrapper {
        width: 100%;

        display: flex;
        justify-content: center;
    }

    /*
     * Desktop keeps the number pad below the board.
     * The board is intentionally wider than the old 540px layout,
     * while the reduced vertical spacing leaves room for the pad.
     */
    .board-wrapper :global(.game) {
        width: 100%;

        display: flex;
        flex-direction: column;
        align-items: center;

        gap: 0.75rem;
    }

    .board-wrapper :global(.board) {
        width: min(100%, 640px);
    }

    .board-wrapper :global(.number-pad) {
        width: min(100%, 640px);
    }

    @keyframes timer-pulse {
        0%,
        100% {
            opacity: 1;
        }

        50% {
            opacity: 0.55;
        }
    }

    @media (max-width: 1250px) and (min-width: 701px) {
        .game-layout {
            width: min(100%, 1120px);
            grid-template-columns: 130px minmax(0, 820px) 130px;
            column-gap: 1rem;
        }

        .board-wrapper :global(.game) {
            gap: 0.65rem;
        }

        .board-wrapper :global(.board) {
            width: min(100%, 600px);
        }

        .timer-progress {
            width: min(100%, 600px);
        }

        .board-wrapper :global(.number-pad) {
            width: min(100%, 600px);
        }

        .score-card {
            min-height: 210px;
            padding: 1rem 0.75rem;
        }

        .score-value {
            font-size: 2rem;
        }
    }

    @media (max-width: 700px) {
        .game-page {
            min-height: auto;
            gap: 0.5rem;
            padding: 0.45rem 0.35rem 1rem;
        }

        .game-header {
            width: 100%;
            gap: 0.4rem;
        }

        .players {
            display: grid;
            width: 100%;
            gap: 0.3rem;
            padding: 0.55rem 0.65rem;
            border-radius: 10px;
        }

        .player {
            gap: 0.4rem;
        }

        .player-name {
            gap: 0.25rem;
        }

        .player-one strong {
            color: #60a5fa;
        }

        .player-two strong {
            color: #fb923c;
        }

        .player strong {
            max-width: 90px;
            font-size: 0.85rem;
        }

        .player small {
            font-size: 0.62rem;
        }

        .mobile-score {
            display: flex;
            align-items: center;
            gap: 0.35rem;

            color: var(--text-subtle);
            font-size: 0.68rem;
        }

        .mobile-score strong {
            font-size: 0.8rem;
        }

        .versus {
            font-size: 0.65rem;
        }

        .game-meta {
            gap: 0.35rem;
        }

        .mode,
        .timer {
            padding: 0.25rem 0.55rem;
            font-size: 0.68rem;
        }

        .timer-progress {
            width: min(calc(100vw - 0.7rem), 540px);
            height: 4px;
        }

        .game-layout {
            width: 100%;
            display: flex;
            flex-direction: column;
            gap: 0.5rem;
            margin-top: 0;
        }

        .score-card {
            display: none;
        }

        .board-area {
            width: 100%;
            gap: 0.5rem;
        }

        .mode-description {
            display: none;
        }

        .status {
            font-size: 0.7rem;
        }

        .result {
            padding: 0.65rem 0.75rem;
            border-radius: 9px;
        }

        .result strong {
            font-size: 1rem;
        }

        .result span {
            font-size: 0.72rem;
        }

        .board-wrapper {
            width: 100%;
        }

        .board-wrapper :global(.game) {
            width: 100%;
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 0.65rem;
        }

        .board-wrapper :global(.board) {
            width: min(calc(100vw - 0.7rem), 540px);
        }
    }

    @media (max-width: 380px) {
        .game-page {
            padding-left: 0.2rem;
            padding-right: 0.2rem;
        }

        .players {
            padding: 0.45rem 0.5rem;
        }

        .player strong {
            max-width: 70px;
            font-size: 0.78rem;
        }

        .mobile-score {
            font-size: 0.6rem;
        }

        .mobile-score strong {
            font-size: 0.72rem;
        }

        .mode,
        .timer {
            padding: 0.22rem 0.45rem;
            font-size: 0.62rem;
        }
    }

    @media (prefers-reduced-motion: reduce) {
        .game-page * {
            scroll-behavior: auto;
        }

        .timer-progress-fill {
            transition: none;
            animation: none;
        }
    }
</style>
