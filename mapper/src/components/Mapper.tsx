import { useCallback, useEffect, useRef } from "react";
import {
  ReactFlow,
  Controls,
  Background,
  useNodesState,
  useEdgesState,
  BackgroundVariant,
  useReactFlow,
  OnConnectStart,
  OnConnectStartParams,
  Edge,
  addEdge,
  Connection,
  Node,
} from "reactflow";
import "reactflow/dist/style.css";
import MapEdge from "./MapEdge";
import { NodeView, NodeData } from "./Node";
import { SubmitMap } from "./SubmitMap";

const initialEdges = [
  { id: "e1-2", source: "1", target: "2", type: "map-edge" },
];

const edgeTypes = {
  "map-edge": MapEdge,
};

const nodeTypes = {
  item: NodeView,
};
let id = 2;
const getId = () => `${id++}`;
export default function App() {
  const connectingNodeId = useRef<string | null>(null);
  const connectionType = useRef<string | null>(null);
  const [nodes, setNodes, onNodesChange] = useNodesState<NodeData>([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);
  const { screenToFlowPosition } = useReactFlow();

  const setDataChange = useCallback(
    (newData: NodeData) => {
      setNodes((nodes: Node<NodeData>[]) => {
        return nodes.map((nd) => {
          if (nd.id === newData.id) {
            return {
              ...nd,
              data: newData,
            };
          }
          return nd;
        });
      });
    },
    [setNodes],
  );

  useEffect(() => {
    const initialNodes: Node<NodeData>[] = [
      {
        id: "1",
        type: "item",
        position: { x: 0, y: 0 },
        data: {
          id: "1",
          heading: "Enter a new heading",
          links: [
            {
              url: "github.com",
              description: "Use this mateiral to learn open source",
            },
          ],
          setDataChange,
        },
      },
    ];
    setNodes(initialNodes);
  }, [setNodes, setDataChange]);
  const onConnect = useCallback(
    (params: Connection) => {
      // reset the start node on connections
      connectingNodeId.current = null;
      setEdges((eds) => addEdge({ ...params, type: "map-edge" }, eds));
    },
    [setEdges],
  );
  const onConnectStart: OnConnectStart = useCallback(
    (_, { nodeId, handleType }: OnConnectStartParams) => {
      connectingNodeId.current = nodeId;
      connectionType.current = handleType;
    },
    [],
  );
  const onConnectEnd = useCallback(
    (event: MouseEvent | TouchEvent) => {
      if (!connectingNodeId.current) return;
      console.log(connectingNodeId.current);

      const targetIsPane = (event.target as Element).classList.contains(
        "react-flow__pane",
      );

      if (targetIsPane) {
        // we need to remove the wrapper bounds, in order to get the correct position
        const id = getId();
        let clientX = 0,
          clientY = 0;
        if (window.TouchEvent && event instanceof TouchEvent) {
          clientX = event.touches[0].clientX;
          clientY = event.touches[0].clientY;
        } else if (window.MouseEvent && event instanceof MouseEvent) {
          clientX = event.clientX;
          clientY = event.clientY;
        }

        const newNode = {
          id,
          type: "item",
          position: screenToFlowPosition({
            x: clientX,
            y: clientY,
          }),
          data: {
            id,
            heading: "Enter a new heading",
            links: [],
            setDataChange: setDataChange,
          },
          origin: [0.5, 0.0],
        };

        setNodes((nds) => nds.concat(newNode));
        setEdges((eds: Edge[]) =>
          eds.concat([
            {
              id,
              type: "map-edge",
              source:
                connectionType.current === "source"
                  ? connectingNodeId.current || ""
                  : id,
              target:
                connectionType.current === "target"
                  ? connectingNodeId.current || ""
                  : id,
            },
          ]),
        );
      }
    },
    [screenToFlowPosition, setNodes, setEdges, setDataChange],
  );

  return (
    <>
      <SubmitMap />
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onConnect={onConnect}
        onConnectStart={onConnectStart}
        onConnectEnd={onConnectEnd}
        edgeTypes={edgeTypes}
        nodeTypes={nodeTypes}
        fitView
        fitViewOptions={{ padding: 2 }}
        nodeOrigin={[0.5, 0]}
      >
        <Controls />
        <Background variant={BackgroundVariant.Dots} gap={12} size={1} />
      </ReactFlow>
    </>
  );
}
