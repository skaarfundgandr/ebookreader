import React from "react";
import { CiCirclePlus } from "react-icons/ci";
import { useNavigate } from "react-router-dom";

export default function AddItem() {
  const navigate = useNavigate();

  const handleAddClick = () => {
    navigate("/library"); // 👈 redirect to Library path
  };

  return (
    <div className="h-12 w-40 rounded-full">
      <button
        onClick={handleAddClick}
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
