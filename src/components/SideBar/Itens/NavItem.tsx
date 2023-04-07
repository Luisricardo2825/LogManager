import React, { ReactNode } from "react";
import { Flex, Icon, Link, FlexProps } from "@chakra-ui/react";
import { IconType } from "react-icons";
import NextLink from "next/link";

interface NavItemProps extends FlexProps {
  icon: IconType;
  children: ReactNode;
  href?: string;
}
const NavItemComponent = ({ icon, children, ...rest }: NavItemProps) => {
  return (
    <NextLink href={rest.href ? rest.href : "#"} passHref>
      <Link
        href="#"
        as={"div"}
        style={{ textDecoration: "none" }}
        _focus={{ boxShadow: "none" }}
      >
        <Flex
          align="center"
          p="4"
          mx="4"
          borderRadius="lg"
          role="group"
          cursor="pointer"
          _hover={{
            bg: "cyan.400",
            color: "white",
          }}
          {...rest}
        >
          {icon && (
            <Icon
              mr="4"
              fontSize="16"
              _groupHover={{
                color: "white",
              }}
              as={icon}
            />
          )}
          {children}
        </Flex>
      </Link>
    </NextLink>
  );
};

export const NavItem = React.memo(NavItemComponent);
