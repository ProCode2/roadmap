import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import Mapper from "./components/Mapper.tsx";
import "./index.css";
import { ReactFlowProvider } from "reactflow";

const rootElem = document.getElementById("root");

const editMode = rootElem?.getAttribute("data-edit");

createRoot(rootElem!).render(
  <StrictMode>
    <ReactFlowProvider>
      <Mapper editMode={editMode ? true : false} />
    </ReactFlowProvider>
  </StrictMode>,
);
