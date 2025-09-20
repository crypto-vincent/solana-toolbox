import { ToolboxIdlTypedef } from './ToolboxIdlTypedef';
import { ToolboxUtils } from './ToolboxUtils';
import { parse } from './ToolboxIdlTypeFlat.parse';
import { hydrate } from './ToolboxIdlTypeFlat.hydrate';
import { serialize } from './ToolboxIdlTypeFull.serialize';
import { ToolboxIdlTypeFull } from './ToolboxIdlTypeFull';

export type ToolboxIdlInstructionBlobConst = {
  value: Buffer;
};
export type ToolboxIdlInstructionBlobArg = {
  path: string;
  typeFull: ToolboxIdlTypeFull | undefined;
};
export type ToolboxIdlInstructionBlobAccount = {
  path: string;
  typeFull: ToolboxIdlTypeFull | undefined;
};

type ToolboxIdlInstructionBlobDiscriminant = 'const' | 'arg' | 'account';
type ToolboxIdlInstructionBlobContent =
  | ToolboxIdlInstructionBlobConst
  | ToolboxIdlInstructionBlobArg
  | ToolboxIdlInstructionBlobAccount;

export class ToolboxIdlInstructionBlob {
  private discriminant: ToolboxIdlInstructionBlobDiscriminant;
  private content: ToolboxIdlInstructionBlobContent;

  private constructor(
    discriminant: ToolboxIdlInstructionBlobDiscriminant,
    content: ToolboxIdlInstructionBlobContent,
  ) {
    this.discriminant = discriminant;
    this.content = content;
  }

  public static const(
    value: ToolboxIdlInstructionBlobConst,
  ): ToolboxIdlInstructionBlob {
    return new ToolboxIdlInstructionBlob('const', value);
  }
  public static arg(
    value: ToolboxIdlInstructionBlobArg,
  ): ToolboxIdlInstructionBlob {
    return new ToolboxIdlInstructionBlob('arg', value);
  }
  public static account(
    value: ToolboxIdlInstructionBlobAccount,
  ): ToolboxIdlInstructionBlob {
    return new ToolboxIdlInstructionBlob('account', value);
  }

  public traverse<P1, P2, T>(
    visitor: {
      const: (value: ToolboxIdlInstructionBlobConst, p1: P1, p2: P2) => T;
      arg: (value: ToolboxIdlInstructionBlobArg, p1: P1, p2: P2) => T;
      account: (value: ToolboxIdlInstructionBlobAccount, p1: P1, p2: P2) => T;
    },
    p1: P1,
    p2: P2,
  ): T {
    return visitor[this.discriminant](this.content as any, p1, p2);
  }

  public static tryParse(
    idlInstructionBlob: any,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlInstructionBlob {
    if (ToolboxUtils.isObject(idlInstructionBlob)) {
      if (idlInstructionBlob.hasOwnProperty('value')) {
        return this.tryParseConst(
          idlInstructionBlob['value'],
          idlInstructionBlob['type'] ?? 'bytes',
          typedefs,
        );
      }
      const idlInstructionBlobPath = ToolboxUtils.expectString(
        idlInstructionBlob['path'],
      );
      const idlInstructionBlobType = idlInstructionBlob['type'];
      if (idlInstructionBlob.hasOwnProperty('arg')) {
        return ToolboxIdlInstructionBlob.tryParseArg(
          idlInstructionBlobPath,
          idlInstructionBlobType,
          typedefs,
        );
      }
      return ToolboxIdlInstructionBlob.tryParseAccount(
        idlInstructionBlobPath,
        idlInstructionBlobType,
        typedefs,
      );
    }
    if (ToolboxUtils.isArray(idlInstructionBlob)) {
      return this.tryParseConst(idlInstructionBlob, 'bytes', typedefs);
    }
    if (ToolboxUtils.isString(idlInstructionBlob)) {
      return this.tryParseConst(idlInstructionBlob, 'string', typedefs);
    }
    throw new Error('Could not parse IDL instruction account PDA blob');
  }

  static tryParseConst(
    idlInstructionBlobValue: any,
    idlInstructionBlobType: any,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlInstructionBlob {
    let typeFlat = parse(idlInstructionBlobType);
    let typeFull = hydrate(typeFlat, new Map(), typedefs);
    let data: Buffer[] = [];
    serialize(typeFull, idlInstructionBlobValue, data, true);
    return ToolboxIdlInstructionBlob.const({
      value: Buffer.concat(data),
    });
  }

  static tryParseArg(
    idlInstructionBlobPath: string,
    idlInstructionBlobType: any,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlInstructionBlob {
    if (idlInstructionBlobType === undefined) {
      return this.arg({
        path: idlInstructionBlobPath,
        typeFull: undefined,
      });
    }
    let typeFlat = parse(idlInstructionBlobType);
    let typeFull = hydrate(typeFlat, new Map(), typedefs);
    return this.arg({
      path: idlInstructionBlobPath,
      typeFull,
    });
  }

  static tryParseAccount(
    idlInstructionBlobPath: string,
    idlInstructionBlobType: any,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlInstructionBlob {
    if (idlInstructionBlobType === undefined) {
      return this.account({
        path: idlInstructionBlobPath,
        typeFull: undefined,
      });
    }
    let typeFlat = parse(idlInstructionBlobType);
    let typeFull = hydrate(typeFlat, new Map(), typedefs);
    return this.account({
      path: idlInstructionBlobPath,
      typeFull,
    });
  }
}
