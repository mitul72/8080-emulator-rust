import { useEffect, useRef } from "react";
import {
  SpaceInvadersMachine,
  init,
} from "../public/pkg/intel_8080_emu_rust"; // Import the WASM module from the generated pkg folder

import Rom from "../public/roms/space_invaders/invaders?raw-hex";

const Emulator = () => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  // function frame(
  //   ctx: CanvasRenderingContext2D | null | undefined,
  //   machine: SpaceInvadersMachine
  // ) {
  //   // const SCREEN_WIDTH = 224;
  //   // const SCREEN_HEIGHT = 256;
  //   const SCALE_FACTOR = 2;

  //   if (!ctx) return;

  //   // Get the ImageData directly from Rust/WebAssembly
  //   const imageData = machine.get_frame_image_data(SCALE_FACTOR);

  //   // Put the imageData onto the canvas context in a single operation
  //   ctx.putImageData(imageData, 0, 0);
  // }

  useEffect(() => {
    init();
    // const canvas = canvasRef.current;
    // const ctx = canvas?.getContext("2d");

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
        // console.log("hello");
        // draw_screen(machine.get_memory());
        // frame(ctx, machine);
        requestAnimationFrame(renderFrame);
      }
      requestAnimationFrame(renderFrame);
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
