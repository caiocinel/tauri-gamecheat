import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./Menu.css";


function Menu() {    
  const [roundSize, setRoundSize] = useState<number | null>(globalThis.roundSize ?? 2);

  useEffect(() => {   
    invoke("update_round_size", { roundSize })
  }, [roundSize]);
  
  return (
    <main className="container" style={{ width: '480px', marginLeft: 'auto', marginRight: 'auto'}}>      
      <button onClick={async () => await invoke("render")}>Render</button>
      <button onClick={async () => await invoke("start")}>Start</button>
      <input type="number" value={roundSize} onChange={(e) => setRoundSize(parseInt(e.target.value))} />
    </main>
  );
}

export default Menu;
