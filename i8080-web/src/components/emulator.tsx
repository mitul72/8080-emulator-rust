import React from 'react'

const EmulatorComponent: React.FC<{ canvasRef: React.RefObject<HTMLCanvasElement> }> = ({ canvasRef }) => {

    return (
        <div className="w-full aspect-square">
            <canvas
                id="gameCanvas"
                ref={canvasRef}
                width={480}
                height={480}
                className="w-full h-full border-2 border-cyan-400 rounded-lg shadow-md shadow-cyan-400/50"
                aria-label="Space Invaders game canvas"
            />
        </div>
    )
}
export default EmulatorComponent