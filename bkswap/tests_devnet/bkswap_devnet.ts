import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import {Keypair, PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, Transaction} from '@solana/web3.js';
import {Bkswap} from "../target/types/bkswap";
import {RaydiumRouter} from "../target/types/raydium_router";
import {RaydiumTestHelper} from "../target/types/raydium_test_helper";
import BigNumber from 'bignumber.js';
import {TOKEN_PROGRAM_ID} from '@solana/spl-token';
import {closeAccount, initializeAccount} from '@project-serum/serum/lib/token-instructions';
import {getMultipleAccountsInfo, Spl, SPL_ACCOUNT_LAYOUT, SPL_MINT_LAYOUT, WSOL,} from '@raydium-io/raydium-sdk';
import {
  createAssociatedTokenAccountIfNotExist,
  createMintPair,
  createSerumMarket,
  getAssociatedPoolKeys,
  getBigNumber,
  getFilteredTokenAccountsByOwner,
  getMarket,
  getMintDecimals,
  getVaultOwnerAndNonce,
  sleep
} from './util';

const marketInfo = {
  serumDexProgram: new PublicKey('DESVgJVGajEgKGXhb6XmqDHGz3VjdgP7rEVESBgxmroY'),
  ammProgram: new PublicKey('9rpQHSyFVM1dkkHFQ2TtTzPEW7DVmEyPmN8wVniqJtuC'),
  serumMarket: new Keypair(),
};
process.env.ANCHOR_WALLET = '/Users/liangjunren/.config/solana/id.json'
process.env.ANCHOR_PROVIDER_URL = 'https://api.devnet.solana.com'


