export interface SudokuTheme {
    id: string;
    name: string;
    icon: string;

    background: string;
    backgroundGlow: string;

    surface: string;
    surfaceElevated: string;

    text: string;
    textMuted: string;
    textSubtle: string;

    border: string;
    borderStrong: string;

    cellBackground: string;
    cellHover: string;
    cellHighlighted: string;
    cellSameNumber: string;
    cellSelected: string;

    givenText: string;
    playerText: string;

    accent: string;
    accentHover: string;
    accentSoft: string;

    success: string;
    error: string;

    buttonBackground: string;
    buttonHover: string;
    buttonText: string;

    inputBackground: string;
}

export const themes: SudokuTheme[] = [
    {
        id: "midnight",
        name: "Midnight",
        icon: "🌑",

        background: "#0b1020",
        backgroundGlow: "rgba(59, 130, 246, 0.10)",

        surface: "#111827",
        surfaceElevated: "#172033",

        text: "#f3f4f6",
        textMuted: "#9ca3af",
        textSubtle: "#6b7280",

        border: "#293548",
        borderStrong: "#53627a",

        cellBackground: "#151e2e",
        cellHover: "#202c40",
        cellHighlighted: "#1b2940",
        cellSameNumber: "#263650",
        cellSelected: "#315b91",

        givenText: "#f9fafb",
        playerText: "#60a5fa",

        accent: "#60a5fa",
        accentHover: "#93c5fd",
        accentSoft: "rgba(96, 165, 250, 0.14)",

        success: "#4ade80",
        error: "#f87171",

        buttonBackground: "#182338",
        buttonHover: "#24334d",
        buttonText: "#e5e7eb",

        inputBackground: "#0f172a",
    },

    {
        id: "ocean",
        name: "Ocean",
        icon: "🌊",

        background: "#07161d",
        backgroundGlow: "rgba(6, 182, 212, 0.12)",

        surface: "#0b2029",
        surfaceElevated: "#10303b",

        text: "#ecfeff",
        textMuted: "#94aeb5",
        textSubtle: "#66828a",

        border: "#21434e",
        borderStrong: "#3a6875",

        cellBackground: "#0d2731",
        cellHover: "#143944",
        cellHighlighted: "#12343f",
        cellSameNumber: "#1b4855",
        cellSelected: "#176477",

        givenText: "#f0fdfa",
        playerText: "#5eead4",

        accent: "#22d3ee",
        accentHover: "#67e8f9",
        accentSoft: "rgba(34, 211, 238, 0.14)",

        success: "#34d399",
        error: "#fb7185",

        buttonBackground: "#12303a",
        buttonHover: "#194551",
        buttonText: "#e2fdfd",

        inputBackground: "#091d25",
    },

    {
        id: "forest",
        name: "Forest",
        icon: "🌲",

        background: "#0a160f",
        backgroundGlow: "rgba(34, 197, 94, 0.10)",

        surface: "#101f16",
        surfaceElevated: "#172a1e",

        text: "#f0fdf4",
        textMuted: "#9ab3a1",
        textSubtle: "#66806f",

        border: "#294333",
        borderStrong: "#4d6b57",

        cellBackground: "#14251a",
        cellHover: "#1b3323",
        cellHighlighted: "#193024",
        cellSameNumber: "#274735",
        cellSelected: "#28603a",

        givenText: "#f0fdf4",
        playerText: "#86efac",

        accent: "#4ade80",
        accentHover: "#86efac",
        accentSoft: "rgba(74, 222, 128, 0.14)",

        success: "#4ade80",
        error: "#fb7185",

        buttonBackground: "#193022",
        buttonHover: "#24442f",
        buttonText: "#e8f8ec",

        inputBackground: "#0d1d13",
    },

    {
        id: "sunset",
        name: "Sunset",
        icon: "🌅",

        background: "#17101a",
        backgroundGlow: "rgba(249, 115, 22, 0.12)",

        surface: "#21151f",
        surfaceElevated: "#2d1b27",

        text: "#fff7ed",
        textMuted: "#bba6b0",
        textSubtle: "#806c77",

        border: "#49313f",
        borderStrong: "#70505e",

        cellBackground: "#281b25",
        cellHover: "#38232f",
        cellHighlighted: "#35212e",
        cellSameNumber: "#4b2937",
        cellSelected: "#8a3f42",

        givenText: "#fff7ed",
        playerText: "#fdba74",

        accent: "#fb923c",
        accentHover: "#fdba74",
        accentSoft: "rgba(251, 146, 60, 0.14)",

        success: "#86efac",
        error: "#fb7185",

        buttonBackground: "#38212e",
        buttonHover: "#4b2a39",
        buttonText: "#fff7ed",

        inputBackground: "#1c1219",
    },

    {
        id: "classic",
        name: "Classic",
        icon: "☀️",

        background: "#f3f4f6",
        backgroundGlow: "rgba(37, 99, 235, 0.06)",

        surface: "#ffffff",
        surfaceElevated: "#f8fafc",

        text: "#111827",
        textMuted: "#6b7280",
        textSubtle: "#9ca3af",

        border: "#d1d5db",
        borderStrong: "#6b7280",

        cellBackground: "#ffffff",
        cellHover: "#f3f4f6",
        cellHighlighted: "#e8eef9",
        cellSameNumber: "#dce7f8",
        cellSelected: "#bfd4f2",

        givenText: "#111827",
        playerText: "#2563eb",

        accent: "#2563eb",
        accentHover: "#1d4ed8",
        accentSoft: "rgba(37, 99, 235, 0.10)",

        success: "#16a34a",
        error: "#dc2626",

        buttonBackground: "#ffffff",
        buttonHover: "#f3f4f6",
        buttonText: "#1f2937",

        inputBackground: "#ffffff",
    },

    {
        id: "high-contrast",
        name: "High Contrast",
        icon: "◐",

        background: "#000000",
        backgroundGlow: "rgba(255, 255, 255, 0.04)",

        surface: "#080808",
        surfaceElevated: "#111111",

        text: "#ffffff",
        textMuted: "#dddddd",
        textSubtle: "#aaaaaa",

        border: "#ffffff",
        borderStrong: "#ffffff",

        cellBackground: "#050505",
        cellHover: "#181818",
        cellHighlighted: "#222222",
        cellSameNumber: "#333333",
        cellSelected: "#555555",

        givenText: "#ffffff",
        playerText: "#00ffff",

        accent: "#00ffff",
        accentHover: "#ffffff",
        accentSoft: "rgba(0, 255, 255, 0.16)",

        success: "#00ff66",
        error: "#ff4444",

        buttonBackground: "#111111",
        buttonHover: "#222222",
        buttonText: "#ffffff",

        inputBackground: "#050505",
    },
];

export const defaultThemeId = "midnight";

export function getTheme(
    id: string,
): SudokuTheme {
    return (
        themes.find(
            (theme) => theme.id === id,
        ) ??
        themes.find(
            (theme) =>
                theme.id === defaultThemeId,
        )!
    );
}