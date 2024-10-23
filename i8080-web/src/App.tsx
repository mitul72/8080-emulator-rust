import { useEffect, useRef, useCallback, useState } from "react";
import { SpaceInvadersMachine, init } from "../public/wasm/intel_8080_emu_rust"; // Import the WASM module from the generated pkg folder

import Rom from "../public/roms/space_invaders/invaders?raw-hex";

import "./Emulator.css";

const SCALE_FACTOR = 2;

class FpsCtrl {
  private delay: number;
  private time: number | null = null;
  private frame: number = -1;
  private tref: number | null = null;
  private callback: (e: { time: number; frame: number }) => void;
  public isPlaying: boolean = false;

  constructor(
    private fps: number,
    callback: (e: { time: number; frame: number }) => void
  ) {
    this.delay = 1000 / fps;
    this.callback = callback;
  }

  private loop = (timestamp: number) => {
    if (this.time === null) this.time = timestamp;
    const seg = Math.floor((timestamp - this.time) / this.delay);
    if (seg > this.frame) {
      this.frame = seg;
      this.callback({
        time: timestamp,
        frame: this.frame,
      });
    }
    this.tref = requestAnimationFrame(this.loop);
  };

  start() {
    if (!this.isPlaying) {
      this.isPlaying = true;
      this.tref = requestAnimationFrame(this.loop);
    }
  }

  pause() {
    if (this.isPlaying) {
      if (this.tref !== null) cancelAnimationFrame(this.tref);
      this.isPlaying = false;
      this.time = null;
      this.frame = -1;
    }
  }

  setFrameRate(newfps: number) {
    this.fps = newfps;
    this.delay = 1000 / this.fps;
    this.frame = -1;
    this.time = null;
  }
}

