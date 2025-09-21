type ToolboxIdlPathPartDiscriminant = 'empty' | 'index' | 'key';
type ToolboxIdlPathPartContent = null | bigint | string;

export class ToolboxIdlPathPart {
  private discriminant: ToolboxIdlPathPartDiscriminant;
  private content: ToolboxIdlPathPartContent;

  private constructor(
    discriminant: ToolboxIdlPathPartDiscriminant,
    content: ToolboxIdlPathPartContent,
  ) {
    this.discriminant = discriminant;
    this.content = content;
  }
  public static empty(): ToolboxIdlPathPart {
    return new ToolboxIdlPathPart('empty', null);
  }
  public static index(value: bigint): ToolboxIdlPathPart {
    return new ToolboxIdlPathPart('index', value);
  }
  public static key(value: string): ToolboxIdlPathPart {
    return new ToolboxIdlPathPart('key', value);
  }

  public isEmpty(): boolean {
    return this.discriminant === 'empty';
  }

  public key(): string | undefined {
    if (this.discriminant === 'key') {
      return this.content as string;
    }
    return undefined;
  }
  public index(): bigint | undefined {
    if (this.discriminant === 'index') {
      return this.content as bigint;
    }
    return undefined;
  }

  public value(): string {
    switch (this.discriminant) {
      case 'empty':
        return '';
      case 'index':
        return (this.content as bigint).toString();
      case 'key':
        return this.content as string;
    }
  }

  public traverse<P1, P2, T>(
    visitor: {
      empty: (value: null, p1: P1, p2: P2) => T;
      index: (value: bigint, p1: P1, p2: P2) => T;
      key: (value: string, p1: P1, p2: P2) => T;
    },
    p1: P1,
    p2: P2,
  ): T {
    switch (this.discriminant) {
      case 'empty':
        return visitor.empty(this.content as null, p1, p2);
      case 'index':
        return visitor.index(this.content as bigint, p1, p2);
      case 'key':
        return visitor.key(this.content as string, p1, p2);
    }
  }
}

export class ToolboxIdlPath {
  private readonly parts: Array<ToolboxIdlPathPart>;

  private constructor(parts: Array<ToolboxIdlPathPart>) {
    this.parts = parts;
  }

  public static tryParse(path: string): ToolboxIdlPath {
    const parts: Array<ToolboxIdlPathPart> = [];
    for (const part of path.split('.')) {
      if (part === '') {
        parts.push(ToolboxIdlPathPart.empty());
      } else if (/^\d+$/.test(part)) {
        parts.push(ToolboxIdlPathPart.index(BigInt(part)));
      } else {
        parts.push(ToolboxIdlPathPart.key(part));
      }
    }
    if (parts[0]!.isEmpty()) {
      parts.shift();
    }
    return new ToolboxIdlPath(parts);
  }

  public splitFirst():
    | { first: ToolboxIdlPathPart; rest: ToolboxIdlPath }
    | undefined {
    if (this.parts.length === 0) {
      return undefined;
    }
    return {
      first: this.parts[0]!,
      rest: new ToolboxIdlPath(this.parts.slice(1)),
    };
  }

  public isEmpty(): boolean {
    return this.parts.length === 0;
  }

  public value(): string {
    return this.parts.map((p) => p.value()).join('.');
  }
}
