import React, { useEffect, useRef } from "react";
import initWasm, {
  SpaceInvadersMachine,
  draw_screen,
  init,
} from "../public/pkg/intel_8080_emu_rust"; // Import the WASM module from the generated pkg folder
import Rom from "../public/roms/space_invaders/invaders?raw-hex";

const Emulator = () => {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    init();
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
        draw_screen(machine.get_memory());
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
