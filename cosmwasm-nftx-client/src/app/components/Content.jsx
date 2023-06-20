"use client"
import React, { useState } from 'react'
import SearchIcon from '@mui/icons-material/Search';
import Card from './Card';
import { useRouter } from 'next/navigation'



const Content = () => {
  
  const [selectedVault, setSelectedVault] = useState(null);
  const router = useRouter();



  const handleVaultClick = (vaultId,e) => {
    e.preventDefault();
    setSelectedVault(vaultId);
    router.push(`/nfts`);
    console.log("clicked");
  };
  
  const cardData = [
    { id: 1, nfts: ['NFT1', 'NFT2', 'NFT3'] },
    { id: 2, nfts: ['NFT4', 'NFT5', 'NFT6'] },
    { id: 3, nfts: ['NFT7', 'NFT8', 'NFT9'] },
    
  ];
  
  return (
    <div>
      <div className="flex justify-between p-4">
        <h1 className='text-white font-bold text-2xl'>ALL COLLECTIONS</h1>
        <div className="border-b-2 mb-5  border-gray-800">
        <SearchIcon className='text-white'/>
         <input type="text" className='bg-transparent mb-2 outline-none pl-2' placeholder='Search Vault' />
        
        </div> 
      </div>
          <div className="flex flex-wrap ml-4 gap-6">
            <Card onClick={ (e)=>handleVaultClick(1,e)}/>
            <Card/>
            <Card/>
            <Card/> 
            <Card/>
            <Card/>
            <Card/>
            <Card/>
            <Card/>
            <Card/>
            <Card/>
            <Card/>
            <Card/>
            <Card/>
            <Card/>
            <Card/>
            <Card/>
            <Card/>
            <Card/>
            <Card/>
            <Card/>
            <Card/>
            <Card/>
            <Card/>
            <Card/>
            <Card/>
            <Card/>
            <Card/>
          </div>
        
          {/* {selectedVault && <Nfts nfts={cardData[selectedVault - 1].nfts} />}           */}
    </div>
  )
}

export default Content