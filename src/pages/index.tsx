import React from "react";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Flex, useToast, UseToastOptions } from "@chakra-ui/react";
import { useSelector } from "react-redux";
import { RootState } from "../redux/store";
import type { LogParams } from "../redux/@types/LogParams";
import TextEditor from "../components/Texteditor";

// emit an event that is only visible to the current window
let count = 0;
let loading = false;
function App() {
  const [log, setLog] = useState<string>("");
  const toast = useToast();
  const { selectedEnviroment } = useSelector<RootState, RootState>(
    (state) => state
  );
  const total = 1000;
  const LoadLog = React.useCallback(async () => {
    const id = "error_log";

    const id_interval = setInterval(
      (function refresh() {
        if (!loading) {
          GetLog(selectedEnviroment, total, setLog, id, toast);
        }
        return refresh;
      })(),
      10000
    );
    return id_interval;
  }, [selectedEnviroment]);

  React.useEffect(() => {
    toast.closeAll();
    const id = LoadLog();
    return () => {
      setLog("");
      id.then((id) => {
        clearInterval(id);
      });
    };
  }, [selectedEnviroment]);

  return (
    <Flex flexDirection={"column"} height={"100%"}>
      <TextEditor value={log} readonly />
    </Flex>
  );
}

function GetLog(
  selectedEnviroment: LogParams,
  total: number,
  setLog: React.Dispatch<React.SetStateAction<string>>,
  id: string,
  toast
) {
  loading = true;
  invoke<string>("get_log_btn", {
    ...selectedEnviroment,
    total: total,
  })
    .then((data) => {
      setLog(data);
    })
    .catch((data) => {
      try {
        if (!isJson(data)) throw new Error("Data is'nt a valid json");
        const body = JSON.parse(data).retBody;
        if (!isJson(body)) throw new Error("Body is'nt a valid json");
      } catch (error) {
        count++;
        const toast_conf: UseToastOptions = {
          id,
          title: "Error ao buscar log",
          description: `${error} x${count}`,
          status: "error",
          isClosable: true,
          position: "top",
        };

        if (toast.isActive(id)) return toast.update(id, toast_conf);
        toast(toast_conf);
      }
    })
    .finally(() => {
      loading = false;
    });
}

function isJson(json: string) {
  try {
    JSON.parse(json);
  } catch (error) {
    return false;
  }
  return true;
}

export default React.memo(App);
