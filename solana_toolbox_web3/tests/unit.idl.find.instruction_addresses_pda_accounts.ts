import { PublicKey } from '@solana/web3.js';
import { ToolboxIdlProgram } from '../src';
import { findInstructionAddressesWithAccounts } from '../src/ToolboxIdlInstruction.find';

it('run', () => {
  // Create an IDL on the fly
  const idlProgram = ToolboxIdlProgram.tryParse({
    instructions: {
      my_ix: {
        discriminator: [33, 34],
        accounts: [
          { name: 'first' },
          {
            name: 'pda',
            pda: {
              seeds: [
                { kind: 'account', path: 'first' },
                { kind: 'account', path: 'first.u8' },
                { kind: 'account', path: 'first.u16' },
                { kind: 'account', path: 'first.u32' },
                { kind: 'account', path: 'first.u64' },
                { account: 'MyAccount', path: 'first.array_u8_2' },
                { account: 'MyAccount', path: 'first.vec_u8_3' },
                { account: 'MyAccount', path: 'first.string' },
                { account: 'MyAccount', path: 'first.inner.u8' },
                { account: 'MyAccount', path: 'first.inner.u16' },
                { kind: 'account', path: 'first.inner.u16', type: 'u8' },
                { kind: 'account', path: 'first.inner.u16', type: 'u32' },
              ],
            },
          },
        ],
      },
    },
    accounts: {
      MyAccount: {
        fields: [
          { name: 'u8', type: 'u8' },
          { name: 'u16', type: 'u16' },
          { name: 'u32', type: 'u32' },
          { name: 'u64', type: 'u64' },
          { name: 'array_u8_2', type: ['u8', 2] },
          { name: 'vec_u8_3', type: ['u8'] },
          { name: 'string', type: 'string' },
          {
            name: 'inner',
            fields: [
              { name: 'u8', type: 'u8' },
              { name: 'u16', type: 'u16' },
            ],
          },
        ],
      },
    },
  });
  // Keys used during the test
  const dummyFirstAddress = PublicKey.unique();
  const dummyProgramId = PublicKey.unique();
  const dummySeeds = [
    dummyFirstAddress.toBytes(),
    Uint8Array.from([77]),
    Uint8Array.from([78, 0]),
    Uint8Array.from([79, 0, 0, 0]),
    Uint8Array.from([80, 0, 0, 0, 0, 0, 0, 0]),
    Uint8Array.from([11, 12]),
    Uint8Array.from([21, 22, 23]),
    new TextEncoder().encode('hello'),
    Uint8Array.from([111]),
    Uint8Array.from([222, 0]),
    Uint8Array.from([222]),
    Uint8Array.from([222, 0, 0, 0]),
  ];
  const dummyPda = PublicKey.findProgramAddressSync(
    dummySeeds,
    dummyProgramId,
  )[0];
  // Assert that the accounts can be properly resolved
  const idlAccount = idlProgram.accounts.get('MyAccount')!;
  const idlInstruction = idlProgram.instructions.get('my_ix')!;
  const instructionAddresses = findInstructionAddressesWithAccounts(
    idlInstruction,
    dummyProgramId,
    {},
    new Map([['first', dummyFirstAddress]]),
    new Map([
      [
        'first',
        {
          u8: 77,
          u16: 78,
          u32: 79,
          u64: 80,
          array_u8_2: [11, 12],
          vec_u8_3: [21, 22, 23],
          string: 'hello',
          inner: {
            u8: 111,
            u16: 222,
          },
        },
      ],
    ]),
    new Map([['first', idlAccount.contentTypeFull]]),
  );
  expect(instructionAddresses.get('pda')).toStrictEqual(dummyPda);
});
