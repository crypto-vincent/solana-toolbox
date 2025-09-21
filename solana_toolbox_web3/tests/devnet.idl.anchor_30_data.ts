import { PublicKey } from '@solana/web3.js';
import { ToolboxEndpoint } from '../src/ToolboxEndpoint';
import { ToolboxIdlService } from '../src/ToolboxIdlService';

it('run', async () => {
  // Create the endpoint
  const endpoint = new ToolboxEndpoint('devnet', 'confirmed');
  // Find an account we can read from the endpoint
  const campaignIndexNumber = 0n;
  const campaignIndexBuffer = Buffer.alloc(8);
  campaignIndexBuffer.writeBigInt64LE(campaignIndexNumber);
  const campaignPda = PublicKey.findProgramAddressSync(
    [Buffer.from('Campaign'), campaignIndexBuffer],
    new PublicKey('UCNcQRtrbGmvuLKA3Jv719Cc6DS4r661ZRpyZduxu2j'),
  );
  const campaign = campaignPda[0];
  const campaignBump = campaignPda[1];
  // Read an account using the IDL directly auto-downloaded from the chain
  const campaignDecoded =
    await new ToolboxIdlService().getAndInferAndDecodeAccount(
      endpoint,
      campaign,
    );
  // Check that the account was parsed properly and values matches
  expect(campaignDecoded.program.metadata.name).toStrictEqual(
    'psyche_crowd_funding',
  );
  expect(campaignDecoded.account.name).toStrictEqual('Campaign');
  expect(campaignDecoded.state['bump']).toStrictEqual(campaignBump);
  expect(campaignDecoded.state['index']).toStrictEqual(
    campaignIndexNumber.toString(),
  );
  expect(campaignDecoded.state['authority']).toStrictEqual(
    'Ady55LhZxWFABzdg8NCNTAZv5XstBqyNZYCMfWqW3Rq9',
  );
  expect(campaignDecoded.state['collateral_mint']).toStrictEqual(
    'EsQycjp856vTPvrxMuH1L6ymd5K63xT7aULGepiTcgM3',
  );
  expect(campaignDecoded.state['redeemable_mint']).toStrictEqual(
    '3dtmuqjKdL12ptVmDPjAXeYJE9nLgA74ti1Gm2ME9qH9',
  );
});
