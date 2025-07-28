import { PublicKey } from '@solana/web3.js';
import { ToolboxIdlTypedef } from './ToolboxIdlTypedef';
import { ToolboxIdlAccount } from './ToolboxIdlAccount';
import { ToolboxIdlInstruction } from './ToolboxIdlInstruction';
import { ToolboxUtils } from './ToolboxUtils';
import { ToolboxIdlError } from './ToolboxIdlError';
import { inflate } from 'pako';
import { ToolboxIdlEvent } from './ToolboxIdlEvent';

// TODO - this should be move to a separate folder
import libNativeAddressLookupTable from '../../solana_toolbox_idl/src/lib/native_address_lookup_table.json';
import libNativeBpfLoader2 from '../../solana_toolbox_idl/src/lib/native_bpf_loader_2.json';
import libNativeBpfLoaderUpgradeable from '../../solana_toolbox_idl/src/lib/native_bpf_loader_upgradeable.json';
import libNativeComputeBudget from '../../solana_toolbox_idl/src/lib/native_compute_budget.json';
import libNativeLoader from '../../solana_toolbox_idl/src/lib/native_loader.json';
import libNativeSystem from '../../solana_toolbox_idl/src/lib/native_system.json';
import libSplToken from '../../solana_toolbox_idl/src/lib/spl_token.json';
import libSplNameService from '../../solana_toolbox_idl/src/lib/spl_name_service.json';
import libSplAssociatedToken from '../../solana_toolbox_idl/src/lib/spl_associated_token.json';
import libMiscLighthouse from '../../solana_toolbox_idl/src/lib/misc_lighthouse.json';

let knownIdls = new Map<string, any>([
  ['11111111111111111111111111111111', libNativeSystem],
  ['AddressLookupTab1e1111111111111111111111111', libNativeAddressLookupTable],
  ['ComputeBudget111111111111111111111111111111', libNativeComputeBudget],
  ['NativeLoader1111111111111111111111111111111', libNativeLoader],
  ['BPFLoader2111111111111111111111111111111111', libNativeBpfLoader2],
  [
    'BPFLoaderUpgradeab1e11111111111111111111111',
    libNativeBpfLoaderUpgradeable,
  ],
  ['TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA', libSplToken],
  ['ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL', libSplAssociatedToken],
  ['namesLPneVptA9Z5rqUDD9tMTWEJwofgaYwp8cawRkX', libSplNameService],
  ['L2TExMFKdjpN9kozasaurPirfHy9P8sbXoAN1qA3S95', libMiscLighthouse],
]);

export type ToolboxIdlProgramMetadata = {
  name?: string;
  docs?: any;
  description?: string;
  address?: PublicKey;
  version?: string;
  spec?: string;
};

export class ToolboxIdlProgram {
  public static readonly DISCRIMINATOR = Buffer.from([
    0x18, 0x46, 0x62, 0xbf, 0x3a, 0x90, 0x7b, 0x9e,
  ]);

  public static readonly Unknown = new ToolboxIdlProgram({
    metadata: {},
    typedefs: new Map(),
    accounts: new Map(),
    instructions: new Map(),
    events: new Map(),
    errors: new Map(),
  });

  public readonly metadata: ToolboxIdlProgramMetadata;
  public readonly typedefs: Map<string, ToolboxIdlTypedef>;
  public readonly accounts: Map<string, ToolboxIdlAccount>;
  public readonly instructions: Map<string, ToolboxIdlInstruction>;
  public readonly events: Map<string, ToolboxIdlEvent>;
  public readonly errors: Map<string, ToolboxIdlError>;

  constructor(value: {
    metadata: ToolboxIdlProgramMetadata;
    typedefs: Map<string, ToolboxIdlTypedef>;
    accounts: Map<string, ToolboxIdlAccount>;
    instructions: Map<string, ToolboxIdlInstruction>;
    events: Map<string, ToolboxIdlEvent>;
    errors: Map<string, ToolboxIdlError>;
  }) {
    this.metadata = value.metadata;
    this.typedefs = value.typedefs;
    this.accounts = value.accounts;
    this.instructions = value.instructions;
    this.events = value.events;
    this.errors = value.errors;
  }

  public static async findAnchorAddress(
    programId: PublicKey,
  ): Promise<PublicKey> {
    let base = PublicKey.findProgramAddressSync([], programId)[0];
    return await PublicKey.createWithSeed(base, 'anchor:idl', programId);
  }

  public static fromLib(programId: PublicKey): ToolboxIdlProgram | undefined {
    let knownIdl = knownIdls.get(programId.toBase58());
    if (knownIdl === undefined) {
      return undefined;
    }
    return ToolboxIdlProgram.tryParse(knownIdl);
  }

