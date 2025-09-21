import { ToolboxIdlPath, ToolboxIdlPathPart } from './ToolboxIdlPath';
import {
  ToolboxIdlTypeFull,
  ToolboxIdlTypeFullArray,
  ToolboxIdlTypeFullEnum,
  ToolboxIdlTypeFullFieldNamed,
  ToolboxIdlTypeFullFields,
  ToolboxIdlTypeFullFieldUnnamed,
  ToolboxIdlTypeFullOption,
  ToolboxIdlTypeFullPadded,
  ToolboxIdlTypeFullString,
  ToolboxIdlTypeFullStruct,
  ToolboxIdlTypeFullTypedef,
  ToolboxIdlTypeFullVec,
} from './ToolboxIdlTypeFull';
import { ToolboxIdlTypePrimitive } from './ToolboxIdlTypePrimitive';

export function pathGetTypeFull(
  path: ToolboxIdlPath,
  typeFull: ToolboxIdlTypeFull,
): ToolboxIdlTypeFull {
  const split = path.splitFirst();
  if (split === undefined) {
    return typeFull;
  }
  const current = split.first;
  const next = split.rest;
  return typeFull.traverse(pathGetTypeFullVisitor, path, current, next);
}

export function pathGetTypeFullFields(
  path: ToolboxIdlPath,
  typeFullFields: ToolboxIdlTypeFullFields,
): ToolboxIdlTypeFull {
  const split = path.splitFirst();
  if (split === undefined) {
    throw new Error('Fields cannot be a standalone type');
  }
  const current = split.first;
  const next = split.rest;
  return typeFullFields.traverse(
    pathGetTypeFullFieldsVisitor,
    path,
    current,
    next,
  );
}

const pathGetTypeFullVisitor = {
  typedef: (
    self: ToolboxIdlTypeFullTypedef,
    path: ToolboxIdlPath,
    _current: ToolboxIdlPathPart,
    _next: ToolboxIdlPath,
  ) => {
    return pathGetTypeFull(path, self.content);
  },
  option: (
    self: ToolboxIdlTypeFullOption,
    path: ToolboxIdlPath,
    _current: ToolboxIdlPathPart,
    _next: ToolboxIdlPath,
  ) => {
    return pathGetTypeFull(path, self.content);
  },
  vec: (
    self: ToolboxIdlTypeFullVec,
    _path: ToolboxIdlPath,
    current: ToolboxIdlPathPart,
    next: ToolboxIdlPath,
  ) => {
    const key = current.key();
    if (key !== undefined) {
      throw new Error(`Vec cannot be accessed by key: '${key}'`);
    }
    return pathGetTypeFull(next, self.items);
  },
  array: (
    self: ToolboxIdlTypeFullArray,
    _path: ToolboxIdlPath,
    current: ToolboxIdlPathPart,
    next: ToolboxIdlPath,
  ) => {
    const key = current.key();
    if (key !== undefined) {
      throw new Error(`Array cannot be accessed by key: '${key}'`);
    }
    return pathGetTypeFull(next, self.items);
  },
  string: (
    _self: ToolboxIdlTypeFullString,
    path: ToolboxIdlPath,
    _current: ToolboxIdlPathPart,
    _next: ToolboxIdlPath,
  ) => {
    throw new Error(`Type string does not contain path: '${path.value()}'`);
  },
  struct: (
    self: ToolboxIdlTypeFullStruct,
    path: ToolboxIdlPath,
    _current: ToolboxIdlPathPart,
    _next: ToolboxIdlPath,
  ) => {
    return pathGetTypeFullFields(path, self.fields);
  },
  enum: (
    self: ToolboxIdlTypeFullEnum,
    _path: ToolboxIdlPath,
    current: ToolboxIdlPathPart,
    next: ToolboxIdlPath,
  ) => {
    return current.traverse(
      {
        empty: () => {
          throw new Error(`Expected enum variant key or index (found empty)`);
        },
        key: (key: string) => {
          for (const variant of self.variants) {
            if (variant.name === key) {
              return pathGetTypeFullFields(next, variant.fields);
            }
          }
          throw new Error(`Could not find enum variant: '${key}'`);
        },
        index: (index: bigint) => {
          for (const variant of self.variants) {
            if (variant.code === index) {
              return pathGetTypeFullFields(next, variant.fields);
            }
          }
          throw new Error(`Could not find enum variant with code: '${index}'`);
        },
      },
      undefined,
      undefined,
    );
  },
  padded: (
    self: ToolboxIdlTypeFullPadded,
    path: ToolboxIdlPath,
    _current: ToolboxIdlPathPart,
    _next: ToolboxIdlPath,
  ) => {
    return pathGetTypeFull(path, self.content);
  },
  primitive: (
    self: ToolboxIdlTypePrimitive,
    path: ToolboxIdlPath,
    _current: ToolboxIdlPathPart,
    _next: ToolboxIdlPath,
  ) => {
    throw new Error(
      `Type primitive '${self}' does not contain path: '${path.value()}'`,
    );
  },
};

const pathGetTypeFullFieldsVisitor = {
  nothing: (_self: {}, path: ToolboxIdlPath) => {
    throw new Error(`Type has no fields: '${path.value()}'`);
  },
  named: (
    self: Array<ToolboxIdlTypeFullFieldNamed>,
    _path: ToolboxIdlPath,
    current: ToolboxIdlPathPart,
    next: ToolboxIdlPath,
  ) => {
    const key = current.value();
    for (const field of self) {
      if (field.name === key) {
        return pathGetTypeFull(next, field.content);
      }
    }
    throw new Error(`Could not find named field: '${key}'`);
  },
  unnamed: (
    self: Array<ToolboxIdlTypeFullFieldUnnamed>,
    _path: ToolboxIdlPath,
    current: ToolboxIdlPathPart,
    next: ToolboxIdlPath,
  ) => {
    const length = self.length;
    const index = current.index();
    if (index === undefined) {
      throw new Error(`Expected index but got '${current.value()}'`);
    }
    const indexNumber = Number(index);
    if (indexNumber < 0 || indexNumber >= length) {
      throw new Error(
        `Index ${indexNumber} out of bounds for fields of length ${length}`,
      );
    }
    return pathGetTypeFull(next, self[indexNumber]!.content);
  },
};
