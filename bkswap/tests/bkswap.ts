import * as anchor from '@project-serum/anchor';
import {Program} from '@project-serum/anchor';
import {Keypair, PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, Transaction} from '@solana/web3.js';
import {Bkswap} from '../target/types/bkswap';
import {RaydiumRouter} from '../target/types/raydium_router';
import {RaydiumTestHelper} from '../target/types/raydium_test_helper';
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
  serumDexProgram: new PublicKey('srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX'),
  ammProgram: new PublicKey('675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8'),
  serumMarket: new PublicKey('2AdaV97p6SfkuMQJdu8DHhBhmJe7oWdvbm52MJfYQmfA'),
};
// WSOL -> USDT
// const ctxAccounts = {
//   ammProgram: new PublicKey('675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8'),
// amm: new PublicKey('7XawhbbxtsRcQA8KTkHT9f9nc6d69UwqCDh6U5EEbEmX'),
// ammAuthority: new PublicKey('5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1'),
// ammOpenOrders: new PublicKey('3oWQRLewGsUMA2pebcpGPPGrzyRNfbs7fQEMUxPAGgff'),
// ammTargetOrders: new PublicKey('9x4knb3nuNAzxsV7YFuGLgnYqKArGemY54r2vFExM1dp'),
// poolCoinTokenAccount: new PublicKey('876Z9waBygfzUrwwKFfnRcc7cfY4EQf6Kz1w7GRgbVYW'),
// poolPcTokenAccount: new PublicKey('CB86HtaqpXbNWbq67L18y5x2RhqoJ6smb7xHUcyWdQAQ'),
// serumProgram: new PublicKey('srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX'),
// serumMarket: new PublicKey('2AdaV97p6SfkuMQJdu8DHhBhmJe7oWdvbm52MJfYQmfA'),
// serumBids: new PublicKey('F4LnU7SarP7nLmGPnDHxnCqZ8gRwiFRgbo5seifyicfo'),
// serumAsks: new PublicKey('BKgZNz8tqJFoZ9gEHKR6k33wBMeXKAaSWpW5zMhSRhr3'),
// serumEventQueue: new PublicKey('9zw6ztEpHfcKccahzTKgPkQNYhJMPwL4iJJc8BAztNYY'),
// serumCoinVaultAccount: new PublicKey('4zVFCGJVQhSvsJ625qTH4WKgvfPQpNpAVUfjpgCxbKh8'),
// serumPcVaultAccount: new PublicKey('9aoqhYjXBqWsTVCEjwtxrotx6sVPGVLmbpVSpSRzTv54'),
// serumVaultSigner: new PublicKey('n8meSpYX5n3oRoToN21PFQ5SSYBDf675eub3WMoJJoA'),
// }

// USDC -> RAY
const ctxAccounts = {
  ammProgram: new PublicKey('675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8'),
  amm: new PublicKey('DVa7Qmb5ct9RCpaU7UTpSaf3GVMYz17vNVU67XpdCRut'),
  ammAuthority: new PublicKey('5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1'),
  ammOpenOrders: new PublicKey('8MSPLj4c2hi1fZGDARvxLXQp1ooDQ8iGnWXbGdwvZxUQ'),
  ammTargetOrders: new PublicKey('3K2uLkKwVVPvZuMhcQAPLF8hw95somMeNwJS7vgWYrsJ'),
  poolCoinTokenAccount: new PublicKey('3wqhzSB9avepM9xMteiZnbJw75zmTBDVmPFLTQAGcSMN'),
  poolPcTokenAccount: new PublicKey('5GtSbKJEPaoumrDzNj4kGkgZtfDyUceKaHrPziazALC1'),
  serumProgram: new PublicKey('srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX'),
  serumMarket: new PublicKey('GpHbiJJ9VHiuHVXeoet121Utrbm1CSNNzYrBKB8Xz2oz'),
  serumBids: new PublicKey('EY9SKG9EyNFuy1JfQrmaUdHUiTqaVfX3zG9gGfWQywTV'),
  serumAsks: new PublicKey('GZ2DN58shX6igfaCquq8BLWE1ytkT5uKZfnAGSiu8mR4'),
  serumEventQueue: new PublicKey('9VWPNiJu7ui3xucUHtKmGCHENWhohVmZxujP4uCB8Fzm'),
  serumCoinVaultAccount: new PublicKey('kh2FSFa3JQn858KwM4NFjJVokQgtEpnqUyrz62QTMN6'),
  serumPcVaultAccount: new PublicKey('JBnZwmn4V8wVHLWzHGeiKZ41icPf6d64cdfHBBaba1w3'),
  serumVaultSigner: new PublicKey('6tooYdTzzBm6dPqwDz29addE2YHzvDVDcAt7CbgXkxCN'),
};