const Emulator = () => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const offscreenCanvasRef = useRef<OffscreenCanvas | null>(null);
  const machineRef = useRef<SpaceInvadersMachine | null>(null);
  const [fps, setFps] = useState(0);
  const fpsCtrlRef = useRef<FpsCtrl | null>(null);
  const frameCountRef = useRef(0);
  const lastFpsUpdateRef = useRef(performance.now());

  const renderFrame = useCallback(() => {
    const offscreenCanvas = offscreenCanvasRef.current;
    const machine = machineRef.current;

    if (offscreenCanvas && machine) {
      const ctx = offscreenCanvas.getContext("2d");
      if (ctx) {
        ctx.clearRect(0, 0, offscreenCanvas.width, offscreenCanvas.height);
        for (let i = 0; i < 3; i++) {
          machine.do_cpu();
        }
        const imageData = machine.get_frame_image_data(SCALE_FACTOR);
        ctx.putImageData(imageData, 0, 0);
      }
    }

    // FPS calculation
    frameCountRef.current++;
    const now = performance.now();
    if (now - lastFpsUpdateRef.current >= 1000) {
      setFps(
        Math.round(
          (frameCountRef.current * 1000) / (now - lastFpsUpdateRef.current)
        )
      );
      frameCountRef.current = 0;
      lastFpsUpdateRef.current = now;
    }
  }, []);

  const handleKeyDown = useCallback((event: KeyboardEvent) => {
    if (machineRef.current) {
      switch (event.key) {
        case 'ArrowLeft':
          machineRef.current.handle_key_down(0x20); // Left
          event.preventDefault();
          break;
        case 'ArrowRight':
          machineRef.current.handle_key_down(0x40); // Right
          event.preventDefault();
          break;
        case ' ':
          machineRef.current.handle_key_down(0x10); // Fire
          event.preventDefault();
          break;
        case '1':
          machineRef.current.handle_key_down(0x04); // 1P Start
          event.preventDefault();
          break;
        case '2':
          machineRef.current.handle_key_down(0x02); // 2P Start
          event.preventDefault();
          break;
        case 'Tab':
          machineRef.current.handle_key_down(0x01); // Coin
          event.preventDefault();
          break;
      }
    }
  }, []);

  const handleKeyUp = useCallback((event: KeyboardEvent) => {
    if (machineRef.current) {
      switch (event.key) {
        case 'ArrowLeft':
          machineRef.current.handle_key_up(0x20); // Left
          break;
        case 'ArrowRight':
          machineRef.current.handle_key_up(0x40); // Right
          break;
        case ' ':
          machineRef.current.handle_key_up(0x10); // Fire
          break;
        case '1':
          machineRef.current.handle_key_up(0x04); // 1P Start
          break;
        case '2':
          machineRef.current.handle_key_up(0x02); // 2P Start
          break;
        case 'Tab':
          machineRef.current.handle_key_up(0x01); // Coin
          break;
      }
    }
  }, []);

  const handleTouchStart = useCallback((action: number, event: React.TouchEvent) => {
    event.preventDefault();
    if (machineRef.current) {
      machineRef.current.handle_key_down(action);
    }
  }, []);

  const handleTouchEnd = useCallback((action: number, event: React.TouchEvent) => {
    event.preventDefault();
    if (machineRef.current) {
      machineRef.current.handle_key_up(action);
    }
  }, []);

  useEffect(() => {
    init();
    const canvas = canvasRef.current;
    if (!canvas) return;

    if ("OffscreenCanvas" in window) {
      offscreenCanvasRef.current = new OffscreenCanvas(448, 512);
    }

    async function loadWasmAndStart() {
      const machine = new SpaceInvadersMachine();
      machineRef.current = machine;

      const romData = new Uint8Array(Rom);
      machine.load_rom(romData, 0);

      fpsCtrlRef.current = new FpsCtrl(60, renderFrame);
      fpsCtrlRef.current.start();

      // Add event listeners for keyboard input
      window.addEventListener('keydown', handleKeyDown);
      window.addEventListener('keyup', handleKeyUp);
    }

    loadWasmAndStart();
    function updateMainCanvas() {
      const ctx = canvas?.getContext("2d");
      const offscreenCanvas = offscreenCanvasRef.current;
      if (ctx && offscreenCanvas && canvas) {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        ctx.drawImage(offscreenCanvas, 0, 0);
      }

      requestAnimationFrame(updateMainCanvas);
    }
    updateMainCanvas();

    return () => {
      machineRef.current = null;
      if (fpsCtrlRef.current) {
        fpsCtrlRef.current.pause();
      }

      // Remove event listeners
      window.removeEventListener('keydown', handleKeyDown);
      window.removeEventListener('keyup', handleKeyUp);
    };
  }, [renderFrame, handleKeyDown, handleKeyUp]);

  return (
    <div className="emulator-container">
      <canvas id="gameCanvas" ref={canvasRef} width="448" height="512" />
      <div className="fps-counter">FPS: {fps}</div>
      <div className="mobile-controls">
        <button 
          onTouchStart={(e) => handleTouchStart(0x20, e)} 
          onTouchEnd={(e) => handleTouchEnd(0x20, e)}
        >
          Left
        </button>
        <button 
          onTouchStart={(e) => handleTouchStart(0x40, e)} 
          onTouchEnd={(e) => handleTouchEnd(0x40, e)}
        >
          Right
        </button>
        <button 
          onTouchStart={(e) => handleTouchStart(0x10, e)} 
          onTouchEnd={(e) => handleTouchEnd(0x10, e)}
        >
          Fire
        </button>
        <button 
          onTouchStart={(e) => handleTouchStart(0x04, e)} 
          onTouchEnd={(e) => handleTouchEnd(0x04, e)}
        >
          1P Start
        </button>
        <button 
          onTouchStart={(e) => handleTouchStart(0x02, e)} 
          onTouchEnd={(e) => handleTouchEnd(0x02, e)}
        >
          2P Start
        </button>
        <button 
          onTouchStart={(e) => handleTouchStart(0x01, e)} 
          onTouchEnd={(e) => handleTouchEnd(0x01, e)}
        >
          Coin
        </button>
      </div>
    </div>
  );
};

export default Emulator;
