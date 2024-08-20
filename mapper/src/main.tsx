import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import Mapper from "./components/Mapper.tsx";
import "./index.css";
import { ReactFlowProvider } from "reactflow";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <ReactFlowProvider>
      <Mapper />
    </ReactFlowProvider>
  </StrictMode>,
);
