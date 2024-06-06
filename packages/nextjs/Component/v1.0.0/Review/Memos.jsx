// Endpoint for  olx Subgraph endpoints:
// Queries (HTTP):     https://api.studio.thegraph.com/query/54911/olx-marketplac/v0.0.1

import { useState, useEffect } from "react";
import { createClient } from "urql";

import { notification } from 'antd';
import Navbar from "../../Course/Nav";
async function Deal(cid){

  try{
  const response1 = await axios.get(`https://api.lighthouse.storage/api/lighthouse/get_proof?network=testnet&cid=${cid}`);
    const dealInfo = response1.data.dealInfo[0].dealId;
    const polygonScanlink = `https://calibration.filfox.info/en/deal/${dealInfo}`;

    notification.success({
      message: 'Click here for the verification',
      description: (
        <div>
          Transaction Hash: <a href={`https://calibration.filfox.info/en/deal/${dealInfo}`} target="_blank" rel="noopener noreferrer">{`https://calibration.filfox.info/en/deal/${dealInfo}`}</a>
        </div>
      )
    });

  }
  catch(error){

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



const Memos = ({ state }) => {
  const [tokens, setTokens] = useState([]);
  const [isLoading, setIsLoading] = useState(true); // Add loading state

  const [memos, setMemos] = useState([]);
  const { contract } = state;

  useEffect(() => {
    const memosMessage = async () => {
      const memos = await contract.getMemos();
      setMemos(memos);
    };
    contract && memosMessage();
  }, [contract]);

  return (
    <>
        <p style={{ textAlign: "center", marginTop: "20px" }}>Messages</p>
      {memos.map((memo) => {
        return (
          <div
            className="container-fluid"
            style={{ width: "100%" }}
            key={Math.random()}
          >
            <table
              style={{
                marginBottom: "10px",
              }}
            >
              <tbody>
                <tr>
                  <td
                    style={{
                      backgroundColor: "#96D4D4",
                      border: "1px solid white",
                      borderCollapse: "collapse",
                      padding: "7px",
                      width: "100px",
                    }}
                  >
                    {memo.name}
                  </td>
                  <td
                    style={{
                      backgroundColor: "#96D4D4",
                      border: "1px solid white",
                      borderCollapse: "collapse",
                      padding: "7px",
                      width: "800px",
                    }}
                  >
                    {new Date(memo.timestamp * 1000).toLocaleString()}
                  </td>
                  <td
                    style={{
                      backgroundColor: "#96D4D4",
                      border: "1px solid white",
                      borderCollapse: "collapse",
                      padding: "7px",
                      width: "300px",
                    }}
                  >
                    {memo.message}
                  </td>
                  <td
                    style={{
                      backgroundColor: "#96D4D4",
                      border: "1px solid white",
                      borderCollapse: "collapse",
                      padding: "7px",
                      width: "400px",
                    }}
                  >
                    {memo.from}
                  </td>
                  
                  <td
                    style={{
                      backgroundColor: "#96D4D4",
                      border: "1px solid white",
                      borderCollapse: "collapse",
                      padding: "7px",
                      width: "400px",
                    }}
                  >
                   <button className=" hover:rotate-2 delay-100 transition ease-in-out   text-center border hover:bg-gray-100 hover:shadow-md border-gray-500 rounded-md mt-4  bg-yellow-500 text-cyan font-bold py-3 px-12 mx-1 rounded" onClick={() => Verify(memo.cid)}>Verify</button>
                
                  </td>
                  <td
                    style={{
                      backgroundColor: "#96D4D4",
                      border: "1px solid white",
                      borderCollapse: "collapse",
                      padding: "7px",
                      width: "400px",
                    }}
                  >

                  <button className=" hover:rotate-2 delay-100 transition ease-in-out   text-center border hover:bg-gray-100 hover:shadow-md border-gray-500 rounded-md mt-4  bg-gray-500 text-cyan font-bold py-3 px-12 rounded" onClick={() => Deal(memo.cid)}>DEALID</button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        );
      })}
      </>
  );
};

export default Memos;
