use crate::{
    auth::login::{login, LoginRet},
    schemas::{get_log_btn_schema::GetLogBtnSchema, login_schema::AccessData},
    utils::{replace_param::replace_param, string_utils::get_json},
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetLogResponse {
    message: String,
    created: bool,
}

/// Retorna N linhas do conteudo do arquivo de log encontrado
///
/// ### Argumentos
/// * `url` - Uma String representando a url de acesso
/// *  `access_data` - Uma struct do tipo AccessData, relacionada as informações de login
/// ```
#[tauri::command]
pub async fn get_log_btn(
    url: String,
    access_data: AccessData,
    total: i32,
    btn_id: String,
) -> Result<String, String> {
    match action(url, access_data, total, btn_id).await {
        Ok(data) => Ok(data),
        Err(e) => Err(e),
    }
}

async fn action(
    url: String,
    access_data: AccessData,
    total: i32,
    btn_id: String,
) -> Result<String, String> {
    let lines_per_file = if total <= 0 { 10000 } else { total }; // Limita a quantidade de linhas de retorno
    let login_resp = match login(url.clone(), access_data.clone()).await {
        Ok(a) => Ok(a),
        Err(e) => Err(e),
    };
    if login_resp.is_err() {
        let error = login_resp.unwrap_err();
        Err(serde_json::to_string(&error).expect("Error Converting"))
    } else {
        let login_data = login_resp.ok().unwrap();
        let original_content = get_log(url.clone(), login_data, btn_id).await; // Converte os dados obtidos para uma string UTF-8

        if original_content.is_err() {
            Err(serde_json::to_string(&"Erro").expect("Error Converting"))
        } else {
            let result = original_content.ok().unwrap();
            let lines = result.lines().rev(); // Inverte as linhas para que sejam lidas de tras para frente

            let mut content = String::new();
            let mut count = 0;
            for line in lines {
                if count > lines_per_file {
                    break;
                }
                content = content + "\n" + line;
                count = count + 1;
            }

            let new_content = ((content.lines().rev()).collect::<Vec<&str>>()).join("\n"); // Reverte a string para o ordem natural que foi lida
            return Ok(new_content);
        }
    }
}

/// Retorna o log gerado pelo sankhya
///
/// ### Argumentos
/// * `url` - Uma String representando a url de acesso
/// *  `access_data` - Uma struct do tipo AccessData, relacionada as informações de login
/// ```
pub async fn get_log(url: String, login_data: LoginRet, btn_id: String) -> Result<String, String> {
    let LoginRet { root, client } = login_data; // pega os dados de login

    let jsession_token = String::from(root.response_body.jsessionid.field); // Pega o jsession ID
    let mut json = get_json("./jsons/getLogBtn.json"); // Pega o json modelo da pasta "jsons"
    json = replace_param(&json, "CODBTN", btn_id); // Cod botão que executa a ação
    let last_char = url.chars().last().unwrap();
    let mut post_url = format!(
        "{}/mge/service.sbr?serviceName=ActionButtonsSP.executeJava&outputType=json&mgeSession={}",
        &url, &jsession_token
    ); // Formata a url para usar o token
    if last_char.eq(&'/') {
        post_url = format!(
            "{}mge/service.sbr?serviceName=ActionButtonsSP.executeJava&outputType=json&mgeSession={}",
            &url, &jsession_token
        ); // Formata a url para usar o token
    }

    let response = client
        .post(post_url.clone())
        .body(json)
        .send()
        .await
        .expect("Erro sending request"); // Faz a requisição http

    let resp = response
        .text_with_charset("utf-8")
        .await
        .expect("{\"message\":\"Erro during conversion\"}"); // tenta converter o arquivo para json

    let json_parsed = serde_json::from_str(&resp); // transforma o json em uma estrutura

    if json_parsed.is_err() {
        return Err("Invalid response".to_owned());
    }
    let result:GetLogBtnSchema = json_parsed.unwrap();
    Ok(result.response_body.log) // retorna o nome do arquivo
}
