const data = JSON.parse(
  JSON.parse(document.querySelector("#json-map").textContent),
);

const nodesMap = new Map(data.nodes.map((node) => [node.id, node]));
const root = nodesMap.get("1"); // Assuming '1' is the root node

let numLeafNodes = 0;
function buildHierarchy(rootId) {
  const rootNode = nodesMap.get(rootId);
  const children = data.edges
    .filter((edge) => edge.source === rootId)
    .map((edge) => buildHierarchy(edge.target));

  if (children.length === 0) {
    numLeafNodes++;
  }

  return {
    id: rootNode.id,
    data: rootNode.data,
    children: children,
  };
}

const treeData = buildHierarchy("1");
const nodeSize = 350;

const totalWidth = numLeafNodes * nodeSize + (numLeafNodes - 1) * 100;
const mapWindowWidth = totalWidth;
const mapWindowHeight = totalWidth;
const canvas = document.getElementById("edges-canvas");
const ctx = canvas.getContext("2d");
ctx.fillStyle = "white";
ctx.fillRect(0, 0, mapWindowWidth, mapWindowHeight);

// Function to get the midpoint of a DOM element by its id
function getMidPoint(id) {
  const element = document.getElementById(id);
  const rect = element.getBoundingClientRect();

  // Get the midpoint relative to the document, not the viewport
  const roadmaps = document.querySelector(".nodes");
  const canvas = document.querySelector("#edges-canvas");
  console.log(
    "canvas",
    canvas.getBoundingClientRect().top,
    canvas.getBoundingClientRect().left,
  );
  console.log(
    "roadmaps",
    roadmaps.getBoundingClientRect().top,
    roadmaps.getBoundingClientRect().left,
  );
  const midX =
    rect.left +
    rect.width / 2 +
    canvas.scrollLeft -
    canvas.getBoundingClientRect().left;
  const midY =
    rect.top +
    rect.height / 2 +
    canvas.scrollTop -
    canvas.getBoundingClientRect().top;
  return { x: midX, y: midY };
}

// Recursive function to draw edges from parent to children
function drawEdges(tree) {
  if (!tree || !tree.children || tree.children.length === 0) return;

  // Get the midpoint of the current parent node
  const parentMidPoint = getMidPoint(tree.id);

  // Iterate over children and draw edges
  tree.children.forEach((child) => {
    const childMidPoint = getMidPoint(child.id);
    console.log(parentMidPoint, childMidPoint);
    // Draw a line from the parent node to the child node
    drawLine(
      parentMidPoint.x,
      parentMidPoint.y,
      childMidPoint.x,
      childMidPoint.y,
    );

    // Recursively call drawEdges for the child node
    drawEdges(child);
  });
}

function drawLine(x1, y1, x2, y2) {
  const canvas = document.getElementById("edges-canvas");
  const ctx = canvas.getContext("2d");

  // Begin a new path
  ctx.beginPath();

  // Move to the starting point
  ctx.moveTo(x1, y1);

  // Draw a line to the ending point
  ctx.lineTo(x2, y2);

  // Set line width and color (optional)
  ctx.lineWidth = 2;
  ctx.strokeStyle = "black";

  // Draw the line
  ctx.stroke();
}

const roadmaps = document.querySelector(".nodes");
let maxHeight = 0;
function createNode(root, ws, we, h) {
  const nodeElement = document.createElement("div");
  nodeElement.classList.add("node");
  nodeElement.style.left = `${ws + (we - ws) / 2}px`;
  nodeElement.style.top = `${h}px`;
  nodeElement.style.maxWidth = `${nodeSize}px`;
  nodeElement.setAttribute("id", root.id);

  nodeElement.innerHTML = `<h2>${root.data.heading}</h2>`;
  root.data.links.forEach((link) => {
    nodeElement.innerHTML += `<div class="node-links"><a href="${link.url}" class="link">${link.url}</a><p>${link.description}</p></div>`;
  });
  roadmaps.appendChild(nodeElement);
  const subWidth = (we - ws) / root.children.length;
  root.children.forEach((n, idx) => {
    const elemHeight = document.getElementById(root.id).offsetHeight;
    const start = ws + subWidth * idx;
    const end = start + subWidth;
    maxHeight = Math.max(maxHeight, h + elemHeight + 100);
    createNode(n, start, end, h + elemHeight + 100);
  });
}

createNode(treeData, 0, mapWindowWidth, 100);
let edgeCanvas = document.getElementById("edges-canvas");
const nodesContainer = document.querySelector(".nodes");
edgeCanvas.width = Math.max(mapWindowWidth, nodesContainer.offsetWidth);
edgeCanvas.style.width = Math.max(mapWindowWidth, nodesContainer.offsetWidth);
edgeCanvas.height = Math.max(maxHeight, nodesContainer.offsetHeight) + 100;
edgeCanvas.style.height =
  Math.max(maxHeight, nodesContainer.offsetHeight) + 100 + "px";

drawEdges(treeData);
