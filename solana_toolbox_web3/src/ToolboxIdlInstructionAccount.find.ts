import { PublicKey } from '@solana/web3.js';
import {
  ToolboxIdlInstructionAccount,
  ToolboxIdlInstructionAccountPda,
} from './ToolboxIdlInstructionAccount';
import {
  ToolboxIdlTypeFull,
  ToolboxIdlTypeFullFields,
} from './ToolboxIdlTypeFull';
import { computeInstructionBlob } from './ToolboxIdlInstructionBlob.compute';

export function findInstructionAccount(
  idlInstructionAccount: ToolboxIdlInstructionAccount,
  instructionProgramId: PublicKey,
  instructionPayload: any,
  instructionAddresses: Map<string, PublicKey>,
  instructionAccountsStates: Map<string, any>,
  instructionArgsTypeFullFields: ToolboxIdlTypeFullFields,
  instructionAccountsContentsTypeFull: Map<string, ToolboxIdlTypeFull>,
): PublicKey {
  let address = instructionAddresses.get(idlInstructionAccount.name);
  if (address) {
    return address;
  }
  if (idlInstructionAccount.address) {
    return idlInstructionAccount.address;
  }
  if (idlInstructionAccount.pda) {
    return findInstructionAccountPda(
      idlInstructionAccount.pda,
      instructionProgramId,
      instructionPayload,
      instructionAddresses,
      instructionAccountsStates,
      instructionArgsTypeFullFields,
      instructionAccountsContentsTypeFull,
    );
  }
  throw new Error(
    `Could not find account automatically: ${idlInstructionAccount.name} (unresolvable)`,
  );
}

export function findInstructionAccountPda(
  instructionAccountPda: ToolboxIdlInstructionAccountPda,
  instructionProgramId: PublicKey,
  instructionPayload: any,
  instructionAddresses: Map<string, PublicKey>,
  instructionAccountsStates: Map<string, any>,
  instructionArgsTypeFullFields: ToolboxIdlTypeFullFields,
  instructionAccountsContentsTypeFull: Map<string, ToolboxIdlTypeFull>,
): PublicKey {
  const computeContext = {
    instructionProgramId,
    instructionPayload,
    instructionAddresses,
    instructionAccountsStates,
    instructionArgsTypeFullFields,
    instructionAccountsContentsTypeFull,
  };
  let pdaSeedsBytes = new Array();
  for (let blob of instructionAccountPda.seeds) {
    let seedBuffer = computeInstructionBlob(blob, computeContext);
    pdaSeedsBytes.push(seedBuffer);
  }
  let pdaProgramId = instructionProgramId;
  if (instructionAccountPda.program) {
    let programBuffer = computeInstructionBlob(
      instructionAccountPda.program,
      computeContext,
    );
    if (programBuffer.length !== 32) {
      throw new Error('PDA program ID seed did not resolve to 32 bytes');
    }
    pdaProgramId = new PublicKey(programBuffer);
  }
  let [pda, _] = PublicKey.findProgramAddressSync(pdaSeedsBytes, pdaProgramId);
  return pda;
}
