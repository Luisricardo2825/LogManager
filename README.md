# 📄Log Manager [![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/)

Ferramenta para visualização de logs do ERP/EIP Sankhya de forma simplificada.

## Instalação

Faça o dowload de uma das [releases](https://github.com/Luisricardo2825/LogManager/releases) e realiza a instalção do binario de acordo com seu sistema operacional.

## Uso/Exemplos

Após o download e instalação de uma das opçoes dispostas em [releases](https://github.com/Luisricardo2825/LogManager/releases).

Execute o programa e dirija-se até a tela "Adicionar Jar":
[]("https://github.com/Luisricardo2825/LogManager/tree/master/images/add_env.png")

```shell
upjar <caminho>
```

## Parametros do arquivo de configuração

| Parâmetro      | Tipo     | Descrição                                          |
| :------------- | :------- | :------------------------------------------------- |
| `url`          | `string` | **Obrigatório**. Url da base                       |
| `user`         | `string` | **Obrigatório**. Usuário para login                |
| `password`     | `string` | **Obrigatório**. Senha para login                  |
| `jarFilePath`  | `string` | **Obrigatório**. Caminho do jar a ser enviado      |
| `resourceId`   | `string` | **Obrigatório**. identificador do modulo           |
| `resourceDesc` | `string` | **Obrigatório**. Descrição do modulo a ser criado. |

## Autores

- Ricardo Alves ([@Luisricardo2825](https://github.com/Luisricardo2825))
