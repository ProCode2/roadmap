import { useEdges, useNodes } from "reactflow";

export function SubmitMap() {
  const nodes = useNodes();
  const edges = useEdges();

  function handleSubmitMap() {
    console.log(nodes, edges);
  }
  return (
    <button
      onClick={handleSubmitMap}
      className="absolute z-30 right-5 top-5 bg-slate-900 text-white rounded-md hover:shadow-md cursor-pointer p-2"
    >
      Submit
    </button>
  );
}
