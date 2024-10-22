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
  do_cpu(): void;
}
