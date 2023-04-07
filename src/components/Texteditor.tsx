import React from "react";
import {
  Box,
  Spinner,
  useBreakpointValue,
  useColorMode,
} from "@chakra-ui/react";
import Editor from "@monaco-editor/react";
interface Props {
  value: string;
  readonly?: boolean;
}
const TextEditor = (props: Props) => {
  const { colorMode } = useColorMode();
  // const size = useBreakpointValue(["95vh", "95vh", "95vh", "97vh"]);

  return (
    <Editor
      height={"100%"}
      onChange={(e) => (e = e)}
      defaultLanguage="javascript"
      theme={colorMode === "light" ? "light" : "vs-dark"}
      defaultValue="// nada por aqui 😴"
      value={props.value}
      options={{
        readOnly: props.readonly,
      }}
      loading={<Spinner />}
    />
  );
};

export default React.memo(TextEditor);
