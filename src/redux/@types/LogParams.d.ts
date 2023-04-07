import { InvokeArgs } from "@tauri-apps/api/tauri";

export interface LogParams extends InvokeArgs {
  url?: string;
  enviromentName?: string;
  accessData?: {
    username?: string;
    password?: string;
  };
  total?: number;
  moduleId?: string;
  btnId?: string;
}
