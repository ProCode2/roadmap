import { faAdd, faTimes } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { useState } from "react";

export default function ListInput({
  title,
  listInput,
  setListInput,
}: {
  title: string;
  listInput: Set<string>;
  setListInput: (prev: Set<string>) => void;
}) {
  const [curText, setCurText] = useState("");
  return (
    <label htmlFor="keywords" className="my-2 w-full">
      <p className="font-bold mb-1">{title}</p>
      <div className="flex justify-start items-start">
        {Array.from(listInput).map((keyword) => (
          <p
            key={keyword}
            className="bg-gray-600 text-white rounded-md text-xs shadow-md p-2 m-2 flex justify-center items-center max-w-max"
          >
            <span className="h-full">{keyword}</span>
            <button
              onClick={() => {
                listInput.delete(keyword);
                setListInput(listInput);
              }}
              className="ml-2 cursor-pointer hover:text-slate-300"
            >
              <FontAwesomeIcon icon={faTimes} className="" />
            </button>
          </p>
        ))}
      </div>
      <p className="border border-slate-700 p-2 rounded-md w-full flex items-center justify-center">
        <input
          name="description"
          placeholder="Enter map description"
          type="text"
          value={curText}
          onChange={(e) => setCurText(e.target.value)}
          className="w-full p-2 rounded-md focus:outline-none focus:ring-none mr-2"
        />

        <button
          onClick={() => {
            listInput.add(curText);
            setListInput(listInput);
            setCurText("");
          }}
          className="bg-slate-900 text-white rounded-md hover:shadow-md cursor-pointer p-2"
        >
          <FontAwesomeIcon icon={faAdd} />
        </button>
      </p>
    </label>
  );
}
