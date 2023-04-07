import React from "react";
import {
  Flex, Link,
  Text, Image
} from "@chakra-ui/react";

export function Footer() {
  return (
    <Flex
      position={"absolute"}
      bottom={"5"}
      justifyContent="center"
      alignItems={"center"}
      width={"full"}
      flexDirection={"column"}
    >
      <Image
        src="https://avatars.githubusercontent.com/u/49820752?v=4"
        width={5}
      />
      <Text fontSize="2xs" fontFamily="monospace" fontWeight="thin">
        By{" "}
        <Link href="https://github.com/Luisricardo2825" target={"_blank"}>
          Ricardo Alves
        </Link>
      </Text>
    </Flex>
  );
}