describe("bkswap", () => {
  const provider = anchor.AnchorProvider.env();
  provider.opts.skipPreflight = true;
  
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  const program = anchor.workspace.RaydiumTestHelper as Program<RaydiumTestHelper>;
  const programRaydiumRouter = anchor.workspace.RaydiumRouter as Program<RaydiumRouter>;
  
  const serumMarketId = marketInfo.serumMarket.publicKey.toString();
  let conn
  let owner
  let tx
  let market
  let poolKeys
  let vaultOwner
  let vaultNonce
  let userCoinTokenAccount
  let userPcTokenAccount
  
  it('swap test!', async () => {
    conn = provider.connection;
    owner = provider.wallet.publicKey;
    
    let alreadCreatedMarket = false;
    
    let multipleInfo = await getMultipleAccountsInfo(conn, [new PublicKey(serumMarketId)]);
    console.log(`multipleInfo:`, multipleInfo);
    if (multipleInfo.length > 0 && multipleInfo[0] !== null) {
      if (multipleInfo[0]?.data.length !== 0) {
        alreadCreatedMarket = true;
      }
    }
    console.log('alreadCreatedMarket:', alreadCreatedMarket);
    if (!alreadCreatedMarket) {
      console.log(`provider:`, provider);
      const {tokenAMintAddress, tokenBMintAddress} = await createMintPair(provider);
      // create serum market
      const createMarketInfo = await createSerumMarket({
        connection: provider.connection,
        wallet: provider.wallet as anchor.Wallet,
        baseMint: tokenAMintAddress,
        quoteMint: tokenBMintAddress,
        baseLotSize: 1,
        quoteLotSize: 1,
        dexProgram: marketInfo.serumDexProgram,
        market: marketInfo.serumMarket,
      });
      console.log(JSON.stringify(createMarketInfo));
      // wait for transaction success
      // await sleep(10000);
    }
    
    // get serum market info
    market = await getMarket(conn, serumMarketId, marketInfo.serumDexProgram.toString());
    console.log('serum market info:', JSON.stringify(market));
    poolKeys = await getAssociatedPoolKeys({
      programId: marketInfo.ammProgram,
      serumProgramId: marketInfo.serumDexProgram,
      marketId: market.address,
      baseMint: market.baseMint,
      quoteMint: market.quoteMint
    });
    console.log('amm poolKeys: ', JSON.stringify(poolKeys));
    
    const ammAuthority = poolKeys.authority;
    const nonce = new anchor.BN(poolKeys.nonce);
    const ammId: PublicKey = poolKeys.id;
    const poolCoinTokenAccount: PublicKey = poolKeys.baseVault;
    const poolPcTokenAccount: PublicKey = poolKeys.quoteVault;
    const lpMintAddress: PublicKey = poolKeys.lpMint;
    const poolTempLpTokenAccount: PublicKey = poolKeys.lpVault;
    const ammTargetOrders: PublicKey = poolKeys.targetOrders;
    const poolWithdrawQueue: PublicKey = poolKeys.withdrawQueue;
    const ammOpenOrders: PublicKey = poolKeys.openOrders;
    
    let alreadPreInitialized = false;
    multipleInfo = await getMultipleAccountsInfo(conn, [lpMintAddress]);
    if (multipleInfo.length > 0 && multipleInfo[0] !== null) {
      const tempLpMint = SPL_MINT_LAYOUT.decode(multipleInfo[0]?.data);
      if (getBigNumber(tempLpMint.supply) !== 0) {
        alreadPreInitialized = true;
      }
    }
    console.log('alreadPreInitialized: ', alreadPreInitialized);
    if (!alreadPreInitialized) {
      /************************************ preInitialize test ***********************************************************************/
      tx = await program.methods.proxyPreInitialize(nonce.toNumber()).accounts({
        ammProgram: marketInfo.ammProgram,
        ammTargetOrders: ammTargetOrders,
        poolWithdrawQueue: poolWithdrawQueue,
        ammAuthority: ammAuthority,
        lpMint: lpMintAddress,
        coinMint: market.baseMintAddress,
        pcMint: market.quoteMintAddress,
        poolCoinTokenAccount: poolCoinTokenAccount,
        poolPcTokenAccount: poolPcTokenAccount,
        poolTempLpTokenAccount: poolTempLpTokenAccount,
        serumMarket: market.address,
        userWallet: owner,
        splTokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
      }).rpc()
      console.log('preinitialize tx: ', tx);
      // await sleep(10000);
    }
    
    let alreadInitialized = false;
    multipleInfo = await getMultipleAccountsInfo(conn, [ammId]);
    if (multipleInfo.length > 0 && multipleInfo[0] !== null) {
      if (multipleInfo[0]?.data.length !== 0) {
        alreadInitialized = true;
      }
    }
    console.log('alreadInitialized:', alreadInitialized);
    if (!alreadInitialized) {
      /************************************ initialize test ***********************************************************************/
        
        // set as you want
      const userInputBaseValue = 1;
      const userInputQuoteValue = 2;
      
      await initAmm(
        conn,
        provider,
        market,
        userInputBaseValue,
        userInputQuoteValue,
        poolCoinTokenAccount,
        poolPcTokenAccount,
        lpMintAddress,
      );
      
      // belongs to owner who create the pool
      const userLpTokenAccountPubKey = await Spl.getAssociatedTokenAccount({mint: lpMintAddress, owner: owner});
      tx = await program.rpc.proxyInitialize(nonce, new anchor.BN(0), {
        accounts: {
          ammProgram: marketInfo.ammProgram,
          amm: ammId,
          ammAuthority: ammAuthority,
          ammOpenOrders: ammOpenOrders,
          lpMint: lpMintAddress,
          coinMint: market.baseMintAddress,
          pcMint: market.quoteMintAddress,
          poolCoinTokenAccount: poolCoinTokenAccount,
          poolPcTokenAccount: poolPcTokenAccount,
          poolWithdrawQueue: poolWithdrawQueue,
          poolTargetOrdersAccount: ammTargetOrders,
          poolLpTokenAccount: userLpTokenAccountPubKey,
          poolTempLpTokenAccount: poolTempLpTokenAccount,
          serumProgram: marketInfo.serumDexProgram,
          serumMarket: serumMarketId,
          userWallet: owner,
          splTokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          rent: SYSVAR_RENT_PUBKEY,
        }
      });
      console.log('initialize tx: ', tx);
      // await sleep(10000);
    }
    
    /************************************ deposit test ***********************************************************************/
    
    const transaction = new Transaction();
    userCoinTokenAccount = await createAssociatedTokenAccountIfNotExist(
      provider.wallet.publicKey,
      market.baseMint,
      transaction,
      provider.connection
    );
    
    userPcTokenAccount = await createAssociatedTokenAccountIfNotExist(
      provider.wallet.publicKey,
      market.quoteMint,
      transaction,
      provider.connection
    );
    
    const userLPTokenAccount: PublicKey = await createAssociatedTokenAccountIfNotExist(
      provider.wallet.publicKey,
      poolKeys.lpMint,
      transaction,
      provider.connection
    );
    
    if (transaction.instructions.length > 0) {
      let txid = provider.sendAndConfirm(transaction, null, {
        skipPreflight: true,
        preflightCommitment: 'confirmed'
      });
      console.log('create user lp token account txid:', txid);
      // checkTxid(provider.connection, txid)
    }
    
    tx = await program.methods.proxyDeposit(
      new anchor.BN(1000000000), // maxCoinAmount
      new anchor.BN(2000000000), // maxPcAmount
      new anchor.BN(1), // baseSide?
    ).accounts({
      userOwner: provider.wallet.publicKey,
      ammProgram: marketInfo.ammProgram,
      amm: poolKeys.id,
      ammAuthority: poolKeys.authority,
      ammOpenOrders: poolKeys.openOrders,
      ammTargetOrders: poolKeys.targetOrders,
      lpMint: poolKeys.lpMint,
      poolCoinTokenAccount: poolKeys.baseVault,
      poolPcTokenAccount: poolKeys.quoteVault,
      serumMarket: serumMarketId,
      serumEventQueue: market.eventQueue,
      userCoinTokenAccount: userCoinTokenAccount,
      userPcTokenAccount: userPcTokenAccount,
      userLpTokenAccount: userLPTokenAccount,
      splTokenProgram: TOKEN_PROGRAM_ID,
    }).rpc();
    console.log('deposit tx: ', tx);
    // await sleep(3000);
    
    /************************************ withdraw test ***********************************************************************/
    
    ({
      vaultOwner,
      vaultNonce
    } = await getVaultOwnerAndNonce(new PublicKey(serumMarketId), marketInfo.serumDexProgram));
    if (vaultNonce.toNumber() != market.vaultSignerNonce) {
      console.log('withdraw vaultOwner:', vaultOwner.toString(), 'vaultNonce: ', vaultNonce.toNumber(), 'market.vaultSignerNonce:', market.vaultSignerNonce.toString());
      throw ('vaultSignerNonce incorrect!');
    }
    tx = await program.rpc.proxyWithdraw(
      new anchor.BN(1000), // lpAmount
      {
        accounts: {
          ammProgram: marketInfo.ammProgram,
          amm: poolKeys.id,
          ammAuthority: poolKeys.authority,
          ammOpenOrders: poolKeys.openOrders,
          ammTargetOrders: poolKeys.targetOrders,
          lpMint: poolKeys.lpMint,
          poolCoinTokenAccount: poolKeys.baseVault,
          poolPcTokenAccount: poolKeys.quoteVault,
          poolWithdrawQueue: poolKeys.withdrawQueue,
          poolTempLpTokenAccount: poolKeys.lpVault,
          serumProgram: marketInfo.serumDexProgram,
          serumMarket: serumMarketId,
          serumCoinVaultAccount: market.baseVault,
          serumPcVaultAccount: market.quoteVault,
          serumVaultSigner: vaultOwner,
          userCoinTokenAccount: userCoinTokenAccount,
          userPcTokenAccount: userPcTokenAccount,
          userLpTokenAccount: userLPTokenAccount,
          userOwner: provider.wallet.publicKey,
          serumEventQ: market.eventQueue,
          serumBids: market.bids,
          serumAsks: market.asks,
          splTokenProgram: TOKEN_PROGRAM_ID,
        },
      });
    
    console.log('withdraw tx: ', tx);
    
    /************************************ swapBaseIn test ********************************************************************** */
    
    tx = await programRaydiumRouter.rpc.proxySwapBaseIn(
      new anchor.BN(100), // amountIn
      new anchor.BN(100), // amountOut
      {
        accounts: {
          ammProgram: marketInfo.ammProgram,
          amm: poolKeys.id,
          ammAuthority: poolKeys.authority,
          ammOpenOrders: poolKeys.openOrders,
          ammTargetOrders: poolKeys.targetOrders,
          poolCoinTokenAccount: poolKeys.baseVault,
          poolPcTokenAccount: poolKeys.quoteVault,
          serumProgram: marketInfo.serumDexProgram,
          serumMarket: serumMarketId,
          serumBids: market.bids,
          serumAsks: market.asks,
          serumEventQueue: market.eventQueue,
          serumCoinVaultAccount: market.baseVault,
          serumPcVaultAccount: market.quoteVault,
          serumVaultSigner: vaultOwner,
          userSourceTokenAccount: userCoinTokenAccount,
          userDestinationTokenAccount: userPcTokenAccount,
          userSourceOwner: provider.wallet.publicKey,
          splTokenProgram: TOKEN_PROGRAM_ID,
        },
      });
    console.log('swap_base_in tx: ', tx);
    
  });
  
  it('swap again', async () => {
    /************************************ swapBaseIn test ********************************************************************** */
    
    tx = await programRaydiumRouter.rpc.proxySwapBaseIn(
      new anchor.BN(100), // amountIn
      new anchor.BN(100), // amountOut
      {
        accounts: {
          ammProgram: marketInfo.ammProgram,
          amm: poolKeys.id,
          ammAuthority: poolKeys.authority,
          ammOpenOrders: poolKeys.openOrders,
          ammTargetOrders: poolKeys.targetOrders,
          poolCoinTokenAccount: poolKeys.baseVault,
          poolPcTokenAccount: poolKeys.quoteVault,
          serumProgram: marketInfo.serumDexProgram,
          serumMarket: serumMarketId,
          serumBids: market.bids,
          serumAsks: market.asks,
          serumEventQueue: market.eventQueue,
          serumCoinVaultAccount: market.baseVault,
          serumPcVaultAccount: market.quoteVault,
          serumVaultSigner: vaultOwner,
          userSourceTokenAccount: userCoinTokenAccount,
          userDestinationTokenAccount: userPcTokenAccount,
          userSourceOwner: provider.wallet.publicKey,
          splTokenProgram: TOKEN_PROGRAM_ID,
        },
      });
    console.log('swap_base_in tx: ', tx);
    
  })
});

