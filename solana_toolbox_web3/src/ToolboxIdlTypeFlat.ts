import { ToolboxIdlTypePrefix } from './ToolboxIdlTypePrefix';
import { ToolboxIdlTypePrimitive } from './ToolboxIdlTypePrimitive';

export type ToolboxIdlTypeFlatDefined = {
  name: string;
  generics: ToolboxIdlTypeFlat[];
};
export type ToolboxIdlTypeFlatGeneric = {
  symbol: string;
};
export type ToolboxIdlTypeFlatOption = {
  prefix: ToolboxIdlTypePrefix;
  content: ToolboxIdlTypeFlat;
};
export type ToolboxIdlTypeFlatVec = {
  prefix: ToolboxIdlTypePrefix;
  items: ToolboxIdlTypeFlat;
};
export type ToolboxIdlTypeFlatArray = {
  items: ToolboxIdlTypeFlat;
  length: ToolboxIdlTypeFlat;
};
export type ToolboxIdlTypeFlatString = {
  prefix: ToolboxIdlTypePrefix;
};
export type ToolboxIdlTypeFlatStruct = {
  fields: ToolboxIdlTypeFlatFields;
};
export type ToolboxIdlTypeFlatEnum = {
  prefix: ToolboxIdlTypePrefix;
  variants: ToolboxIdlTypeFlatEnumVariant[];
};
export type ToolboxIdlTypeFlatPadded = {
  before: number;
  minSize: number;
  after: number;
  content: ToolboxIdlTypeFlat;
};
export type ToolboxIdlTypeFlatConst = {
  literal: number;
};

type ToolboxIdlTypeFlatDiscriminant =
  | 'defined'
  | 'generic'
  | 'option'
  | 'vec'
  | 'array'
  | 'string'
  | 'struct'
  | 'enum'
  | 'padded'
  | 'const'
  | 'primitive';
type ToolboxIdlTypeFlatContent =
  | ToolboxIdlTypeFlatDefined
  | ToolboxIdlTypeFlatGeneric
  | ToolboxIdlTypeFlatOption
  | ToolboxIdlTypeFlatVec
  | ToolboxIdlTypeFlatArray
  | ToolboxIdlTypeFlatString
  | ToolboxIdlTypeFlatStruct
  | ToolboxIdlTypeFlatEnum
  | ToolboxIdlTypeFlatPadded
  | ToolboxIdlTypeFlatConst
  | ToolboxIdlTypePrimitive;

export class ToolboxIdlTypeFlat {
  private discriminant: ToolboxIdlTypeFlatDiscriminant;
  private content: ToolboxIdlTypeFlatContent;

  private constructor(
    discriminant: ToolboxIdlTypeFlatDiscriminant,
    content: ToolboxIdlTypeFlatContent,
  ) {
    this.discriminant = discriminant;
    this.content = content;
  }

  public static defined(value: ToolboxIdlTypeFlatDefined): ToolboxIdlTypeFlat {
    return new ToolboxIdlTypeFlat('defined', value);
  }
  public static generic(value: ToolboxIdlTypeFlatGeneric): ToolboxIdlTypeFlat {
    return new ToolboxIdlTypeFlat('generic', value);
  }
  public static option(value: ToolboxIdlTypeFlatOption): ToolboxIdlTypeFlat {
    return new ToolboxIdlTypeFlat('option', value);
  }
  public static vec(value: ToolboxIdlTypeFlatVec): ToolboxIdlTypeFlat {
    return new ToolboxIdlTypeFlat('vec', value);
  }
  public static array(value: ToolboxIdlTypeFlatArray): ToolboxIdlTypeFlat {
    return new ToolboxIdlTypeFlat('array', value);
  }
  public static string(value: ToolboxIdlTypeFlatString): ToolboxIdlTypeFlat {
    return new ToolboxIdlTypeFlat('string', value);
  }
  public static struct(value: ToolboxIdlTypeFlatStruct): ToolboxIdlTypeFlat {
    return new ToolboxIdlTypeFlat('struct', value);
  }
  public static enum(value: ToolboxIdlTypeFlatEnum): ToolboxIdlTypeFlat {
    return new ToolboxIdlTypeFlat('enum', value);
  }
  public static padded(value: ToolboxIdlTypeFlatPadded): ToolboxIdlTypeFlat {
    return new ToolboxIdlTypeFlat('padded', value);
  }
  public static const(value: ToolboxIdlTypeFlatConst): ToolboxIdlTypeFlat {
    return new ToolboxIdlTypeFlat('const', value);
  }
  public static primitive(value: ToolboxIdlTypePrimitive): ToolboxIdlTypeFlat {
    return new ToolboxIdlTypeFlat('primitive', value);
  }

