import { useEffect, useRef, useCallback, useState } from "react";
import {
  SpaceInvadersMachine,
  init,
} from "../public/pkg/intel_8080_emu_rust"; // Import the WASM module from the generated pkg folder

import Rom from "../public/roms/space_invaders/invaders?raw-hex";

const SCALE_FACTOR = 2;

const Emulator = () => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const machineRef = useRef<SpaceInvadersMachine | null>(null);
  const [fps, setFps] = useState(0);
  const lastFrameTimeRef = useRef(performance.now());
  const frameCountRef = useRef(0);

  const renderFrame = useCallback(() => {
    const ctx = canvasRef.current?.getContext("2d");
    const machine = machineRef.current;

    if (ctx && machine) {
      machine.do_cpu();
      const imageData = machine.get_frame_image_data(SCALE_FACTOR);
      ctx.putImageData(imageData, 0, 0);
    }

    // FPS calculation
    const now = performance.now();
    frameCountRef.current++;
    if (now - lastFrameTimeRef.current >= 1000) {
      setFps(Math.round(frameCountRef.current * 1000 / (now - lastFrameTimeRef.current)));
      frameCountRef.current = 0;
      lastFrameTimeRef.current = now;
    }

    requestAnimationFrame(renderFrame);
  }, []);

  useEffect(() => {
    init();
    const canvas = canvasRef.current;
    if (!canvas) return;

    async function loadWasmAndStart() {
      const machine = new SpaceInvadersMachine();
      machineRef.current = machine;

      const romData = new Uint8Array(Rom);
      machine.load_rom(romData, 0);

      requestAnimationFrame(renderFrame);
    }

    loadWasmAndStart();

    return () => {
      machineRef.current = null;
    };
  }, [renderFrame]);

  return (
    <div className="emulator-container">
      <canvas
        id="gameCanvas"
        ref={canvasRef}
        width="448"
        height="512"
      />
      <div className="fps-counter">FPS: {fps}</div>
    </div>
  );
};

export default Emulator;
