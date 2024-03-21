import * as anchor from "@project-serum/anchor";
import {Program} from "@project-serum/anchor";
import {Keypair, PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, Transaction} from '@solana/web3.js';
import {Bkswap} from "../target/types/bkswap";

process.env.ANCHOR_WALLET = '/Users/liangjunren/.config/solana/id.json'
process.env.ANCHOR_PROVIDER_URL = 'https://api.mainnet-beta.solana.com'
const bkswapInfo = {
  bkswapAccount: 'H9RzbX3Zrf8C1SUmCoaqmH7ZAatSRZvcZfSdVoeCv9wd',
  authority: 'ETM3cd7uogsUUNwFyfGr5Qd31YjEbGdtueanVq7nFqaM',
  bkswapProgramId: '3Hy9MBHowHUmhmSP1KahEvvtC8e6DxzLGeFjuiBQwRUA',
  feeReceiver: '4yPafXXCHDkF8VQvM7F3oA3XBJ8fHc3SsAJyxchRFEaK',
  feeRate: 30
};
async function main() {
  const provider = anchor.AnchorProvider.env();
  provider.opts.skipPreflight = true;
  
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  // const program = anchor.workspace.RaydiumTestHelper as Program<RaydiumTestHelper>;
  const programBkswap = anchor.workspace.Bkswap as Program<Bkswap>;
  let tx
   tx = await programBkswap.rpc.setIsPaused(
    false,
    {
      accounts: {
        bkswapAccount: bkswapInfo.bkswapAccount,
        authority: provider.wallet.publicKey,
      },
    });
   // tx = await programBkswap.rpc.setFeeRate(
   //  100,
   //  {
   //    accounts: {
   //      bkswapAccount: bkswapInfo.bkswapAccount,
   //      authority: provider.wallet.publicKey,
   //    },
   //  });
  console.log('init tx: ', tx);
}

main()
