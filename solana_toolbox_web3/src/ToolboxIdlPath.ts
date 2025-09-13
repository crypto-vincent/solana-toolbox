export class ToolboxIdlPathPart {
  private discriminant: 'empty' | 'account' | 'instruction' | 'type';
}

export class ToolboxIdlPath {
  private parts: ToolboxIdlPathPart[];
}
