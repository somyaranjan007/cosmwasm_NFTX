import React from 'react';

const Card = ({ onClick }) => {
  return (
    <div className='bg-zinc-900 text-white border rounded-lg m-4 p-3 w-1/5 border-gray-800 cursor-pointer' onClick={onClick}>
      <div className="flex items-center justify-center">
        <img className='h-40 w-40 object-cover rounded-lg' src="https://i.seadn.io/gae/1SAW17tltJnfK9pRoWHp4cKb7od5gE0u_og1kqG08UwZT9HMm84PGnPjoEUVLEKg4pNT2UWzqZFqfb9tyjoJP9XqFyCEKKeHKAC4PQ?w=500&auto=format" alt="" />
      </div>
      <div className="flex ml-4 p-1 justify-start gap-2">
        <img className='h-8' src="https://images.nftx.io/cdn-cgi/image/anim=false,width=150,height=150,format=auto/https://assets.nftx.io/vaults-v2/0/256x256.png" alt="" />
        <h3 className='text-2xl font-semibold'>PUNK</h3>
      </div>
      <h5 className='text-sm ml-4 p-1 text-gray-600 overflow-hidden'>CryptoPunk</h5>
      <div className="flex gap-10 items-center justify-center">
        <div>
          <h6 className='text-xs'>Price</h6>
          <p>$100</p>
        </div>
        <div>
          <h6 className='text-xs'>Items</h6>
          <p>100</p>
        </div>
      </div>
    </div>
  )
}

export default Card;
