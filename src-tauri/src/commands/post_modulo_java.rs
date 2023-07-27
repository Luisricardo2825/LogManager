use std::fs;

use crate::{
    auth::logout::logout,
    schemas::{
        login_schema::AccessData, post_add_btn::PostAddBtn,
        post_cria_modulo_schema::PostCriaModuloSchema,
    },
    utils::{replace_param::replace_param, string_utils::get_json},
};

use upjar::{
    auth::login::LoginRet,
    commands::post_modulo_java::{post_modulo_java, PostModuleRet},
    schemas::builder_config::BuilderConfig,
};

/// Retorna N linhas do conteudo do arquivo de log encontrado
///
/// ### Argumentos
/// * `url` - Uma String representando a url de acesso
/// *  `access_data` - Uma struct do tipo AccessData, relacionada as informações de login
/// ```
#[tauri::command]
pub async fn post_mod(url: String, access_data: AccessData) -> Result<(String, String), String> {
    match action(url, access_data).await {
        Ok(data) => Ok(data),
        Err(e) => Err(e),
    }
}

async fn action(url: String, access_data: AccessData) -> Result<(String, String), String> {
    let lib_dir = fs::read_dir("..\\lib").unwrap().last().unwrap();
    let last_file_path = lib_dir.unwrap().path().display().to_string();

    let config = BuilderConfig {
        jar_file_path: last_file_path.to_owned(),
        user: access_data.username,
        password: access_data.password,
        resource_desc: "Lib para queries".to_owned(),
        resource_id: "br.com.utilityLib.sql".to_owned(),
        url: url.clone(),
    };
    let original_content = post_modulo_java(&config).await; // Converte os dados obtidos para uma string UTF-8

    if original_content.is_err() {
        let error = original_content.err().unwrap();
        Err(serde_json::to_string(&error)
            .expect("{\"message\":\"Error converting value of 'original_content'\"}"))
    } else {
        let PostModuleRet {
            module_id,
            login_data,
        } = original_content.ok().unwrap();
        let LoginRet { root, client } = login_data;
        let jsession_token = String::from(root.response_body.jsessionid.field); // Pega o jsession ID
        let last_char = url.chars().last().unwrap();

        let btn_result =
            post_add_btn(&config, &client, jsession_token, last_char, &module_id).await;
        if btn_result.is_ok() {
            let btn_id = btn_result.unwrap();
            return Ok((module_id, btn_id));
        };
        logout(client, &url)
            .await
            .expect("{\"message\":\"Can't logout from Sankhya\"}");

        return Err(
            serde_json::to_string(&"{\"message\":\"Can't get any data\"}")
                .expect("{\"message\":\"Can't retrieve result error message\"}"),
        );
    }
}

async fn post_add_btn(
    config: &BuilderConfig,
    client: &reqwest::Client,
    jsession_token: String,
    last_char: char,
    cod_module: &String,
) -> Result<String, String> {
    let url = (&config).url.to_owned();
    let desc = "Query tool".to_owned();

    let mut json = get_json("./jsons/postBtnJava.json");
    json = replace_param(&json, "codModulo", cod_module.to_owned());
    json = replace_param(&json, "descBtn", desc.clone());

    if let Some(value) = get_existing_btn(
        url.clone(),
        (&jsession_token).to_owned(),
        last_char,
        &client,
        desc,
    )
    .await
    {
        return value;
    }
    let endpoint = "mge/service.sbr?serviceName=DatasetSP.save&outputType=json&mgeSession=";

    let mut post_url = format!("{}/{}{}", &url, &endpoint, &jsession_token);

    if last_char.eq(&'/') {
        // Formata a url para usar o token
        post_url = format!("{}{}{}", &url, &endpoint, &jsession_token);
    }

    let response = client
        .post(post_url)
        .body(json)
        .send()
        .await
        .expect("{\"message\":\"Erro sending request\"}");
    let resp_btn = response
        .text()
        .await
        .expect("{\"message\":\"Error getting btn json\"}");
    let json_btn_parsed: PostAddBtn = serde_json::from_str(&resp_btn).unwrap();

    let response = json_btn_parsed.response_body.to_owned();
    let result = response.result.get(0).unwrap().get(0).unwrap().to_owned();
    Ok(result)
}

async fn get_existing_btn(
    url: String,
    jsession_token: String,
    last_char: char,
    client: &reqwest::Client,
    desc: String,
) -> Option<Result<String, String>> {
    let mut json = get_json("./jsons/getBtn.json");
    json = replace_param(&json, "descBtn", desc);

    let endpoint_modulo =
        "mge/service.sbr?serviceName=DatasetSP.loadRecords&outputType=json&mgeSession=";

    let mut get_url = format!("{}/{}{}", &url, &endpoint_modulo, &jsession_token);

    if last_char.eq(&'/') {
        // Formata a url para usar o token
        get_url = format!("{}{}{}", &url, &endpoint_modulo, &jsession_token);
    }

    let module = client
        .post(get_url)
        .body(json)
        .send()
        .await
        .expect("{\"message\":\"Error sending request\"}");

    let resp_module = module
        .text()
        .await
        .expect("{\"message\":\"Error during conversion\"}");

    let json_module_parsed: PostCriaModuloSchema = serde_json::from_str(&resp_module).unwrap();
    let response = (&json_module_parsed).response_body.as_ref().unwrap();
    let result = response.result.get(0);
    if result.is_some() {
        let ret = result.unwrap().get(0).unwrap().to_owned();
        return Some(Ok(ret));
    }
    return None;
}
