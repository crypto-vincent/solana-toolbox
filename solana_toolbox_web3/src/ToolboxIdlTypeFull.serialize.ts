import { PublicKey } from '@solana/web3.js';
import {
  ToolboxIdlTypeFull,
  ToolboxIdlTypeFullArray,
  ToolboxIdlTypeFullEnum,
  ToolboxIdlTypeFullEnumVariant,
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
import { ToolboxUtils } from './ToolboxUtils';
import { ToolboxIdlTypePrefix } from './ToolboxIdlTypePrefix';

export function serialize(
  typeFull: ToolboxIdlTypeFull,
  value: any,
  data: Array<Buffer>,
  prefixed: boolean,
) {
  typeFull.traverse(serializeVisitor, value, data, prefixed);
}

export function serializeFields(
  typeFullFields: ToolboxIdlTypeFullFields,
  value: any,
  data: Array<Buffer>,
  prefixed: boolean,
) {
  typeFullFields.traverse(serializeFieldsVisitor, value, data, prefixed);
}

export function serializePrefix(
  prefix: ToolboxIdlTypePrefix,
  value: bigint,
  data: Array<Buffer>,
) {
  const buffer = Buffer.alloc(prefix.size);
  prefix.traverse(serializePrefixVisitor, buffer, value);
  data.push(buffer);
}

export function serializePrimitive(
  primitive: ToolboxIdlTypePrimitive,
  value: any,
  data: Array<Buffer>,
) {
  const buffer = Buffer.alloc(primitive.size);
  primitive.traverse(serializePrimitiveVisitor, buffer, value);
  data.push(buffer);
}

const serializeVisitor = {
  typedef: (
    self: ToolboxIdlTypeFullTypedef,
    value: any,
    data: Array<Buffer>,
    prefixed: boolean,
  ) => {
    ToolboxUtils.withContext(() => {
      return serialize(self.content, value, data, prefixed);
    }, `Serialize: Typedef: ${self.name}`);
  },
  option: (
    self: ToolboxIdlTypeFullOption,
    value: any,
    data: Array<Buffer>,
    prefixed: boolean,
  ) => {
    if (value === null) {
      serializePrefix(self.prefix, 0n, data);
      return;
    }
    serializePrefix(self.prefix, 1n, data);
    serialize(self.content, value, data, prefixed);
  },
  vec: (
    self: ToolboxIdlTypeFullVec,
    value: any,
    data: Array<Buffer>,
    prefixed: boolean,
  ) => {
    if (self.items.isPrimitive(ToolboxIdlTypePrimitive.U8)) {
      const bytes = ToolboxUtils.expectBytes(value);
      if (prefixed) {
        serializePrefix(self.prefix, BigInt(bytes.length), data);
      }
      data.push(bytes);
      return;
    }
    const array = ToolboxUtils.expectArray(value);
    if (prefixed) {
      serializePrefix(self.prefix, BigInt(array.length), data);
    }
    for (const item of array) {
      serialize(self.items, item, data, prefixed);
    }
  },
  array: (
    self: ToolboxIdlTypeFullArray,
    value: any,
    data: Array<Buffer>,
    prefixed: boolean,
  ) => {
    if (self.items.isPrimitive(ToolboxIdlTypePrimitive.U8)) {
      const bytes = ToolboxUtils.expectBytes(value);
      if (bytes.length != self.length) {
        throw new Error(
          `Expected an array of size: ${self.length}, found: ${bytes.length}`,
        );
      }
      data.push(bytes);
      return;
    }
    const array = ToolboxUtils.expectArray(value);
    if (array.length != self.length) {
      throw new Error(
        `Expected an array of size: ${self.length}, found: ${array.length}`,
      );
    }
    for (const item of array) {
      serialize(self.items, item, data, prefixed);
    }
  },
  string: (
    self: ToolboxIdlTypeFullString,
    value: any,
    data: Array<Buffer>,
    prefixed: boolean,
  ) => {
    const string = ToolboxUtils.expectString(value);
    const bytes = Buffer.from(string, 'utf8');
    if (prefixed) {
      serializePrefix(self.prefix, BigInt(bytes.length), data);
    }
    data.push(bytes);
  },
  struct: (
    self: ToolboxIdlTypeFullStruct,
    value: any,
    data: Array<Buffer>,
    prefixed: boolean,
  ) => {
    serializeFields(self.fields, value, data, prefixed);
  },
  enum: (
    self: ToolboxIdlTypeFullEnum,
    value: any,
    data: Array<Buffer>,
    prefixed: boolean,
  ) => {
    if (self.variants.length == 0) {
      if (value !== null) {
        throw new Error('Expected value to be null for empty enum');
      }
      return;
    }
    function serializeEnumVariant(
      variant: ToolboxIdlTypeFullEnumVariant,
      value: any,
    ) {
      ToolboxUtils.withContext(() => {
        serializePrefix(self.prefix, variant.code, data);
        serializeFields(variant.fields, value, data, prefixed);
      }, `Serialize: Enum Variant: ${variant.name}`);
    }
    if (ToolboxUtils.isNumber(value)) {
      for (const variant of self.variants) {
        if (variant.code == value) {
          return serializeEnumVariant(variant, undefined);
        }
      }
      throw new Error(`Could not find enum variant with code: ${value}`);
    }
    if (ToolboxUtils.isString(value)) {
      for (const variant of self.variants) {
        if (variant.name == value) {
          return serializeEnumVariant(variant, undefined);
        }
      }
      throw new Error(`Could not find enum variant with name: ${value}`);
    }
    if (ToolboxUtils.isObject(value)) {
      for (const variant of self.variants) {
        if (value.hasOwnProperty(variant.name)) {
          return serializeEnumVariant(variant, value[variant.name]);
        }
      }
      throw new Error('Could not guess enum variant from object key');
    }
    throw new Error('Expected enum value to be: number/string or object');
  },
  padded: (
    self: ToolboxIdlTypeFullPadded,
    value: any,
    data: Array<Buffer>,
    prefixed: boolean,
  ) => {
    if (self.before) {
      data.push(Buffer.alloc(self.before));
    }
    const contentData: Array<Buffer> = [];
    serialize(self.content, value, contentData, prefixed);
    for (const contentBuffer of contentData) {
      data.push(contentBuffer);
    }
    const contentSize = contentData.reduce((size, contentBuffer) => {
      return size + contentBuffer.length;
    }, 0);
    if (self.minSize > contentSize) {
      data.push(Buffer.alloc(self.minSize - contentSize));
    }
    if (self.after) {
      data.push(Buffer.alloc(self.after));
    }
  },
  primitive: (
    self: ToolboxIdlTypePrimitive,
    value: any,
    data: Array<Buffer>,
    _prefixed: boolean,
  ) => {
    serializePrimitive(self, value, data);
  },
};

const serializeFieldsVisitor = {
  nothing: (
    _self: {},
    _value: any,
    _data: Array<Buffer>,
    _prefixed: boolean,
  ) => {
    return;
  },
  named: (
    self: Array<ToolboxIdlTypeFullFieldNamed>,
    value: any,
    data: Array<Buffer>,
    prefixed: boolean,
  ) => {
    if (self.length <= 0) {
      return;
    }
    ToolboxUtils.expectObject(value);
    for (const field of self) {
      ToolboxUtils.withContext(() => {
        serialize(field.content, value[field.name], data, prefixed);
      }, `Serialize: Field: ${field.name}`);
    }
  },
  unnamed: (
    self: Array<ToolboxIdlTypeFullFieldUnnamed>,
    value: any,
    data: Array<Buffer>,
    prefixed: boolean,
  ) => {
    if (self.length <= 0) {
      return;
    }
    ToolboxUtils.expectArray(value);
    for (const field of self) {
      ToolboxUtils.withContext(() => {
        serialize(field.content, value[field.position], data, prefixed);
      }, `Serialize: Field: ${field.position}`);
    }
  },
};

const serializePrefixVisitor = {
  u8: (buffer: Buffer, value: bigint) => {
    buffer.writeUInt8(Number(value));
  },
  u16: (buffer: Buffer, value: bigint) => {
    buffer.writeUInt16LE(Number(value));
  },
  u32: (buffer: Buffer, value: bigint) => {
    buffer.writeUInt32LE(Number(value));
  },
  u64: (buffer: Buffer, value: bigint) => {
    buffer.writeBigUInt64LE(value);
  },
  u128: (buffer: Buffer, value: bigint) => {
    const low = value & 0xffffffffffffffffn;
    const high = (value >> 64n) & 0xffffffffffffffffn;
    buffer.writeBigUInt64LE(low, 0);
    buffer.writeBigUInt64LE(high, 8);
  },
};

const serializePrimitiveVisitor = {
  u8: (buffer: Buffer, value: any) => {
    buffer.writeUInt8(ToolboxUtils.expectNumber(value));
  },
  u16: (buffer: Buffer, value: any) => {
    buffer.writeUInt16LE(ToolboxUtils.expectNumber(value));
  },
  u32: (buffer: Buffer, value: any) => {
    buffer.writeUInt32LE(ToolboxUtils.expectNumber(value));
  },
  u64: (buffer: Buffer, value: any) => {
    buffer.writeBigUInt64LE(expectInteger(value));
  },
  u128: (buffer: Buffer, value: any) => {
    const num = expectInteger(value);
    const low = num & 0xffffffffffffffffn;
    const high = (num >> 64n) & 0xffffffffffffffffn;
    buffer.writeBigUInt64LE(low, 0);
    buffer.writeBigUInt64LE(high, 8);
  },
  i8: (buffer: Buffer, value: any) => {
    buffer.writeInt8(ToolboxUtils.expectNumber(value));
  },
  i16: (buffer: Buffer, value: any) => {
    buffer.writeInt16LE(ToolboxUtils.expectNumber(value));
  },
  i32: (buffer: Buffer, value: any) => {
    buffer.writeInt32LE(ToolboxUtils.expectNumber(value));
  },
  i64: (buffer: Buffer, value: any) => {
    buffer.writeBigInt64LE(expectInteger(value));
  },
  i128: (buffer: Buffer, value: any) => {
    const num = expectInteger(value);
    const low = BigInt.asIntN(64, num);
    const high = BigInt.asIntN(64, num >> 64n);
    buffer.writeBigInt64LE(low, 0);
    buffer.writeBigInt64LE(high, 8);
  },
  f32: (buffer: Buffer, value: any) => {
    buffer.writeFloatLE(ToolboxUtils.expectNumber(value));
  },
  f64: (buffer: Buffer, value: any) => {
    buffer.writeDoubleLE(ToolboxUtils.expectNumber(value));
  },
  bool: (buffer: Buffer, value: any) => {
    if (ToolboxUtils.expectBoolean(value)) {
      buffer.writeUInt8(1);
    } else {
      buffer.writeUInt8(0);
    }
  },
  pubkey: (buffer: Buffer, value: any) => {
    buffer.set(new PublicKey(ToolboxUtils.expectString(value)).toBuffer());
  },
};

function expectInteger(value: any): bigint {
  if (ToolboxUtils.isNumber(value) || ToolboxUtils.isString(value)) {
    return BigInt(value);
  }
  throw new Error(`Expected an integer (found: ${typeof value})`);
}
