export class ToolboxIdlTypePrefix {
  public static readonly U8 = new ToolboxIdlTypePrefix('u8', 1);
  public static readonly U16 = new ToolboxIdlTypePrefix('u16', 2);
  public static readonly U32 = new ToolboxIdlTypePrefix('u32', 4);
  public static readonly U64 = new ToolboxIdlTypePrefix('u64', 8);
  public static readonly U128 = new ToolboxIdlTypePrefix('u128', 16);

  public static readonly prefixesBySize = (() => {
    const prefixes = [
      ToolboxIdlTypePrefix.U8,
      ToolboxIdlTypePrefix.U16,
      ToolboxIdlTypePrefix.U32,
      ToolboxIdlTypePrefix.U64,
      ToolboxIdlTypePrefix.U128,
    ];
    const prefixesBySize = new Map<number, ToolboxIdlTypePrefix>();
    for (const prefix of prefixes) {
      prefixesBySize.set(prefix.size, prefix);
    }
    return prefixesBySize;
  })();

  public readonly name: string;
  public readonly size: number;

  private constructor(name: string, size: number) {
    this.name = name;
    this.size = size;
  }

  public traverse<P1, P2, T>(
    visitor: {
      u8: (p1: P1, p2: P2) => T;
      u16: (p1: P1, p2: P2) => T;
      u32: (p1: P1, p2: P2) => T;
      u64: (p1: P1, p2: P2) => T;
      u128: (p1: P1, p2: P2) => T;
    },
    p1: P1,
    p2: P2,
  ): T {
    return visitor[this.name as keyof typeof visitor](p1, p2);
  }
}
