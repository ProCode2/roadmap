// get the data emebedded in the html as json script
// need to parse twice cause the first pass is a JSON string (could be improved)
const data = JSON.parse(
  JSON.parse(document.querySelector("#json-map").textContent),
);
let numLeafNodes = 0;
const nodeSize = 350;
// max height of the tree
let maxHeight = 0;
// this is where nodes are added
const roadmaps = document.querySelector(".nodes");

// this function recursively creates a tree struture from nodes and edges arrays
function buildHierarchy(rootId, nodesMap) {
  const rootNode = nodesMap.get(rootId);
  const children = data.edges
    .filter((edge) => edge.source === rootId)
    .map((edge) => buildHierarchy(edge.target, nodesMap));

  if (children.length === 0) {
    numLeafNodes++;
  }

  return {
    id: rootNode.id,
    data: rootNode.data,
    children: children,
  };
}

// Function to get the midpoint of a DOM element by its id
function getMidPoint(id) {
  const element = document.getElementById(id);
  const rect = element.getBoundingClientRect();

  // Get the midpoint relative to the document, not the viewport
  const canvas = document.querySelector("#edges-canvas");
  // when user scrolls through the page, the cordinates changes the canvas scrolls are added to adjust for that
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

// draw a line from one point to another
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

// this function recursively creates the nodes at specific positions int the middle of ws and wr at h height
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

function enableZoom(mapSelector) {
  const map = document.querySelector(mapSelector);
  let scale = 1;
  const minScale = 0.5; // Minimum zoom level (e.g., 50%)
  const maxScale = 3; // Maximum zoom level (e.g., 300%)
  const zoomSpeed = 0.1; // Speed of zooming (adjust for sensitivity)

  map.addEventListener("wheel", (event) => {
    event.preventDefault();

    // Determine scroll direction (positive for zoom out, negative for zoom in)
    if (event.deltaY < 0) {
      // Zoom in
      scale = Math.min(scale + zoomSpeed, maxScale);
    } else {
      // Zoom out
      scale = Math.max(scale - zoomSpeed, minScale);
    }

    // Apply the scaling transformation
    map.style.transform = `scale(${scale})`;
    map.style.transformOrigin = "center"; // Make sure zoom is centered
  });
}

// this function sets up the entire map view
function setup() {
  // create a map of nodes from the json data for easy access
  const nodesMap = new Map(data.nodes.map((node) => [node.id, node]));

  const treeData = buildHierarchy("1", nodesMap);

  // total width of the map according to the number of leaf nodes
  const totalWidth = numLeafNodes * nodeSize + (numLeafNodes - 1) * 100;
  const mapWindowWidth = totalWidth;
  const mapWindowHeight = totalWidth;
  const canvas = document.getElementById("edges-canvas");
  const ctx = canvas.getContext("2d");
  ctx.fillStyle = "white";
  ctx.fillRect(0, 0, mapWindowWidth, mapWindowHeight);

  createNode(treeData, 0, mapWindowWidth, 100);
  let edgeCanvas = document.getElementById("edges-canvas");
  const nodesContainer = document.querySelector(".nodes");
  edgeCanvas.width = Math.max(mapWindowWidth, nodesContainer.offsetWidth);
  edgeCanvas.style.width = Math.max(mapWindowWidth, nodesContainer.offsetWidth);
  edgeCanvas.height = Math.max(maxHeight, nodesContainer.offsetHeight) + 100;
  edgeCanvas.style.height =
    Math.max(maxHeight, nodesContainer.offsetHeight) + 100 + "px";

  drawEdges(treeData);
  enableZoom(".nodes");
}

// do magic
setup();
