import { Button, Frog, TextInput } from 'frog'
import { devtools } from 'frog/dev'
import { pinata } from 'frog/hubs'
import { neynar } from 'frog/middlewares'
import { serveStatic } from 'frog/serve-static'
import { handle } from 'frog/vercel'
import { createClient, cacheExchange, fetchExchange } from '@urql/core';
import React, { useState, useEffect } from 'react';



let address: string | null = null;

let balance: Number | null = null;

export const app = new Frog({
  basePath: '/api',
  // Supply a Hub API URL to enable frame verification.
  hub: pinata(),
}).use(
  neynar({
    apiKey: 'NEYNAR_FROG_FM',
    features: ['interactor', 'cast'],
  })
)

app.frame('/', (c) => {
  
  
  const { buttonValue, inputText, status } = c

  

  // return c.error({ message: "Please like and recast..." });


  const fruit = inputText || buttonValue
  return c.res({
    image: (
      <div
        style={{
          alignItems: 'center',
          background:
            status === 'response'
              ? 'linear-gradient(to right, #ff4e5f, #frb47b)' // More colorful gradient
              : 'linear-gradient(to right, #432681, #13101F)', // Different gradient for non-response
          backgroundSize: '100% 100%',
          display: 'flex',
          flexDirection: 'column',
          flexWrap: 'nowrap',
          height: '100%',
          justifyContent: 'center',
          textAlign: 'center',
          width: '100%',
          border: '5px solid #12ffff', // Add a border
          borderRadius: '15px', // Add rounded corners
          boxShadow: '0 4px 8px rgba(0, 0, 0, 0.2)', // Add a shadow for depth
        }}
      >
        <div
          style={{
            color: 'white',
            fontSize: 60,
            fontStyle: 'normal',
            letterSpacing: '-0.025em',
            lineHeight: 1.4,
            marginTop: 30,
            padding: '0 120px',
            whiteSpace: 'pre-wrap',
            textShadow: '2px 2px 4px rgba(0, 0, 0, 0.5)', // Add text shadow for better readability
          }}
        >
          {status === 'response'
            ? `Nice choice.${fruit ? ` ${fruit.toUpperCase()}!!` : ''}`
            : 'GetMoney, See Content Review!'}
        </div>
      </div>
    ),
    
    intents: [
     
      <Button action='/domain'>Get Started</Button>,
      status === 'response' && <Button.Reset>Reset</Button.Reset>,
    ],
    
  })
})



app.frame('/domain', async(c) => {
  const { buttonValue, inputText, status } = c;

  const QueryURL = "https://api.studio.thegraph.com/query/67475/getfarm/v0.0.1";

  const client = createClient({
    url: QueryURL,
  });

  const query = `
  {
    donations(first: 10, orderBy: id) {
      from
      id
      message
      name
      timestamp
    }
  }
  `;

  let donations = [];

  try {
    const response = await client.query(query).toPromise();
    if (response.error) {
      console.error("Error in query response:", response.error);
      throw new Error("Query failed");
    }
    donations = response.data.donations;

    console.log(donations);

  } catch (error) {
    console.error("Error during GraphQL query:", error);
  }

  const fruit = inputText || buttonValue;
  return c.res({
    image: (
      <div
        style={{
          alignItems: 'center',
          background:
            status === 'response'
              ? 'linear-gradient(to right, #432889, #17101F)'
              : 'black',
          backgroundSize: '100% 100%',
          display: 'flex',
          flexDirection: 'column',
          flexWrap: 'nowrap',
          height: '100%',
          justifyContent: 'center',
          textAlign: 'center',
          width: '100%',
        }}
      >
        <div
          style={{
            color: 'white',
            fontSize: 60,
            fontStyle: 'normal',
            letterSpacing: '-0.025em',
            lineHeight: 1.4,
            marginTop: 30,
            padding: '0 120px',
            whiteSpace: 'pre-wrap',
          }}
        >
          {status === 'response'
            ? "REVIEWS"
            : 'Welcome!'}
        </div>
        {
  donations.map((donation, index) => (
    <div
      key={index} // Use a unique key if 'id' is not available
      style={{
        color: 'white',
        fontSize: 40,
        fontStyle: 'normal',
        letterSpacing: '-0.0025em',
        lineHeight: 1.4,
        marginTop: 5,
        padding: '0 10px',
        whiteSpace: 'pre-wrap',
      }}
    >
      {status === 'response'
        ? `Name: ${donation.name || 'No message available'} - Message: ${donation.message || 'Anonymous'}`
        : 'Welcome!'}
    </div>
  ))
}

     

      </div>
    ),
    intents: [
      <Button.Link href="https://get-money.vercel.app/">Visit Dapp</Button.Link>,
      status === 'response' && <Button.Reset>Reset</Button.Reset>,
    ],
  });
});


// @ts-ignore
const isEdgeFunction = typeof EdgeFunction !== 'undefined'
const isProduction = isEdgeFunction || import.meta.env?.MODE !== 'development'
devtools(app, isProduction ? { assetsPath: '/.frog' } : { serveStatic })

export const GET = handle(app)
export const POST = handle(app)
