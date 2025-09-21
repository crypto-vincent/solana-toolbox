import { PublicKey } from '@solana/web3.js';
import { ToolboxEndpoint } from '../src/ToolboxEndpoint';
import { ToolboxIdlService } from '../src/ToolboxIdlService';

it('run', async () => {
  // Create the endpoint
  const endpoint = new ToolboxEndpoint('devnet', 'confirmed');
  // The devnet program we'll lookup
  const programId = new PublicKey(
    'Ee5CDFHQmdUQMEnM3dJZMiLaBuP2Wr8WBVYM7UZPPb6E',
  );
  // Important account addresses
  const realmPda = PublicKey.findProgramAddressSync(
    [Buffer.from('realm')],
    programId,
  );
  const realm = realmPda[0];
  const realmBump = realmPda[1];
  const uctMintPda = PublicKey.findProgramAddressSync(
    [Buffer.from('uct_mint'), realm.toBuffer()],
    programId,
  );
  const uctMint = uctMintPda[0];
  const uctMintBump = uctMintPda[1];
  // Actually fetch our account using the auto-resolved IDL on-chain
  const realmDecoded =
    await new ToolboxIdlService().getAndInferAndDecodeAccount(endpoint, realm);
  // Check that the account was parsed properly and values matches
  expect(realmDecoded.program.metadata.name).toStrictEqual('redemption');
  expect(realmDecoded.account.name).toStrictEqual('Realm');
  expect(realmDecoded.state['bump']).toStrictEqual(realmBump);
  expect(realmDecoded.state['usdc_mint']).toStrictEqual(
    'H7JmSvR6w6Qrp9wEbw4xGEBkbh95Jc9C4yXYYYvWmF8B',
  );
  expect(realmDecoded.state['uct_mint_bump']).toStrictEqual(uctMintBump);
  expect(realmDecoded.state['uct_mint']).toStrictEqual(uctMint.toBase58());
});
