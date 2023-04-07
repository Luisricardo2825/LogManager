import React, { ReactNode } from "react";
import {
  Box,
  useColorModeValue,
  useDisclosure,
  BoxProps,
  IconButton,
  Drawer,
  DrawerOverlay,
  DrawerContent,
  DrawerHeader,
  DrawerBody,
  DrawerCloseButton,
} from "@chakra-ui/react";
import { AiOutlineMenu } from "react-icons/ai";
import { IconType } from "react-icons";
import { useSelector, useDispatch } from "react-redux";
import { RootState } from "../../redux/store";
import { EnviromentSelect } from "./EnviromentSelect";
import { NavItens } from "./Itens";
import { Logo } from "./Logo";
import { Footer } from "./Footer";
export interface LinkItemProps {
  name: string;
  icon: IconType;
  href: string;
  onClick?(e: React.MouseEvent<HTMLDivElement, MouseEvent>, props: any): void;
  Wrapper?: (props: { children: any }) => JSX.Element;
  LeftElement?(props: any): JSX.Element;
}

function SideBar() {
  const state = useSelector<RootState, RootState>((state) => state);
  const { isOpen, onToggle, onClose } = useDisclosure();
  return (
    <Box overflow={"hidden"} bg={useColorModeValue("gray.100", "gray.900")}>
      <IconButton
        transition={"all 0.2s ease-in-out"}
        icon={<AiOutlineMenu size={20} />}
        aria-label={"Menu lateral"}
        onClick={onToggle}
        pos="fixed"
        bg={"transparent"}
        zIndex={"8"}
        variant={"solid"}
        rounded={"none"}
        height={"30"}
        top={0}
      />
      <Drawer
        placement={"left"}
        onClose={onClose}
        returnFocusOnClose={true}
        isOpen={isOpen}
      >
        <DrawerOverlay />
        <DrawerContent
          w={60}
          maxW={60}
          bg={useColorModeValue("gray.100", "gray.900")}
        >
          <DrawerCloseButton />
          <SidebarContent data={state} />
        </DrawerContent>
      </Drawer>
    </Box>
  );
}

export default React.memo(SideBar);
interface SidebarProps extends BoxProps {
  data: RootState;
}

const SidebarContent = React.memo(({ data, ...rest }: SidebarProps) => {
  const { isOpen, onOpen, onClose: customOnClose } = useDisclosure();
  const dispatch = useDispatch();
  const { enviroments, selectedEnviroment } = data;
  const options = enviroments.map((item) => {
    return {
      label: item.enviromentName,
      value: JSON.stringify(item),
    };
  });

  return (
    <Box {...rest}>
      <DrawerHeader borderBottomWidth="1px">
        <Logo />
      </DrawerHeader>
      <DrawerBody padding={0} as={"div"}>
        <EnviromentSelect
          dispatch={dispatch}
          selectedEnviroment={selectedEnviroment}
          options={options}
        />
        <NavItens
          isOpen={isOpen}
          onOpen={onOpen}
          customOnClose={customOnClose}
          dispatch={dispatch}
          selectedEnviroment={selectedEnviroment}
        />

        <Footer />
      </DrawerBody>
    </Box>
  );
});
