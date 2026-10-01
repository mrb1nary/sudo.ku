/* tslint:disable */
/* eslint-disable */

export class SudokuGame {
    free(): void;
    [Symbol.dispose](): void;
    static base(): SudokuGame;
    can_place(row: number, col: number, value: number): boolean;
    clear_move(row: number, col: number): boolean;
    find_empty(): Uint32Array | undefined;
    static from_puzzle(puzzle: Uint8Array): SudokuGame | undefined;
    static generate(difficulty: number): SudokuGame | undefined;
    get_board(): Uint8Array;
    get_cell(row: number, col: number): number;
    is_complete(): boolean;
    is_correct(row: number, col: number, value: number): boolean;
    is_empty(row: number, col: number): boolean;
    is_given(row: number, col: number): boolean;
    is_solved(): boolean;
    make_move(row: number, col: number, value: number): boolean;
    constructor();
    static random(): SudokuGame;
    reset(): void;
    solve(): boolean;
    solve_with_config(use_hidden_singles: boolean): boolean;
    solve_with_stats(): Uint32Array;
    solve_with_stats_config(use_hidden_singles: boolean): Uint32Array;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_sudokugame_free: (a: number, b: number) => void;
    readonly sudokugame_base: () => number;
    readonly sudokugame_can_place: (a: number, b: number, c: number, d: number) => number;
    readonly sudokugame_clear_move: (a: number, b: number, c: number) => number;
    readonly sudokugame_find_empty: (a: number) => [number, number];
    readonly sudokugame_from_puzzle: (a: number, b: number) => number;
    readonly sudokugame_generate: (a: number) => number;
    readonly sudokugame_get_board: (a: number) => [number, number];
    readonly sudokugame_get_cell: (a: number, b: number, c: number) => number;
    readonly sudokugame_is_complete: (a: number) => number;
    readonly sudokugame_is_correct: (a: number, b: number, c: number, d: number) => number;
    readonly sudokugame_is_empty: (a: number, b: number, c: number) => number;
    readonly sudokugame_is_given: (a: number, b: number, c: number) => number;
    readonly sudokugame_is_solved: (a: number) => number;
    readonly sudokugame_make_move: (a: number, b: number, c: number, d: number) => number;
    readonly sudokugame_new: () => number;
    readonly sudokugame_random: () => number;
    readonly sudokugame_reset: (a: number) => void;
    readonly sudokugame_solve: (a: number) => number;
    readonly sudokugame_solve_with_config: (a: number, b: number) => number;
    readonly sudokugame_solve_with_stats: (a: number) => [number, number];
    readonly sudokugame_solve_with_stats_config: (a: number, b: number) => [number, number];
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
