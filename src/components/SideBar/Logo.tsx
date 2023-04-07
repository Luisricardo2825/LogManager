import React from "react";
import {
  Avatar,
  AvatarGroup,
  Flex,
  Text,
  useColorMode,
} from "@chakra-ui/react";

export function Logo(): JSX.Element {
  const { colorMode } = useColorMode();
  const iniColor = colorMode === "light" ? "#3d6632" : "#3d6632";
  const endColor = colorMode === "light" ? "#0acc27" : "#43eb5f";
  return (
    <Flex h="20" alignItems="center" justifyContent="center">
      <AvatarGroup as={"div"} size="sm" mx={0}>
        <Avatar src={"/assets/icon.png"} />
      </AvatarGroup>
      <Text
        bgGradient={`linear(to-r, ${iniColor}, ${endColor})`}
        bgClip="text"
        fontSize="1.2rem"
        fontWeight="extrabold"

      >
        Log Manager
      </Text>
    </Flex>
  );
}
