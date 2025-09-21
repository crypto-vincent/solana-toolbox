import { PublicKey } from '@solana/web3.js';
import { ToolboxIdlProgram } from '../src';
import { findInstructionAddresses } from '../src/ToolboxIdlInstruction.find';

it('run', () => {
  // Create an IDL on the fly
  const idlProgram = ToolboxIdlProgram.tryParse({
    instructions: {
      my_ix: {
        discriminator: [77, 78],
        accounts: [
          {
            name: 'pda',
            pda: {
              seeds: [
                { kind: 'arg', path: 'u8' },
                { kind: 'arg', path: 'u16' },
                { kind: 'arg', path: 'u32' },
                { kind: 'arg', path: 'u64' },
                { kind: 'arg', path: 'array_u8_2' },
                { kind: 'arg', path: 'vec_u8_3' },
                { kind: 'arg', path: 'string' },
                { kind: 'arg', path: 'inner.u8' },
                { kind: 'arg', path: 'inner.u16' },
                { kind: 'arg', path: 'inner.u16', type: 'u8' },
                { kind: 'arg', path: 'inner.u16', type: 'u32' },
              ],
            },
          },
        ],
        args: [
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
  const dummyProgramId = PublicKey.unique();
  const dummySeeds = [
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
  const instructionAddresses = findInstructionAddresses(
    idlProgram.instructions.get('my_ix')!,
    dummyProgramId,
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
    new Map(),
  );
  expect(instructionAddresses.get('pda')).toStrictEqual(dummyPda);
});
