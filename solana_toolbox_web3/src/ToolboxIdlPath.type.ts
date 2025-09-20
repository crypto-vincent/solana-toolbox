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
  let split = path.splitFirst();
  if (split === undefined) {
    return typeFull;
  }
  let current = split.first;
  let next = split.rest;
  return typeFull.traverse(pathGetTypeFullVisitor, path, current, next);
}

export function pathGetTypeFullFields(
  path: ToolboxIdlPath,
  typeFullFields: ToolboxIdlTypeFullFields,
): ToolboxIdlTypeFull {
  let split = path.splitFirst();
  if (split === undefined) {
    throw new Error('Fields cannot be a standalone type');
  }
  let current = split.first;
  let next = split.rest;
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
    _self: ToolboxIdlTypeFullEnum,
    path: ToolboxIdlPath,
    _current: ToolboxIdlPathPart,
    _next: ToolboxIdlPath,
  ) => {
    // TODO - support enum variants with fields
    throw new Error(`Cannot traverse into enum variant: '${path.value()}'`);
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
    let key = current.value();
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
    let index = current.index();
    if (index === undefined) {
      throw new Error(`Expected index but got '${current.value()}'`);
    }
    const length = self.length;
    if (index < 0 || index >= length) {
      throw new Error(
        `Index ${index} out of bounds for fields of length ${length}`,
      );
    }
    return pathGetTypeFull(next, self[index]!.content);
  },
};
