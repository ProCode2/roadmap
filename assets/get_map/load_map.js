const data = JSON.parse(
	JSON.parse(document.querySelector("#json-map").textContent),
);

// Convert nodes and edges into a hierarchical structure
const nodesMap = new Map(data.nodes.map((node) => [node.id, node]));
const root = nodesMap.get("1"); // Assuming '1' is the root node

function buildHierarchy(rootId) {
	const rootNode = nodesMap.get(rootId);
	const children = data.edges
		.filter((edge) => edge.source === rootId)
		.map((edge) => buildHierarchy(edge.target));

	return {
		id: rootNode.id,
		data: rootNode.data,
		children: children,
	};
}

const hierarchyData = buildHierarchy(root.id);

// Set up D3 tree layout for vertical tree
const margin = { top: 20, right: 120, bottom: 20, left: 120 };
const width = 960 - margin.right - margin.left;
const height = 800 - margin.top - margin.bottom; // Increased height

const svg = d3
	.select("#roadmap")
	.append("svg")
	.attr("width", width + margin.right + margin.left)
	.attr("height", height + margin.top + margin.bottom)
	.style("position", "absolute")
	.style("top", "0")
	.style("left", "0")
	.append("g")
	.attr("transform", `translate(${margin.left},${margin.top})`);

const treeLayout = d3.tree().size([height, width]);

const rootNode = d3.hierarchy(hierarchyData);
treeLayout(rootNode);

// Render edges (links)
svg
	.selectAll(".edge")
	.data(rootNode.links())
	.enter()
	.append("path")
	.attr("class", "edge")
	.attr("d", (d) => {
		const sourceX = d.source.x;
		const sourceY = d.source.y;
		const targetX = d.target.x;
		const targetY = d.target.y;
		return `M${sourceX},${sourceY}L${targetX},${targetY}`;
	});

// Render nodes as HTML elements
const roadmap = document.getElementById("roadmap");

rootNode.descendants().forEach((d) => {
	const nodeElement = document.createElement("div");
	nodeElement.classList.add("node");
	nodeElement.style.left = d.x + margin.left + "px"; // Position node horizontally
	nodeElement.style.top = d.y + margin.top + "px"; // Position node vertically
	nodeElement.innerHTML = `<strong>${d.data.data.heading}</strong>`;
	d.data.data.links.forEach((link) => {
		nodeElement.innerHTML += `<a href="${link.url}" class="link">${link.description}</a>`;
	});
	roadmap.appendChild(nodeElement);
});
