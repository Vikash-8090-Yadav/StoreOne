"use client";


import { ethers } from 'ethers';
import { useEffect, useState } from 'react';
import axios from 'axios';
import Web3Modal from 'web3modal';



import { notification } from 'antd';



import { useRouter } from 'next/navigation';

import { marketplaceAddress } from '../../config';
import NFTMarketplace from "../../artifacts/contracts/NFTMarketplace.sol/NFTMarketplace.json"


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


export default function MyAssets() {
  const [nfts, setNfts] = useState([]);
  const [loadingState, setLoadingState] = useState('not-loaded');
  const router = useRouter();

  useEffect(() => {
    loadNFTs();
  }, []);

  async function loadNFTs() {
    const web3Modal = new Web3Modal({
      network: 'mainnet',
      cacheProvider: true,
    });
    const connection = await web3Modal.connect();
    const provider = new ethers.providers.Web3Provider(connection);
    const signer = provider.getSigner();

    const marketplaceContract = new ethers.Contract(
      marketplaceAddress,
      NFTMarketplace.abi,
      signer
    );
    const data = await marketplaceContract.fetchMyNFTs();

    const items = await Promise.all(
      data.map(async (i) => {
        const tokenURI = await marketplaceContract.tokenURI(i.tokenId);
        const meta = await axios.get(tokenURI);
        let price = ethers.utils.formatUnits(i.price.toString(), 'ether');
        let item = {
          price,
          tokenId: i.tokenId.toNumber(),
          seller: i.seller,
          owner: i.owner,
          image: meta.data.image,
          tokenURI,
          cid1:meta.data.cid1,
        };
        return item;
      })
    );
    setNfts(items);
    setLoadingState('loaded');
  }

  function listNFT(nft) {
    console.log('nft:', nft);
    router.push(`/resell-nft?id=${nft.tokenId}&tokenURI=${nft.tokenURI}`);
  }

  if (loadingState === 'loaded' && !nfts.length)
    return <h1 className="py-10 px-20  text-white text-3xl">No Items owned</h1>;

  return (
    <div className="flex justify-center">
      
      <div className="p-4">
        
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 pt-4">
          {nfts.map((nft, i) => (
            <div key={i} className="border shadow rounded-xl overflow-hidden bg-white">
              <img src={nft.image} className="rounded" />
              <div className="p-4">
                <p className="text-2xl font-bold">Price - {nft.price} eth</p>
                
                <button
                  className="mt-4 w-full bg-pink-500 text-white font-bold py-2 px-12 rounded"
                  onClick={() => listNFT(nft)}
                >
                  List
                </button>
              </div>
              
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
