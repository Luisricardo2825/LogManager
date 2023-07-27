import {
  Flex,
  Box,
  FormControl,
  FormLabel,
  Input,
  Stack,
  Button,
  Heading,
  Text,
  useColorModeValue,
  useToast,
} from "@chakra-ui/react";
import React from "react";
import type { LogParams } from "../redux/@types/LogParams";
import { useDispatch } from "react-redux";
import { FiPlus } from "react-icons/fi";
import { invoke } from "@tauri-apps/api/tauri";

function Form(props) {
  return (
    <Stack spacing={4}>
      <FormControl id="env">
        <FormLabel>Nome ambiente</FormLabel>
        <Input
          type="text"
          onChange={(e) =>
            props.handleOnchange({
              enviromentName: e.currentTarget.value,
            })
          }
        />
      </FormControl>
      <FormControl id="user">
        <FormLabel>Usuário</FormLabel>
        <Input
          type="user"
          onChange={(e) =>
            props.handleOnchange({
              accessData: {
                ...props.userData?.accessData,
                username: e.currentTarget.value,
              },
            })
          }
        />
      </FormControl>
      <FormControl id="pass">
        <FormLabel>Senha</FormLabel>
        <Input
          type="password"
          onChange={(e) =>
            props.handleOnchange({
              accessData: {
                ...props.userData?.accessData,
                password: e.currentTarget.value,
              },
            })
          }
        />
      </FormControl>
      <FormControl id="url">
        <FormLabel>Url da base</FormLabel>
        <Input
          type="url"
          onChange={(e) =>
            props.handleOnchange({
              url: e.currentTarget.value,
            })
          }
        />
      </FormControl>
      <Stack spacing={10}>
        <Button
          leftIcon={<FiPlus />}
          bg={"blue.400"}
          color={"white"}
          _hover={{
            bg: "blue.500",
          }}
          onClick={props.handleOnClick}
        >
          Adicionar
        </Button>
      </Stack>
    </Stack>
  );
}

export default function Login() {
  const [userData, setUsedata] = React.useState<LogParams>();
  const dispatch = useDispatch();
  const toast = useToast();
  const handleOnchange = (object: LogParams) => {
    setUsedata((prev) => ({ ...prev, ...object }));
  };

  const handleOnClick = async () => {
    try {
      const url = new URL(userData.url);
      console.log(url.origin)
      const payload: LogParams = {
        ...userData,
        accessData: {
          username: userData.accessData.username,
          password: userData.accessData.password || "",
        },
      };
      const [moduleId, btnId] = await invoke<string[]>("post_mod", {
        ...payload,
      });
      setUsedata({ ...userData, url: url.origin });
      dispatch({
        type: "AddEnviroment",
        payload: { ...payload, moduleId, btnId },
      });

      toast({
        title: "Adicionado.",
        description: "Adicionado com sucesso!",
        status: "success",
        duration: 5000,
        isClosable: true,
        position: "top",
      });
    } catch (e) {
      const json = JSON.parse(e || "{}");
      const retBody = JSON.parse(json?.retBody || "{}");
      toast({
        title: "Erro",
        description: "" + retBody?.statusMessage +e,
        status: "error",
        duration: 5000,
        isClosable: true,
        position: "top",
      });
    }
  };
  return (
    <Flex
      minH={"100%"}
      align={"center"}
      justify={"center"}
      bg={useColorModeValue("gray.50", "gray.800")}
    >
      <Stack spacing={5} mx={"auto"} maxW={"lg"} px={6}>
        <Stack align={"center"}>
          <Heading fontSize={"4xl"}>Faça o login da conta Sup</Heading>
          <Text fontSize={"lg"} color={"gray.600"}>
            ou de uma que possua permissão para gerar logs
          </Text>
        </Stack>
        <Box
          rounded={"lg"}
          bg={useColorModeValue("white", "gray.700")}
          boxShadow={"lg"}
          px={8}
          py={5}
        >
          <Form
            userData={userData}
            handleOnchange={handleOnchange}
            handleOnClick={handleOnClick}
          ></Form>
        </Box>
      </Stack>
    </Flex>
  );
}
