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
        mistakesUsed,
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

    function isYou(id: number | null): boolean {
        return id !== null && id === yourPlayerId;
    }
</script>

<div class="game-page">
    <header class="game-header">
        <div class="players">
            <div class="player player-one">
                <span class="player-dot"></span>
                <div>
                    <strong>{playerOneName}</strong>
                    {#if isYou(playerOneId)}
                        <small>You</small>
                    {/if}
                </div>
            </div>

            <div class="versus">VS</div>

            <div class="player player-two">
                <span class="player-dot"></span>
                <div>
                    <strong>{playerTwoName}</strong>
                    {#if isYou(playerTwoId)}
                        <small>You</small>
                    {/if}
                </div>
            </div>
        </div>

        <div class="game-meta">
            <span class="mode">
                {boardMode === "shared"
                    ? "Shared Board"
                    : "Separate Boards"}
            </span>

            {#if remainingSeconds !== null}
                <span class="timer">
                    ⏱ {formatTime(remainingSeconds)}
                </span>
            {/if}
        </div>
    </header>

    <div class="scoreboard">
        <div class="score-card player-one-card">
            <div class="score-name">
                {playerOneName}
                {#if isYou(playerOneId)}
                    <span>· You</span>
                {/if}
            </div>

            <strong>
                {scoreFor(playerOneId)?.score ?? 0}
            </strong>

            <span>
                {scoreFor(playerOneId)?.mistakes_used ?? 0}
                /
                {scoreFor(playerOneId)?.mistakes_allowed ?? mistakesAllowed}
                mistakes
            </span>
        </div>

        <div class="score-card player-two-card">
            <div class="score-name">
                {playerTwoName}
                {#if isYou(playerTwoId)}
                    <span>· You</span>
                {/if}
            </div>

            <strong>
                {scoreFor(playerTwoId)?.score ?? 0}
            </strong>

            <span>
                {scoreFor(playerTwoId)?.mistakes_used ?? 0}
                /
                {scoreFor(playerTwoId)?.mistakes_allowed ?? mistakesAllowed}
                mistakes
            </span>
        </div>
    </div>

    {#if boardMode === "shared"}
        <div class="legend">
            <span class="player-one-label">
                ● {playerOneName}
            </span>
            <span class="player-two-label">
                ● {playerTwoName}
            </span>
        </div>
    {:else}
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
</div>

<style>
    .game-page {
        min-height: 100vh;
        width: 100%;
        padding: 1.5rem 1rem 2rem;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
    }

    .game-header {
        width: min(94vw, 620px);
        display: flex;
        flex-direction: column;
        gap: 0.9rem;
    }

    .players {
        display: grid;
        grid-template-columns: 1fr auto 1fr;
        align-items: center;
        gap: 0.75rem;
        padding: 0.9rem 1rem;
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

    .player > div {
        display: flex;
        flex-direction: column;
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

    .player-dot {
        flex: 0 0 auto;
        width: 10px;
        height: 10px;
        border-radius: 50%;
    }

    .player-one .player-dot {
        background: #60a5fa;
    }

    .player-two .player-dot {
        background: #fb923c;
    }

    .player-one strong,
    .player-one-label {
        color: #60a5fa;
    }

    .player-two strong,
    .player-two-label {
        color: #fb923c;
    }

    .versus {
        color: var(--text-subtle);
        font-size: 0.75rem;
        font-weight: 800;
    }

    .game-meta {
        display: flex;
        justify-content: center;
        align-items: center;
        gap: 0.75rem;
        flex-wrap: wrap;
    }

    .mode,
    .timer {
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

    .scoreboard {
        width: min(94vw, 620px);
        display: grid;
        grid-template-columns: repeat(2, 1fr);
        gap: 0.75rem;
    }

    .score-card {
        padding: 0.75rem 1rem;
        border: 1px solid var(--border);
        border-radius: 10px;
        background: var(--surface);
        text-align: center;
    }

    .player-one-card {
        border-color: color-mix(in srgb, #60a5fa 35%, var(--border));
    }

    .player-two-card {
        border-color: color-mix(in srgb, #fb923c 35%, var(--border));
    }

    .score-name {
        color: var(--text-muted);
        font-size: 0.8rem;
        font-weight: 650;
    }

    .score-name span {
        color: var(--text-subtle);
        font-weight: 500;
    }

    .score-card strong {
        display: block;
        margin: 0.1rem 0;
        font-size: 1.4rem;
    }

    .score-card > span {
        color: var(--text-subtle);
        font-size: 0.7rem;
    }

    .legend {
        display: flex;
        justify-content: center;
        gap: 1rem;
        font-size: 0.8rem;
    }

    .mode-description {
        margin: 0;
        color: var(--text-muted);
        font-size: 0.8rem;
    }

    .status {
        margin: 0;
        color: var(--text-muted);
        font-size: 0.85rem;
    }

    .result {
        width: min(94vw, 620px);
        display: flex;
        flex-direction: column;
        gap: 0.35rem;
        padding: 1rem;
        border: 1px solid var(--border);
        border-radius: 12px;
        background: var(--surface);
        text-align: center;
    }

    .result strong {
        font-size: 1.25rem;
    }

    .result span {
        color: var(--text-muted);
        font-size: 0.85rem;
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

    @media (max-width: 480px) {
        .game-page {
            padding: 1rem 0.5rem 1.5rem;
        }

        .players {
            padding: 0.75rem;
        }

        .scoreboard {
            gap: 0.5rem;
        }

        .score-card {
            padding: 0.65rem 0.5rem;
        }
    }
</style>
