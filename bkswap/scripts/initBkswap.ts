import * as anchor from "@project-serum/anchor";
import {Program} from "@project-serum/anchor";
import {Keypair, PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, Transaction} from '@solana/web3.js';
import {Bkswap} from "../target/types/bkswap";

process.env.ANCHOR_WALLET = '/Users/liangjunren/.config/solana/id.json'
process.env.ANCHOR_PROVIDER_URL = 'https://api.mainnet-beta.solana.com'

async function main() {
  const provider = anchor.AnchorProvider.env();
  provider.opts.skipPreflight = true;
  
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  // const program = anchor.workspace.RaydiumTestHelper as Program<RaydiumTestHelper>;
  const programBkswap = anchor.workspace.Bkswap as Program<Bkswap>;
  let signers=[]
  let bkswapAccount = Keypair.generate();
  signers.push(bkswapAccount)
  console.log(`bkswap_account:`, bkswapAccount, bkswapAccount.secretKey);
  let tx = await programBkswap.rpc.initialize(
    provider.wallet.publicKey,
    new PublicKey('4yPafXXCHDkF8VQvM7F3oA3XBJ8fHc3SsAJyxchRFEaK'),
    30,
    {
      accounts: {
        bkswapAccount: bkswapAccount.publicKey,
        funder: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId
      },
      signers:[bkswapAccount]
    });
  
  console.log('init tx: ', tx);
}

main()
