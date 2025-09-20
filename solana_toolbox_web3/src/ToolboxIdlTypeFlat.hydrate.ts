import { ToolboxIdlTypedef } from './ToolboxIdlTypedef';
import {
  ToolboxIdlTypeFlat,
  ToolboxIdlTypeFlatArray,
  ToolboxIdlTypeFlatConst,
  ToolboxIdlTypeFlatDefined,
  ToolboxIdlTypeFlatEnum,
  ToolboxIdlTypeFlatFieldNamed,
  ToolboxIdlTypeFlatFields,
  ToolboxIdlTypeFlatFieldUnnamed,
  ToolboxIdlTypeFlatGeneric,
  ToolboxIdlTypeFlatOption,
  ToolboxIdlTypeFlatPadded,
  ToolboxIdlTypeFlatString,
  ToolboxIdlTypeFlatStruct,
  ToolboxIdlTypeFlatVec,
} from './ToolboxIdlTypeFlat';
import {
  ToolboxIdlTypeFull,
  ToolboxIdlTypeFullFields,
} from './ToolboxIdlTypeFull';
import { bytemuck } from './ToolboxIdlTypeFull.bytemuck';
import { ToolboxIdlTypePrimitive } from './ToolboxIdlTypePrimitive';

export function hydrate(
  typeFlat: ToolboxIdlTypeFlat,
  genericsBySymbol: Map<string, ToolboxIdlTypeFull | number>,
  typedefs: Map<string, ToolboxIdlTypedef>,
): ToolboxIdlTypeFull {
  const typeFullOrConstLiteral = hydrateOrConstLiteral(
    typeFlat,
    genericsBySymbol,
    typedefs,
  );
  if (typeof typeFullOrConstLiteral === 'number') {
    throw new Error('Const is not supported as a standalone type');
  }
  return typeFullOrConstLiteral;
}

export function hydrateOrConstLiteral(
  typeFlat: ToolboxIdlTypeFlat,
  genericsBySymbol: Map<string, ToolboxIdlTypeFull | number>,
  typedefs: Map<string, ToolboxIdlTypedef>,
): ToolboxIdlTypeFull | number {
  return typeFlat.traverse(
    hydrateOrConstLiteralVisitor,
    genericsBySymbol,
    typedefs,
  );
}

