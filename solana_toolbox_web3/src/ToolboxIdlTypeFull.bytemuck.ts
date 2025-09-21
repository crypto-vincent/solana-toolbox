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
import { ToolboxIdlTypePrefix } from './ToolboxIdlTypePrefix';
import { ToolboxIdlTypePrimitive } from './ToolboxIdlTypePrimitive';
import { ToolboxUtils } from './ToolboxUtils';

type ToolboxIdlTypeFullPod = {
  alignment: number;
  size: number;
  value: ToolboxIdlTypeFull;
};

type ToolboxIdlTypeFullPodFields = {
  alignment: number;
  size: number;
  value: ToolboxIdlTypeFullFields;
};

export function bytemuck(
  typedef: ToolboxIdlTypeFullTypedef,
): ToolboxIdlTypeFullPod {
  return ToolboxUtils.withContext(() => {
    let contentPod;
    if (typedef.repr === undefined) {
      contentPod = bytemuckRust(typedef.content);
    } else if (typedef.repr === 'c') {
      contentPod = bytemuckC(typedef.content);
    } else if (typedef.repr === 'rust') {
      contentPod = bytemuckRust(typedef.content);
    } else if (typedef.repr === 'transparent') {
      contentPod = bytemuckRust(typedef.content);
    } else {
      throw new Error(`Bytemuck: Unsupported repr: ${typedef.repr}`);
    }
    return {
      alignment: contentPod.alignment,
      size: contentPod.size,
      value: ToolboxIdlTypeFull.typedef({
        name: typedef.name,
        repr: typedef.repr,
        content: contentPod.value,
      }),
    };
  }, `Bytemuck: Typedef: ${typedef.name}`);
}

function bytemuckC(value: ToolboxIdlTypeFull): ToolboxIdlTypeFullPod {
  return value.traverse(bytemuckCVisitor, undefined, undefined, undefined);
}

function bytemuckRust(value: ToolboxIdlTypeFull): ToolboxIdlTypeFullPod {
  return value.traverse(bytemuckRustVisitor, undefined, undefined, undefined);
}

function bytemuckFields(
  typeFields: ToolboxIdlTypeFullFields,
  prefixSize: number,
  rustReorder: boolean,
): ToolboxIdlTypeFullPodFields {
  return typeFields.traverse(
    bytemuckFieldsVisitor,
    prefixSize,
    rustReorder,
    undefined,
  );
}

const bytemuckCVisitor = {
  typedef: (self: ToolboxIdlTypeFullTypedef): ToolboxIdlTypeFullPod => {
    return bytemuck(self);
  },
  option: (self: ToolboxIdlTypeFullOption): ToolboxIdlTypeFullPod => {
    const contentPod = bytemuckC(self.content);
    const alignment = Math.max(self.prefix.size, contentPod.alignment);
    const size = alignment + contentPod.size;
    return {
      alignment,
      size,
      value: ToolboxIdlTypeFull.padded({
        before: 0,
        minSize: size,
        after: 0,
        content: ToolboxIdlTypeFull.option({
          prefix: internalPrefixFromAlignment(alignment),
          content: contentPod.value,
        }),
      }),
    };
  },
  vec: (_self: ToolboxIdlTypeFullVec): ToolboxIdlTypeFullPod => {
    throw new Error('Bytemuck: Repr(C): Vec is not supported');
  },
  array: (self: ToolboxIdlTypeFullArray): ToolboxIdlTypeFullPod => {
    const itemsPod = bytemuckC(self.items);
    const alignment = itemsPod.alignment;
    const size = itemsPod.size * self.length;
    return {
      alignment,
      size,
      value: ToolboxIdlTypeFull.array({
        items: itemsPod.value,
        length: self.length,
      }),
    };
  },
  string: (_self: ToolboxIdlTypeFullString): ToolboxIdlTypeFullPod => {
    throw new Error('Bytemuck: Repr(C): String is not supported');
  },
  struct: (self: ToolboxIdlTypeFullStruct): ToolboxIdlTypeFullPod => {
    const fieldsPod = bytemuckFields(self.fields, 0, false);
    return {
      alignment: fieldsPod.alignment,
      size: fieldsPod.size,
      value: ToolboxIdlTypeFull.struct({
        fields: fieldsPod.value,
      }),
    };
  },
  enum: (self: ToolboxIdlTypeFullEnum): ToolboxIdlTypeFullPod => {
    if (self.variants.length == 0) {
      return {
        alignment: 1,
        size: 0,
        value: ToolboxIdlTypeFull.enum(self),
      };
    }
    let alignment = Math.max(4, self.prefix.size);
    let size = 0;
    const variantsReprC = [];
    for (const variant of self.variants) {
      const variantFieldsPod = ToolboxUtils.withContext(() => {
        return bytemuckFields(variant.fields, 0, false);
      }, `Bytemuck: Repr(C): Enum Variant: ${variant.name}`);
      alignment = Math.max(alignment, variantFieldsPod.alignment);
      size = Math.max(size, variantFieldsPod.size);
      variantsReprC.push({
        name: variant.name,
        code: variant.code,
        fields: variantFieldsPod.value,
      });
    }
    size += alignment;
    return {
      alignment,
      size,
      value: ToolboxIdlTypeFull.padded({
        before: 0,
        minSize: size,
        after: 0,
        content: ToolboxIdlTypeFull.enum({
          prefix: internalPrefixFromAlignment(alignment),
          variants: variantsReprC,
        }),
      }),
    };
  },
  padded: (_self: ToolboxIdlTypeFullPadded): ToolboxIdlTypeFullPod => {
    throw new Error('Bytemuck: Repr(C): Padded is not supported');
  },
  primitive: (self: ToolboxIdlTypePrimitive): ToolboxIdlTypeFullPod => {
    return {
      alignment: self.alignment,
      size: self.size,
      value: ToolboxIdlTypeFull.primitive(self),
    };
  },
};

