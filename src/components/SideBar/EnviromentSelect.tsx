import React from "react";
import { Box, Container, Flex, Select } from "@chakra-ui/react";

function EnviromentSelectComponent(props) {
  const currentValue = JSON.stringify(props.selectedEnviroment);
  const { selectedEnviroment, options } = props;
  return (
    <Select
      size="md"
      rounded={"full"}
      placeholder={"Select option"}
      value={currentValue}
      onChange={(item) => {
        if (item.currentTarget.value.trim().length > 0) {
          const value = JSON.parse(item.currentTarget.value);
          props.dispatch({
            type: "AddSelectedEnviroment",
            payload: value,
          });
        }
      }}
    >
      {selectedEnviroment?.enviromentName && (
        <Container
          as="option"
          minH={"200px"}
          value={JSON.stringify(selectedEnviroment)}
          key={"default"}
        >
          {selectedEnviroment.enviromentName}
        </Container>
      )}
      {options
        .filter((item) => item.label !== selectedEnviroment?.enviromentName)
        .map((item, idx) => {
          return (
            <Container
              as="option"
              bg={"red"}
              value={item.value}
              key={idx}
              borderRadius={"0"}
            >
              {item.label}
            </Container>
          );
        })}
    </Select>
  );
}
export const EnviromentSelect = React.memo(EnviromentSelectComponent);
