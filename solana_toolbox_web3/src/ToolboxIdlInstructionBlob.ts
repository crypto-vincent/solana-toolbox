import { ToolboxIdlTypedef } from './ToolboxIdlTypedef';
import { ToolboxUtils } from './ToolboxUtils';
import { parse } from './ToolboxIdlTypeFlat.parse';
import { hydrate } from './ToolboxIdlTypeFlat.hydrate';
import { serialize } from './ToolboxIdlTypeFull.serialize';
import {
  ToolboxIdlTypeFull,
  ToolboxIdlTypeFullFields,
} from './ToolboxIdlTypeFull';
import { ToolboxIdlPath } from './ToolboxIdlPath';
import { pathGetTypeFullFields } from './ToolboxIdlPath.type';

export type ToolboxIdlInstructionBlobConst = {
  value: Buffer;
};
export type ToolboxIdlInstructionBlobArg = {
  path: ToolboxIdlPath;
  typeFull: ToolboxIdlTypeFull;
};
export type ToolboxIdlInstructionBlobAccount = {
  path: ToolboxIdlPath;
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
    instructionArgsTypeFullFields: ToolboxIdlTypeFullFields,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlInstructionBlob {
    if (ToolboxUtils.isObject(idlInstructionBlob)) {
      if (idlInstructionBlob.hasOwnProperty('value')) {
        if (idlInstructionBlob.hasOwnProperty('type')) {
          return this.tryParseConstTyped(
            idlInstructionBlob['value'],
            idlInstructionBlob['type'],
            typedefs,
          );
        } else {
          return this.tryParseConstUntyped(
            idlInstructionBlob['value'],
            typedefs,
          );
        }
      }
      const idlInstructionBlobPath = ToolboxUtils.expectString(
        idlInstructionBlob['path'],
      );
      const idlInstructionBlobType = idlInstructionBlob['type'];
      if (idlInstructionBlob['kind'] === 'arg') {
        return ToolboxIdlInstructionBlob.tryParseArg(
          idlInstructionBlobPath,
          idlInstructionBlobType,
          instructionArgsTypeFullFields,
          typedefs,
        );
      }
      return ToolboxIdlInstructionBlob.tryParseAccount(
        idlInstructionBlobPath,
        idlInstructionBlobType,
        typedefs,
      );
    }
    return this.tryParseConstUntyped(idlInstructionBlob, typedefs);
  }

  static tryParseConstUntyped(
    idlInstructionBlobValue: any,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlInstructionBlob {
    if (ToolboxUtils.isString(idlInstructionBlobValue)) {
      return this.tryParseConstTyped(
        idlInstructionBlobValue,
        'string',
        typedefs,
      );
    }
    if (ToolboxUtils.isArray(idlInstructionBlobValue)) {
      return this.tryParseConstTyped(
        idlInstructionBlobValue,
        'bytes',
        typedefs,
      );
    }
    throw new Error('Could not parse IDL instruction account PDA blob');
  }

  static tryParseConstTyped(
    idlInstructionBlobValue: any,
    idlInstructionBlobType: any,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlInstructionBlob {
    const typeFlat = parse(idlInstructionBlobType);
    const typeFull = hydrate(typeFlat, new Map(), typedefs);
    const data: Buffer[] = [];
    serialize(typeFull, idlInstructionBlobValue, data, false);
    return ToolboxIdlInstructionBlob.const({
      value: Buffer.concat(data),
    });
  }

  static tryParseArg(
    idlInstructionBlobPath: string,
    idlInstructionBlobType: any,
    instructionArgsTypeFullFields: ToolboxIdlTypeFullFields,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlInstructionBlob {
    const path = ToolboxIdlPath.tryParse(idlInstructionBlobPath);
    if (idlInstructionBlobType === undefined) {
      const typeFull = pathGetTypeFullFields(
        path,
        instructionArgsTypeFullFields,
      );
      return this.arg({
        path,
        typeFull,
      });
    }
    const typeFlat = parse(idlInstructionBlobType);
    const typeFull = hydrate(typeFlat, new Map(), typedefs);
    return this.arg({
      path,
      typeFull,
    });
  }

  static tryParseAccount(
    idlInstructionBlobPath: string,
    idlInstructionBlobType: any,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlInstructionBlob {
    const path = ToolboxIdlPath.tryParse(idlInstructionBlobPath);
    if (idlInstructionBlobType === undefined) {
      return this.account({
        path,
        typeFull: undefined,
      });
    }
    const typeFlat = parse(idlInstructionBlobType);
    const typeFull = hydrate(typeFlat, new Map(), typedefs);
    return this.account({
      path,
      typeFull,
    });
  }
}
