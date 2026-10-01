<script lang="ts">
  import { onMount } from "svelte";
  import SudokuBoard from "./lib/SudokuBoard.svelte";
  import MultiplayerLobby from "./lib/MultiplayerLobby.svelte";
  import {
    defaultThemeId,
    getTheme,
    themes,
  } from "./lib/themes";

  type Route =
          | "/"
          | "/multiplayer";

  let path = $state<Route>(
          getRoute(window.location.pathname),
  );

  let multiplayerStarted =
          $state(false);

  let selectedTheme = $state(
          defaultThemeId,
  );

  function getRoute(
          pathname: string,
  ): Route {
    if (
            pathname === "/multiplayer"
    ) {
      return "/multiplayer";
    }

    return "/";
  }

  function navigate(to: Route) {
    if (
            to === "/" &&
            multiplayerStarted
    ) {
      return;
    }

    if (path === to) {
      return;
    }

    window.history.pushState(
            {},
            "",
            to,
    );

    path = to;
  }

  function handlePopState() {
    const nextPath =
            getRoute(
                    window.location.pathname,
            );

    /*
     * Once multiplayer has started we don't
     * allow browser back navigation to silently
     * return to single-player mode.
     */
    if (
            nextPath === "/" &&
            multiplayerStarted
    ) {
      window.history.pushState(
              {},
              "",
              "/multiplayer",
      );

      path = "/multiplayer";

      return;
    }

    path = nextPath;
  }

  function handleMultiplayerStarted() {
    multiplayerStarted = true;
  }

  function applyTheme(
          themeId: string,
  ) {
    const theme =
            getTheme(themeId);

    selectedTheme = theme.id;

    const root =
            document.documentElement;

    root.style.setProperty(
            "--background",
            theme.background,
    );

    root.style.setProperty(
            "--background-glow",
            theme.backgroundGlow,
    );

    root.style.setProperty(
            "--surface",
            theme.surface,
    );

    root.style.setProperty(
            "--surface-elevated",
            theme.surfaceElevated,
    );

    root.style.setProperty(
            "--text",
            theme.text,
    );

    root.style.setProperty(
            "--text-muted",
            theme.textMuted,
    );

    root.style.setProperty(
            "--text-subtle",
            theme.textSubtle,
    );

    root.style.setProperty(
            "--border",
            theme.border,
    );

    root.style.setProperty(
            "--border-strong",
            theme.borderStrong,
    );

    root.style.setProperty(
            "--cell-bg",
            theme.cellBackground,
    );

    root.style.setProperty(
            "--cell-hover",
            theme.cellHover,
    );

    root.style.setProperty(
            "--cell-highlighted",
            theme.cellHighlighted,
    );

    root.style.setProperty(
            "--cell-same-number",
            theme.cellSameNumber,
    );

    root.style.setProperty(
            "--cell-selected",
            theme.cellSelected,
    );

    root.style.setProperty(
            "--given-text",
            theme.givenText,
    );

    root.style.setProperty(
            "--player-text",
            theme.playerText,
    );

    root.style.setProperty(
            "--accent",
            theme.accent,
    );

    root.style.setProperty(
            "--accent-hover",
            theme.accentHover,
    );

    root.style.setProperty(
            "--accent-soft",
            theme.accentSoft,
    );

    root.style.setProperty(
            "--success",
            theme.success,
    );

    root.style.setProperty(
            "--error",
            theme.error,
    );

    root.style.setProperty(
            "--button-bg",
            theme.buttonBackground,
    );

    root.style.setProperty(
            "--button-hover",
            theme.buttonHover,
    );

    root.style.setProperty(
            "--button-text",
            theme.buttonText,
    );

    root.style.setProperty(
            "--input-bg",
            theme.inputBackground,
    );

    localStorage.setItem(
            "sudo-ku-theme",
            theme.id,
    );
  }

  onMount(() => {
    const savedTheme =
            localStorage.getItem(
                    "sudo-ku-theme",
            );

    applyTheme(
            savedTheme ??
            defaultThemeId,
    );

    window.addEventListener(
            "popstate",
            handlePopState,
    );

    return () => {
      window.removeEventListener(
              "popstate",
              handlePopState,
      );
    };
  });
