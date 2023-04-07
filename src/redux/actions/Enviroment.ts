import type { LogParams } from "../@types/LogParams";

export function AddEnviroment(
  state: LogParams[] = [],
  action: { type: any; payload: LogParams }
): LogParams[] {
  const newState = state.find((env) => env.url === action.payload.url);
  if (!Boolean(newState)) {
    return [...state, action.payload];
  }

  return [...state];
}

export function RemoveEnviroment(
  state: LogParams[] = [],
  action: { type: any; payload: LogParams }
): LogParams[] {
  const newState = state.filter((env) => env.url !== action.payload.url);
  return [...newState];
}
