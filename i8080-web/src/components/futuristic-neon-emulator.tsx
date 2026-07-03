import { useEffect, useState } from "react";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Github } from "lucide-react";
import { Button } from "@/components/ui/button";
import EmulatorComponent from "./emulator";

const CONTROLS = [
  { key: "Tab", action: "Insert Coin" },
  { key: "1", action: "1 Player Start" },
  { key: "2", action: "2 Player Start" },
  { key: "← →", action: "Move Left / Right" },
  { key: "Space", action: "Fire" },
];

function ControlsModal({ onClose }: { onClose: () => void }) {
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm">
      <div className="w-full max-w-sm mx-4 border-2 rounded-xl border-cyan-400 bg-gray-900 shadow-lg shadow-cyan-400/40 p-8">
        <h2 className="mb-1 text-2xl font-bold text-center text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-purple-600">
          Controls
        </h2>
        <p className="mb-6 text-sm text-center text-gray-400">Press <span className="text-cyan-400">Tab</span> to insert a coin, then <span className="text-cyan-400">1</span> to start.</p>
        <table className="w-full mb-8 text-sm">
          <tbody>
            {CONTROLS.map(({ key, action }) => (
              <tr key={key} className="border-b border-cyan-400/20 last:border-0">
                <td className="py-2 pr-4">
                  <kbd className="px-2 py-1 font-mono text-xs rounded border border-cyan-400/60 bg-gray-800 text-cyan-300">
                    {key}
                  </kbd>
                </td>
                <td className="py-2 text-gray-300">{action}</td>
              </tr>
            ))}
          </tbody>
        </table>
        <Button
          onClick={onClose}
          className="w-full bg-cyan-400 text-gray-900 hover:bg-cyan-300 font-semibold"
        >
          Play
        </Button>
      </div>
    </div>
  );
}

type Instruction = {
  address: string;
  opcode: string;
  mnemonic: string;
};

