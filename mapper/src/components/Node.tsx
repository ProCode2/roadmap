import { ComponentProps, memo, useState } from "react";
import { Handle, Position, NodeProps, HandleProps } from "reactflow";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
	faCaretDown,
	faCaretUp,
	faTrash,
} from "@fortawesome/free-solid-svg-icons";

export interface NodeLink {
	url: string;
	description: string;
}

export interface NodeData {
	id: string;
	heading: string;
	links: NodeLink[];
	setDataChange: (data: NodeData) => void;
}

function NodeLinkItem({
	link,
	setLinkChange,
	deleteLink,
}: {
	link: NodeLink;
	setLinkChange: (link: NodeLink) => void;
	deleteLink: (link: NodeLink) => void;
}) {
	const [showDesc, setShowDesc] = useState(false);
	return (
		<div className="">
			<div className="flex space-x-3 justify-between items-center">
				<input
					type="text"
					placeholder="Material Url"
					className="p-2"
					value={link.url}
					onChange={(e) => setLinkChange({ ...link, url: e.target.value })}
				/>
				<button
					className="bg-slate-900 text-white rounded-md hover:shadow-md cursor-pointer p-2"
					onClick={() => deleteLink(link)}
				>
					<FontAwesomeIcon icon={faTrash} />
				</button>
				<button
					className="bg-slate-900 text-white rounded-md hover:shadow-md cursor-pointer p-2"
					onClick={() => setShowDesc((shd: boolean) => !shd)}
				>
					<FontAwesomeIcon icon={showDesc ? faCaretUp : faCaretDown} />
				</button>
			</div>
			{showDesc ? (
				<div className="mt-2">
					<textarea
						placeholder="Material Description"
						className="p-2"
						value={link.description}
						onChange={(e) =>
							setLinkChange({ ...link, description: e.target.value })
						}
					/>
				</div>
			) : null}
		</div>
	);
}

function NodeContent({ data }: { data: NodeData }) {
	function setLinkChange(link: NodeLink, idx: number) {
		const links = data.links;
		links[idx] = link;
		data.setDataChange({ ...data, links });
	}
	function deleteLink(link: NodeLink) {
		let links = data.links;
		links = links.filter((l) => l.url !== link.url);
		data.setDataChange({ ...data, links });
	}
	return (
		<div className="w-full h-full rounded-md shadow hover:shadow-lg border bg-white px-3 py-4 flex space-y-2 flex-col">
			<div className="">
				<input
					type="text"
					placeholder="Title of this step"
					className="p-2"
					value={data.heading}
					onChange={(e) =>
						data.setDataChange({ ...data, heading: e.target.value })
					}
				/>
			</div>
			<div>
				<button
					className="bg-slate-900 text-white rounded-md hover:shadow-md cursor-pointer p-2"
					onClick={() =>
						data.setDataChange({
							...data,
							links: [
								...data.links,
								{ url: "another", description: "another link" },
							],
						})
					}
				>
					Add material
				</button>
			</div>
			<div className="flex flex-col space-y-4">
				{data.links.map((l, idx) => (
					<NodeLinkItem
						link={l}
						setLinkChange={(link) => setLinkChange(link, idx)}
						deleteLink={deleteLink}
					/>
				))}
			</div>
		</div>
	);
}

function NodeHandle({
	className,
	ref,
	...rest
}: ComponentProps<"div"> & HandleProps) {
	return (
		<Handle
			className={`${className} w-5 h-5 bg-slate-800 rounded-md hover:shadow-md`}
			{...rest}
		/>
	);
}
export const NodeView = memo(
	({ id, data, isConnectable }: NodeProps<NodeData>) => {
		return (
			<div className="p-2">
				<NodeContent data={data} />
				<NodeHandle
					type="target"
					position={Position.Top}
					style={{ background: "#555" }}
					onConnect={(params) => console.log("handle onConnect", params)}
					isConnectable={isConnectable}
				/>
				<NodeHandle
					type="source"
					position={Position.Bottom}
					id="b"
					style={{ bottom: 10, top: "auto", background: "#555" }}
					isConnectable={isConnectable}
				/>
			</div>
		);
	},
);
