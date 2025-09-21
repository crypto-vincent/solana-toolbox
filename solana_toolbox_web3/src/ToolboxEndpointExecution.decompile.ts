import { PublicKey, TransactionInstruction } from '@solana/web3.js';

export function decompileTransactionPayerAddress(
  staticAddresses: Array<PublicKey>,
) {
  const payerAddress = staticAddresses[0];
  if (payerAddress === undefined) {
    throw new Error('No static addresses provided');
  }
  return payerAddress;
}

export function decompileTransactionInstructions(
  headerNumRequiredSignatures: number,
  headerNumReadonlySignedAccounts: number,
  headerNumReadonlyUnsignedAccounts: number,
  staticAddresses: Array<PublicKey>,
  loadedWritableAddresses: Array<PublicKey>,
  loadedReadonlyAddresses: Array<PublicKey>,
  compiledInstructions: {
    programIdIndex: number;
    accountsIndexes: Array<number>;
    data: Buffer;
  }[],
) {
  const signerAddresses = decompileTransactionSignerAddresses(
    headerNumRequiredSignatures,
    staticAddresses,
  );
  const readonlyAddresses = decompiledTransactionStaticReadonlyAddresses(
    headerNumRequiredSignatures,
    headerNumReadonlySignedAccounts,
    headerNumReadonlyUnsignedAccounts,
    staticAddresses,
  );
  for (const loadedReadonlyAddress of loadedReadonlyAddresses) {
    readonlyAddresses.add(loadedReadonlyAddress);
  }
  const usedAddresses = [];
  usedAddresses.push(...staticAddresses);
  usedAddresses.push(...loadedWritableAddresses);
  usedAddresses.push(...loadedReadonlyAddresses);
  const instructions = [];
  for (const compiledInstruction of compiledInstructions) {
    const instructionProgramId =
      usedAddresses[compiledInstruction.programIdIndex];
    if (instructionProgramId === undefined) {
      throw new Error(
        `Invalid program ID index: ${compiledInstruction.programIdIndex}`,
      );
    }
    const instructionAccounts = [];
    for (const accountIndex of compiledInstruction.accountsIndexes) {
      const accountAddress = usedAddresses[accountIndex];
      if (accountAddress === undefined) {
        throw new Error(`Invalid account index: ${accountIndex}`);
      }
      const accountIsSigner = signerAddresses.has(accountAddress);
      const accountIsReadonly = readonlyAddresses.has(accountAddress);
      instructionAccounts.push({
        pubkey: accountAddress,
        isSigner: accountIsSigner,
        isWritable: !accountIsReadonly,
      });
    }
    instructions.push(
      new TransactionInstruction({
        programId: instructionProgramId,
        keys: instructionAccounts,
        data: compiledInstruction.data,
      }),
    );
  }
  return instructions;
}

function decompileTransactionSignerAddresses(
  headerNumRequiredSignatures: number,
  staticAddresses: Array<PublicKey>,
): Set<PublicKey> {
  const signerAddresses = new Set<PublicKey>();
  for (let index = 0; index < headerNumRequiredSignatures; index++) {
    signerAddresses.add(staticAddresses[index]!);
  }
  return signerAddresses;
}

function decompiledTransactionStaticReadonlyAddresses(
  headerNumRequiredSignatures: number,
  headerNumReadonlySignedAccounts: number,
  headerNumReadonlyUnsignedAccounts: number,
  staticAddresses: Array<PublicKey>,
): Set<PublicKey> {
  const readonlyAddresses = new Set<PublicKey>();
  for (
    let index = headerNumRequiredSignatures - headerNumReadonlySignedAccounts;
    index < headerNumRequiredSignatures;
    index++
  ) {
    readonlyAddresses.add(staticAddresses[index]!);
  }
  for (
    let index = staticAddresses.length - headerNumReadonlyUnsignedAccounts;
    index < staticAddresses.length;
    index++
  ) {
    readonlyAddresses.add(staticAddresses[index]!);
  }
  return readonlyAddresses;
}
