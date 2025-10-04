import { CiCirclePlus } from "react-icons/ci";

export default function AddItem() {
  return (
    <div className="h-12 w-40 rounded-full">
      <button
        className="
          flex items-center justify-start p-2 h-full w-full 
          bg-[var(--color-primary)] text-white rounded-full
          transition-colors duration-150 
          hover:bg-[#ea580c]
          active:bg-[#c2410c]
        "
      >
        <CiCirclePlus size={30} className="m-1" />
        <span className="text-lg text-center font-semibold">Add Book</span>
      </button>
    </div>
  );
}
