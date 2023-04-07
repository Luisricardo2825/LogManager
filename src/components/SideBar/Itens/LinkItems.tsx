import { FiDelete, FiPlus, FiServer } from "react-icons/fi";
import { DeleteModal } from "../DeleteModal";
import { LinkItemProps } from "../SideBar";


export const LinkItems: Array<LinkItemProps> = [
  {
    name: "Adicionar ambiente",
    icon: FiPlus,
    href: "/AddEnv",
  },
  {
    name: "Remover ambiente",
    icon: FiDelete,
    href: "#",
    onClick(e, props) {
      props.onOpen();
    },
    LeftElement: DeleteModal,
  },
  { name: "Log Server", icon: FiServer, href: "/" },
];
