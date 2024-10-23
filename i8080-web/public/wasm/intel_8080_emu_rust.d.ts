/* tslint:disable */
/* eslint-disable */
export function main(): void;
export function init(): void;
/**
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
export function add(a: number, b: number): number;
/**
 * @param {number} ptr
 */
export function draw_screen(ptr: number): void;
export class SpaceInvadersMachine {
  free(): void;
  constructor();
  /**
   * @param {Uint8Array} rom_data
   * @param {number} offset
   */
  load_rom(rom_data: Uint8Array, offset: number): void;
  /**
   * @returns {number}
   */
  get_memory(): number;
  /**
   * @returns {number}
   */
  get_framebuffer_ptr(): number;
  /**
   * @returns {number}
   */
  get_framebuffer_len(): number;
  start_emulation(): void;
  draw_screen(): void;
  do_cpu(): void;
  /**
   * @param {number} scale_factor
   * @returns {ImageData}
   */
  get_frame_image_data(scale_factor: number): ImageData;
  /**
   * @param {number} key
   */
  handle_key_down(key: number): void;
  /**
   * @param {number} key
   */
  handle_key_up(key: number): void;
}
