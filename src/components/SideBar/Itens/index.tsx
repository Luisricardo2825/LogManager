import React from "react";
import { Box } from "@chakra-ui/react";
import { NavItem } from "./NavItem";
import { LinkItems } from "./LinkItems";

function NavItensComponent(props) {
  return (
    <Box>
      {LinkItems.map((link) => {
        let { LeftElement, Wrapper } = link;
        if (!Wrapper) Wrapper = ({ children }) => <div>{children}</div>;
        if (!LeftElement) LeftElement = (props) => <div></div>;

        return (
          <div key={link.name}>
            <LeftElement {...props} />
            <Wrapper {...props}>
              <NavItem
                icon={link.icon}
                href={link.href}
                onClick={(e) => {
                  if (link.onClick) link.onClick(e, props);
                }}
              >
                {link.name}
              </NavItem>
            </Wrapper>
          </div>
        );
      })}
    </Box>
  );
}
export const NavItens = React.memo(NavItensComponent);