  public static structNothing(): ToolboxIdlTypeFlat {
    return new ToolboxIdlTypeFlat('struct', {
      fields: ToolboxIdlTypeFlatFields.nothing(),
    });
  }

  public traverse<P1, P2, T>(
    visitor: {
      defined: (value: ToolboxIdlTypeFlatDefined, p1: P1, p2: P2) => T;
      generic: (value: ToolboxIdlTypeFlatGeneric, p1: P1, p2: P2) => T;
      option: (value: ToolboxIdlTypeFlatOption, p1: P1, p2: P2) => T;
      vec: (value: ToolboxIdlTypeFlatVec, p1: P1, p2: P2) => T;
      array: (value: ToolboxIdlTypeFlatArray, p1: P1, p2: P2) => T;
      string: (value: ToolboxIdlTypeFlatString, p1: P1, p2: P2) => T;
      struct: (value: ToolboxIdlTypeFlatStruct, p1: P1, p2: P2) => T;
      enum: (value: ToolboxIdlTypeFlatEnum, p1: P1, p2: P2) => T;
      padded: (value: ToolboxIdlTypeFlatPadded, p1: P1, p2: P2) => T;
      const: (value: ToolboxIdlTypeFlatConst, p1: P1, p2: P2) => T;
      primitive: (value: ToolboxIdlTypePrimitive, p1: P1, p2: P2) => T;
    },
    p1: P1,
    p2: P2,
  ): T {
    return visitor[this.discriminant](this.content as any, p1, p2);
  }
}

export type ToolboxIdlTypeFlatFieldNamed = {
  name: string;
  docs: any;
  content: ToolboxIdlTypeFlat;
};
export type ToolboxIdlTypeFlatFieldUnnamed = {
  docs: any;
  content: ToolboxIdlTypeFlat;
};

type ToolboxIdlTypeFlatFieldsDiscriminant = 'nothing' | 'named' | 'unnamed';
type ToolboxIdlTypeFlatFieldsContent =
  | {}
  | ToolboxIdlTypeFlatFieldNamed[]
  | ToolboxIdlTypeFlatFieldUnnamed[];

export class ToolboxIdlTypeFlatFields {
  private readonly discriminant: ToolboxIdlTypeFlatFieldsDiscriminant;
  private readonly content: ToolboxIdlTypeFlatFieldsContent;

  private constructor(
    discriminant: ToolboxIdlTypeFlatFieldsDiscriminant,
    content: ToolboxIdlTypeFlatFieldsContent,
  ) {
    this.discriminant = discriminant;
    this.content = content;
  }

  public static nothing(): ToolboxIdlTypeFlatFields {
    return new ToolboxIdlTypeFlatFields('nothing', {});
  }
  public static named(
    value: ToolboxIdlTypeFlatFieldNamed[],
  ): ToolboxIdlTypeFlatFields {
    return new ToolboxIdlTypeFlatFields('named', value);
  }
  public static unnamed(
    value: ToolboxIdlTypeFlatFieldUnnamed[],
  ): ToolboxIdlTypeFlatFields {
    return new ToolboxIdlTypeFlatFields('unnamed', value);
  }

  public isNothing(): boolean {
    return this.discriminant === 'nothing';
  }

  public traverse<P1, P2, T>(
    visitor: {
      nothing: (value: {}, p1: P1, p2: P2) => T;
      named: (value: ToolboxIdlTypeFlatFieldNamed[], p1: P1, p2: P2) => T;
      unnamed: (value: ToolboxIdlTypeFlatFieldUnnamed[], p1: P1, p2: P2) => T;
    },
    p1: P1,
    p2: P2,
  ) {
    return visitor[this.discriminant](this.content as any, p1, p2);
  }
}

export type ToolboxIdlTypeFlatEnumVariant = {
  name: string;
  docs: any;
  code: bigint;
  fields: ToolboxIdlTypeFlatFields;
};
