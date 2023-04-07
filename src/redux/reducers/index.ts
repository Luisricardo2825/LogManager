import { AddEnviroment, RemoveEnviroment } from "../actions/Enviroment";
import { LogParams } from "../@types/LogParams";

export const initialState: LogParams[] = [];

export function enviromentsReducer(
  state = initialState,
  action: { type: any; payload: LogParams }
): LogParams[] {
  switch (action.type) {
    case "AddEnviroment":
      return [...AddEnviroment(state, action)];
    case "RemoveEnviroment":
      return [...RemoveEnviroment(state, action)];
    default:
      return state;
  }
}

export function enviromentReducer(
  state: LogParams | null = null,
  action: { type: any; payload: LogParams }
): LogParams {
  switch (action.type) {
    case "AddSelectedEnviroment":
      return { ...state, ...action.payload };
    case "RemoveSelectedEnviroment":
      return { ...state, ...action.payload };
    default:
      return state;
  }
}
