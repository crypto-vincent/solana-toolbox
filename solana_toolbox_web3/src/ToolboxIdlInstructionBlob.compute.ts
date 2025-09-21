import { PublicKey } from '@solana/web3.js';
import {
  ToolboxIdlInstructionBlob,
  ToolboxIdlInstructionBlobAccount,
  ToolboxIdlInstructionBlobArg,
  ToolboxIdlInstructionBlobConst,
} from './ToolboxIdlInstructionBlob';
import { ToolboxIdlTypeFull } from './ToolboxIdlTypeFull';
import { serialize } from './ToolboxIdlTypeFull.serialize';
import { pathGetJsonValue } from './ToolboxIdlPath.json';
import { pathGetTypeFull } from './ToolboxIdlPath.type';

export type ToolboxIdlInstructionBlobComputeContext = {
  instructionProgramId: PublicKey;
  instructionPayload: any;
  instructionAddresses: Map<string, PublicKey>;
  instructionAccountsStates: Map<string, any>;
  instructionAccountsContentsTypeFull: Map<string, ToolboxIdlTypeFull>;
};

export function computeInstructionBlob(
  blob: ToolboxIdlInstructionBlob,
  context: ToolboxIdlInstructionBlobComputeContext,
): Buffer {
  return blob.traverse(computeVisitor, context, undefined);
}

const computeVisitor = {
  const: (
    self: ToolboxIdlInstructionBlobConst,
    _context: ToolboxIdlInstructionBlobComputeContext,
  ) => {
    return self.value;
  },
  arg: (
    self: ToolboxIdlInstructionBlobArg,
    context: ToolboxIdlInstructionBlobComputeContext,
  ) => {
    const value = pathGetJsonValue(self.path, context.instructionPayload);
    const data = new Array<Buffer>();
    serialize(self.typeFull, value, data, false);
    return Buffer.concat(data);
  },
  account: (
    self: ToolboxIdlInstructionBlobAccount,
    context: ToolboxIdlInstructionBlobComputeContext,
  ) => {
    if (self.path.isEmpty()) {
      throw new Error(
        'PDA Blob account path is empty (should have at least the account name)',
      );
    }
    const { first: current, rest: next } = self.path.splitFirst()!;
    const instructionAccountName = current.key();
    if (!instructionAccountName) {
      throw new Error(
        'PDA Blob account path first part should be an account name',
      );
    }
    const instructionAccountContentPath = next;
    if (instructionAccountContentPath.isEmpty()) {
      const instructionAddress = context.instructionAddresses.get(
        instructionAccountName,
      )!;
      if (!instructionAddress) {
        throw new Error(
          `Could not find address for account: ${instructionAccountName}`,
        );
      }
      return instructionAddress.toBuffer();
    }
    const instructionAccountContentState =
      context.instructionAccountsStates.get(instructionAccountName);
    if (instructionAccountContentState === undefined) {
      throw new Error(
        `Could not find state for account: ${instructionAccountName}`,
      );
    }
    const value = pathGetJsonValue(
      instructionAccountContentPath,
      instructionAccountContentState,
    );
    const data = new Array<Buffer>();
    if (self.typeFull !== undefined) {
      serialize(self.typeFull, value, data, false);
      return Buffer.concat(data);
    }
    const instructionAccountContentTypeFull =
      context.instructionAccountsContentsTypeFull.get(instructionAccountName);
    if (instructionAccountContentTypeFull === undefined) {
      throw new Error(
        `Could not find content type for account: ${instructionAccountName}`,
      );
    }
    const typeFull = pathGetTypeFull(
      instructionAccountContentPath,
      instructionAccountContentTypeFull,
    );
    serialize(typeFull, value, data, false);
    return Buffer.concat(data);
  },
};
