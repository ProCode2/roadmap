export function SubmitMap({
  setOpen,
}: {
  open: boolean;
  setOpen: (state: boolean) => void;
}) {
  function handleSubmitMap() {
    setOpen(true);
  }
  return (
    <button
      onClick={handleSubmitMap}
      className="absolute z-30 right-5 top-5 bg-slate-900 text-white rounded-md hover:shadow-md cursor-pointer p-2"
    >
      Add Meta Data
    </button>
  );
}