const bkswapInfo = {
  bkswapAccount: 'H9RzbX3Zrf8C1SUmCoaqmH7ZAatSRZvcZfSdVoeCv9wd',
  authority: 'ETM3cd7uogsUUNwFyfGr5Qd31YjEbGdtueanVq7nFqaM',
  bkswapProgramId: '3Hy9MBHowHUmhmSP1KahEvvtC8e6DxzLGeFjuiBQwRUA',
  feeReceiver: '4yPafXXCHDkF8VQvM7F3oA3XBJ8fHc3SsAJyxchRFEaK',
  feeRate: 30
};

process.env.ANCHOR_WALLET = '/Users/liangjunren/.config/solana/id.json';
process.env.ANCHOR_PROVIDER_URL = 'https://api.mainnet-beta.solana.com';

describe('bkswap', () => {
  const provider = anchor.AnchorProvider.env();
  provider.opts.skipPreflight = true;
  
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  // const program = anchor.workspace.RaydiumTestHelper as Program<RaydiumTestHelper>;
  const programRaydiumRouter = anchor.workspace.RaydiumRouter as Program<RaydiumRouter>;
  
  it('swap test!', async () => {
    let conn = provider.connection;
    let owner = provider.wallet.publicKey;
    
    // let alreadCreatedMarket = false;
    //
    // let multipleInfo = await getMultipleAccountsInfo(conn, [new PublicKey(serumMarketId)]);
    // console.log(`multipleInfo:`, multipleInfo);
    // if (multipleInfo.length > 0 && multipleInfo[0] !== null) {
    //   if (multipleInfo[0]?.data.length !== 0) {
    //     alreadCreatedMarket = true;
    //   }
    // }
    // console.log('alreadCreatedMarket:', alreadCreatedMarket);
    // if (!alreadCreatedMarket) {
    //   console.log(`provider:`, provider);
    //   const {tokenAMintAddress, tokenBMintAddress} = await createMintPair(provider);
    //   // create serum market
    //   const createMarketInfo = await createSerumMarket({
    //     connection: provider.connection,
    //     wallet: provider.wallet as anchor.Wallet,
    //     baseMint: tokenAMintAddress,
    //     quoteMint: tokenBMintAddress,
    //     baseLotSize: 1,
    //     quoteLotSize: 1,
    //     dexProgram: marketInfo.serumDexProgram,
    //     market: marketInfo.serumMarket,
    //   });
    //   console.log(JSON.stringify(createMarketInfo));
    //   // wait for transaction success
    //   // await sleep(10000);
    // }
    //
    // // get serum market info
    // market = await getMarket(conn, serumMarketId, marketInfo.serumDexProgram.toString());
    // console.log('serum market info:', JSON.stringify(market));
    // poolKeys = await getAssociatedPoolKeys({
    //   programId: marketInfo.ammProgram,
    //   serumProgramId: marketInfo.serumDexProgram,
    //   marketId: market.address,
    //   baseMint: market.baseMint,
    //   quoteMint: market.quoteMint
    // });
    // console.log('amm poolKeys: ', JSON.stringify(poolKeys));
    //
    // const ammAuthority = poolKeys.authority;
    // const nonce = new anchor.BN(poolKeys.nonce);
    // const ammId: PublicKey = poolKeys.id;
    // const poolCoinTokenAccount: PublicKey = poolKeys.baseVault;
    // const poolPcTokenAccount: PublicKey = poolKeys.quoteVault;
    // const lpMintAddress: PublicKey = poolKeys.lpMint;
    // const poolTempLpTokenAccount: PublicKey = poolKeys.lpVault;
    // const ammTargetOrders: PublicKey = poolKeys.targetOrders;
    // const poolWithdrawQueue: PublicKey = poolKeys.withdrawQueue;
    // const ammOpenOrders: PublicKey = poolKeys.openOrders;
    //
    // let alreadPreInitialized = false;
    // multipleInfo = await getMultipleAccountsInfo(conn, [lpMintAddress]);
    // if (multipleInfo.length > 0 && multipleInfo[0] !== null) {
    //   const tempLpMint = SPL_MINT_LAYOUT.decode(multipleInfo[0]?.data);
    //   if (getBigNumber(tempLpMint.supply) !== 0) {
    //     alreadPreInitialized = true;
    //   }
    // }
    // console.log('alreadPreInitialized: ', alreadPreInitialized);
    // if (!alreadPreInitialized) {
    //   /************************************ preInitialize test ***********************************************************************/
    //   tx = await program.methods.proxyPreInitialize(nonce.toNumber()).accounts({
    //     ammProgram: marketInfo.ammProgram,
    //     ammTargetOrders: ammTargetOrders,
    //     poolWithdrawQueue: poolWithdrawQueue,
    //     ammAuthority: ammAuthority,
    //     lpMint: lpMintAddress,
    //     coinMint: market.baseMintAddress,
    //     pcMint: market.quoteMintAddress,
    //     poolCoinTokenAccount: poolCoinTokenAccount,
    //     poolPcTokenAccount: poolPcTokenAccount,
    //     poolTempLpTokenAccount: poolTempLpTokenAccount,
    //     serumMarket: market.address,
    //     userWallet: owner,
    //     splTokenProgram: TOKEN_PROGRAM_ID,
    //     systemProgram: SystemProgram.programId,
    //     rent: SYSVAR_RENT_PUBKEY,
    //   }).rpc()
    //   console.log('preinitialize tx: ', tx);
    //   // await sleep(10000);
    // }
    //
    // let alreadInitialized = false;
    // multipleInfo = await getMultipleAccountsInfo(conn, [ammId]);
    // if (multipleInfo.length > 0 && multipleInfo[0] !== null) {
    //   if (multipleInfo[0]?.data.length !== 0) {
    //     alreadInitialized = true;
    //   }
    // }
    // console.log('alreadInitialized:', alreadInitialized);
    // if (!alreadInitialized) {
    //   /************************************ initialize test ***********************************************************************/
    //
    //     // set as you want
    //   const userInputBaseValue = 1;
    //   const userInputQuoteValue = 2;
    //
    //   await initAmm(
    //     conn,
    //     provider,
    //     market,
    //     userInputBaseValue,
    //     userInputQuoteValue,
    //     poolCoinTokenAccount,
    //     poolPcTokenAccount,
    //     lpMintAddress,
    //   );
    //
    //   // belongs to owner who create the pool
    //   const userLpTokenAccountPubKey = await Spl.getAssociatedTokenAccount({mint: lpMintAddress, owner: owner});
    //   tx = await program.rpc.proxyInitialize(nonce, new anchor.BN(0), {
    //     accounts: {
    //       ammProgram: marketInfo.ammProgram,
    //       amm: ammId,
    //       ammAuthority: ammAuthority,
    //       ammOpenOrders: ammOpenOrders,
    //       lpMint: lpMintAddress,
    //       coinMint: market.baseMintAddress,
    //       pcMint: market.quoteMintAddress,
    //       poolCoinTokenAccount: poolCoinTokenAccount,
    //       poolPcTokenAccount: poolPcTokenAccount,
    //       poolWithdrawQueue: poolWithdrawQueue,
    //       poolTargetOrdersAccount: ammTargetOrders,
    //       poolLpTokenAccount: userLpTokenAccountPubKey,
    //       poolTempLpTokenAccount: poolTempLpTokenAccount,
    //       serumProgram: marketInfo.serumDexProgram,
    //       serumMarket: serumMarketId,
    //       userWallet: owner,
    //       splTokenProgram: TOKEN_PROGRAM_ID,
    //       systemProgram: SystemProgram.programId,
    //       rent: SYSVAR_RENT_PUBKEY,
    //     }
    //   });
    //   console.log('initialize tx: ', tx);
    //   // await sleep(10000);
    // }
    //
    // /************************************ deposit test ***********************************************************************/
    //
    // const transaction = new Transaction();
    // userCoinTokenAccount = await createAssociatedTokenAccountIfNotExist(
    //   provider.wallet.publicKey,
    //   market.baseMint,
    //   transaction,
    //   provider.connection
    // );
    //
    // userPcTokenAccount = await createAssociatedTokenAccountIfNotExist(
    //   provider.wallet.publicKey,
    //   market.quoteMint,
    //   transaction,
    //   provider.connection
    // );
    //
    // const userLPTokenAccount: PublicKey = await createAssociatedTokenAccountIfNotExist(
    //   provider.wallet.publicKey,
    //   poolKeys.lpMint,
    //   transaction,
    //   provider.connection
    // );
    //
    // if (transaction.instructions.length > 0) {
    //   let txid = provider.sendAndConfirm(transaction, null, {
    //     skipPreflight: true,
    //     preflightCommitment: 'confirmed'
    //   });
    //   console.log('create user lp token account txid:', txid);
    //   // checkTxid(provider.connection, txid)
    // }
    //
    
    const market = await getMarket(conn, ctxAccounts.serumMarket.toString(), marketInfo.serumDexProgram.toString());
    
    const transaction = new Transaction();
    
    let userCoinTokenAccount = await createAssociatedTokenAccountIfNotExist(
      provider.wallet.publicKey,
      market.baseMint,
      transaction,
      provider.connection
    );
    
    let userPcTokenAccount = await createAssociatedTokenAccountIfNotExist(
      provider.wallet.publicKey,
      market.quoteMint,
      transaction,
      provider.connection
    );
    
    let feeReceiver = new PublicKey(bkswapInfo.feeReceiver);
    let feeToTokenAccount = await createAssociatedTokenAccountIfNotExistWithPayer(
      feeReceiver,
      provider.wallet.publicKey,
      market.baseMint,
      transaction,
      provider.connection
    );
    
    if (transaction.instructions.length > 0) {
      let txResult = await provider.sendAndConfirm(transaction, null, {
        skipPreflight: true,
        preflightCommitment: 'confirmed'
      });
      console.log('create user token account txid:', txResult);
      // await sleep(3000);
    }
    console.log(`programRaydiumRouter:`, programRaydiumRouter);
    let tx = await programRaydiumRouter.rpc.proxySwapBaseIn(
      new anchor.BN(10000), // amountIn
      new anchor.BN(0), // amountOut
      {
        accounts: {
          ...ctxAccounts,
          
          userSourceTokenAccount: userCoinTokenAccount,
          userDestinationTokenAccount: userPcTokenAccount,
          userSourceOwner: provider.wallet.publicKey,
          splTokenProgram: TOKEN_PROGRAM_ID,
          
          feeToTokenAccount: feeToTokenAccount,
          bkswapProgram: new PublicKey(bkswapInfo.bkswapProgramId),
          bkswapAccount: new PublicKey(bkswapInfo.bkswapAccount)
        }
      }
    );
    
    console.log('swap_base_in tx: ', tx);
    
  });
  
});

export async function createAssociatedTokenAccountIfNotExistWithPayer(
  owner: PublicKey,
  payer: PublicKey,
  mint: PublicKey,
  transaction: Transaction,
  conn: any,
) {
  const associatedAccount = await Spl.getAssociatedTokenAccount({mint, owner});
  const associatedAccountInfo = await conn.getAccountInfo(associatedAccount);
  if (!associatedAccountInfo) {
    transaction.add(Spl.makeCreateAssociatedTokenAccountInstruction({
        mint,
        associatedAccount,
        owner,
        payer,
        instructionsType: []
      })
    );
  }
  return associatedAccount;
}
