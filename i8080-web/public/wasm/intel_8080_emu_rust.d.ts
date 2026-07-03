/* tslint:disable */
/* eslint-disable */
/**
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
export function add(a: number, b: number): number;
export function main(): void;
/**
 * @param {number} ptr
 */
export function draw_screen(ptr: number): void;
export function init(): void;
export class SpaceInvadersMachine {
  free(): void;
  /**
   * @returns {number}
   */
  get_memory(): number;
  draw_screen(): void;
  /**
   * @returns {any}
   */
  get_cpu_state(): any;
  /**
   * @param {number} key
   */
  handle_key_up(key: number): void;
  /**
   * @param {number} key
   */
  handle_key_down(key: number): void;
  start_emulation(): void;
  /**
   * @returns {number}
   */
  get_framebuffer_len(): number;
  /**
   * @returns {number}
   */
  get_framebuffer_ptr(): number;
  /**
   * @returns {ImageData}
   */
  get_frame_image_data(): ImageData;
  /**
   * @returns {any}
   */
  get_last_instructions(): any;
  constructor();
  do_cpu(): void;
  /**
   * @param {Uint8Array} rom_data
   * @param {number} offset
   */
  load_rom(rom_data: Uint8Array, offset: number): void;
}
