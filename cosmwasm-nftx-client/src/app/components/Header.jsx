"use client"
import React, { useEffect, useState } from "react";
import { useRouter } from 'next/navigation'
import AcUnitIcon from '@mui/icons-material/AcUnit';
import ArrowDropDownIcon from '@mui/icons-material/ArrowDropDown';
import MonetizationOnIcon from '@mui/icons-material/MonetizationOn';

const Header = () => {
  const [isHovered, setIsHovered] = useState(false);
  const router = useRouter();


  const transfer_nft=(e)=>{
    e.preventDefault();
    setIsHovered(false);
    router.push("/transfer");
    
    
  };
  

  return <div className="bg-black px-4 border-b-2 border-gray-800 h-24 flex justify-between sticky top-0">
    <div className="flex items-center gap-6 ml-2 ">
    <a href="/" className="text-white h-8 w-8  mb-3"><AcUnitIcon className="w-11 h-11" /></a>    
  <input type="text" className="w-96 rounded-[7px] border border-gray-800 bg-zinc-900 px-3 py-2.5 text-white" placeholder="Search for Vaults" />

    </div>
    <div className="relative flex items-center gap-4 text-slate-200 font-semibold mr-2  ">
      <a className="hover:border-2 border-gray-500 hover:rounded-md inline-block rounded-full py-2 px-4" href="">Create</a>
    <a className="hover:border-2 border-gray-500 hover:rounded-md inline-block rounded-full py-2 px-4 cursor-pointer" onClick={transfer_nft}>Transfer</a>
      <a className="hover:border-2 border-gray-500 hover:rounded-md inline-block rounded-full py-2 px-4" href="">Redeem</a>
      <a href=""><MonetizationOnIcon/></a>
      <button className="bg-white text-black p-2 rounded-lg">Connect</button>
      
    </div>

  </div>;
};

export default Header;
