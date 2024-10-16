import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {

  if(window.location.pathname === "/overlay"){    

    useEffect(() => {
      const canvas = document.getElementById("overlay") as HTMLCanvasElement;

      canvas.width = 3440;
      canvas.height = 1440;

      const ctx = canvas.getContext("2d") as CanvasRenderingContext2D;

      const render = async () => {
        const { width, height } = canvas;
        ctx.clearRect(0, 0, width, height);
        ctx.fillStyle = "#00ffff33";
        ctx.fillRect(0, 0, width, height);
      };

      render();
    }, [window.location.pathname])


    return (      
      <canvas id="overlay"/>
    );
  }




  return (
    <main className="container">
      <button type="submit" onClick={async () => await invoke("render")}>Render</button>                  
    </main>
  );
}

export default App;