  public static tryParseFromAccountData(
    accountData: Buffer,
  ): ToolboxIdlProgram {
    let discriminator = accountData.subarray(0, 8);
    if (!discriminator.equals(ToolboxIdlProgram.DISCRIMINATOR)) {
      throw new Error('Invalid IDL program discriminator');
    }
    let contentLength = accountData.readUInt32LE(40);
    let contentRaw = accountData.subarray(44, 44 + contentLength);
    let contentEncoded = inflate(contentRaw);
    let contentDecoded = new TextDecoder('utf8').decode(contentEncoded);
    return ToolboxIdlProgram.tryParseFromString(contentDecoded);
  }

  public static tryParseFromString(idlString: string): ToolboxIdlProgram {
    let idlRoot = JSON.parse(idlString);
    return ToolboxIdlProgram.tryParse(idlRoot);
  }

  public static tryParse(idlRoot: any): ToolboxIdlProgram {
    let metadata = {
      ...ToolboxIdlProgram.tryParseMetadata(idlRoot),
      ...ToolboxIdlProgram.tryParseMetadata(idlRoot['metadata']),
    };
    let typedefs = ToolboxIdlProgram.tryParseScopedNamedValues(
      idlRoot,
      'types',
      false,
      undefined,
      undefined,
      ToolboxIdlTypedef.tryParse,
    );
    let accounts = ToolboxIdlProgram.tryParseScopedNamedValues(
      idlRoot,
      'accounts',
      false,
      typedefs,
      undefined,
      ToolboxIdlAccount.tryParse,
    );
    let instructions = ToolboxIdlProgram.tryParseScopedNamedValues(
      idlRoot,
      'instructions',
      true,
      accounts,
      typedefs,
      ToolboxIdlInstruction.tryParse,
    );
    let events = ToolboxIdlProgram.tryParseScopedNamedValues(
      idlRoot,
      'events',
      false,
      typedefs,
      undefined,
      ToolboxIdlEvent.tryParse,
    );
    let errors = ToolboxIdlProgram.tryParseScopedNamedValues(
      idlRoot,
      'errors',
      false,
      undefined,
      undefined,
      ToolboxIdlError.tryParse,
    );
    return new ToolboxIdlProgram({
      metadata,
      typedefs,
      accounts,
      instructions,
      events,
      errors,
    });
  }

  static tryParseMetadata(idlMetadata: any): ToolboxIdlProgramMetadata {
    if (!idlMetadata) {
      return {};
    }
    let rawName = idlMetadata['name'];
    let rawDocs = idlMetadata['docs'];
    let rawDescription = idlMetadata['description'];
    let rawAddress = idlMetadata['address'];
    let rawVersion = idlMetadata['version'];
    let rawSpec = idlMetadata['spec'];
    return {
      name: rawName ? ToolboxUtils.expectString(rawName) : undefined,
      docs: rawDocs,
      description: rawDescription
        ? ToolboxUtils.expectString(rawDescription)
        : undefined,
      address: rawAddress
        ? new PublicKey(ToolboxUtils.expectString(rawAddress))
        : undefined,
      version: rawVersion ? ToolboxUtils.expectString(rawVersion) : undefined,
      spec: rawSpec ? ToolboxUtils.expectString(rawSpec) : undefined,
    };
  }

  static tryParseScopedNamedValues<T, P1, P2>(
    idlRoot: any,
    collectionKey: string,
    convertNameToSnakeCase: boolean,
    param1: P1,
    param2: P2,
    parsingFunction: (name: string, value: any, param1: P1, param2: P2) => T,
  ): Map<string, T> {
    let values = new Map();
    let collection = idlRoot[collectionKey];
    if (ToolboxUtils.isArray(collection)) {
      for (let item of collection) {
        let name = ToolboxUtils.expectString(item['name']);
        if (convertNameToSnakeCase) {
          name = ToolboxUtils.convertToSnakeCase(name);
        }
        values.set(name, parsingFunction(name, item, param1, param2));
      }
    }
    if (ToolboxUtils.isObject(collection)) {
      Object.entries(collection).forEach(([key, value]) => {
        if (convertNameToSnakeCase) {
          key = ToolboxUtils.convertToSnakeCase(key);
        }
        values.set(key, parsingFunction(key, value, param1, param2));
      });
    }
    return values;
  }

  public guessAccount(accountData: Buffer): ToolboxIdlAccount | undefined {
    for (let account of this.accounts.values()) {
      try {
        account.check(accountData);
        return account;
      } catch {}
    }
    return undefined;
  }

  public guessInstruction(
    instructionData: Buffer,
  ): ToolboxIdlInstruction | undefined {
    for (let instruction of this.instructions.values()) {
      try {
        instruction.checkPayload(instructionData);
        return instruction;
      } catch {}
    }
    return undefined;
  }

  public guessEvent(eventData: Buffer): ToolboxIdlEvent | undefined {
    for (let event of this.events.values()) {
      try {
        event.check(eventData);
        return event;
      } catch {}
    }
    return undefined;
  }

  public guessError(errorCode: number): ToolboxIdlError | undefined {
    for (let error of this.errors.values()) {
      if (error.code === errorCode) {
        return error;
      }
    }
    return undefined;
  }
}
