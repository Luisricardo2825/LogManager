import { combineReducers, configureStore } from "@reduxjs/toolkit";
import { enviromentReducer, enviromentsReducer } from "../reducers";
import { persistReducer, persistStore } from "redux-persist";
import storage from "redux-persist/lib/storage";
import thunk from "redux-thunk";

// Configuração de persistência
const rootPersistConfig = {
  key: "root",
  storage,
  // blacklist: ["selectedEnviroment"],
};

// Adicionar os reducers aqui
const reducers = {
  enviroments: enviromentsReducer,
  selectedEnviroment: enviromentReducer,
};

const combinedReducers = combineReducers(reducers);
const reducer = persistReducer(rootPersistConfig, combinedReducers);

export const store = configureStore({
  reducer: reducer,
  devTools: process.env.NODE_ENV !== "production",
  middleware: [thunk],
});
export const storePersistor = persistStore(store);
export type RootState = ReturnType<typeof store.getState>;
