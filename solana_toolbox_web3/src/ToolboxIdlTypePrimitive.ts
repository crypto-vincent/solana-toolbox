export class ToolboxIdlTypePrimitive {
  public static readonly U8 = new ToolboxIdlTypePrimitive({
    name: 'u8',
    size: 1,
    alignment: 1,
  });
  public static readonly U16 = new ToolboxIdlTypePrimitive({
    name: 'u16',
    size: 2,
    alignment: 2,
  });
  public static readonly U32 = new ToolboxIdlTypePrimitive({
    name: 'u32',
    size: 4,
    alignment: 4,
  });
  public static readonly U64 = new ToolboxIdlTypePrimitive({
    name: 'u64',
    size: 8,
    alignment: 8,
  });
  public static readonly U128 = new ToolboxIdlTypePrimitive({
    name: 'u128',
    size: 16,
    alignment: 16,
  });
  public static readonly I8 = new ToolboxIdlTypePrimitive({
    name: 'i8',
    size: 1,
    alignment: 1,
  });
  public static readonly I16 = new ToolboxIdlTypePrimitive({
    name: 'i16',
    size: 2,
    alignment: 2,
  });
  public static readonly I32 = new ToolboxIdlTypePrimitive({
    name: 'i32',
    size: 4,
    alignment: 4,
  });
  public static readonly I64 = new ToolboxIdlTypePrimitive({
    name: 'i64',
    size: 8,
    alignment: 8,
  });
  public static readonly I128 = new ToolboxIdlTypePrimitive({
    name: 'i128',
    size: 16,
    alignment: 16,
  });
  public static readonly F32 = new ToolboxIdlTypePrimitive({
    name: 'f32',
    size: 4,
    alignment: 4,
  });
  public static readonly F64 = new ToolboxIdlTypePrimitive({
    name: 'f64',
    size: 8,
    alignment: 8,
  });
  public static readonly Bool = new ToolboxIdlTypePrimitive({
    name: 'bool',
    size: 1,
    alignment: 1,
  });
  public static readonly Pubkey = new ToolboxIdlTypePrimitive({
    name: 'pubkey',
    size: 32,
    alignment: 1,
  });

  public static readonly primitivesByName = (() => {
    const primitives = [
      ToolboxIdlTypePrimitive.U8,
      ToolboxIdlTypePrimitive.U16,
      ToolboxIdlTypePrimitive.U32,
      ToolboxIdlTypePrimitive.U64,
      ToolboxIdlTypePrimitive.U128,
      ToolboxIdlTypePrimitive.I8,
      ToolboxIdlTypePrimitive.I16,
      ToolboxIdlTypePrimitive.I32,
      ToolboxIdlTypePrimitive.I64,
      ToolboxIdlTypePrimitive.I128,
      ToolboxIdlTypePrimitive.F32,
      ToolboxIdlTypePrimitive.F64,
      ToolboxIdlTypePrimitive.Bool,
      ToolboxIdlTypePrimitive.Pubkey,
    ];
    const primitivesByName = new Map<string, ToolboxIdlTypePrimitive>();
    for (const primitive of primitives) {
      primitivesByName.set(primitive.name, primitive);
    }
    return primitivesByName;
  })();

  public readonly name: string;
  public readonly size: number;
  public readonly alignment: number;

  private constructor(value: {
    name: string;
    size: number;
    alignment: number;
  }) {
    this.name = value.name;
    this.size = value.size;
    this.alignment = value.alignment;
  }

  public traverse<P1, P2, T>(
    visitor: {
      u8: (p1: P1, p2: P2) => T;
      u16: (p1: P1, p2: P2) => T;
      u32: (p1: P1, p2: P2) => T;
      u64: (p1: P1, p2: P2) => T;
      u128: (p1: P1, p2: P2) => T;
      i8: (p1: P1, p2: P2) => T;
      i16: (p1: P1, p2: P2) => T;
      i32: (p1: P1, p2: P2) => T;
      i64: (p1: P1, p2: P2) => T;
      i128: (p1: P1, p2: P2) => T;
      f32: (p1: P1, p2: P2) => T;
      f64: (p1: P1, p2: P2) => T;
      bool: (p1: P1, p2: P2) => T;
      pubkey: (p1: P1, p2: P2) => T;
    },
    p1: P1,
    p2: P2,
  ): T {
    return visitor[this.name as keyof typeof visitor](p1, p2);
  }
}