let hydrateOrConstLiteralVisitor = {
  defined: (
    self: ToolboxIdlTypeFlatDefined,
    genericsBySymbol: Map<string, ToolboxIdlTypeFull | number>,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlTypeFull | number => {
    const typedef = typedefs.get(self.name);
    if (typedef === undefined) {
      throw new Error(`Could not resolve type named: ${self.name}`);
    }
    if (self.generics.length < typedef.generics.length) {
      throw new Error('Insufficient set of generics');
    }
    const genericsFull = self.generics.map(
      (genericFlat: ToolboxIdlTypeFlat) => {
        return hydrateOrConstLiteral(genericFlat, genericsBySymbol, typedefs);
      },
    ); // TODO - this could be safer and cleaner by using strong types
    const innerGenericsBySymbol = new Map<
      string,
      ToolboxIdlTypeFull | number
    >();
    for (let i = 0; i < typedef.generics.length; i++) {
      innerGenericsBySymbol.set(typedef.generics[i]!, genericsFull[i]!);
    }
    const typeFull = hydrate(typedef.typeFlat, innerGenericsBySymbol, typedefs);
    const typeTypedef = {
      name: typedef.name,
      repr: typedef.repr,
      content: typeFull,
    };
    if (typedef.serialization === 'bytemuck') {
      return bytemuck(typeTypedef).value;
    }
    return ToolboxIdlTypeFull.typedef(typeTypedef);
  },
  generic: (
    self: ToolboxIdlTypeFlatGeneric,
    genericsBySymbol: Map<string, ToolboxIdlTypeFull | number>,
    _typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlTypeFull | number => {
    let typeFull = genericsBySymbol.get(self.symbol);
    if (typeFull === undefined) {
      throw new Error(`Could not resolve generic named: ${self.symbol}`);
    }
    return typeFull;
  },
  option: (
    self: ToolboxIdlTypeFlatOption,
    genericsBySymbol: Map<string, ToolboxIdlTypeFull | number>,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlTypeFull | number => {
    return ToolboxIdlTypeFull.option({
      prefix: self.prefix,
      content: hydrate(self.content, genericsBySymbol, typedefs),
    });
  },
  vec: (
    self: ToolboxIdlTypeFlatVec,
    genericsBySymbol: Map<string, ToolboxIdlTypeFull | number>,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlTypeFull | number => {
    return ToolboxIdlTypeFull.vec({
      prefix: self.prefix,
      items: hydrate(self.items, genericsBySymbol, typedefs),
    });
  },
  array: (
    self: ToolboxIdlTypeFlatArray,
    genericsBySymbol: Map<string, ToolboxIdlTypeFull | number>,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlTypeFull | number => {
    const length = hydrateOrConstLiteral(
      self.length,
      genericsBySymbol,
      typedefs,
    );
    if (typeof length !== 'number') {
      throw new Error('Array length must resolve to a const literal number');
    }
    return ToolboxIdlTypeFull.array({
      length,
      items: hydrate(self.items, genericsBySymbol, typedefs),
    });
  },
  string: (
    self: ToolboxIdlTypeFlatString,
    _genericsBySymbol: Map<string, ToolboxIdlTypeFull | number>,
    _typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlTypeFull | number => {
    return ToolboxIdlTypeFull.string({
      prefix: self.prefix,
    });
  },
  struct: (
    self: ToolboxIdlTypeFlatStruct,
    genericsBySymbol: Map<string, ToolboxIdlTypeFull | number>,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlTypeFull | number => {
    return ToolboxIdlTypeFull.struct({
      fields: hydrateFields(self.fields, genericsBySymbol, typedefs),
    });
  },
  enum: (
    self: ToolboxIdlTypeFlatEnum,
    genericsBySymbol: Map<string, ToolboxIdlTypeFull | number>,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlTypeFull | number => {
    return ToolboxIdlTypeFull.enum({
      prefix: self.prefix,
      variants: self.variants.map((variant) => {
        return {
          name: variant.name,
          code: variant.code,
          fields: hydrateFields(variant.fields, genericsBySymbol, typedefs),
        };
      }),
    });
  },
  padded: (
    self: ToolboxIdlTypeFlatPadded,
    genericsBySymbol: Map<string, ToolboxIdlTypeFull | number>,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlTypeFull | number => {
    return ToolboxIdlTypeFull.padded({
      before: self.before,
      minSize: self.minSize,
      after: self.after,
      content: hydrate(self.content, genericsBySymbol, typedefs),
    });
  },
  const: (
    self: ToolboxIdlTypeFlatConst,
    _genericsBySymbol: Map<string, ToolboxIdlTypeFull | number>,
    _typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlTypeFull | number => {
    return self.literal;
  },
  primitive: (
    self: ToolboxIdlTypePrimitive,
    _genericsBySymbol: Map<string, ToolboxIdlTypeFull | number>,
    _typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlTypeFull | number => {
    return ToolboxIdlTypeFull.primitive(self);
  },
};

export function hydrateFields(
  typeFlatFields: ToolboxIdlTypeFlatFields,
  genericsBySymbol: Map<string, ToolboxIdlTypeFull | number>,
  typedefs: Map<string, ToolboxIdlTypedef>,
): ToolboxIdlTypeFullFields {
  return typeFlatFields.traverse(
    hydrateFieldsVisitor,
    genericsBySymbol,
    typedefs,
  );
}

let hydrateFieldsVisitor = {
  nothing: (
    _self: {},
    _genericsBySymbol: Map<string, ToolboxIdlTypeFull | number>,
    _typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlTypeFullFields => {
    return ToolboxIdlTypeFullFields.nothing();
  },
  named: (
    self: ToolboxIdlTypeFlatFieldNamed[],
    genericsBySymbol: Map<string, ToolboxIdlTypeFull | number>,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlTypeFullFields => {
    return ToolboxIdlTypeFullFields.named(
      self.map((field) => {
        return {
          name: field.name,
          content: hydrate(field.content, genericsBySymbol, typedefs),
        };
      }),
    );
  },
  unnamed: (
    self: ToolboxIdlTypeFlatFieldUnnamed[],
    genericsBySymbol: Map<string, ToolboxIdlTypeFull | number>,
    typedefs: Map<string, ToolboxIdlTypedef>,
  ): ToolboxIdlTypeFullFields => {
    return ToolboxIdlTypeFullFields.unnamed(
      self.map((field, index) => {
        return {
          position: index,
          content: hydrate(field.content, genericsBySymbol, typedefs),
        };
      }),
    );
  },
};
