import { PublicKey } from '@solana/web3.js';
import { ToolboxIdlAccount } from './ToolboxIdlAccount';
import { ToolboxIdlTypedef } from './ToolboxIdlTypedef';
import { ToolboxUtils } from './ToolboxUtils';
import {
  ToolboxIdlTypeFlat,
  ToolboxIdlTypeFlatFields,
} from './ToolboxIdlTypeFlat';
import { parse } from './ToolboxIdlTypeFlat.parse';
import { hydrate } from './ToolboxIdlTypeFlat.hydrate';
import { serialize } from './ToolboxIdlTypeFull.serialize';

export type ToolboxIdlInstructionAccountPdaBlobDiscriminant =
  | 'const'
  | 'arg'
  | 'account';

export type ToolboxIdlInstructionAccountPdaBlobConst = {
  value: Buffer;
};

export type ToolboxIdlInstructionAccountPdaBlobArg = {
  path: string;
  typeFlat: ToolboxIdlTypeFlat;
  typeFull: ToolboxIdlTypeFlat;
};

export type ToolboxIdlInstructionAccountPdaBlobAccount = {
  path: string;
};

export type ToolboxIdlInstructionAccountPdaBlobContent =
  | ToolboxIdlInstructionAccountPdaBlobConst
  | ToolboxIdlInstructionAccountPdaBlobArg
  | ToolboxIdlInstructionAccountPdaBlobAccount;

export class ToolboxIdlInstructionAccountPdaBlob {
  private discriminant: ToolboxIdlInstructionAccountPdaBlobDiscriminant;
  private content: ToolboxIdlInstructionAccountPdaBlobContent;

  private constructor(
    discriminant: ToolboxIdlInstructionAccountPdaBlobDiscriminant,
    content: ToolboxIdlInstructionAccountPdaBlobContent,
  ) {
    this.discriminant = discriminant;
    this.content = content;
  }

  public static const(
    value: ToolboxIdlInstructionAccountPdaBlobConst,
  ): ToolboxIdlInstructionAccountPdaBlob {
    return new ToolboxIdlInstructionAccountPdaBlob('const', value);
  }
  public static arg(
    value: ToolboxIdlInstructionAccountPdaBlobArg,
  ): ToolboxIdlInstructionAccountPdaBlob {
    return new ToolboxIdlInstructionAccountPdaBlob('arg', value);
  }
  public static account(
    value: ToolboxIdlInstructionAccountPdaBlobAccount,
  ): ToolboxIdlInstructionAccountPdaBlob {
    return new ToolboxIdlInstructionAccountPdaBlob('account', value);
  }

  public traverse<P1, P2, T>(
    visitor: {
      const: (
        value: ToolboxIdlInstructionAccountPdaBlobConst,
        param1: P1,
        param2: P2,
      ) => T;
      arg: (
        value: ToolboxIdlInstructionAccountPdaBlobArg,
        param1: P1,
        param2: P2,
      ) => T;
      account: (
        value: ToolboxIdlInstructionAccountPdaBlobAccount,
        param1: P1,
        param2: P2,
      ) => T;
    },
    param1: P1,
    param2: P2,
  ): T {
    return visitor[this.discriminant](this.content as any, param1, param2);
  }

  public static tryParse(
    idlInstructionAccountPdaBlob: any,
    argsTypeFlatFields: ToolboxIdlTypeFlatFields,
    accounts: Map<string, ToolboxIdlAccount>,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlInstructionAccountPdaBlob {
    if (ToolboxUtils.isObject(idlInstructionAccountPdaBlob)) {
      if (idlInstructionAccountPdaBlob.hasOwnProperty('value')) {
        return this.tryParseTypeValue(
          idlInstructionAccountPdaBlob['type'],
          idlInstructionAccountPdaBlob['value'],
          typedefs,
        );
      }
      if (idlInstructionAccountPdaBlob.hasOwnProperty('path')) {
        let path = ToolboxUtils.expectString(
          idlInstructionAccountPdaBlob['path'],
        );
        if (idlInstructionAccountPdaBlob['kind'] === 'arg') {
          return this.arg({ path });
        }
        return this.account({ path });
      }
    }
    if (ToolboxUtils.isArray(idlInstructionAccountPdaBlob)) {
      return this.tryParseTypeValue(
        'bytes',
        idlInstructionAccountPdaBlob,
        typedefs,
      );
    }
    if (ToolboxUtils.isString(idlInstructionAccountPdaBlob)) {
      return this.tryParseTypeValue(
        'string',
        idlInstructionAccountPdaBlob,
        typedefs,
      );
    }
    throw new Error('Could not parse IDL instruction account PDA blob');
  }

  static tryParseTypeValue(
    idlInstructionAccountPdaBlobType: any,
    idlInstructionAccountPdaBlobValue: any,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlInstructionAccountPdaBlob {
    let typeFlat = parse(idlInstructionAccountPdaBlobType);
    let typeFull = hydrate(typeFlat, new Map(), typedefs);
    let data: Buffer[] = [];
    serialize(typeFull, idlInstructionAccountPdaBlobValue, data, true);
    return ToolboxIdlInstructionAccountPdaBlob.const({
      value: Buffer.concat(data),
    });
  }

  static tryParseArgPath(
    idlInstructionAccountPdaBlobPath: string,
    argsTypeFlatFields: ToolboxIdlTypeFlatFields,
  ): ToolboxIdlInstructionAccountPdaBlob {}
}
