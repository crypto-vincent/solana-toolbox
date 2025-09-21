import { PublicKey } from '@solana/web3.js';
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

export function deserialize(
  typeFull: ToolboxIdlTypeFull,
  data: Buffer,
  dataOffset: number,
): [number, any] {
  return typeFull.traverse(deserializeVisitor, data, dataOffset, undefined);
}

export function deserializeFields(
  fields: ToolboxIdlTypeFullFields,
  data: Buffer,
  dataOffset: number,
): [number, any] {
  return fields.traverse(deserializeFieldsVisitor, data, dataOffset, undefined);
}

export function deserializePrefix(
  prefix: ToolboxIdlTypePrefix,
  data: Buffer,
  dataOffset: number,
): [number, bigint] {
  return [
    prefix.size,
    prefix.traverse(deserializePrefixVisitor, data, dataOffset),
  ];
}

export function deserializePrimitive(
  primitive: ToolboxIdlTypePrimitive,
  data: Buffer,
  dataOffset: number,
): [number, any] {
  return [
    primitive.size,
    primitive.traverse(deserializePrimitiveVisitor, data, dataOffset),
  ];
}

const deserializeVisitor = {
  typedef: (
    self: ToolboxIdlTypeFullTypedef,
    data: Buffer,
    dataOffset: number,
  ): [number, any] => {
    return ToolboxUtils.withContext(() => {
      return deserialize(self.content, data, dataOffset);
    }, `Deserialize: Typedef: ${self.name} (offset: ${dataOffset})`);
  },
  option: (
    self: ToolboxIdlTypeFullOption,
    data: Buffer,
    dataOffset: number,
  ): [number, any] => {
    let [dataSize, dataPrefix] = deserializePrefix(
      self.prefix,
      data,
      dataOffset,
    );
    if ((dataPrefix & 1n) == 0n) {
      return [dataSize, null];
    }
    const dataContentOffset = dataOffset + dataSize;
    const [dataContentSize, dataContent] = deserialize(
      self.content,
      data,
      dataContentOffset,
    );
    dataSize += dataContentSize;
    return [dataSize, dataContent];
  },
  vec: (
    self: ToolboxIdlTypeFullVec,
    data: Buffer,
    dataOffset: number,
  ): [number, any] => {
    let [dataSize, dataPrefix] = deserializePrefix(
      self.prefix,
      data,
      dataOffset,
    );
    const dataLength = Number(dataPrefix);
    const dataItems = [];
    for (let i = 0; i < dataLength; i++) {
      const dataItemOffset = dataOffset + dataSize;
      const [dataItemSize, dataItem] = deserialize(
        self.items,
        data,
        dataItemOffset,
      );
      dataSize += dataItemSize;
      dataItems.push(dataItem);
    }
    return [dataSize, dataItems];
  },
  array: (
    self: ToolboxIdlTypeFullArray,
    data: Buffer,
    dataOffset: number,
  ): [number, any] => {
    let dataSize = 0;
    const dataItems = [];
    for (let i = 0; i < self.length; i++) {
      const dataItemOffset = dataOffset + dataSize;
      const [dataItemSize, dataItem] = deserialize(
        self.items,
        data,
        dataItemOffset,
      );
      dataSize += dataItemSize;
      dataItems.push(dataItem);
    }
    return [dataSize, dataItems];
  },
  string: (
    self: ToolboxIdlTypeFullString,
    data: Buffer,
    dataOffset: number,
  ): [number, any] => {
    let [dataSize, dataPrefix] = deserializePrefix(
      self.prefix,
      data,
      dataOffset,
    );
    const dataLength = Number(dataPrefix);
    const dataCharsOffset = dataOffset + dataSize;
    const dataString = data.toString(
      'utf8',
      dataCharsOffset,
      dataCharsOffset + dataLength,
    );
    dataSize += dataLength;
    return [dataSize, dataString];
  },
  struct: (
    self: ToolboxIdlTypeFullStruct,
    data: Buffer,
    dataOffset: number,
  ): [number, any] => {
    return deserializeFields(self.fields, data, dataOffset);
  },
  enum: (
    self: ToolboxIdlTypeFullEnum,
    data: Buffer,
    dataOffset: number,
  ): [number, any] => {
    if (self.variants.length == 0) {
      return [0, null];
    }
    let enumMask = 0n;
    for (const variant of self.variants) {
      enumMask |= variant.code;
    }
    let [dataSize, dataPrefix] = deserializePrefix(
      self.prefix,
      data,
      dataOffset,
    );
    const dataVariantOffset = dataOffset + dataSize;
    for (const variant of self.variants) {
      if (variant.code === (dataPrefix & enumMask)) {
        if (variant.fields.isNothing()) {
          return [dataSize, variant.name];
        }
        const [dataVariantSize, dataVariant] = ToolboxUtils.withContext(() => {
          return deserializeFields(variant.fields, data, dataVariantOffset);
        }, `Deserialize: Enum Variant: ${variant.name} (offset: ${dataVariantOffset})`);
        dataSize += dataVariantSize;
        return [dataSize, { [variant.name]: dataVariant }];
      }
    }
    throw new Error(
      `Deserialize: Unknown enum code: ${dataPrefix} (offset: ${dataOffset})`,
    );
  },
  padded: (
    self: ToolboxIdlTypeFullPadded,
    data: Buffer,
    dataOffset: number,
  ): [number, any] => {
    let dataSize = self.before;
    const dataContentOffset = dataOffset + dataSize;
    const [dataContentSize, dataContent] = deserialize(
      self.content,
      data,
      dataContentOffset,
    );
    dataSize += Math.max(dataContentSize, self.minSize);
    dataSize += self.after;
    return [dataSize, dataContent];
  },
  primitive: (
    self: ToolboxIdlTypePrimitive,
    data: Buffer,
    dataOffset: number,
  ): [number, any] => {
    return deserializePrimitive(self, data, dataOffset);
  },
};

