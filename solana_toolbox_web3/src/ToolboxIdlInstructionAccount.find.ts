import { PublicKey } from '@solana/web3.js';
import {
  ToolboxIdlInstructionAccount,
  ToolboxIdlInstructionAccountPda,
} from './ToolboxIdlInstructionAccount';
import { ToolboxIdlTypeFull } from './ToolboxIdlTypeFull';
import { computeInstructionBlob } from './ToolboxIdlInstructionBlob.compute';

export function findInstructionAccount(
  idlInstructionAccount: ToolboxIdlInstructionAccount,
  instructionProgramId: PublicKey,
  instructionPayload: any,
  instructionAddresses: Map<string, PublicKey>,
  instructionAccountsStates: Map<string, any>,
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
  instructionAccountsContentsTypeFull: Map<string, ToolboxIdlTypeFull>,
): PublicKey {
  const computeContext = {
    instructionProgramId,
    instructionPayload,
    instructionAddresses,
    instructionAccountsStates,
    instructionAccountsContentsTypeFull,
  };
  let pdaSeedsBytes = [];
  for (const blob of instructionAccountPda.seeds) {
    pdaSeedsBytes.push(computeInstructionBlob(blob, computeContext));
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
