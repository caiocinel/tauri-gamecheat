import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { cursorPosition } from "@tauri-apps/api/window";

function App() {  
  if(window.location.pathname === "/overlay"){       
    const render = useCallback(async () => {
      const canvas = document.getElementById("overlay") as HTMLCanvasElement;
      const ctx = canvas.getContext("2d") as CanvasRenderingContext2D;
      
      const { x, y } = await cursorPosition();      

      const rect = canvas.getBoundingClientRect();
      const scaleX = canvas.width / rect.width;
      const scaleY = canvas.height / rect.height;

      const mouseX = (x - rect.left) * scaleX;
      const mouseY = (y - rect.top) * scaleY;


      ctx.clearRect(0, 0, canvas.width, canvas.height);           

      ctx.beginPath();
      ctx.arc(mouseX, mouseY, 2, 0, 2 * Math.PI, false);      
      ctx.fillStyle = '#ffffff55';
      ctx.fill();      

      requestAnimationFrame(render);
    }, []);

    useEffect(() => {
      render();      
    }, [render]);


    return (      
      <canvas style={{ height: '100vh', width: '100vw', display: 'block' }} id="overlay"/>
    );
  }




  return (
    <main className="container">
      <button type="submit" onClick={async () => await invoke("render")}>Render</button>                  
    </main>
  );
}

export default App;
