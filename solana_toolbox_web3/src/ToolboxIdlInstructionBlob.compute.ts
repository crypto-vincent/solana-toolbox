import { PublicKey } from '@solana/web3.js';
import {
  ToolboxIdlInstructionBlob,
  ToolboxIdlInstructionBlobAccount,
  ToolboxIdlInstructionBlobArg,
  ToolboxIdlInstructionBlobConst,
} from './ToolboxIdlInstructionBlob';
import {
  ToolboxIdlTypeFullFields,
  ToolboxIdlTypeFull,
} from './ToolboxIdlTypeFull';

export type ToolboxIdlInstructionBlobComputeContext = {
  instructionProgramId: PublicKey;
  instructionPayload: any;
  instructionAddresses: Map<string, PublicKey>;
  instructionAccountsStates: Map<string, any>;
  instructionArgsTypeFullFields: ToolboxIdlTypeFullFields;
  instructionAccountsContentsTypeFull: Map<string, ToolboxIdlTypeFull>;
};

export function computeInstructionBlob(
  blob: ToolboxIdlInstructionBlob,
  context: ToolboxIdlInstructionBlobComputeContext,
): Buffer {
  blob.traverse(computeVisitor, context, undefined);
  // TODO - finish this
  return Buffer.from([]);
}

let computeVisitor = {
  const: (
    self: ToolboxIdlInstructionBlobConst,
    context: ToolboxIdlInstructionBlobComputeContext,
  ) => {
    self.value;
  },
  arg: (
    self: ToolboxIdlInstructionBlobArg,
    context: ToolboxIdlInstructionBlobComputeContext,
  ) => {
    if (!self.typeFull) {
    }
  },
  account: (
    self: ToolboxIdlInstructionBlobAccount,
    context: ToolboxIdlInstructionBlobComputeContext,
  ) => {},
};