const bytemuckRustVisitor = {
  typedef: (self: ToolboxIdlTypeFullTypedef): ToolboxIdlTypeFullPod => {
    return bytemuck(self);
  },
  option: (self: ToolboxIdlTypeFullOption): ToolboxIdlTypeFullPod => {
    const contentPod = bytemuckRust(self.content);
    const alignment = Math.max(self.prefix.size, contentPod.alignment);
    const size = alignment + contentPod.size;
    return {
      alignment,
      size,
      value: ToolboxIdlTypeFull.padded({
        before: 0,
        minSize: size,
        after: 0,
        content: ToolboxIdlTypeFull.option({
          prefix: internalPrefixFromAlignment(alignment),
          content: contentPod.value,
        }),
      }),
    };
  },
  vec: (_self: ToolboxIdlTypeFullVec): ToolboxIdlTypeFullPod => {
    throw new Error('Bytemuck: Repr(Rust): Vec is not supported');
  },
  array: (self: ToolboxIdlTypeFullArray): ToolboxIdlTypeFullPod => {
    const itemsPod = bytemuckRust(self.items);
    const alignment = itemsPod.alignment;
    const size = itemsPod.size * self.length;
    return {
      alignment,
      size,
      value: ToolboxIdlTypeFull.array({
        items: itemsPod.value,
        length: self.length,
      }),
    };
  },
  string: (_self: ToolboxIdlTypeFullString): ToolboxIdlTypeFullPod => {
    throw new Error('Bytemuck: Repr(Rust): String is not supported');
  },
  struct: (self: ToolboxIdlTypeFullStruct): ToolboxIdlTypeFullPod => {
    const fieldsPod = bytemuckFields(self.fields, 0, true);
    return {
      alignment: fieldsPod.alignment,
      size: fieldsPod.size,
      value: ToolboxIdlTypeFull.struct({
        fields: fieldsPod.value,
      }),
    };
  },
  enum: (self: ToolboxIdlTypeFullEnum): ToolboxIdlTypeFullPod => {
    if (self.variants.length == 0) {
      return {
        alignment: 1,
        size: 0,
        value: ToolboxIdlTypeFull.enum(self),
      };
    }
    let alignment = self.prefix.size;
    let size = self.prefix.size;
    const variantsReprRust = [];
    for (const variant of self.variants) {
      const variantFieldsPod = ToolboxUtils.withContext(() => {
        return bytemuckFields(variant.fields, self.prefix.size, true);
      }, `Bytemuck: Repr(Rust): Enum Variant: ${variant.name}`);
      alignment = Math.max(alignment, variantFieldsPod.alignment);
      size = Math.max(size, variantFieldsPod.size);
      variantsReprRust.push({
        name: variant.name,
        code: variant.code,
        fields: variantFieldsPod.value,
      });
    }
    size += internalAlignmentPaddingNeeded(size, alignment);
    return {
      alignment,
      size,
      value: ToolboxIdlTypeFull.padded({
        before: 0,
        minSize: size,
        after: 0,
        content: ToolboxIdlTypeFull.enum({
          prefix: self.prefix,
          variants: variantsReprRust,
        }),
      }),
    };
  },
  padded: (_self: ToolboxIdlTypeFullPadded): ToolboxIdlTypeFullPod => {
    throw new Error('Bytemuck: Repr(Rust): Padded is not supported');
  },
  primitive: (self: ToolboxIdlTypePrimitive): ToolboxIdlTypeFullPod => {
    return {
      alignment: self.alignment,
      size: self.size,
      value: ToolboxIdlTypeFull.primitive(self),
    };
  },
};

