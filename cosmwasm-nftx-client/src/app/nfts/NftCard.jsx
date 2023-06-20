import React from "react";
import DiamondIcon from '@mui/icons-material/Diamond';

const NftCard = () => {
  return (
    <div
        className=" bg-zinc-900 text-white border rounded-lg m-4 p-2 w-1/6  border-gray-800 cursor-pointer "
        onClick={() => {}}>

        <div className="flex justify-between gap-3 text-slate-300 ">
          <h3 className="text-sm font-semibold p-2">#1</h3>
          <p>
            <span>1</span>
            <DiamondIcon className="h-5" />
          </p>
        </div>
        <div className="flex justify-center items-center">
          <img
            className="h-40 w-40 object-cover rounded-lg "
            src="https://i.seadn.io/gae/1SAW17tltJnfK9pRoWHp4cKb7od5gE0u_og1kqG08UwZT9HMm84PGnPjoEUVLEKg4pNT2UWzqZFqfb9tyjoJP9XqFyCEKKeHKAC4PQ?w=500&auto=format"
            alt=""
          />
        </div>
      </div>
    
  );
};

export default NftCard;
