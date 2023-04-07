import type { AppProps } from "next/app";

import "../style.css";
import "../App.css";
// 1. import `ChakraProvider` component
import { Box, ChakraProvider, Flex, useColorModeValue } from "@chakra-ui/react";
import SideBar from "../components/SideBar/SideBar";
import { Provider } from "react-redux";
import { store, storePersistor } from "../redux/store";
import { PersistGate } from "redux-persist/integration/react";
import WindowTopBar from "../components/WindowTopBar";
import React from "react";

const isProd = process.env.NODE_ENV === "production";
// This default export is required in a new `pages/_app.js` file.
export default function MyApp({ Component, pageProps }: AppProps) {
  if (isProd) DisableBrowserFunctions(); //Desabilita funções de recarregar e de menu de contexto dentro do browser

  React.useEffect(() => {
    customToolBar();

    return () => {
      customToolBar(false);
    };
  }, []);
  return (
    <ChakraProvider>
      {/* Habilita o Devtools no ambiente dev */}
      {!isProd && <script src="http://localhost:8097"></script>}
      <Flex>
        <WindowTopBar />
      </Flex>
      <Provider store={store}>
        <PersistGate persistor={storePersistor} loading={null}>
          <SideBar />
          <Box
            transition={"all 0.2s ease-in-out"}
            bg={useColorModeValue("gray.50", "gray.800")}
            minH={["95vh", "95vh", "95vh", "97vh"]}
            maxH={["95vh", "95vh", "95vh", "97vh"]}
            height={["95vh", "95vh", "95vh", "97vh"]}
            overflow={"hidden"}
          >
            <Component {...pageProps} />
          </Box>
        </PersistGate>
      </Provider>
    </ChakraProvider>
  );
}

function DisableBrowserFunctions() {
  if (typeof document !== "undefined") {
    document.addEventListener("contextmenu", (event) => event.preventDefault());
    document.addEventListener("keydown", (e) => {
      e = e || (window.event as KeyboardEvent);
      if (
        e.code == "F5" ||
        (e.ctrlKey && e.code.toLocaleUpperCase() == "KEYR")
      ) {
        e.preventDefault();
      }
    });
  }
}

async function customToolBar(add = true) {
  const { appWindow } = await import("@tauri-apps/api/window");
  if (add) {
    document
      .getElementById("titlebar-minimize")
      .addEventListener("click", () => appWindow.minimize());
    document
      .getElementById("titlebar-maximize")
      .addEventListener("click", () => appWindow.toggleMaximize());
    document
      .getElementById("titlebar-close")
      .addEventListener("click", () => appWindow.close());
  } else {
    document
      .getElementById("titlebar-minimize")
      .removeEventListener("click", () => {});
    document
      .getElementById("titlebar-maximize")
      .removeEventListener("click", () => {});
    document
      .getElementById("titlebar-close")
      .removeEventListener("click", () => {});
  }
}
