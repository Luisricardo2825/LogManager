import React from "react";
import { Confirm } from "../Confirm";

function DeleteModalComponent(props) {
  return (
    <Confirm
      body="Confirma a deleção do ambiente?"
      header="Confirma deleção"
      confirm={() => {
        props.dispatch({
          type: "RemoveEnviroment",
          payload: props.selectedEnviroment,
        });
        props.dispatch({
          type: "RemoveSelectedEnviroment",
          payload: {
            enviromentName: undefined,
            total: 0,
            accessData: {
              password: undefined,
              username: undefined,
            },
            url: undefined,
          },
        });
      }}
      isOpen={props.isOpen}
      onClose={props.customOnClose}
    ></Confirm>
  );
}

export const DeleteModal = React.memo(DeleteModalComponent);
