import { PublicKey } from '@solana/web3.js';
import { ToolboxIdlAccount } from './ToolboxIdlAccount';
import { ToolboxIdlTypedef } from './ToolboxIdlTypedef';
import { ToolboxUtils } from './ToolboxUtils';
import { ToolboxIdlInstructionAccountPdaBlob } from './ToolboxIdlInstructionAccountPdaBlob';

export type ToolboxIdlInstructionAccountPda = {
  seeds: ToolboxIdlInstructionAccountPdaBlob[];
  program: ToolboxIdlInstructionAccountPdaBlob;
};

export class ToolboxIdlInstructionAccount {
  public readonly name: string;
  public readonly docs: any;
  public readonly writable: boolean;
  public readonly signer: boolean;
  public readonly optional: boolean;
  public readonly address: PublicKey | undefined;
  public readonly pda: ToolboxIdlInstructionAccountPda | undefined;

  constructor(value: {
    name: string;
    docs: any;
    writable: boolean;
    signer: boolean;
    optional: boolean;
    address?: PublicKey;
    pda?: ToolboxIdlInstructionAccountPda;
  }) {
    this.name = value.name;
    this.docs = value.docs;
    this.writable = value.writable;
    this.signer = value.signer;
    this.optional = value.optional;
    this.address = value.address;
    this.pda = value.pda;
  }

  public static tryParse(
    idlInstructionAccount: any,
    accounts: Map<string, ToolboxIdlAccount>,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlInstructionAccount {
    ToolboxUtils.expectObject(idlInstructionAccount);
    let name = ToolboxUtils.convertToSnakeCase(
      ToolboxUtils.expectString(idlInstructionAccount['name']),
    );
    let docs = idlInstructionAccount['docs'];
    let writable = ToolboxUtils.expectBoolean(
      idlInstructionAccount['writable'] ??
        idlInstructionAccount['isMut'] ??
        false,
    );
    let signer = ToolboxUtils.expectBoolean(
      idlInstructionAccount['signer'] ??
        idlInstructionAccount['isSigner'] ??
        false,
    );
    let optional = ToolboxUtils.expectBoolean(
      idlInstructionAccount['optional'] ??
        idlInstructionAccount['isOptional'] ??
        false,
    );
    let address = undefined;
    if (idlInstructionAccount['address']) {
      address = new PublicKey(
        ToolboxUtils.expectString(idlInstructionAccount['address']),
      );
    }
    return new ToolboxIdlInstructionAccount({
      name,
      docs,
      writable,
      signer,
      optional,
      address,
    });
  }
}
