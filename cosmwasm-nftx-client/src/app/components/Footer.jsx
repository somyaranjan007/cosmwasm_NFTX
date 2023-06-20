"use client"
import React from 'react'
import AcUnitIcon from '@mui/icons-material/AcUnit';


const Footer = () => {
  return (
    <div>
        <div className="bg-gradient-to-r from-transparent to-transparent via-gray-500  h-px my-10"></div>
    <div className='flex flex-col flex-1 justify-center gap-6'>
         <a href="/" className="text-white flex w-full justify-center"><AcUnitIcon className=' h-16 w-16'/></a> 
        <li className='flex text-sm font-light gap-10 justify-center text-start'>
            <a href="">Research</a>
            <a href="">Documentation</a>
            <a href="">Manage</a>
            <a href="">Discord</a>
            <a href="">Contact Us</a>
        </li>
    </div>
    <div className=' '>
        <a className='flex justify-center text-slate-200 font-extralight p-4'>created by abc</a>
    </div>
    </div>
  )
}

export default Footer