import {
  Connection,
  Keypair,
  Signer,
  PublicKey,
  Transaction,
  TransactionSignature,
  ConfirmOptions,
  sendAndConfirmRawTransaction,
  RpcResponseAndContext,
  SimulatedTransactionResponse,
  Commitment,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import * as splToken from '@solana/spl-token'
import fs from 'fs'
import * as anchor from '@project-serum/anchor'
import * as presale_api from './presale_api'

const sleep = (ms : number) => {
    return new Promise(resolve => setTimeout(resolve, ms));
};

async function airdrop(conn : Connection, address : PublicKey){
  let hash = await conn.requestAirdrop(address,LAMPORTS_PER_SOL)
  await conn.confirmTransaction(hash)
  await sleep(10000)
}

async function displayPresaleCreator(conn : Connection, address1 : PublicKey, mint1 : any, address2 : PublicKey, mint2 : any){
  let amount1=(await mint1.getAccountInfo(address1)).amount.words[0]
  let amount2=(await mint2.getAccountInfo(address2)).amount.words[0]
  console.log("Presale creator :  TokenForSale -- " + amount1 + "   TokenBeingRaised -- " + amount2 + "\n")  
}

async function displayBiddersState(conn : Connection, bidders : any, mint1 : any, mint2 : any){
  console.log("bidders :    For Sale     BeingRaised")
  for(let i=0;i<3;i++){
    let bidder = bidders[i]
    let amount1=(await mint1.getAccountInfo(bidder.bidder_token_1)).amount.words[0]
    let amount2=(await mint2.getAccountInfo(bidder.bidder_token_2)).amount.words[0]
    let str="bidder " + (i+1) + "  :      " +  amount1 + "        " + amount2
    console.log(str)
  }
}

async function test() {
  console.log("Start test")
  let conn = new Connection("https://api.devnet.solana.com",'confirmed')
  let presaleCreator = Keypair.generate()
  let tokenCreator2 = Keypair.generate()
  
  console.log("Presale creator Airdroping. Waiting...")
  await airdrop(conn,presaleCreator.publicKey)
  console.log("Token creator Airdroping. Waiting...")
  await airdrop(conn,tokenCreator2.publicKey)
 
  let tokenMint1 = await splToken.Token.createMint(conn, presaleCreator, presaleCreator.publicKey, null, 2, splToken.TOKEN_PROGRAM_ID)
  let tokenMint2 = await splToken.Token.createMint(conn, tokenCreator2, tokenCreator2.publicKey, null, 3, splToken.TOKEN_PROGRAM_ID)
  let auth_token = await tokenMint1.createAccount(presaleCreator.publicKey)
  await tokenMint1.mintTo(auth_token,presaleCreator,[],10000)
  let presale_pot = await tokenMint2.createAccount(presaleCreator.publicKey)
  let presale = Keypair.generate()

  await presale_api.initializePresale(
    conn,presaleCreator,presale,presale_pot,tokenMint1.publicKey,tokenMint2.publicKey,100,1000,40000,100,27,0,
  )

  let bidders = []
  for(let i=0;i<3;i++){
    let bidder = Keypair.generate()
    let client = Keypair.generate()
    console.log("Bidder  " + (i+1) + "  Airdroping.  Waiting...")
    await airdrop(conn, bidder.publicKey)
    let bidder_token_1 = await tokenMint1.createAccount(bidder.publicKey)
    let bidder_token_2 = await tokenMint2.createAccount(bidder.publicKey)
    await tokenMint2.mintTo(bidder_token_2,tokenCreator2,[],1000)
    await presale_api.initializeClient(
      conn,bidder,client,bidder_token_1,presale.publicKey,
    )
    bidders.push({
      bidder : bidder,
      bidder_token_1 : bidder_token_1,
      bidder_token_2 : bidder_token_2,
      client : client.publicKey,
    })
  }

  console.log("Add Bidder2 to whitelist\n")
  await presale_api.addToWhitelist(
    conn,presaleCreator,bidders[1].bidder.publicKey,bidders[1].client,presale.publicKey
  )

  // console.log("Stop whitelist\n")
  // await presale_api.stopWhitelist(
  //   conn,presaleCreator,presale.publicKey
  // )

  await displayBiddersState(conn,bidders,tokenMint1,tokenMint2)
  await displayPresaleCreator(conn,auth_token,tokenMint1,presale_pot,tokenMint2)
  console.log("Start presale\n")
  await presale_api.startPresale(
    conn,presaleCreator,presale.publicKey
  )

  console.log("Bidder2 -- buy  -- 100\n")
  await presale_api.buy(
    conn,bidders[1].bidder,bidders[1].bidder_token_2,tokenMint2.publicKey,presale.publicKey,bidders[1].client,100,
  )
  await displayBiddersState(conn,bidders,tokenMint1,tokenMint2)
  
  // You get error message because bidder1 is not whitelisted 
  // console.log("Bidder1 -- buy  -- 100")
  // await presale_api.buy(
  //   conn,bidders[0].bidder,bidders[0].bidder_token_2,tokenMint2.publicKey,presale.publicKey,bidders[0].client,100,
  // )
  // await displayBiddersState(conn,bidders,tokenMint1,tokenMint2)  

  console.log("Stop Presale\n")
  await presale_api.stopPresale(
    conn,presaleCreator,presale.publicKey
  )

  // You get error message because Presale is ended
  // console.log("Bidder2 -- buy  -- 100")
  // await presale_api.buy(
  //   conn,bidders[1].bidder,bidders[1].bidder_token_2,tokenMint2.publicKey,presale.publicKey,bidders[1].client,100,
  // )
  // await displayBiddersState(conn,bidders,tokenMint1,tokenMint2)

  console.log("Distribute Tokens\n")
  await presale_api.distributeTokens(
    conn,presaleCreator,auth_token,tokenMint1.publicKey,presale.publicKey,10
  )
  await displayBiddersState(conn,bidders,tokenMint1,tokenMint2)
  await displayPresaleCreator(conn,auth_token,tokenMint1,presale_pot,tokenMint2)

  console.log("End test")
}

test()
