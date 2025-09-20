import { PublicKey } from '@solana/web3.js';
import { ToolboxIdlInstruction } from './ToolboxIdlInstruction';
import { ToolboxIdlTypeFull } from './ToolboxIdlTypeFull';
import { findInstructionAccount } from './ToolboxIdlInstructionAccount.find';

export function findInstructionAddresses(
  idlInstruction: ToolboxIdlInstruction,
  instructionProgramId: PublicKey,
  instructionPayload: any,
  instructionAddresses: Map<string, PublicKey>,
): Map<string, PublicKey> {
  return findInstructionAddressesWithAccounts(
    idlInstruction,
    instructionProgramId,
    instructionPayload,
    instructionAddresses,
    new Map(),
    new Map(),
  );
}

export function findInstructionAddressesWithAccounts(
  idlInstruction: ToolboxIdlInstruction,
  instructionProgramId: PublicKey,
  instructionPayload: any,
  instructionAddresses: Map<string, PublicKey>,
  instructionAccountsStates: Map<string, any>,
  instructionAccountsContentsTypeFull: Map<string, ToolboxIdlTypeFull>,
): Map<string, PublicKey> {
  instructionAddresses = new Map<string, PublicKey>(instructionAddresses);
  while (true) {
    let madeProgress = false;
    for (let idlInstructionAccount of idlInstruction.accounts) {
      if (instructionAddresses.has(idlInstructionAccount.name)) {
        continue;
      }
      try {
        let instructionAddress = findInstructionAccount(
          idlInstructionAccount,
          instructionProgramId,
          instructionPayload,
          instructionAddresses,
          instructionAccountsStates,
          instructionAccountsContentsTypeFull,
        );
        instructionAddresses.set(
          idlInstructionAccount.name,
          instructionAddress,
        );
        madeProgress = true;
      } catch (_error) {
        // Ignore errors, we might not have enough info yet
      }
    }
    if (!madeProgress) {
      break;
    }
  }
  return instructionAddresses;
}
