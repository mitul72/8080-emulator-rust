import { useEffect, useRef } from "react";
import {
  SpaceInvadersMachine,
  // draw_screen,
  init,
} from "../public/pkg/intel_8080_emu_rust"; // Import the WASM module from the generated pkg folder
import { memory } from "../public/pkg/intel_8080_emu_rust_bg.wasm";

import Rom from "../public/roms/space_invaders/invaders?raw-hex";

const Emulator = () => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  function frame(
    ctx: CanvasRenderingContext2D | null | undefined,
    machine: SpaceInvadersMachine
  ) {
    const SCREEN_WIDTH = 224;
    const SCREEN_HEIGHT = 256;
    const SCALE_FACTOR = 2;

    if (!ctx) return;

    // Access the memory from WebAssembly and the framebuffer
    const memPtr = machine.get_framebuffer_ptr(); // Memory pointer from the WASM
    const memLength = machine.get_framebuffer_len();

    // Create an ImageData object to hold pixel data for the canvas
    const imageData = ctx.createImageData(
      SCREEN_WIDTH * SCALE_FACTOR,
      SCREEN_HEIGHT * SCALE_FACTOR
    );
    const pixels = imageData.data; // Access the pixel array

    // Create a Uint8Array view over the WebAssembly memory
    const framebuffer = new Uint8Array(memory.buffer, memPtr, memLength);

    // Loop through the framebuffer and set pixels
    for (let i = 0; i < memLength; i++) {
      const byte = framebuffer[i];
      const basePixelIndex = i * 8;

      for (let bit = 0; bit < 8; bit++) {
        const pixelOn = (byte >> bit) & 1;
        const pixelIndex = basePixelIndex + bit;

        // Calculate x and y position for each pixel
        const x = pixelIndex % SCREEN_HEIGHT;
        const y = Math.floor(pixelIndex / SCREEN_HEIGHT);

        // Set the color (white if pixel is on, black otherwise)
        const color = pixelOn ? 255 : 0;

        // Set the color for this pixel in the imageData buffer
        const scaledX = (SCREEN_HEIGHT - x) * SCALE_FACTOR;
        const scaledY = y * SCALE_FACTOR;

        for (let dy = 0; dy < SCALE_FACTOR; dy++) {
          for (let dx = 0; dx < SCALE_FACTOR; dx++) {
            const pixelIdx =
              4 *
              ((scaledX + dy) * (SCREEN_WIDTH * SCALE_FACTOR) + (scaledY + dx));
            pixels[pixelIdx] = color; // Red
            pixels[pixelIdx + 1] = color; // Green
            pixels[pixelIdx + 2] = color; // Blue
            pixels[pixelIdx + 3] = 255; // Alpha (fully opaque)
          }
        }
      }
    }

    // Put the imageData onto the canvas context in a single operation
    ctx.putImageData(imageData, 0, 0);
  }

  useEffect(() => {
    init();
    const canvas = canvasRef.current;
    const ctx = canvas?.getContext("2d");

    async function loadWasmAndStart() {
      const machine = new SpaceInvadersMachine();

      // Load ROMs and pass them to WebAssembly

      let offset = 0;

      const romData = new Uint8Array(Rom);

      machine.load_rom(romData, offset);
      offset += romData.length;

      // Start the emulation

      // while (true) {
      function renderFrame() {
        machine.start_emulation();
        frame(ctx, machine);
        // draw_screen(machine.get_memory());
        // machine.draw_screen();
        requestAnimationFrame(renderFrame); // Schedule the next frame
      }
      requestAnimationFrame(renderFrame); // Schedule the first frame
    }
    loadWasmAndStart();
  }, []);

  return (
    <div>
      <canvas
        id="gameCanvas"
        ref={canvasRef}
        width="448"
        height="512"
        style={{ border: "1px solid black" }}
      />
    </div>
  );
};

export default Emulator;