</script>

<header>
  <div class="nav">
    <div class="brand">
            <span class="brand-mark">
                9
            </span>

      <span class="brand-name">
                sudo.ku
            </span>
    </div>

    <nav class="mode-switch">
      <button
              class:active={
                    path === "/"
                }
              type="button"
              onclick={() =>
                    navigate("/")
                }
              disabled={
                    multiplayerStarted
                }
              aria-current={
                    path === "/"
                        ? "page"
                        : undefined
                }
              title={
                    multiplayerStarted
                        ? "Single Player is disabled while a multiplayer game is active"
                        : "Single Player"
                }
      >
        Single Player
      </button>

      <button
              class:active={
                    path ===
                    "/multiplayer"
                }
              type="button"
              onclick={() =>
                    navigate(
                        "/multiplayer",
                    )
                }
              aria-current={
                    path ===
                    "/multiplayer"
                        ? "page"
                        : undefined
                }
      >
        Multiplayer
      </button>
    </nav>

    <label class="theme-picker">
            <span class="theme-label">
                Theme
            </span>

      <span class="theme-select-wrap">
                <span
                        class="theme-icon"
                >
                    {getTheme(
                            selectedTheme,
                    ).icon}
                </span>

                <select
                        value={selectedTheme}
                        onchange={(event) =>
                        applyTheme(
                            event
                                .currentTarget
                                .value,
                        )
                    }
                        aria-label="Select theme"
                >
                    {#each themes as theme}
                        <option
                                value={
                                theme.id
                            }
                        >
                            {theme.name}
                        </option>
                    {/each}
                </select>
            </span>
    </label>
  </div>
</header>

{#if path === "/multiplayer"}
  <main class="page">
    <MultiplayerLobby
            onGameStarted={
                handleMultiplayerStarted
            }
    />
  </main>
{:else}
  <main class="page">
    <section class="hero">
      <div>
        <p class="eyebrow">
          PUZZLE ENGINE
        </p>

        <h1>sudo.ku</h1>

        <p class="subtitle">
          A fast, focused
          Sudoku experience.
        </p>
      </div>
    </section>

    <section class="game-shell">
      <SudokuBoard />
    </section>
  </main>
{/if}

<style>
  header {
    position: sticky;
    top: 0;
    z-index: 20;

    width: 100%;

    border-bottom: 1px solid
    var(--border);

    background:
            color-mix(
                    in srgb,
                    var(--background) 88%,
                    transparent
            );

    backdrop-filter: blur(18px);
    -webkit-backdrop-filter: blur(18px);
  }

  .nav {
    display: grid;

    grid-template-columns:
            1fr auto 1fr;

    align-items: center;

    width: min(
            calc(100% - 2rem),
            1100px
    );

    min-height: 68px;

    margin: 0 auto;

    gap: 1rem;
  }

  .brand {
    display: flex;

    align-items: center;

    gap: 0.65rem;

    justify-self: start;

    color: var(--text);
  }

  .brand-mark {
    display: grid;

    place-items: center;

    width: 32px;
    height: 32px;

    border: 1px solid
    var(--accent);

    border-radius: 9px;

    background:
            var(--accent-soft);

    color: var(--accent);

    font-size: 0.95rem;
    font-weight: 800;

    box-shadow:
            0 0 20px
            color-mix(
                    in srgb,
                    var(--accent) 15%,
                    transparent
            );
  }

  .brand-name {
    font-size: 1rem;
    font-weight: 700;

    letter-spacing: -0.02em;
  }

  .mode-switch {
    display: flex;

    align-items: center;

    gap: 0.25rem;

    padding: 0.25rem;

    border: 1px solid
    var(--border);

    border-radius: 10px;

    background:
            var(--surface);
  }

  .mode-switch button {
    padding: 0.5rem 0.9rem;

    border: 0;

    border-radius: 7px;

    background: transparent;

    color: var(--text-muted);

    font: inherit;
    font-size: 0.85rem;
    font-weight: 600;

    cursor: pointer;

    transition:
            background 140ms ease,
            color 140ms ease,
            opacity 140ms ease;
  }

  .mode-switch button:hover:not(
        :disabled
    ) {
    background:
            var(--button-hover);

    color: var(--text);
  }

  .mode-switch button.active {
    background:
            var(--accent-soft);

    color: var(--accent);
  }

  .mode-switch button:disabled {
    opacity: 0.35;

    cursor: not-allowed;
  }

  .mode-switch button.active:disabled {
    opacity: 0.6;
  }

  .theme-picker {
    display: flex;

    align-items: center;

    justify-self: end;

    gap: 0.55rem;
  }

  .theme-label {
    color: var(--text-subtle);

    font-size: 0.75rem;
    font-weight: 600;

    text-transform: uppercase;
    letter-spacing: 0.08em;
  }

  .theme-select-wrap {
    position: relative;

    display: flex;

    align-items: center;

    gap: 0.35rem;

    padding: 0 0.55rem;

    border: 1px solid
    var(--border);

    border-radius: 9px;

    background:
            var(--surface);

    transition:
            border-color 140ms ease,
            background 140ms ease;
  }

  .theme-select-wrap:hover {
    border-color:
            var(--border-strong);
  }

  .theme-icon {
    pointer-events: none;

    font-size: 0.9rem;
  }

  .theme-select-wrap select {
    appearance: none;

    min-width: 95px;

    padding: 0.5rem 1.2rem
    0.5rem 0.1rem;

    border: 0;
    outline: 0;

    background: transparent;

    color: var(--text);

    font: inherit;
    font-size: 0.8rem;
    font-weight: 600;

    cursor: pointer;
  }

  .page {
    display: flex;

    flex-direction: column;

    align-items: center;

    width: 100%;

    min-height:
            calc(100vh - 68px);

    padding: 2.5rem 1rem 4rem;

    gap: 2rem;
  }

  .hero {
    width: min(
            90vw,
            540px
    );

    text-align: center;
  }

  .eyebrow {
    margin: 0 0 0.5rem;

    color: var(--accent);

    font-size: 0.68rem;
    font-weight: 800;

    letter-spacing: 0.16em;
  }

  h1 {
    margin: 0;

    color: var(--text);

    font-size: clamp(
            2rem,
            6vw,
            3rem
    );

    line-height: 1;

    letter-spacing: -0.06em;
  }

  .subtitle {
    margin: 0.75rem 0 0;

    color: var(--text-muted);

    font-size: 0.9rem;
  }

  .game-shell {
    display: flex;

    justify-content: center;

    width: min(
            100%,
            620px
    );

    padding: 1rem;

    border: 1px solid
    var(--border);

    border-radius: 18px;

    background:
            linear-gradient(
                    145deg,
                    var(--surface-elevated),
                    var(--surface)
            );

    box-shadow:
            0 20px 60px
            color-mix(
                    in srgb,
                    var(--background)
                    60%,
                    transparent
            ),
            0 0 50px
            var(--background-glow);
  }

  @media (max-width: 700px) {
    .nav {
      grid-template-columns:
                1fr auto;

      grid-template-areas:
                "brand theme"
                "mode mode";

      padding: 0.75rem 0;

      min-height: auto;
    }

    .brand {
      grid-area: brand;
    }

    .theme-picker {
      grid-area: theme;
    }

    .mode-switch {
      grid-area: mode;

      justify-self: center;
    }

    .theme-label {
      display: none;
    }

    .page {
      padding-top: 1.75rem;
    }

    .game-shell {
      padding: 0.65rem;

      border-radius: 14px;
    }
  }
</style>