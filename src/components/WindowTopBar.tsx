import {
  Box,
  Button,
  DarkMode,
  Flex,
  IconButton,
  useColorMode,
  useColorModeValue,
} from "@chakra-ui/react";
import React from "react";
import { FiMaximize, FiMinus, FiMoon, FiSun, FiX } from "react-icons/fi";

export default function WindowTopBar() {
  const { colorMode, toggleColorMode } = useColorMode();

  return (
    <Flex
      width={"100%"}
      zIndex={"7"}
      bg={useColorModeValue("white", "gray.900")}
      flexDir={"row-reverse"}
      data-tauri-drag-region
      className="titlebar"
    >
      <Box bg="teal">
        <DarkMode>
          <Button
            onClick={toggleColorMode}
            variant={"ghost"}
            height={30}
            borderRadius={0}
          >
            {colorMode === "light" ? <FiMoon /> : <FiSun />}
          </Button>
          <IconButton
            aria-label="minimize"
            className="titlebar-button"
            id="titlebar-minimize"
            variant={"ghost"}
            icon={<FiMinus />}
            height={30}
            borderRadius={0}
          />

          <IconButton
            aria-label="maximize"
            className="titlebar-button"
            id="titlebar-maximize"
            variant={"ghost"}
            icon={<FiMaximize />}
            height={30}
            borderRadius={0}
          />
          <IconButton
            aria-label="close"
            className="titlebar-button"
            id="titlebar-close"
            variant={"ghost"}
            icon={<FiX />}
            height={30}
            borderRadius={0}
          />
        </DarkMode>
      </Box>
    </Flex>
  );
}