async function initAmm(
  conn: any,
  provider: anchor.AnchorProvider,
  market: any,
  userInputBaseValue: number,
  userInputQuoteValue: number,
  poolCoinTokenAccount: PublicKey,
  poolPcTokenAccount: PublicKey,
  lpMintAddress: PublicKey,
) {
  const baseMintDecimals = new BigNumber(await getMintDecimals(conn, market.baseMintAddress as PublicKey));
  const quoteMintDecimals = new BigNumber(await getMintDecimals(conn, market.quoteMintAddress as PublicKey));
  const coinVol = new BigNumber(10).exponentiatedBy(baseMintDecimals).multipliedBy(userInputBaseValue);
  const pcVol = new BigNumber(10).exponentiatedBy(quoteMintDecimals).multipliedBy(userInputQuoteValue);
  const transaction = new Transaction();
  const signers: any = [];
  const owner = provider.wallet.publicKey;
  const baseTokenAccount = await getFilteredTokenAccountsByOwner(conn, owner, market.baseMintAddress);
  const quoteTokenAccount = await getFilteredTokenAccountsByOwner(conn, owner, market.quoteMintAddress);
  const baseTokenList: any = baseTokenAccount.value.map((item: any) => {
    if (item.account.data.parsed.info.tokenAmount.amount >= getBigNumber(coinVol)) {
      return item.pubkey;
    }
    return null;
  });
  const quoteTokenList: any = quoteTokenAccount.value.map((item: any) => {
    if (item.account.data.parsed.info.tokenAmount.amount >= getBigNumber(pcVol)) {
      return item.pubkey;
    }
    return null;
  });
  let baseToken: string | null = null;
  for (const item of baseTokenList) {
    if (item !== null) {
      baseToken = item;
    }
  }
  let quoteToken: string | null = null;
  for (const item of quoteTokenList) {
    if (item !== null) {
      quoteToken = item;
    }
  }
  if (
    (baseToken === null && market.baseMintAddress.toString() !== WSOL.mint) ||
    (quoteToken === null && market.quoteMintAddress.toString() !== WSOL.mint)
  ) {
    throw new Error('no money');
  }
  
  const destLpToken: PublicKey = await createAssociatedTokenAccountIfNotExist(
    owner,
    lpMintAddress,
    transaction,
    conn
  );
  
  if (market.baseMintAddress.toString() === WSOL.mint) {
    const newAccount = new Keypair();
    transaction.add(
      SystemProgram.createAccount({
        fromPubkey: owner,
        newAccountPubkey: newAccount.publicKey,
        lamports: parseInt(coinVol.toFixed()) + 1e7,
        space: SPL_ACCOUNT_LAYOUT.span,
        programId: TOKEN_PROGRAM_ID
      })
    );
    transaction.add(
      initializeAccount({
        account: newAccount.publicKey,
        mint: new PublicKey(WSOL.mint),
        owner
      })
    );
    
    transaction.add(Spl.makeTransferInstruction({
      source: newAccount.publicKey,
      destination: poolCoinTokenAccount,
      owner: owner,
      amount: parseInt(coinVol.toFixed()),
      instructionsType:[]
    }));
    
    transaction.add(
      closeAccount({
        source: newAccount.publicKey,
        destination: owner,
        owner
      })
    );
    
    signers.push(newAccount);
  } else {
    transaction.add(
      Spl.makeTransferInstruction({
        source: new PublicKey(baseToken),
        destination: poolCoinTokenAccount,
        owner: owner,
        amount: parseInt(coinVol.toFixed()),
        instructionsType:[]
      })
    );
  }
  if (market.quoteMintAddress.toString() === WSOL.mint) {
    const newAccount = new Keypair();
    transaction.add(
      SystemProgram.createAccount({
        fromPubkey: owner,
        newAccountPubkey: newAccount.publicKey,
        lamports: parseInt(pcVol.toFixed()) + 1e7,
        space: SPL_ACCOUNT_LAYOUT.span,
        programId: TOKEN_PROGRAM_ID
      })
    );
    transaction.add(
      initializeAccount({
        account: newAccount.publicKey,
        mint: new PublicKey(WSOL.mint),
        owner
      })
    );
    transaction.add(Spl.makeTransferInstruction({
      source: newAccount.publicKey,
      destination: poolPcTokenAccount,
      owner: owner,
      amount: parseInt(pcVol.toFixed()),
      instructionsType:[]
    }));
    
    transaction.add(
      closeAccount({
        source: newAccount.publicKey,
        destination: owner,
        owner
      })
    );
    signers.push(newAccount);
  } else {
    transaction.add(
      Spl.makeTransferInstruction({
        source: new PublicKey(quoteToken),
        destination: poolPcTokenAccount,
        owner: owner,
        amount: parseInt(pcVol.toFixed()),
        instructionsType:[]
      })
    );
  }
  
  let txid = await provider.sendAndConfirm(transaction, signers, {
    skipPreflight: true,
    preflightCommitment: 'confirmed'
  });
  console.log('initAMM txid:', txid);
  // checkTxid(conn, txid)
}
