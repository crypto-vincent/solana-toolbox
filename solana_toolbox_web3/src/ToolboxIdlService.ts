import {
  AccountInfo,
  PublicKey,
  SystemProgram,
  TransactionInstruction,
} from '@solana/web3.js';
import { ToolboxIdlProgram } from './ToolboxIdlProgram';
import { ToolboxEndpoint } from './ToolboxEndpoint';
import { ToolboxIdlAccount } from './ToolboxIdlAccount';
import { ToolboxIdlInstruction } from './ToolboxIdlInstruction';
import { findInstructionAccount } from './ToolboxIdlInstructionAccount.find';

export class ToolboxIdlService {
  private cachedPrograms: Map<PublicKey, ToolboxIdlProgram | undefined>;

  constructor() {
    this.cachedPrograms = new Map<PublicKey, ToolboxIdlProgram | undefined>();
  }

  public setProgram(
    programId: PublicKey,
    idlProgram: ToolboxIdlProgram | undefined,
  ) {
    this.cachedPrograms.set(programId, idlProgram);
  }

  public async getOrResolveProgram(
    endpoint: ToolboxEndpoint,
    programId: PublicKey,
  ): Promise<ToolboxIdlProgram | undefined> {
    let cachedProgram = this.cachedPrograms.get(programId);
    if (cachedProgram !== undefined) {
      return cachedProgram;
    }
    let resolvedProgram = await ToolboxIdlService.resolveProgram(
      endpoint,
      programId,
    );
    this.cachedPrograms.set(programId, resolvedProgram);
    return resolvedProgram;
  }

  static async resolveProgram(
    endpoint: ToolboxEndpoint,
    programId: PublicKey,
  ): Promise<ToolboxIdlProgram | undefined> {
    let libProgram = ToolboxIdlProgram.fromLib(programId);
    if (libProgram !== undefined) {
      return libProgram;
    }
    let account = await endpoint.getAccount(
      await ToolboxIdlProgram.findAnchorAddress(programId),
    );
    if (account === undefined) {
      return undefined;
    }
    return ToolboxIdlProgram.tryParseFromAccountData(account.data);
  }

  public async getAndInferAndDecodeAccount(
    endpoint: ToolboxEndpoint,
    address: PublicKey,
  ) {
    let account = (await endpoint.getAccount(address)) ?? {
      lamports: 0,
      owner: SystemProgram.programId,
      data: Buffer.from([]),
      executable: false,
    };
    return this.inferAndDecodeAccount(endpoint, account);
  }

  public async inferAndDecodeAccount(
    endpoint: ToolboxEndpoint,
    account: AccountInfo<Buffer>,
  ) {
    let idlProgram =
      (await this.getOrResolveProgram(endpoint, account.owner)) ??
      ToolboxIdlProgram.Unknown;
    let idlAccount =
      idlProgram.guessAccount(account.data) ?? ToolboxIdlAccount.Unknown;
    let accountState = idlAccount.decode(account.data);
    return {
      lamports: account.lamports,
      owner: account.owner,
      program: idlProgram,
      account: idlAccount,
      state: accountState,
      space: account.data.length,
    };
  }

  public async inferAndDecodeInstruction(
    endpoint: ToolboxEndpoint,
    instruction: TransactionInstruction,
  ) {
    let idlProgram =
      (await this.getOrResolveProgram(endpoint, instruction.programId)) ??
      ToolboxIdlProgram.Unknown;
    let idlInstruction =
      idlProgram.guessInstruction(instruction.data) ??
      ToolboxIdlInstruction.Unknown;
    let { instructionProgramId, instructionAddresses, instructionPayload } =
      idlInstruction.decode(instruction);
    return {
      program: idlProgram,
      instruction: idlInstruction,
      instructionProgramId,
      instructionAddresses,
      instructionPayload,
    };
  }

  public async resolveAndEncodeInstruction(
    endpoint: ToolboxEndpoint,
    idlInstruction: ToolboxIdlInstruction,
    instructionProgramId: PublicKey,
    instructionPayload: any,
    instructionAddresses: Map<string, PublicKey>,
  ): Promise<TransactionInstruction> {
    return idlInstruction.encode(
      instructionProgramId,
      instructionPayload,
      await this.resolveInstructionAddresses(
        endpoint,
        idlInstruction,
        instructionProgramId,
        instructionPayload,
        instructionAddresses,
      ),
    );
  }

  public async resolveInstructionAddresses(
    endpoint: ToolboxEndpoint,
    idlInstruction: ToolboxIdlInstruction,
    instructionProgramId: PublicKey,
    instructionPayload: any,
    instructionAddresses: Map<string, PublicKey>,
  ): Promise<Map<string, PublicKey>> {
    instructionAddresses = new Map(instructionAddresses);
    const instructionAccountsStates = new Map();
    const instructionAccountsContentsTypeFull = new Map();
    for (const instructionAccountName in instructionAddresses) {
      const instructionAddress = instructionAddresses.get(
        instructionAccountName,
      );
      if (!instructionAddress) {
        continue;
      }
      let accountInfo = await this.getAndInferAndDecodeAccount(
        endpoint,
        instructionAddress,
      );
      instructionAccountsStates.set(instructionAccountName, accountInfo.state);
      instructionAccountsContentsTypeFull.set(
        instructionAccountName,
        accountInfo.account.contentTypeFull,
      );
    }
    while (true) {
      let madeProgress = false;
      for (const idlInstructionAccount of idlInstruction.accounts) {
        if (instructionAddresses.has(idlInstructionAccount.name)) {
          continue;
        }
        try {
          const instructionAddress = findInstructionAccount(
            idlInstructionAccount,
            instructionProgramId,
            instructionPayload,
            instructionAddresses,
            instructionAccountsStates,
            instructionAccountsContentsTypeFull,
          );
          madeProgress = true;
          instructionAddresses.set(
            idlInstructionAccount.name,
            instructionAddress,
          );
          let accountInfo = await this.getAndInferAndDecodeAccount(
            endpoint,
            instructionAddress,
          );
          instructionAccountsStates.set(
            idlInstructionAccount.name,
            accountInfo.state,
          );
          instructionAccountsContentsTypeFull.set(
            idlInstructionAccount.name,
            accountInfo.account.contentTypeFull,
          );
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
}
