import { ethers } from "ethers";
import Image from "next/image";
// import Coffee from "../public/Images/Coffee.png"
import { Button } from "antd";

import lighthouse from '@lighthouse-web3/sdk'
import axios from 'axios';
import Navbar from "../../Course/Nav";

import { Modal, Input, Tooltip } from 'antd'
import { notification } from 'antd';
const apiKey = "b3b18111.271ba6addd39409a80ac3fee4d78070c" 


let walletprovider;
if (typeof window !== 'undefined' && typeof window.ethereum !== 'undefined') {
    const {ethereum} =window;
    walletprovider = new  ethers.providers.Web3Provider(
      ethereum
    )
} else {
  
}


const Buy = ({ state }) => {

  const buyChai = async (event) => {

    alert("This is under the hook")
    
    event.preventDefault();
    const { contract } = state;
    const name = document.querySelector("#name").value;
    const message = document.querySelector("#message").value;

    console.log(name, message, contract);
  ;

    const amount = { value: ethers.utils.parseEther("0.001") };

    const data = JSON.stringify({
      name,
    message,
    amount
    });

    const LightHouseresponse = await lighthouse.uploadText(data, apiKey, "Uploaded Image")

      const cid1 = LightHouseresponse.data.Hash;

      const formData = new FormData();
  const requestReceivedTime = new Date()
  
  const endDate = requestReceivedTime.setMonth(requestReceivedTime.getMonth() + 1)
  const replicationTarget = 2
  const epochs = 4 // how many epochs before deal end should deal be renewed
  formData.append('cid', cid1)
  formData.append('endDate', endDate)
  formData.append('replicationTarget', replicationTarget)
  formData.append('epochs', epochs)

   console.log("The cid for the raas is",cid1);

  const response = await axios.post(
      `http://localhost:1337/api/register_job`,
      formData
  )
  console.log(response.data)
  

  notification.success({
    message: ' RAAS JOB Registered Sucessfully',
  });  


    const transaction = await contract.buyChai(name, message,cid1,amount);
    await transaction.wait();

  

    console.log("Transaction is done");
    
  };
  

  return (
    <>
 
      <div name = "contact" className = "w-full  ml-28 p-6">
        
        <div className = "flex flex-col p-4 justify-center max-w-screen-lg mx-auto ">
            <div className = " mn pb-8">
                <p className = "text-4xl font-bold text-center  flex items-center justify-center">COMMENT !!
                  {/* <Image src = {Coffee} height="50" width="50" className = "mx-3 transform flip-horizontal" /> */}
                </p>
                <p className = "py-6 text-center text-xl font-semibold">Comment and Enjoy!! .</p>
            </div>

            <div className = "flex justify-center items-center">
                <form  onSubmit={buyChai} className = "flex flex-col w-full md:w-1/2">
                    <input type = "text" id = "name" placeholder = "Enter Item name" className = "p-2 bg-transparent border-2 border-white rounded-md focus:outline-none text-white" />
                    <textarea placeholder = "Enter your Review Message" id = "message" rows = "8" className = "p-2 bg-transparent border-2 border-white rounded-md focus:outline-none text-white" />
                    <button  type="submit"
                  disabled={!state.contract} className = " btn btn-primary px-6 py-6 bg-gradient-to-b from-cyan-500 to-blue-500 my-8 mx-auto flex items-center rounded-md hover:scale-110 duration-150 text-white  font-semibold" >Complete Review and Get Direct NFT </button>
                </form>
            </div>
        </div>
      </div>
    </>
  );
};
export default Buy;