export function FuturisticNeonEmulatorComponent({
  canvasRef,
  cpuState,
  instructions: wasmInstructions,
}: {
  canvasRef: React.RefObject<HTMLCanvasElement>;
  cpuState: any;
  instructions: any;
}) {
  const [showControls, setShowControls] = useState(true);
  const [instructions, setInstructions] = useState<Instruction[]>([]);
  useEffect(() => {
    if (wasmInstructions && Array.isArray(wasmInstructions)) {
      const convertedInstructions = wasmInstructions.map((inst: any) => ({
        address: inst.address.toString(16).padStart(4, "0"),
        opcode: inst.opcode.toString(16).padStart(2, "0"),
        mnemonic: inst.mnemonic,
      }));
      setInstructions(convertedInstructions.reverse()); // Show newest first
    }
  }, [wasmInstructions]);

  return (
    <div className="container min-w-full min-h-screen px-4 py-8 mx-auto font-sans bg-gray-900 text-cyan-300">
      {showControls && <ControlsModal onClose={() => setShowControls(false)} />}
      <div className="flex flex-col items-center justify-between mb-8 sm:flex-row">
        <h1 className="mb-4 text-3xl font-bold text-center text-transparent sm:text-4xl md:text-5xl sm:mb-0 sm:text-left bg-clip-text bg-gradient-to-r from-cyan-400 to-purple-600">
          Space Invaders Emulator
        </h1>
        <a
          href="https://github.com/mitul72/8080-emulator-rust"
          target="_blank"
          rel="noopener noreferrer"
        >
          <Button
            variant="outline"
            className="transition-colors duration-300 bg-gray-900/50 border-cyan-400 text-cyan-400 hover:bg-cyan-400 hover:text-gray-900"
          >
            <Github className="w-4 h-4 mr-2" />
            GitHub
          </Button>
        </a>
      </div>
      <div className="flex flex-col gap-8 lg:flex-row">
        <Card className="w-full bg-gray-800 border-2 shadow-lg lg:w-1/2 border-cyan-400 rounded-xl shadow-cyan-400/50">
          <CardHeader>
            <CardTitle className="text-xl text-cyan-400 sm:text-2xl">
              Holographic Display
            </CardTitle>
          </CardHeader>
          <CardContent>
            <EmulatorComponent canvasRef={canvasRef} />
          </CardContent>
        </Card>
        <div className="w-full space-y-8 lg:w-1/2">
          <Card className="bg-gray-800 border-2 border-purple-400 shadow-lg rounded-xl shadow-purple-400/50">
            <CardHeader>
              <CardTitle className="text-xl text-purple-400 sm:text-2xl">
                Instruction Log
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="overflow-auto max-h-60 scrollbar-thin scrollbar-thumb-purple-400 scrollbar-track-gray-800">
                <Table>
                  <TableHeader>
                    <TableRow className="bg-gray-700">
                      <TableHead className="text-purple-400">Address</TableHead>
                      <TableHead className="text-purple-400">Opcode</TableHead>
                      <TableHead className="text-purple-400">
                        Mnemonic
                      </TableHead>
                    </TableRow>
                  </TableHeader>
                  <TableBody>
                    {instructions.map((instr, index) => (
                      <TableRow
                        key={index}
                        className="border-b border-purple-400/30"
                      >
                        <TableCell className="text-cyan-400">
                          {instr.address}
                        </TableCell>
                        <TableCell className="text-cyan-400">
                          {instr.opcode}
                        </TableCell>
                        <TableCell className="text-cyan-400">
                          {instr.mnemonic}
                        </TableCell>
                      </TableRow>
                    ))}
                  </TableBody>
                </Table>
              </div>
            </CardContent>
          </Card>
          <Card className="bg-gray-800 border-2 shadow-lg border-cyan-400 rounded-xl shadow-cyan-400/50">
            <CardHeader>
              <CardTitle className="text-xl text-cyan-400 sm:text-2xl">
                CPU Status
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
                <div>
                  <h3 className="mb-2 text-lg font-semibold text-purple-400">
                    Registers
                  </h3>
                  <div className="grid grid-cols-2 gap-2 text-cyan-400">
                    <div>
                      A: 0x
                      {cpuState?.get("a")?.toString(16).padStart(2, "0") ||
                        "00"}
                    </div>
                    <div>
                      B: 0x
                      {cpuState?.get("b")?.toString(16).padStart(2, "0") ||
                        "00"}
                    </div>
                    <div>
                      C: 0x
                      {cpuState?.get("c")?.toString(16).padStart(2, "0") ||
                        "00"}
                    </div>
                    <div>
                      D: 0x
                      {cpuState?.get("d")?.toString(16).padStart(2, "0") ||
                        "00"}
                    </div>
                    <div>
                      E: 0x
                      {cpuState?.get("e")?.toString(16).padStart(2, "0") ||
                        "00"}
                    </div>
                    <div>
                      H: 0x
                      {cpuState?.get("h")?.toString(16).padStart(2, "0") ||
                        "00"}
                    </div>
                    <div>
                      L: 0x
                      {cpuState?.get("l")?.toString(16).padStart(2, "0") ||
                        "00"}
                    </div>
                  </div>
                  <div className="mt-2 text-cyan-400">
                    <div>
                      SP: 0x
                      {cpuState?.get("sp")?.toString(16).padStart(4, "0") ||
                        "0000"}
                    </div>
                  </div>
                </div>
                <div>
                  <h3 className="mb-2 text-lg font-semibold text-purple-400">
                    Flags
                  </h3>
                  <div className="grid grid-cols-1 gap-2 text-cyan-400">
                    <div>Zero: {cpuState?.get("flags") & 0x40 ? "1" : "0"}</div>
                    <div>Sign: {cpuState?.get("flags") & 0x80 ? "1" : "0"}</div>
                    <div>
                      Parity: {cpuState?.get("flags") & 0x04 ? "1" : "0"}
                    </div>
                    <div>
                      Carry: {cpuState?.get("flags") & 0x01 ? "1" : "0"}
                    </div>
                    <div>
                      Auxiliary Carry:{" "}
                      {cpuState?.get("flags") & 0x10 ? "1" : "0"}
                    </div>
                  </div>
                </div>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  );
}
