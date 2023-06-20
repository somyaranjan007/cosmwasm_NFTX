"use client"
import React from "react";
import MenuIcon from '@mui/icons-material/Menu';
import NftCard from "./NftCard";

const Nfts = () => {
  return (
    <div>
      <div className="flex justify-between">
        <div className="flex gap-5 p-10 ">
        <div className="flex justify-end items-center">
          <img
            className="h-8 "
            src="https://images.nftx.io/cdn-cgi/image/anim=false,width=150,height=150,format=auto/https://assets.nftx.io/vaults-v2/0/256x256.png"
            alt=""
          />
        </div>
        <div className="">
          <h3 className="text-lg font-semibold text-slate-200">PUNK</h3>
          <h4 className="text-sm font-light text-gray-200">cryptopunk</h4>
        </div>

        </div>
        <MenuIcon className="m-11"/>

      </div>
      <div className="flex flex-wrap gap-4">

      <NftCard />
      <NftCard />
      <NftCard />
      <NftCard />
      <NftCard />
      <NftCard />
      <NftCard />
      <NftCard />
      <NftCard />
      <NftCard />
      
      

      </div>

    </div> 
  );
};

export default Nfts;
