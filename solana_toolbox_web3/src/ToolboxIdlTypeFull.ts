import { ToolboxIdlTypePrefix } from './ToolboxIdlTypePrefix';
import { ToolboxIdlTypePrimitive } from './ToolboxIdlTypePrimitive';

export type ToolboxIdlTypeFullTypedef = {
  name: string;
  repr: string | undefined;
  content: ToolboxIdlTypeFull;
};
export type ToolboxIdlTypeFullOption = {
  prefix: ToolboxIdlTypePrefix;
  content: ToolboxIdlTypeFull;
};
export type ToolboxIdlTypeFullVec = {
  prefix: ToolboxIdlTypePrefix;
  items: ToolboxIdlTypeFull;
};
export type ToolboxIdlTypeFullArray = {
  items: ToolboxIdlTypeFull;
  length: number;
};
export type ToolboxIdlTypeFullString = {
  prefix: ToolboxIdlTypePrefix;
};
export type ToolboxIdlTypeFullStruct = {
  fields: ToolboxIdlTypeFullFields;
};
export type ToolboxIdlTypeFullEnum = {
  prefix: ToolboxIdlTypePrefix;
  variants: Array<ToolboxIdlTypeFullEnumVariant>;
};
export type ToolboxIdlTypeFullPadded = {
  before: number;
  minSize: number;
  after: number;
  content: ToolboxIdlTypeFull;
};

type ToolboxIdlTypeFullDiscriminant =
  | 'typedef'
  | 'option'
  | 'vec'
  | 'array'
  | 'string'
  | 'struct'
  | 'enum'
  | 'padded'
  | 'primitive';
type ToolboxIdlTypeFullContent =
  | ToolboxIdlTypeFullTypedef
  | ToolboxIdlTypeFullOption
  | ToolboxIdlTypeFullVec
  | ToolboxIdlTypeFullArray
  | ToolboxIdlTypeFullString
  | ToolboxIdlTypeFullStruct
  | ToolboxIdlTypeFullEnum
  | ToolboxIdlTypeFullPadded
  | ToolboxIdlTypePrimitive;

export class ToolboxIdlTypeFull {
  private readonly discriminant: ToolboxIdlTypeFullDiscriminant;
  private readonly content: ToolboxIdlTypeFullContent;

  private constructor(
    discriminant: ToolboxIdlTypeFullDiscriminant,
    content: ToolboxIdlTypeFullContent,
  ) {
    this.discriminant = discriminant;
    this.content = content;
  }

  public static typedef(value: ToolboxIdlTypeFullTypedef): ToolboxIdlTypeFull {
    return new ToolboxIdlTypeFull('typedef', value);
  }
  public static option(value: ToolboxIdlTypeFullOption): ToolboxIdlTypeFull {
    return new ToolboxIdlTypeFull('option', value);
  }
  public static vec(value: ToolboxIdlTypeFullVec): ToolboxIdlTypeFull {
    return new ToolboxIdlTypeFull('vec', value);
  }
  public static array(value: ToolboxIdlTypeFullArray): ToolboxIdlTypeFull {
    return new ToolboxIdlTypeFull('array', value);
  }
  public static string(value: ToolboxIdlTypeFullString): ToolboxIdlTypeFull {
    return new ToolboxIdlTypeFull('string', value);
  }
  public static struct(value: ToolboxIdlTypeFullStruct): ToolboxIdlTypeFull {
    return new ToolboxIdlTypeFull('struct', value);
  }
  public static enum(value: ToolboxIdlTypeFullEnum): ToolboxIdlTypeFull {
    return new ToolboxIdlTypeFull('enum', value);
  }
  public static padded(value: ToolboxIdlTypeFullPadded): ToolboxIdlTypeFull {
    return new ToolboxIdlTypeFull('padded', value);
  }
  public static primitive(value: ToolboxIdlTypePrimitive): ToolboxIdlTypeFull {
    return new ToolboxIdlTypeFull('primitive', value);
  }

