import { listen } from "@tauri-apps/api/event";
import { useCallback, useEffect, useState } from "react";
import "./Overlay.css";

type Entity = {
  name: string;
  health: number;
  screen_pos: { x: number, y: number };
};


export default function Overlay() {  
  const [entityList, setEntityList] = useState<Entity[]>(null);
  const [playerCount, setPlayerCount] = useState<number>(0);

  useEffect(() => {

    listen('update_player_count', (event: any) => {
      setPlayerCount(event.payload);
    });

    listen<Entity[]>('update-entitylist', (event) => {
      setEntityList(event.payload);            
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

  
    //loop players and render health/name
    if(entityList) {
      entityList.forEach((entity, i) => {
        ctx.fillText(`${entity.name} - ${entity.health}`, 10, 70 + (i * 20));
        
        
        ctx.strokeStyle = 'red';
        ctx.lineWidth = 1;
        ctx.strokeRect(entity.screen_pos.x, entity.screen_pos.y, 50, 50);

        
      }
      );
    }


    globalThis.animationId = requestAnimationFrame(render);
  }, [playerCount, entityList]);

  useEffect(() => {        
    render();
    
     return () => {      
        if(globalThis.animationId)
          cancelAnimationFrame(globalThis.animationId);
     };
  }, [render, playerCount, entityList]);

  return (
    <canvas width={1920} height={1080} style={{ height: '100vh', width: '100vw', display: 'block' }} id="overlay" />
  );
}