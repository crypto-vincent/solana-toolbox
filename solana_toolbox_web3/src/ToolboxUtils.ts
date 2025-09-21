import { sha256 } from 'sha.js';
import { parse } from './ToolboxIdlTypeFlat.parse';
import { hydrate } from './ToolboxIdlTypeFlat.hydrate';
import { serialize } from './ToolboxIdlTypeFull.serialize';
import bs58 from 'bs58';

export class ToolboxUtils {
  public static isBoolean(value: any): boolean {
    return typeof value === 'boolean' || value instanceof Boolean;
  }
  public static isNumber(value: any): boolean {
    return typeof value === 'number' || value instanceof Number;
  }
  public static isString(value: any): boolean {
    return typeof value === 'string' || value instanceof String;
  }
  public static isArray(value: any): boolean {
    return Array.isArray(value);
  }
  public static isObject(value: any): boolean {
    return typeof value === 'object' && !Array.isArray(value) && value !== null;
  }

  public static expectBoolean(value: any): boolean {
    if (!ToolboxUtils.isBoolean(value)) {
      throw new Error(`Expected a boolean (found: ${typeof value})`);
    }
    return value;
  }
  public static expectNumber(value: any): number {
    if (!ToolboxUtils.isNumber(value)) {
      throw new Error(`Expected a number (found: ${typeof value})`);
    }
    return value;
  }
  public static expectString(value: any): string {
    if (!ToolboxUtils.isString(value)) {
      throw new Error(`Expected a string (found: ${typeof value})`);
    }
    return value;
  }
  public static expectArray(value: any): any[] {
    if (!ToolboxUtils.isArray(value)) {
      throw new Error(`Expected an array (found: ${typeof value})`);
    }
    return value;
  }
  public static expectObject(value: any): Record<string, any> {
    if (!ToolboxUtils.isObject(value)) {
      throw new Error(`Expected an object (found: ${typeof value})`);
    }
    return value;
  }

  public static convertToSnakeCase(value: string) {
    return value
      .replace(/([a-z0-9])([A-Z])/g, '$1_$2')
      .replace(/([A-Z])([A-Z][a-z])/g, '$1_$2')
      .toLowerCase();
  }

  public static discriminator(value: string) {
    return new sha256().update(value).digest().subarray(0, 8);
  }

  public static withContext<T>(fn: () => T, message: string): T {
    try {
      return fn();
    } catch (err) {
      throw new Error(
        `${message}\n > ${err instanceof Error ? err.message : String(err)}`,
      );
    }
  }

  public static expectBytes(value: any): Buffer {
    if (ToolboxUtils.isString(value)) {
      return Buffer.from(value, 'utf8');
    }
    if (ToolboxUtils.isArray(value)) {
      const bytes = Buffer.alloc(value.length);
      for (let i = 0; i < value.length; i++) {
        const byte = ToolboxUtils.expectNumber(value[i]);
        if (byte < 0 || byte > 255) {
          throw new Error(
            `Expected byte to be in range 0-255 (found: ${byte})`,
          );
        }
        bytes[i] = byte;
      }
      return bytes;
    }
    if (ToolboxUtils.isObject(value)) {
      if (value.hasOwnProperty('base16')) {
        const base16 = ToolboxUtils.expectString(value['base16']);
        return Buffer.from(base16, 'hex');
      }
      if (value.hasOwnProperty('base58')) {
        const base58 = ToolboxUtils.expectString(value['base58']);
        return bs58.decode(base58);
      }
      if (value.hasOwnProperty('base64')) {
        const base64 = ToolboxUtils.expectString(value['base64']);
        return Buffer.from(base64, 'base64');
      }
      if (value.hasOwnProperty('utf8')) {
        const utf8 = ToolboxUtils.expectString(value['utf8']);
        return Buffer.from(utf8, 'utf8');
      }
      if (value.hasOwnProperty('value')) {
        const typeFlat = parse(value['type']);
        const typeFull = hydrate(typeFlat, new Map(), new Map());
        const data: Array<Buffer> = [];
        serialize(typeFull, value['value'], data, !!value['prefixed']);
        return Buffer.concat(data);
      }
    }
    throw new Error(
      `Expected value to be a string, array, or object (found: ${typeof value})`,
    );
  }
}