const bytemuckFieldsVisitor = {
  nothing: (
    _self: null,
    _prefixSize: number,
    _rustReorder: boolean,
  ): ToolboxIdlTypeFullPodFields => {
    return {
      alignment: 1,
      size: 0,
      value: ToolboxIdlTypeFullFields.nothing(),
    };
  },
  named: (
    self: Array<ToolboxIdlTypeFullFieldNamed>,
    prefixSize: number,
    rustReorder: boolean,
  ): ToolboxIdlTypeFullPodFields => {
    const fieldsInfosPods = self.map((field, index) => {
      const contentPod = ToolboxUtils.withContext(() => {
        return bytemuckRust(field.content);
      }, `Bytemuck: Field: ${field.name}`);
      return {
        index: index,
        alignment: contentPod.alignment,
        size: contentPod.size,
        meta: field.name,
        type: contentPod.value,
      };
    });
    if (rustReorder) {
      internalVerifyUnstableOrder(prefixSize, fieldsInfosPods);
    }
    const fieldsInfosPadded = internalFieldsInfoAligned(
      prefixSize,
      fieldsInfosPods,
    );
    return {
      alignment: fieldsInfosPadded.alignment,
      size: fieldsInfosPadded.size,
      value: ToolboxIdlTypeFullFields.named(
        fieldsInfosPadded.value.map((fieldInfo) => {
          return {
            name: fieldInfo.meta,
            content: fieldInfo.type,
          };
        }),
      ),
    };
  },
  unnamed: (
    self: Array<ToolboxIdlTypeFullFieldUnnamed>,
    prefixSize: number,
    rustReorder: boolean,
  ): ToolboxIdlTypeFullPodFields => {
    const fieldsInfosPods = self.map((field, index) => {
      const contentPod = ToolboxUtils.withContext(() => {
        return bytemuckRust(field.content);
      }, `Bytemuck: Field: ${field.position}`);
      return {
        index: index,
        alignment: contentPod.alignment,
        size: contentPod.size,
        meta: field.position,
        type: contentPod.value,
      };
    });
    if (rustReorder) {
      internalVerifyUnstableOrder(prefixSize, fieldsInfosPods);
    }
    const fieldsInfosPadded = internalFieldsInfoAligned(
      prefixSize,
      fieldsInfosPods,
    );
    return {
      alignment: fieldsInfosPadded.alignment,
      size: fieldsInfosPadded.size,
      value: ToolboxIdlTypeFullFields.unnamed(
        fieldsInfosPadded.value.map((fieldInfo) => {
          return {
            position: fieldInfo.meta,
            content: fieldInfo.type,
          };
        }),
      ),
    };
  },
};

function internalFieldsInfoAligned<T>(
  prefixSize: number,
  fieldsInfo: Array<{
    index: number;
    alignment: number;
    size: number;
    meta: T;
    type: ToolboxIdlTypeFull;
  }>,
) {
  let alignment = prefixSize;
  let size = prefixSize;
  const lastFieldIndex = fieldsInfo.length - 1;
  const fieldsInfoPadded = [];
  for (const fieldInfo of fieldsInfo) {
    const {
      index: fieldIndex,
      alignment: fieldAlignment,
      size: fieldSize,
      meta: fieldMeta,
      type: fieldType,
    } = fieldInfo;
    alignment = Math.max(alignment, fieldAlignment);
    const paddingBefore = internalAlignmentPaddingNeeded(size, fieldAlignment);
    size += paddingBefore + fieldSize;
    let paddingAfter = 0;
    if (fieldIndex == lastFieldIndex) {
      paddingAfter = internalAlignmentPaddingNeeded(size, alignment);
    }
    size += paddingAfter;
    if (paddingBefore == 0 && paddingAfter == 0) {
      fieldsInfoPadded.push({ meta: fieldMeta, type: fieldType });
    } else {
      fieldsInfoPadded.push({
        meta: fieldMeta,
        type: ToolboxIdlTypeFull.padded({
          before: paddingBefore,
          minSize: fieldSize,
          after: paddingAfter,
          content: fieldType,
        }),
      });
    }
  }
  return {
    alignment,
    size,
    value: fieldsInfoPadded,
  };
}

function internalAlignmentPaddingNeeded(
  offset: number,
  alignment: number,
): number {
  const missalignment = offset % alignment;
  if (missalignment == 0) {
    return 0;
  }
  return alignment - missalignment;
}

function internalVerifyUnstableOrder(
  prefixSize: number,
  fieldsInfo: Array<any>,
) {
  if (prefixSize == 0 && fieldsInfo.length <= 2) {
    return;
  }
  if (fieldsInfo.length <= 1) {
    return;
  }
  throw new Error(
    'Bytemuck: Repr(Rust): Structs/Enums/Tuples fields ordering is compiler-dependent. Use Repr(C) instead.',
  );
}

function internalPrefixFromAlignment(alignment: number): ToolboxIdlTypePrefix {
  const prefix = ToolboxIdlTypePrefix.prefixesBySize.get(alignment);
  if (prefix === undefined) {
    throw new Error(`Bytemuck: Unknown alignment: ${alignment}`);
  }
  return prefix;
}