const deserializeFieldsVisitor = {
  nothing: (_self: {}, _data: Buffer, _dataOffset: number): [number, any] => {
    return [0, null];
  },
  named: (
    self: Array<ToolboxIdlTypeFullFieldNamed>,
    data: Buffer,
    dataOffset: number,
  ): [number, any] => {
    let dataSize = 0;
    const dataFields: Record<string, any> = {};
    for (const field of self) {
      const dataFieldOffset = dataOffset + dataSize;
      const [dataFieldSize, dataField] = ToolboxUtils.withContext(() => {
        return deserialize(field.content, data, dataFieldOffset);
      }, `Deserialize: Field: ${field.name} (offset: ${dataFieldOffset})`);
      dataSize += dataFieldSize;
      dataFields[field.name] = dataField;
    }
    return [dataSize, dataFields];
  },
  unnamed: (
    self: Array<ToolboxIdlTypeFullFieldUnnamed>,
    data: Buffer,
    dataOffset: number,
  ): [number, any] => {
    let dataSize = 0;
    const dataFields = [];
    for (const field of self) {
      const dataFieldOffset = dataOffset + dataSize;
      const [dataFieldSize, dataField] = ToolboxUtils.withContext(() => {
        return deserialize(field.content, data, dataFieldOffset);
      }, `Deserialize: Field: ${field.position} (offset: ${dataFieldOffset})`);
      dataSize += dataFieldSize;
      dataFields.push(dataField);
    }
    return [dataSize, dataFields];
  },
};

const deserializePrefixVisitor = {
  u8: (data: Buffer, dataOffset: number): bigint => {
    return BigInt(data.readUInt8(dataOffset));
  },
  u16: (data: Buffer, dataOffset: number): bigint => {
    return BigInt(data.readUInt16LE(dataOffset));
  },
  u32: (data: Buffer, dataOffset: number): bigint => {
    return BigInt(data.readUInt32LE(dataOffset));
  },
  u64: (data: Buffer, dataOffset: number): bigint => {
    return data.readBigUInt64LE(dataOffset);
  },
  u128: (data: Buffer, dataOffset: number): bigint => {
    const low = data.readBigUInt64LE(dataOffset);
    const high = data.readBigUInt64LE(dataOffset + 8);
    return low | (high << 64n);
  },
};

const deserializePrimitiveVisitor = {
  u8: (data: Buffer, dataOffset: number): any => {
    return data.readUInt8(dataOffset);
  },
  u16: (data: Buffer, dataOffset: number): any => {
    return data.readUInt16LE(dataOffset);
  },
  u32: (data: Buffer, dataOffset: number): any => {
    return data.readUInt32LE(dataOffset);
  },
  u64: (data: Buffer, dataOffset: number): any => {
    return data.readBigUInt64LE(dataOffset).toString();
  },
  u128: (data: Buffer, dataOffset: number): any => {
    const low = data.readBigUInt64LE(dataOffset);
    const high = data.readBigUInt64LE(dataOffset + 8);
    return (low | (high << 64n)).toString();
  },
  i8: (data: Buffer, dataOffset: number): any => {
    return data.readInt8(dataOffset);
  },
  i16: (data: Buffer, dataOffset: number): any => {
    return data.readInt16LE(dataOffset);
  },
  i32: (data: Buffer, dataOffset: number): any => {
    return data.readInt32LE(dataOffset);
  },
  i64: (data: Buffer, dataOffset: number): any => {
    return data.readBigInt64LE(dataOffset).toString();
  },
  i128: (data: Buffer, dataOffset: number): any => {
    const low = data.readBigUInt64LE(dataOffset);
    const high = data.readBigInt64LE(dataOffset + 8);
    return (low | (high << 64n)).toString();
  },
  f32: (data: Buffer, dataOffset: number): any => {
    return data.readFloatLE(dataOffset);
  },
  f64: (data: Buffer, dataOffset: number): any => {
    return data.readDoubleLE(dataOffset);
  },
  bool: (data: Buffer, dataOffset: number): any => {
    return data.readUInt8(dataOffset) != 0;
  },
  pubkey: (data: Buffer, dataOffset: number): any => {
    return new PublicKey(data.subarray(dataOffset, dataOffset + 32)).toBase58();
  },
};
