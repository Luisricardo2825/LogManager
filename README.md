# 📄Log Manager [![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/)

Ferramenta para visualização de logs do ERP/EIP Sankhya de forma simplificada.

## Instalação

Faça o dowload de uma das [releases](https://github.com/Luisricardo2825/LogManager/releases) e realiza a instalção do binario de acordo com seu sistema operacional.

## Uso/Exemplos

Após o download e instalação de uma das opçoes dispostas em [releases](https://github.com/Luisricardo2825/LogManager/releases).

#### Execute o programa e dirija-se até a tela "Adicionar Jar":
![](https://raw.githubusercontent.com/Luisricardo2825/LogManager/main/images/add_env_toast.png)

### Selecione o ambiente que deseja visualizar o log:
![](https://raw.githubusercontent.com/Luisricardo2825/LogManager/main/images/select_env.png)

### Selecione a opção `Log Server` e o log do ambiente selecionado será exibido:
![](https://raw.githubusercontent.com/Luisricardo2825/LogManager/main/images/log_server.png)

## Autores

- Ricardo Alves ([@Luisricardo2825](https://github.com/Luisricardo2825))

## FAQ

#### Como é realizado a consulta?

No momento em que o ambiente é adicionado, é criado na base um `modulo java` com o identificador `br.com.ricardo.utilityLib`, que possui o binario [UtilityLibJava.jar](https://github.com/Luisricardo2825/LogManager/blob/main/src-tauri/jsons/UtilityLibJava.jar). É criado também um botão de ação com a descrição `Utility lib`(Encontrado na instacia `ArquivoModulo`) a ser chamado pelo Manager via API, no qual é responsável por retornar o log do servidor.

#### Como o ambiente é armazenado?

O ambiente configurado/adicionado é salvo dentro do [localStorage](https://developer.mozilla.org/pt-BR/docs/Web/API/Window/localStorage), não sendo salvo em qualquer outro lugar ou arquivo, onde, caso o `localStorage` seja limpo, irá também remover todos os ambientes configurados.