  public static structNothing(): ToolboxIdlTypeFull {
    return new ToolboxIdlTypeFull('struct', {
      fields: ToolboxIdlTypeFullFields.nothing(),
    });
  }

  public isPrimitive(primitive: ToolboxIdlTypePrimitive): boolean {
    return this.discriminant === 'primitive' && this.content === primitive;
  }

  public traverse<P1, P2, P3, T>(
    visitor: {
      typedef: (value: ToolboxIdlTypeFullTypedef, p1: P1, p2: P2, p3: P3) => T;
      option: (value: ToolboxIdlTypeFullOption, p1: P1, p2: P2, p3: P3) => T;
      vec: (value: ToolboxIdlTypeFullVec, p1: P1, p2: P2, p3: P3) => T;
      array: (value: ToolboxIdlTypeFullArray, p1: P1, p2: P2, p3: P3) => T;
      string: (value: ToolboxIdlTypeFullString, p1: P1, p2: P2, p3: P3) => T;
      struct: (value: ToolboxIdlTypeFullStruct, p1: P1, p2: P2, p3: P3) => T;
      enum: (value: ToolboxIdlTypeFullEnum, p1: P1, p2: P2, p3: P3) => T;
      padded: (value: ToolboxIdlTypeFullPadded, p1: P1, p2: P2, p3: P3) => T;
      primitive: (value: ToolboxIdlTypePrimitive, p1: P1, p2: P2, p3: P3) => T;
    },
    p1: P1,
    p2: P2,
    p3: P3,
  ): T {
    return visitor[this.discriminant](this.content as any, p1, p2, p3);
  }
}

export type ToolboxIdlTypeFullFieldNamed = {
  name: string;
  content: ToolboxIdlTypeFull;
};
export type ToolboxIdlTypeFullFieldUnnamed = {
  position: number;
  content: ToolboxIdlTypeFull;
};

type ToolboxIdlTypeFullFieldsDiscriminant = 'nothing' | 'named' | 'unnamed';
type ToolboxIdlTypeFullFieldsContent =
  | {}
  | Array<ToolboxIdlTypeFullFieldNamed>
  | Array<ToolboxIdlTypeFullFieldUnnamed>;

export class ToolboxIdlTypeFullFields {
  private readonly discriminant: ToolboxIdlTypeFullFieldsDiscriminant;
  private readonly content: ToolboxIdlTypeFullFieldsContent;

  private constructor(
    discriminant: ToolboxIdlTypeFullFieldsDiscriminant,
    content: ToolboxIdlTypeFullFieldsContent,
  ) {
    this.discriminant = discriminant;
    this.content = content;
  }

  public static nothing(): ToolboxIdlTypeFullFields {
    return new ToolboxIdlTypeFullFields('nothing', {});
  }
  public static named(
    value: Array<ToolboxIdlTypeFullFieldNamed>,
  ): ToolboxIdlTypeFullFields {
    return new ToolboxIdlTypeFullFields('named', value);
  }
  public static unnamed(
    value: Array<ToolboxIdlTypeFullFieldUnnamed>,
  ): ToolboxIdlTypeFullFields {
    return new ToolboxIdlTypeFullFields('unnamed', value);
  }

  public isNothing(): boolean {
    return this.discriminant === 'nothing';
  }

  public traverse<P1, P2, P3, T>(
    visitor: {
      nothing: (value: {}, p1: P1, p2: P2, p3: P3) => T;
      named: (
        value: Array<ToolboxIdlTypeFullFieldNamed>,
        p1: P1,
        p2: P2,
        p3: P3,
      ) => T;
      unnamed: (
        value: Array<ToolboxIdlTypeFullFieldUnnamed>,
        p1: P1,
        p2: P2,
        p3: P3,
      ) => T;
    },
    p1: P1,
    p2: P2,
    p3: P3,
  ) {
    return visitor[this.discriminant](this.content as any, p1, p2, p3);
  }
}

export type ToolboxIdlTypeFullEnumVariant = {
  name: string;
  code: bigint;
  fields: ToolboxIdlTypeFullFields;
};
