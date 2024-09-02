const data = JSON.parse(
	JSON.parse(document.querySelector("#json-map").textContent),
);
console.log(data);

const container = d3.select("#map-container");
const svg = d3.select("#edges-container");

// Set up the force simulation with initial settings
const simulation = d3
	.forceSimulation(data.nodes)
	.force(
		"link",
		d3
			.forceLink(data.edges)
			.id((d) => d.id)
			.distance(200),
	)
	.force("charge", d3.forceManyBody().strength(-500))
	.force(
		"center",
		d3.forceCenter(
			container.node().clientWidth / 2,
			container.node().clientHeight / 2,
		),
	)
	.force(
		"y",
		d3
			.forceY((d) => {
				const levels = { 1: 0, 2: 1, 3: 2 }; // Assign levels manually
				return levels[d.id] * 300;
			})
			.strength(1),
	) // Ensures vertical hierarchy
	.force("collision", d3.forceCollide().radius(150))
	.on("end", finalizePositions);

// Create HTML nodes
const nodes = container
	.selectAll(".node")
	.data(data.nodes)
	.enter()
	.append("div")
	.attr("class", "node");

nodes.append("h3").text((d) => d.data.heading);

const linkLists = nodes.append("ul");

const linkItems = linkLists
	.selectAll("li")
	.data((d) => d.data.links)
	.enter()
	.append("li");

linkItems
	.append("a")
	.attr("href", (d) => d.url)
	.attr("target", "_blank")
	.text((d) => d.url);

linkItems.append("p").text((d) => d.description);

// Create edges
const edges = svg.selectAll("line").data(data.edges).enter().append("line");

// Finalize node positions
function finalizePositions() {
	nodes
		.style("left", (d) => `${d.x - 125}px`)
		.style("top", (d) => `${d.y - 50}px`);

	edges
		.attr("x1", (d) => d.source.x)
		.attr("y1", (d) => d.source.y)
		.attr("x2", (d) => d.target.x)
		.attr("y2", (d) => d.target.y);

	// Stop the simulation to make the layout static
	simulation.stop();
}
