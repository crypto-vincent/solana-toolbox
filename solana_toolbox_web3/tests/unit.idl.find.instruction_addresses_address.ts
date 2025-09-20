import { PublicKey } from '@solana/web3.js';
import { ToolboxIdlProgram } from '../src';
import { findInstructionAddresses } from '../src/ToolboxIdlInstruction.find';

it('run', () => {
  // Keys used during the test
  let dummyAddress = PublicKey.unique();
  // Create an IDL on the fly
  let idlProgram = ToolboxIdlProgram.tryParse({
    instructions: {
      my_ix: {
        discriminator: [77, 78],
        accounts: [
          {
            name: 'const_address',
            address: dummyAddress.toBase58(),
          },
        ],
      },
    },
  });
  // Assert that the accounts can be properly resolved
  let instruction_addresses = findInstructionAddresses(
    idlProgram.instructions.get('my_ix')!,
    PublicKey.unique(),
    null,
    new Map(),
  );
  expect(instruction_addresses.get('const_address')).toStrictEqual(
    dummyAddress,
  );
});
