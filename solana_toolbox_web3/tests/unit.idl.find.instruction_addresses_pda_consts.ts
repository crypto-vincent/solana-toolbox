import { PublicKey } from '@solana/web3.js';
import { ToolboxIdlProgram } from '../src';
import { findInstructionAddresses } from '../src/ToolboxIdlInstruction.find';

it('run', () => {
  // Keys used during the test
  let programId1 = PublicKey.unique();
  let programId2 = PublicKey.unique();
  // Create IDLs on the fly
  let idlProgram1 = ToolboxIdlProgram.tryParse({
    instructions: {
      my_ix: {
        discriminator: [77, 78],
        accounts: [
          {
            name: 'const_bytes_without_program',
            pda: {
              seeds: [
                { kind: 'const', type: ['u8'], value: [41, 0, 0, 0] },
                { kind: 'const', value: [42, 0, 0, 0] },
              ],
            },
          },
          {
            name: 'const_bytes_with_program',
            pda: {
              seeds: [
                { kind: 'const', type: ['u8'], value: [41, 0, 0, 0] },
                { kind: 'const', value: [42, 0, 0, 0] },
              ],
              program: {
                kind: 'const',
                value: [...programId2.toBytes()],
              },
            },
          },
          {
            name: 'const_string_without_program',
            pda: {
              seeds: [
                { kind: 'const', type: 'string', value: 'hello' },
                { kind: 'const', value: 'world' },
              ],
            },
          },
          {
            name: 'const_string_with_program',
            pda: {
              seeds: [
                { kind: 'const', type: 'string', value: 'hello' },
                { kind: 'const', value: 'world' },
              ],
              program: {
                kind: 'const',
                value: [...programId2.toBytes()],
              },
            },
          },
        ],
      },
    },
  });
  let idlProgram2 = ToolboxIdlProgram.tryParse({
    instructions: {
      my_ix: {
        discriminator: [77, 78],
        accounts: [
          {
            name: 'const_bytes_without_program',
            pda: {
              seeds: [
                [41, 0, 0, 0],
                [42, 0, 0, 0],
              ],
            },
          },
          {
            name: 'const_bytes_with_program',
            pda: {
              seeds: [
                [41, 0, 0, 0],
                [42, 0, 0, 0],
              ],
              program: { value: [...programId2.toBytes()] },
            },
          },
          {
            name: 'const_string_without_program',
            pda: {
              seeds: ['hello', 'world'],
            },
          },
          {
            name: 'const_string_with_program',
            pda: {
              seeds: ['hello', 'world'],
              program: { value: [...programId2.toBytes()] },
            },
          },
        ],
      },
    },
  });
  // Make sure the IDLs are equivalent
  expect(idlProgram1).toStrictEqual(idlProgram2);
  // Pdas based off of const bytes seeds
  let pdaSeedsConstBytes = [
    Uint8Array.from([41, 0, 0, 0]),
    Uint8Array.from([42, 0, 0, 0]),
  ];
  let pdaConstBytes1 = PublicKey.findProgramAddressSync(
    pdaSeedsConstBytes,
    programId1,
  )[0];
  let pdaConstBytes2 = PublicKey.findProgramAddressSync(
    pdaSeedsConstBytes,
    programId2,
  )[0];
  // Pdas based off of const string seeds
  let pdaSeedsConstString = [
    new TextEncoder().encode('hello'),
    new TextEncoder().encode('world'),
  ];
  let pdaConstString1 = PublicKey.findProgramAddressSync(
    pdaSeedsConstString,
    programId1,
  )[0];
  let pdaConstString2 = PublicKey.findProgramAddressSync(
    pdaSeedsConstString,
    programId2,
  )[0];
  // Assert that the accounts can be properly resolved
  let instructionAddresses = findInstructionAddresses(
    idlProgram1.instructions.get('my_ix')!,
    programId1,
    null,
    new Map(),
  );
  expect(instructionAddresses.get('const_bytes_without_program')).toStrictEqual(
    pdaConstBytes1,
  );
  expect(instructionAddresses.get('const_bytes_with_program')).toStrictEqual(
    pdaConstBytes2,
  );
  expect(
    instructionAddresses.get('const_string_without_program'),
  ).toStrictEqual(pdaConstString1);
  expect(instructionAddresses.get('const_string_with_program')).toStrictEqual(
    pdaConstString2,
  );
});
