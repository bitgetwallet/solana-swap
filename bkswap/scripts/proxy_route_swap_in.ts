import * as anchor from '@project-serum/anchor';
import {Program} from '@project-serum/anchor';
import {Keypair, Secp256k1Program, PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY,Transaction} from '@solana/web3.js';
import {RaydiumExtraRouter} from '../target/types/raydium_extra_router';
import { getPayer, createKeypairFromFile } from '../../solana-bridge/scripts/utils'; 
import path from 'path';
import {TOKEN_PROGRAM_ID} from '@solana/spl-token';
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
} from './utils';

process.env.ANCHOR_WALLET = process.env.HOME + '/.config/solana/id.json';
process.env.ANCHOR_PROVIDER_URL = 'https://api.mainnet-beta.solana.com';
//process.env.ANCHOR_PROVIDER_URL = 'https://api.devnet.solana.com';

let payer : Keypair;
let coSigner : Keypair;

async function initializeCosigner(): Promise<void> {
    const SECOND_SIGNER_KEYPAIR_PATH = path.resolve(__dirname, '../keys/wallet-pair-cosigner.json');
    console.log("SECOND_SIGNER_KEYPAIR_PATH is",SECOND_SIGNER_KEYPAIR_PATH);
    coSigner = await createKeypairFromFile(SECOND_SIGNER_KEYPAIR_PATH);
    console.log("====coSigner is ===",coSigner.publicKey.toBase58());
}

const raydium_extra_router = new PublicKey('DGumNmyMhSeuNYsJLWD7VELLoENEenp99pTtDqSR9dUy');

const marketInfo = {
  serumDexProgram: new PublicKey('srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX'),
  ammProgram: new PublicKey('675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8'),
  serumMarket: new PublicKey('2AdaV97p6SfkuMQJdu8DHhBhmJe7oWdvbm52MJfYQmfA'),
};

const bkswapInfo = {
  bkswapAccount: 'BvwpPr6taomKafRUABnx2pXx8Yf4V9SBikPQCtX51Gtk',
  authority: 'ETM3cd7uogsUUNwFyfGr5Qd31YjEbGdtueanVq7nFqaM',
  bkswapProgramId: '2avRnwrjSBU4NAmThSa3nKvzp2E3AwqAfxH8rKrDTRdw',
  feeReceiver: '7nbCmPsrwWpFGf4u4qVt1dF6y2CvCJ8LyTjqvAcAv9FP',// new address
  feeRate: 30
};


// USDC -> RAY
const ctxAccounts = {
  ammProgram: new PublicKey('675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8'),
  // amm: new PublicKey('DVa7Qmb5ct9RCpaU7UTpSaf3GVMYz17vNVU67XpdCRut'),
  route_from_amm:new PublicKey(''),
  route_to_amm:new PublicKey(''),

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


async function main() {
  
  const provider = anchor.AnchorProvider.env();
  provider.opts.skipPreflight = true;

  let conn = provider.connection;
  let owner = provider.wallet.publicKey;
  
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  
  const program = anchor.workspace.RaydiumExtraRouter as Program<RaydiumExtraRouter>;
  console.log("=== program is ===",program.programId);

  payer = await getPayer();
  await initializeCosigner();
  let signers = [];
  signers.push(payer);
  signers.push(coSigner);
  console.log(`payer:`, payer.publicKey.toBase58());
  console.log(`coSigner:`, coSigner.publicKey.toBase58());

  const market = await getMarket(conn, ctxAccounts.serumMarket.toString(), marketInfo.serumDexProgram.toString());
  
  const transaction = new Transaction();

  const poolKeys = await getAssociatedPoolKeys({
    programId: marketInfo.ammProgram,
    serumProgramId: marketInfo.serumDexProgram,
    marketId: market.address,
    baseMint: market.baseMint,
    quoteMint: market.quoteMint
})
console.log("amm poolKeys: ", JSON.stringify(poolKeys))

let userRouteTokenAccount = await createAssociatedTokenAccountIfNotExist(
  raydium_extra_router,
  market.baseMint,
  transaction,
  provider.connection
);

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

  let id:anchor.BN = new anchor.BN(1234);
  let amount:anchor.BN = new anchor.BN(1e4);
  let amount_in:anchor.BN = new anchor.BN(1e5);
  let minimum_amount_out:anchor.BN = new anchor.BN(1e5);
  let tx = await program.methods.proxyRouteSwapIn(amount_in,minimum_amount_out)
    .accounts({
      ...ctxAccounts,

      userSourceTokenAccount: userCoinTokenAccount,
      userRouteTokenAccount: userPcTokenAccount,
      userPdaAccount: userCoinTokenAccount,
      userSourceOwner: provider.wallet.publicKey,

      bkswapAccount: new PublicKey(bkswapInfo.bkswapAccount),
      feeToTokenAccount: feeToTokenAccount,
      bkswapProgram: new PublicKey(bkswapInfo.bkswapProgramId),
      splTokenProgram: TOKEN_PROGRAM_ID,
      

    })
    .signers(signers)
    .rpc();
  
    console.log('send tx: ', tx);
}

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

main().then(
    () => process.exit(),
    err => {
      console.error("===err==",err);
      process.exit(-1);
    },
  );
