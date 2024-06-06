import { ethers } from 'ethers'
import { useEffect, useState } from 'react'
import axios from 'axios'
import Web3Modal from 'web3modal'
import { notification } from 'antd';
import {
  marketplaceAddress
} from '../config'

import NFTMarketplace from '../artifacts/contracts/NFTMarketplace.sol/NFTMarketplace.json'



async function PODSI(cid){
  try {
    // https://gateway.lighthouse.storage/ipfs/QmTMVyisen2BrnoHg1fyA53n6fnqETMvedcDRPsoNH5rN8
    const response1 = await axios.get(`https://api.lighthouse.storage/api/lighthouse/get_proof?network=testnet&cid=${cid}`);
    

    
    notification.success({
      message: ' Click here for the verification',
      description: (
        <div  >
          Transaction Hash: <a href={`https://api.lighthouse.storage/api/lighthouse/get_proof?network=testnet&cid=${cid}`} target="_blank" rel="noopener noreferrer">{'CLICK HERE'}</a>
        </div>
      )
    });
  } catch (error) {
    console.error('Error occurred:', error);
    // Handle the error here, you can log it or show a toast message indicating the error.
    // For example:
    notification.error({
      message: "Miners still verifying!",
      description: (
        <div>
          
        </div>
      )
    });
    
  }
}


async function Verify(cid){


  try {
    // https://gateway.lighthouse.storage/ipfs/QmTMVyisen2BrnoHg1fyA53n6fnqETMvedcDRPsoNH5rN8
    // const response1 = await axios.get(`https://api.lighthouse.storage/api/lighthouse/get_proof?network=testnet&cid=${cid}`);
    // const dealInfo = response1.data.dealInfo[0].dealId;
    // const polygonScanlink = `https://calibration.filfox.info/en/deal/${dealInfo}`;
    

    const verifyData = ` https://gateway.lighthouse.storage/ipfs/${cid}`;
    notification.success({
      message: ' Click here for the verification',
      description: (
        <div  >
          Transaction Hash: <a href={`https://gateway.lighthouse.storage/ipfs/${cid}`} target="_blank" rel="noopener noreferrer">{cid}</a>
        </div>
      )
    });
  } catch (error) {
    console.error('Error occurred:', error);
    // Handle the error here, you can log it or show a toast message indicating the error.
    // For example:
    notification.error({
      message: "Miners still verifying!",
      description: (
        <div>
          
        </div>
      )
    });
    
  }
}






export default function CreatorDashboard() {
  const [nfts, setNfts] = useState([])
  const [loadingState, setLoadingState] = useState('not-loaded')
  useEffect(() => {
    loadNFTs()
  }, [])
  async function loadNFTs() {
    const provider = new ethers.providers.Web3Provider(window.ethereum);
    await provider.send('eth_requestAccounts', []);
    const signer = provider.getSigner();

    const contract = new ethers.Contract(marketplaceAddress, NFTMarketplace.abi, signer)
    const data = await contract.fetchItemsListed()

    const items = await Promise.all(data.map(async i => {
      const tokenUri = await contract.tokenURI(i.tokenId)
      const meta = await axios.get(tokenUri)
      let price = ethers.utils.formatUnits(i.price.toString(), 'ether')
      let item = {
        price,
        tokenId: i.tokenId.toNumber(),
        seller: i.seller,
        owner: i.owner,
        image: meta.data.image,
        cid1:meta.data.cid1,
      }
      return item
    }))

    setNfts(items)
    setLoadingState('loaded') 
  }
  if (loadingState === 'loaded' && !nfts.length) return (<h1 className="py-10  text-white px-20 text-3xl">No Item listed</h1>)
  return (
    <div>
      <div className="p-4">
        <h2 className="text-2xl py-2">Items Listed</h2>
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 pt-4">
          {
            nfts.map((nft, i) => (
              <div key={i} className="border shadow rounded-xl overflow-hidden">
                <img src={nft.image} className="rounded" />
                <div className="p-4 bg-black">
                  <p className="text-2xl font-bold text-white">Price - {nft.price} tFIL</p>
                </div>
                <div className="p-1    umrk bg-black">
                  <p className="text-2xl font-bold text-white">Storage Provider t017840 </p>
                  <button className=" hover:rotate-2 delay-100 transition ease-in-out   text-center border hover:bg-gray-100 hover:shadow-md border-gray-500 rounded-md mt-4  bg-yellow-500 text-cyan font-bold py-3 px-12 mx-1 rounded" onClick={() => Verify(nft.cid1)}>Verify</button>
                  <button className=" hover:rotate-2 delay-100 transition ease-in-out   text-center border hover:bg-gray-100 hover:shadow-md border-gray-500 rounded-md mt-4  bg-gray-500 text-cyan font-bold py-3 px-12 rounded" onClick={() => PODSI(nft.cid1)}>PODSI</button>
                </div>
              </div>
            ))
          }
        </div>
      </div>
    </div>
  )
}
