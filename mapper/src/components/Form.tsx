import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faTimes } from "@fortawesome/free-solid-svg-icons";
import { useState } from "react";
import ListInput from "./ListInput";
import { useEdges, useNodes } from "reactflow";

interface IFormData {
	title: string;
	description: string;
	keywords: Set<string>;
	sources: Set<string>;
	tags: Set<string>;
	content: string;
}

export default function Form({
	setOpen,
}: {
	open: boolean;
	setOpen: (state: boolean) => void;
}) {
	const [formState, setForm] = useState<IFormData>({
		title: "",
		description: "",
		keywords: new Set<string>(),
		sources: new Set<string>(),
		tags: new Set<string>(),
		content: "",
	});
	const nodes = useNodes();
	const edges = useEdges();
	async function handleSubmit() {
		console.log({
			...formState,
			keywords: Array.from(formState.keywords),
			sources: Array.from(formState.sources),
			tags: Array.from(formState.tags),
			content: JSON.stringify({ nodes, edges }),
		});
		const res = await fetch("/roadmaps", {
			method: "POST",
			mode: "cors",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({
				...formState,
				keywords: Array.from(formState.keywords),
				sources: Array.from(formState.sources),
				tags: Array.from(formState.tags),
				content: JSON.stringify({ nodes, edges }),
			}),
		});
		if (res.status !== 200) {
			alert("Something went wrong, please try again.");
		}
		const mapData = await res.json();
		console.log(mapData);
		window.location.href = "/roadmaps/" + mapData.id;
	}

	return (
		<div className="z-40 w-full h-full bg-black/50 absolute left-0 right-0">
			<div className="z-40 right-0 left-0 w-full h-screen overflow-auto md:w-[500px] bg-white pb-4 px-2">
				<div className="w-full p-2 flex justify-end">
					<button
						className="bg-slate-900 text-white rounded-md hover:shadow-md cursor-pointer p-2"
						onClick={() => setOpen(false)}
					>
						<FontAwesomeIcon icon={faTimes} />
					</button>
				</div>
				<h2 className="text-2xl mb-3 p-2 text-center font-bold">
					Boost Your Search Rankings with Key Metadata!
				</h2>
				<div className="w-full h-full p-2 flex flex-col items-start justify-start font-poppins">
					<label htmlFor="title" className="my-2 w-full">
						<p className="font-bold mb-1">Title</p>
						<input
							name="title"
							placeholder="Enter map title"
							type="text"
							value={formState.title}
							onChange={(e) =>
								setForm((prev) => ({ ...prev, title: e.target.value }))
							}
							className="border border-slate-700 p-2 rounded-md w-full"
						/>
					</label>
					<label htmlFor="description" className="my-2 w-full">
						<p className="font-bold mb-1">Description</p>
						<input
							name="description"
							placeholder="Enter map description"
							type="text"
							value={formState.description}
							onChange={(e) =>
								setForm((prev) => ({ ...prev, description: e.target.value }))
							}
							className="border border-slate-700 p-2 rounded-md w-full"
						/>
					</label>
					<ListInput
						title="Keywords"
						listInput={formState.keywords}
						setListInput={(prev: Set<string>) => {
							setForm((p) => ({ ...p, keywords: prev }));
						}}
					/>
					<ListInput
						title="Tags"
						listInput={formState.tags}
						setListInput={(prev: Set<string>) => {
							setForm((p) => ({ ...p, tags: prev }));
						}}
					/>
					<ListInput
						title="Sources"
						listInput={formState.sources}
						setListInput={(prev: Set<string>) => {
							setForm((p) => ({ ...p, sources: prev }));
						}}
					/>
					<button
						onClick={handleSubmit}
						className="bg-slate-900 text-white rounded-md hover:shadow-md cursor-pointer w-full mt-4 py-2"
					>
						Submit
					</button>
				</div>
			</div>
		</div>
	);
}
