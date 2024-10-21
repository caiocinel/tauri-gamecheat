import React from "react";
import ReactDOM from "react-dom/client";
import Menu from "./menu/Menu";
import Overlay from "./overlay/Overlay";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    {window.location.pathname === "/overlay" ? <Overlay /> : <Menu />}
  </React.StrictMode>,
);
