import { ethers } from 'ethers'
import { useEffect, useState } from 'react'
import axios from 'axios'
import Web3Modal from 'web3modal'
import Navbar from "../Component/Course/Nav";
import { createClient } from "urql";
import { ToastContainer, toast } from 'react-toastify';
import { notification } from 'antd';


import {
  marketplaceAddress
} from '../config'

import NFTMarketplace from '../abi/marketplace.json'








export default function Home() {
  const [nfts, setNfts] = useState([])
  const [loadingState, setLoadingState] = useState('not-loaded')
  const [isLoading, setIsLoading] = useState(true); 
  const [tokens, setTokens] = useState([]);


  
  const QueryURL = "https://api.studio.thegraph.com/query/67475/sbs-ticket/v0.0.1";

  let query = `
    {
      sbsticketTokenItems {
        token_uri
      price
      new_token_id
      }
    }
  `;

  const client = createClient({
    url: QueryURL
  });

  useEffect(() => {
  if (!client) {
    return;
  }

  const getTokens = async () => {  
    
    try {
      const { data } = await client.query(query).toPromise();
      // console.log(data.sbsticketTokenItems);
      setTokens(data.sbsticketTokenItems);
      
      setIsLoading(false); // Data is loaded
      await loadNFTs();
    } catch (error) {
      console.error("Error fetching data:", error);
    }
  };

  getTokens();
  // fetchNftTransactions();
}, [client]);

async function loadNFTs() {
  // await test();
  
  try {

    const items = await Promise.all(tokens.map(async token => {
      // alert("try")
      const meta = await axios.get(token.token_uri);
      console.log("The i item is".meta);
      const price = await token.price / 10 ** 18;
      const tokenId = await token.new_token_id;
      // alert("inside try")
      return {
        price,
        tokenId,
        image: meta.data.image,
        name: meta.data.name,
        description: meta.data.description,
      };
    }));
    
    setNfts(items);
    setLoadingState('loaded');
  } catch (error) {
    console.error("Error loading NFTs:", error);
    setLoadingState('error');
  }
}
  async function buyNft(nft) {
    /* needs the user to sign the transaction, so will use Web3Provider and sign it */
    const provider = new ethers.providers.Web3Provider(window.ethereum);
    await provider.send('eth_requestAccounts', []);
    const signer = provider.getSigner();
    const contract = new ethers.Contract(marketplaceAddress, NFTMarketplace.abi, signer)

    /* user will be prompted to pay the asking proces to complete the transaction */
    const price = ethers.utils.parseUnits(nft.price.toString(), 'ether')   
    const transaction = await contract.createMarketSale(nft.tokenId, {
      value: price
    })
    await transaction.wait()
    loadNFTs()
  }
  if (loadingState === 'loaded' && !nfts.length) return (<h1 className="px-20 py-10 text-white text-3xl">No Item in marketplace</h1>)
  return (
    <>
  
    <div className="flex mrkt  justify-center">
      {/* <Navbar/> */}
      <div className="px-10" style={{ maxWidth: '1600px' }}>
        <div className="grid flex  grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 pt-1 unmrk">
          {
            nfts.map((nft, i) => (
              <div key={i} className="border rounded-t-md  umrkt shadow rounded-xl overflow-hidden">
                <img   height="25px"  className = " w-full rounded-t-md duration-200 hover:scale-110 hover:overflow-hidden" src={nft.image} />
                <div className="p-1">
                  <p style={{ height: '100%' }} className="text-2xl font-semibold">{nft.name}</p>
                  <div style={{ height: '70px', overflow: 'hidden' }}>
                    <p className="text-gray-400">{nft.description}</p>
                  </div>
                 
                </div>
              
                <div className="p-1  umrk bg-black">
                  <p className="text-2xl font-bold text-white">{nft.price} ETH</p>
                  <button className=" hover:rotate-2 delay-100 transition ease-in-out   text-center border hover:bg-gray-100 hover:shadow-md border-gray-500 rounded-md mt-4 w-full bg-green-500 text-cyan font-bold py-2 px-12 rounded" onClick={() => buyNft(nft)}>Buy</button>
                </div>
                
              </div>
            ))
          }
        </div>
      </div>
    </div>
    </>)
}