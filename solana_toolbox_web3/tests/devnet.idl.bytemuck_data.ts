import { PublicKey } from '@solana/web3.js';
import { ToolboxEndpoint } from '../src/ToolboxEndpoint';
import { ToolboxIdlService } from '../src/ToolboxIdlService';

it('run', async () => {
  // Create the endpoint
  const endpoint = new ToolboxEndpoint('devnet', 'confirmed');
  // Actually fetch our account using the auto-resolved IDL on-chain
  const address = new PublicKey('FdoXZqdMysWbzB8j5bK6U5J1Dczsos1vGwQi5Tur2mwk');
  const decoded = await new ToolboxIdlService().getAndInferAndDecodeAccount(
    endpoint,
    address,
  );
  // Check that the account was parsed properly and values matches
  expect(decoded.state['state']['metadata']['vocab_size']).toStrictEqual(
    129280n.toString(),
  );
  expect(
    decoded.state['state']['coordinator']['config']['min_clients'],
  ).toStrictEqual(24);
});
