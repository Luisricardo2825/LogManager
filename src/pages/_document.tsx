import React from "react";
import { Html, Head, Main, NextScript } from "next/document";

export default function Document() {
  return (
    <Html>
      <React.StrictMode>
        <Head />
        <body>
          <Main />
          <NextScript />
        </body>
      </React.StrictMode>
    </Html>
  );
}
