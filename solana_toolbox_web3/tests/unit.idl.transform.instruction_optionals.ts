import { PublicKey } from '@solana/web3.js';
import { ToolboxIdlProgram } from '../src/ToolboxIdlProgram';

it('run', () => {
  // Create an IDL on the fly
  const idlProgram = ToolboxIdlProgram.tryParse({
    instructions: {
      my_ix: {
        accounts: [
          { name: 'acc_0' },
          { name: 'acc_1_1' },
          { name: 'acc_2_1', optional: true },
          { name: 'acc_3_1', optional: true },
          { name: 'acc_4_2' },
          { name: 'acc_5_3' },
          { name: 'acc_6_3', optional: true },
          { name: 'acc_7_3', optional: true },
        ],
      },
    },
  });
  // Choose the instruction
  const idlInstruction = idlProgram.instructions.get('my_ix')!;
  // Use dummy accounts
  const acc_0 = PublicKey.unique();
  const acc_1_1 = PublicKey.unique();
  const acc_2_1 = PublicKey.unique();
  const acc_3_1 = PublicKey.unique();
  const acc_4_2 = PublicKey.unique();
  const acc_5_3 = PublicKey.unique();
  const acc_6_3 = PublicKey.unique();
  const acc_7_3 = PublicKey.unique();
  // Check that we we can encode the instruction with none of the optional accounts
  const caseEmptyAddresses = new Map([
    ['acc_0', acc_0],
    ['acc_1_1', acc_1_1],
    ['acc_4_2', acc_4_2],
    ['acc_5_3', acc_5_3],
  ]);
  const caseEmptyMetas = [
    { pubkey: acc_0, isWritable: false, isSigner: false },
    { pubkey: acc_1_1, isWritable: false, isSigner: false },
    { pubkey: acc_4_2, isWritable: false, isSigner: false },
    { pubkey: acc_5_3, isWritable: false, isSigner: false },
  ];
  expect(idlInstruction.encodeAddresses(caseEmptyAddresses)).toStrictEqual(
    caseEmptyMetas,
  );
  expect(idlInstruction.decodeAddresses(caseEmptyMetas)).toStrictEqual(
    caseEmptyAddresses,
  );
  // Check that we we can encode the instruction with all of the optional accounts
  const caseFullAddresses = new Map([
    ['acc_0', acc_0],
    ['acc_1_1', acc_1_1],
    ['acc_2_1', acc_2_1],
    ['acc_3_1', acc_3_1],
    ['acc_4_2', acc_4_2],
    ['acc_5_3', acc_5_3],
    ['acc_6_3', acc_6_3],
    ['acc_7_3', acc_7_3],
  ]);
  const caseFullMetas = [
    { pubkey: acc_0, isWritable: false, isSigner: false },
    { pubkey: acc_1_1, isWritable: false, isSigner: false },
    { pubkey: acc_2_1, isWritable: false, isSigner: false },
    { pubkey: acc_3_1, isWritable: false, isSigner: false },
    { pubkey: acc_4_2, isWritable: false, isSigner: false },
    { pubkey: acc_5_3, isWritable: false, isSigner: false },
    { pubkey: acc_6_3, isWritable: false, isSigner: false },
    { pubkey: acc_7_3, isWritable: false, isSigner: false },
  ];
  expect(idlInstruction.encodeAddresses(caseFullAddresses)).toStrictEqual(
    caseFullMetas,
  );
  expect(idlInstruction.decodeAddresses(caseFullMetas)).toStrictEqual(
    caseFullAddresses,
  );
  // Check that we we can encode the instruction with all of the optional accounts
  const casePartial1Addresses = new Map([
    ['acc_0', acc_0],
    ['acc_1_1', acc_1_1],
    ['acc_2_1', acc_2_1],
    ['acc_4_2', acc_4_2],
    ['acc_5_3', acc_5_3],
  ]);
  const casePartial1Metas = [
    { pubkey: acc_0, isWritable: false, isSigner: false },
    { pubkey: acc_1_1, isWritable: false, isSigner: false },
    { pubkey: acc_2_1, isWritable: false, isSigner: false },
    { pubkey: acc_4_2, isWritable: false, isSigner: false },
    { pubkey: acc_5_3, isWritable: false, isSigner: false },
  ];
  expect(idlInstruction.encodeAddresses(casePartial1Addresses)).toStrictEqual(
    casePartial1Metas,
  );
  expect(idlInstruction.decodeAddresses(casePartial1Metas)).toStrictEqual(
    casePartial1Addresses,
  );
  // Check that we we can encode the instruction with all of the optional accounts
  const casePartial3Addresses = new Map([
    ['acc_0', acc_0],
    ['acc_1_1', acc_1_1],
    ['acc_2_1', acc_2_1],
    ['acc_3_1', acc_3_1],
    ['acc_4_2', acc_4_2],
    ['acc_5_3', acc_5_3],
    ['acc_6_3', acc_6_3],
  ]);
  const casePartial3Metas = [
    { pubkey: acc_0, isWritable: false, isSigner: false },
    { pubkey: acc_1_1, isWritable: false, isSigner: false },
    { pubkey: acc_2_1, isWritable: false, isSigner: false },
    { pubkey: acc_3_1, isWritable: false, isSigner: false },
    { pubkey: acc_4_2, isWritable: false, isSigner: false },
    { pubkey: acc_5_3, isWritable: false, isSigner: false },
    { pubkey: acc_6_3, isWritable: false, isSigner: false },
  ];
  expect(idlInstruction.encodeAddresses(casePartial3Addresses)).toStrictEqual(
    casePartial3Metas,
  );
  expect(idlInstruction.decodeAddresses(casePartial3Metas)).toStrictEqual(
    casePartial3Addresses,
  );
});
