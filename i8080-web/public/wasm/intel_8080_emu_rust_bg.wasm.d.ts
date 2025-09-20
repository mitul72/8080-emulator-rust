/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export function init(): void;
export function add(a: number, b: number): number;
export function draw_screen(a: number): void;
export function __wbg_spaceinvadersmachine_free(a: number, b: number): void;
export function spaceinvadersmachine_new(): Array;
export function spaceinvadersmachine_load_rom(a: number, b: number, c: number, d: number): void;
export function spaceinvadersmachine_get_memory(a: number): number;
export function spaceinvadersmachine_get_cpu_state(a: number): number;
export function spaceinvadersmachine_get_last_instructions(a: number): number;
export function spaceinvadersmachine_get_framebuffer_ptr(a: number): number;
export function spaceinvadersmachine_get_framebuffer_len(a: number): number;
export function spaceinvadersmachine_start_emulation(a: number): void;
export function spaceinvadersmachine_draw_screen(a: number): void;
export function spaceinvadersmachine_do_cpu(a: number): void;
export function spaceinvadersmachine_get_frame_image_data(a: number, b: number): Array;
export function spaceinvadersmachine_handle_key_down(a: number, b: number): void;
export function spaceinvadersmachine_handle_key_up(a: number, b: number): void;
export function main(): void;
export function __wbindgen_malloc(a: number, b: number): number;
export function __wbindgen_realloc(a: number, b: number, c: number, d: number): number;
export const __wbindgen_export_2: WebAssembly.Table;
export function __externref_table_dealloc(a: number): void;
export function __wbindgen_free(a: number, b: number, c: number): void;
export function __externref_table_alloc(): number;
export function __wbindgen_exn_store(a: number): void;
export function __wbindgen_start(): void;
