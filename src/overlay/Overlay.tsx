import { listen } from "@tauri-apps/api/event";
import { cursorPosition } from "@tauri-apps/api/window";
import { useCallback, useEffect, useState } from "react";
export default function Overlay() {
  const [roundSize, setRoundSize] = useState<number | null>(globalThis.roundSize ?? 2);
  const [playerData, setPlayerData] = useState<any>(null);
  const [playerCount, setPlayerCount] = useState<number>(0);

  useEffect(() => {
    listen('change-roundsize', (event: any) => {
      setRoundSize(event.payload);
    });

    listen('update_player_count', (event: any) => {
      setPlayerCount(event.payload);
    });


  }, []);

  const render = useCallback( () => {    
    const canvas = document.getElementById("overlay") as HTMLCanvasElement;
    const ctx = canvas.getContext("2d") as CanvasRenderingContext2D;
  
    ctx.clearRect(0, 0, canvas.width, canvas.height);    
    
    // ctx.beginPath();
    // ctx.strokeStyle = 'red';
    // ctx.lineWidth = 1;
    // ctx.moveTo(roundSize, 50);
    // ctx.lineTo(300, 300);
    // ctx.moveTo(300, 50);
    // ctx.lineTo(roundSize, 300);
    // ctx.stroke();

    
    ctx.font = "12px Arial antialiased";
    ctx.fillStyle = "red";
    ctx.fillText(`Player Count: ${playerCount}`, 10, 50);

    globalThis.animationId = requestAnimationFrame(render);
  }, [roundSize, playerCount]);

  useEffect(() => {        
    render();
    
     return () => {      
        if(globalThis.animationId)
          cancelAnimationFrame(globalThis.animationId);
     };
  }, [render, roundSize, playerCount]);

  return (
    <canvas width={1920} height={1080} style={{ height: '100vh', width: '100vw', display: 'block' }} id="overlay" />
  );
}