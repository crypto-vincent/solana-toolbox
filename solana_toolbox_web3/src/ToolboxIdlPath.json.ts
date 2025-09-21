import { ToolboxIdlPath } from './ToolboxIdlPath';
import { ToolboxUtils } from './ToolboxUtils';

export function pathGetJsonValue(path: ToolboxIdlPath, value: any): any {
  const split = path.splitFirst();
  if (split === undefined) {
    return value;
  }
  const current = split.first;
  const next = split.rest;
  if (ToolboxUtils.isArray(value)) {
    const length = value.length;
    const index = current.isEmpty() ? 0n : current.index();
    if (index === undefined) {
      throw new Error(`Expected array index but got '${current.value()}'`);
    }
    const indexNumber = Number(index);
    if (indexNumber < 0 || indexNumber >= length) {
      throw new Error(
        `Index ${indexNumber} out of bounds for array of length ${length}`,
      );
    }
    return pathGetJsonValue(next, value[indexNumber]);
  }
  if (ToolboxUtils.isObject(value)) {
    const key = current.value();
    if (!value.hasOwnProperty(key)) {
      throw new Error(`Key '${key}' not found in object`);
    }
    return pathGetJsonValue(next, value[key]);
  }
  throw new Error(
    `Expected array or object but got '${typeof value}' when traversing path`,
  );
}
