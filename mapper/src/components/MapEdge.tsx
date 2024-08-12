import {
  BaseEdge,
  EdgeLabelRenderer,
  getBezierPath,
  useReactFlow,
} from 'reactflow';

interface EdgeProps {
  id: string;
  sourceX: number;
  sourceY: number;
  targetX: number;
  targetY: number;
}

export default function MapEdge({ id, sourceX, sourceY, targetX, targetY }: EdgeProps) {
  const { setEdges } = useReactFlow();
  const [edgePath, labelX, labelY] = getBezierPath({
    sourceX,
    sourceY,
    targetX,
    targetY,
  });

  return (
    <>
      <BaseEdge id={id} path={edgePath} />
      <EdgeLabelRenderer>
        <button
          style={{
            position: 'absolute',
            transform: `translate(-50%, -50%) translate(${labelX}px,${labelY}px)`,
            pointerEvents: 'all',
          }}
          className="nodrag nopan w-12 h-12 rounded-full bg-slate-900 text-white font-mono hover:shadow-lg flex justify-center items-center"
          onClick={() => {
            setEdges((es) => es.filter((e) => e.id !== id));
          }}
        >
          X
        </button>
      </EdgeLabelRenderer>
    </>
  );
}
