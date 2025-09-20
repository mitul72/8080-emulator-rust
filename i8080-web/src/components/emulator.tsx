import React from 'react'

const EmulatorComponent: React.FC<{ canvasRef: React.RefObject<HTMLCanvasElement> }> = ({ canvasRef }) => {

    return (
        <div className="w-full max-w-md mx-auto">
            <canvas
                id="gameCanvas"
                ref={canvasRef}
                width={448}
                height={512}
                className="w-full h-auto border-2 border-cyan-400 rounded-lg shadow-md shadow-cyan-400/50"
                style={{ aspectRatio: '448/512' }}
                aria-label="Space Invaders game canvas"
            />
        </div>
    )
}
export default EmulatorComponent